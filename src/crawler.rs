use futures::future::join_all;
use std::collections::HashSet;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use governor::{Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex, Notify, RwLock, Semaphore};
use tracing::{debug, warn};
use url::Url;

use crate::cache::{Cache, ResourceMeta};
use crate::config::Config;
use crate::extract::extract_page;
use crate::robots::RobotsManager;
use crate::security::{is_safe_image_content_type, sanitize_html_for_md, sanitize_markdown};
use crate::sink::{FileSink, Sink, Stats};
use crate::sitemap::fetch_and_parse_sitemaps;
use crate::util::{
    is_same_host, normalize_url, path_for_asset, path_for_url, relpath, site_name_from_url,
};
use regex::Regex;
use std::time::Duration;

pub struct CrawlConfig {
    pub base_url: Url,
    pub output_dir: PathBuf,
    pub user_agent: String,
    pub max_depth: Option<usize>,
    pub rate_limit_per_sec: u32,
    pub follow_sitemaps: bool,
    pub concurrency: usize,
    pub timeout: Option<Duration>,
    pub resume: bool,
    pub config: Config,
    pub silence: bool,
}

pub async fn crawl(cfg: CrawlConfig) -> Result<Stats, Box<dyn std::error::Error>> {
    let base_origin = origin_of(&cfg.base_url)?;
    std::fs::create_dir_all(&cfg.output_dir)?;

    let client = build_client(&cfg.user_agent)?;

    let limiter = Arc::new(RateLimiter::direct(Quota::per_second(
        NonZeroU32::new(cfg.rate_limit_per_sec).unwrap(),
    )));

    let mut robots_loaded = RobotsManager::new(cfg.user_agent.clone());
    robots_loaded.load_for(&client, &base_origin).await;
    let robots = Arc::new(Mutex::new(robots_loaded));

    let visited: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
    let cache = Cache::open(&cfg.output_dir.join(".docrawl_cache"))
        .ok()
        .map(Arc::new);

    // Prepare file sink by default
    let host_dir = cfg.output_dir.join(site_name_from_url(&base_origin));
    let sink = Arc::new(FileSink::new(host_dir.clone()));

    // Optional sitemap seeding (collected first; scheduled below)
    let mut seeds: Vec<(Url, usize)> = vec![(cfg.base_url.clone(), 0)];
    if cfg.follow_sitemaps {
        let sitemap_urls = fetch_and_parse_sitemaps(&client, &base_origin).await;
        for u in sitemap_urls {
            if within_scope(&base_origin, &u, &cfg.config) {
                seeds.push((u, 0));
            }
        }
    }

    // Precompile exclude patterns
    let exclude_res: Vec<Regex> = cfg
        .config
        .exclude_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();
    let seeds_count = seeds.len();

    let saved_pages = Arc::new(AtomicUsize::new(0));
    let saved_assets = Arc::new(AtomicUsize::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Create single progress bar with better formatting unless silenced
    let pb = if cfg.silence {
        None
    } else {
        let p = ProgressBar::new_spinner();
        p.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{elapsed_precise}] {msg} | Pages: {pos} | {per_sec}",
            )
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        p.enable_steady_tick(std::time::Duration::from_millis(80));
        Some(p)
    };

    let pb_shared = pb.clone();

    // Start time for calculating crawl speed
    let start_time = Instant::now();

    // Shared state
    let exclude_res = Arc::new(exclude_res);
    let base_arc = Arc::new(base_origin);
    let output_dir = cfg.output_dir.clone();
    let max_depth = cfg.max_depth;
    let config = cfg.config.clone();
    let user_client = client.clone();
    let semaphore = Arc::new(Semaphore::new(cfg.concurrency));
    let pending = Arc::new(AtomicUsize::new(0));
    let notify = Arc::new(Notify::new());
    // Set up timeout if provided (after notify creation)
    if let Some(dur) = cfg.timeout {
        let stop_flag_timeout = stop_flag.clone();
        let notify_timeout = notify.clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            stop_flag_timeout.store(true, Ordering::SeqCst);
            notify_timeout.notify_waiters();
        });
    }

    // Channel for discovered links from workers
    let (tx, mut rx) = mpsc::unbounded_channel::<(Url, usize)>();

    // Enqueue helper
    let enqueue = {
        let visited = visited.clone();
        let cache = cache.clone();
        let exclude_res = exclude_res.clone();
        let base = base_arc.clone();
        let robots = robots.clone();
        let limiter = limiter.clone();
        let client = user_client.clone();
        let sink = sink.clone();
        let output_dir = output_dir.clone();
        let saved_pages = saved_pages.clone();
        let saved_assets = saved_assets.clone();
        let semaphore = semaphore.clone();
        let pending_ctr = pending.clone();
        let notify = notify.clone();
        let config = config.clone();
        let tx_outer = tx.clone();
        move |url: Url, depth: usize| {
            debug!("Enqueue called for {} at depth {}", url, depth);
            // Depth/scope quick check
            if let Some(maxd) = max_depth {
                if depth > maxd {
                    debug!("Skipping {}: depth {} > max {}", url, depth, maxd);
                    return;
                }
            }
            if !within_scope(&base, &url, &config) {
                debug!("Skipping {}: out of scope", url);
                return;
            }
            if stop_flag.load(Ordering::SeqCst) {
                debug!("Skipping {}: stop flag set", url);
                return;
            }

            pending_ctr.fetch_add(1, Ordering::SeqCst);
            let visited = visited.clone();
            let cache = cache.clone();
            let exclude_res = exclude_res.clone();
            let base = base.clone();
            let robots = robots.clone();
            let limiter = limiter.clone();
            let client = client.clone();
            let sink = sink.clone();
            let output_dir = output_dir.clone();
            let saved_pages = saved_pages.clone();
            let saved_assets = saved_assets.clone();
            let semaphore = semaphore.clone();
            let pending_ctr = pending_ctr.clone();
            let notify = notify.clone();
            let config = config.clone();
            let pb_shared = pb_shared.clone();
            let stop_flag_task = stop_flag.clone();
            let tx = tx_outer.clone();
            // Persist to frontier before spawn
            if let Some(c) = &cache {
                let canon = normalize_url(&url);
                let _ = c.add_frontier(&canon, depth);
            }
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                // Deduplicate
                let canon = normalize_url(&url);
                debug!("Processing {} (canonical: {})", url, canon);
                if let Some(c) = &cache {
                    let _ = c.remove_frontier(&canon);
                }
                {
                    let mut vis = visited.write().await;
                    if !vis.insert(canon.clone()) {
                        debug!("Already visited: {}", canon);
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 {
                            notify.notify_waiters();
                        }
                        return;
                    }
                }
                if let Some(c) = &cache {
                    let cache_result = c.check_and_mark_visited(&canon);
                    debug!("Cache check for {}: {:?}", canon, cache_result);
                    if !cache_result.unwrap_or(true) {
                        debug!("Already in cache, skipping: {}", canon);
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 {
                            notify.notify_waiters();
                        }
                        return;
                    }
                }
                // Exclude
                if exclude_res.iter().any(|re| re.is_match(canon.as_str())) {
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 {
                        notify.notify_waiters();
                    }
                    return;
                }
                // robots
                {
                    let rb = robots.lock().await;
                    if !rb.allowed(&base, url.path()) {
                        debug!("Blocked by robots.txt: {}", url);
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 {
                            notify.notify_waiters();
                        }
                        return;
                    }
                }

                // Prepare conditional headers from cache meta
                let mut req = client.get(url.clone());
                let cached_meta = cache
                    .as_ref()
                    .and_then(|c| c.get_meta(&canon).ok())
                    .flatten();
                if let Some(m) = &cached_meta {
                    if let Some(etag) = &m.etag {
                        req = req.header(reqwest::header::IF_NONE_MATCH, etag);
                    }
                    if let Some(lm) = &m.last_modified {
                        req = req.header(reqwest::header::IF_MODIFIED_SINCE, lm);
                    }
                }

                // Fetch
                // Keep position updated but avoid per-task messages to reduce flicker
                if let Some(pb) = &pb_shared {
                    pb.set_position(saved_pages.load(Ordering::SeqCst) as u64);
                }
                limiter.until_ready().await;
                let resp = match req.send().await {
                    Ok(r) => r,
                    Err(e) => {
                        warn!(error=%e, "request failed");
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 {
                            notify.notify_waiters();
                        }
                        return;
                    }
                };
                if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
                    debug!("Not modified: {}", url);
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 {
                        notify.notify_waiters();
                    }
                    return;
                }
                let resp = match resp.error_for_status() {
                    Ok(r) => r,
                    Err(e) => {
                        warn!(error=%e, "bad status");
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 {
                            notify.notify_waiters();
                        }
                        return;
                    }
                };

                let final_url = resp.url().clone();
                let content_type = resp
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_lowercase();
                if !content_type.contains("text/html")
                    && !content_type.contains("application/xhtml")
                {
                    debug!("Skipping non-HTML: {} [{}]", final_url, content_type);
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 {
                        notify.notify_waiters();
                    }
                    return;
                }
                let headers = resp.headers().clone();
                let body = match resp.text().await {
                    Ok(t) => t,
                    Err(e) => {
                        warn!(error=%e, "read body failed");
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 {
                            notify.notify_waiters();
                        }
                        return;
                    }
                };

                // Save new conditional headers
                if let Some(c) = &cache {
                    let meta = ResourceMeta {
                        etag: headers
                            .get(reqwest::header::ETAG)
                            .and_then(|v| v.to_str().ok())
                            .map(|s| s.to_string()),
                        last_modified: headers
                            .get(reqwest::header::LAST_MODIFIED)
                            .and_then(|v| v.to_str().ok())
                            .map(|s| s.to_string()),
                    };
                    let _ = c.set_meta(&canon, &meta);
                }

                let page = extract_page(&final_url, &body, config.selectors.as_deref());
                let sanitized_html = sanitize_html_for_md(&final_url, &page.main_html);
                let mut md = html2md::rewrite_html(&sanitized_html, false);
                let (md_checked, security_flags) = sanitize_markdown(&md);
                md = md_checked;
                let out_path = path_for_url(&output_dir, &base, &final_url);

                let quarantined = !security_flags.is_empty();
                if !quarantined && !config.skip_assets {
                    // Download images in parallel without rate limiting
                    let mut replace_pairs: Vec<(String, String)> = vec![];
                    let mut asset_futures = vec![];

                    for img in page.images.iter() {
                        if !config.external_assets && !within_scope(&base, img, &config) {
                            continue;
                        }
                        let asset_path = path_for_asset(&output_dir, &base, img);
                        let img_url = img.clone();
                        let client = client.clone();
                        let config = config.clone();

                        // Spawn parallel asset fetches without rate limiting
                        let fut = async move {
                            match fetch_asset_checked(&client, img_url.clone(), config.allow_svg)
                                .await
                            {
                                Ok((ct, bytes)) => Some((img_url, asset_path.clone(), ct, bytes)),
                                Err(e) => {
                                    debug!(error=%e, "asset download failed: {}", img_url);
                                    None
                                }
                            }
                        };
                        asset_futures.push(fut);
                    }

                    // Wait for all assets to download in parallel
                    let asset_results = join_all(asset_futures).await;

                    for (img_url, asset_path, ct, bytes) in asset_results.into_iter().flatten() {
                        if let Err(e) = sink
                            .save_asset(&img_url, &asset_path, ct.as_deref(), bytes)
                            .await
                        {
                            debug!(error=%e, "asset save failed: {}", img_url);
                        } else {
                            saved_assets.fetch_add(1, Ordering::SeqCst);
                        }

                        if let Some(rel) = relpath(&out_path, &asset_path) {
                            replace_pairs.push((
                                img_url.as_str().to_string(),
                                rel.to_string_lossy().to_string(),
                            ));
                        }
                    }

                    if !replace_pairs.is_empty() {
                        md = rewrite_md_images(md, &replace_pairs);
                    }
                }

                let body_to_write = if quarantined {
                    let reasons = security_flags
                        .iter()
                        .map(|f| format!("- {}", f))
                        .collect::<Vec<_>>()
                        .join("\n");
                    format!(
                        "Content skipped by docrawl due to detected security issues.\n\nDetected flags:\n{}\n\nReason: One or more patterns indicating potential prompt-injection, risky content, or unsafe assets were found. To protect downstream consumers, docrawl omitted page content.",
                        reasons
                    )
                } else {
                    md
                };

                if let Err(e) = sink
                    .save_page(
                        &out_path,
                        &page.title,
                        &final_url,
                        &body_to_write,
                        &security_flags,
                        quarantined,
                    )
                    .await
                {
                    warn!(error=%e, "save page failed");
                }
                let new_total = saved_pages.fetch_add(1, Ordering::SeqCst) + 1;
                if let Some(max) = config.max_pages {
                    if new_total >= max {
                        stop_flag_task.store(true, Ordering::SeqCst);
                    }
                }

                // Update progress with better path display
                let pages_count = saved_pages.load(Ordering::SeqCst);
                if let Some(pb) = &pb_shared {
                    pb.set_position(pages_count as u64);
                }

                // Avoid per-task "Saved" messages; main loop shows overall crawling status

                // Use debug instead of info to avoid interfering with progress bar
                debug!("Saved {}", out_path.display());

                // Enqueue links
                let next_depth = depth.saturating_add(1);
                debug!("Found {} links on {}", page.links.len(), final_url);
                if !stop_flag_task.load(Ordering::SeqCst) {
                    for link in page.links {
                        debug!("Sending link to queue: {} at depth {}", link, next_depth);
                        let _ = tx.send((link, next_depth));
                    }
                }

                let remaining = pending_ctr.fetch_sub(1, Ordering::SeqCst) - 1;
                debug!("Task completed. Pending tasks: {}", remaining);
                if remaining == 0 {
                    debug!("No more pending tasks, notifying waiters");
                    notify.notify_waiters();
                }
            });
        }
    };

    // Seed work
    if let Some(c) = &cache {
        if cfg.resume {
            if let Ok(list) = c.list_frontier() {
                if !list.is_empty() {
                    for (u, d) in list {
                        if let Ok(url) = Url::parse(&u) {
                            enqueue(url, d);
                        }
                    }
                } else {
                    debug!("Enqueueing {} seed URLs", seeds.len());
                    for (u, d) in seeds {
                        debug!("Seed URL: {} at depth {}", u, d);
                        enqueue(u, d);
                    }
                }
            } else {
                debug!("Enqueueing {} seed URLs", seeds.len());
                for (u, d) in seeds {
                    debug!("Seed URL: {} at depth {}", u, d);
                    enqueue(u, d);
                }
            }
        } else {
            let _ = c.clear_frontier();
            debug!("Enqueueing {} seed URLs", seeds.len());
            for (u, d) in seeds {
                debug!("Seed URL: {} at depth {}", u, d);
                enqueue(u, d);
            }
        }
    } else {
        debug!("Enqueueing {} seed URLs", seeds.len());
        for (u, d) in seeds {
            debug!("Seed URL: {} at depth {}", u, d);
            enqueue(u, d);
        }
    }

    let pb_main = pb.clone();
    if let Some(pb) = &pb_main {
        pb.set_message(format!(
            "Starting crawl of {}",
            base_arc.host_str().unwrap_or("site")
        ));
    }

    // Track if we've processed any URLs yet
    let mut has_processed_any = false;
    let follow_sitemaps = cfg.follow_sitemaps;

    // Wait for all tasks; process discovered links as they arrive
    loop {
        let pending_count = pending.load(Ordering::SeqCst);
        let total_processed =
            saved_pages.load(Ordering::SeqCst) + saved_assets.load(Ordering::SeqCst);
        debug!(
            "Main loop: pending tasks = {}, total processed = {}",
            pending_count, total_processed
        );

        // Update pending count in UI
        if pending_count > 0 {
            if let Some(pb) = &pb_main {
                pb.set_message(format!(
                    "Crawling {} | Pending: {}",
                    base_arc.host_str().unwrap_or("site"),
                    pending_count
                ));
            }
            has_processed_any = true;
        }

        // Only exit if pending is 0 AND we've either processed something or waited a bit
        if pending_count == 0 {
            // Check if there are any messages in the channel before exiting
            match rx.try_recv() {
                Ok((u, d)) => {
                    debug!("Found pending message in channel: {} at depth {}", u, d);
                    enqueue(u, d);
                    continue;
                }
                Err(_) => {
                    // Give a grace period for initial tasks to start, especially in sitemap mode
                    if !has_processed_any && total_processed == 0 {
                        debug!("No tasks started yet, waiting for initial processing...");
                        // Much longer wait for sitemap mode with many seeds
                        let wait_time = if follow_sitemaps && seeds_count > 10 {
                            1000 + (seeds_count * 10).min(2000) // 1-3 seconds based on seed count
                        } else if follow_sitemaps {
                            500
                        } else {
                            100
                        };
                        debug!(
                            "Waiting {}ms for {} seeds to start processing",
                            wait_time, seeds_count
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(wait_time as u64))
                            .await;
                        // Check one more time
                        if pending.load(Ordering::SeqCst) > 0 {
                            continue;
                        }
                    }
                    debug!("No pending tasks and no messages in channel, exiting");
                    break;
                }
            }
        }

        tokio::select! {
            _ = notify.notified() => { /* check pending again */ },
            maybe = rx.recv() => {
                if let Some((u,d)) = maybe {
                    debug!("Received link from channel: {} at depth {}", u, d);
                    enqueue(u,d);
                }
            }
        }
    }

    // Show final statistics
    let total_pages = saved_pages.load(Ordering::SeqCst);
    let total_assets = saved_assets.load(Ordering::SeqCst);
    let elapsed = start_time.elapsed();

    if let Some(pb) = pb {
        pb.finish_with_message(format!(
            "Done! Crawled {} pages, {} assets in {:.1}s",
            total_pages,
            total_assets,
            elapsed.as_secs_f32()
        ));
    }
    let _ = sink.finalize().await;
    Ok(Stats {
        pages: saved_pages.load(Ordering::SeqCst),
        assets: saved_assets.load(Ordering::SeqCst),
    })
}

fn origin_of(u: &Url) -> Result<Url, Box<dyn std::error::Error>> {
    let mut o = u.clone();
    o.set_path("");
    o.set_query(None);
    o.set_fragment(None);
    Ok(o)
}

fn build_client(
    user_agent: &str,
) -> Result<reqwest_middleware::ClientWithMiddleware, Box<dyn std::error::Error>> {
    let mut builder = reqwest::Client::builder()
        .user_agent(user_agent.to_string())
        .redirect(reqwest::redirect::Policy::limited(10))
        .cookie_store(true)
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .tcp_keepalive(std::time::Duration::from_secs(30))
        .http2_adaptive_window(true);
    if std::env::var_os("HTTP_PROXY").is_none() && std::env::var_os("HTTPS_PROXY").is_none() {
        builder = builder.no_proxy();
    }
    let base = builder.build()?;

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(base)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    Ok(client)
}

async fn fetch_asset_checked(
    client: &reqwest_middleware::ClientWithMiddleware,
    url: Url,
    allow_svg: bool,
) -> Result<(Option<String>, bytes::Bytes), Box<dyn std::error::Error + Send + Sync>> {
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        return Err(format!("non-success: {}", resp.status()).into());
    }
    let ct_str = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    if ct_str
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("image/svg+xml"))
        .unwrap_or(false)
        && !allow_svg
    {
        return Err("svg not allowed".into());
    }
    if !(is_safe_image_content_type(ct_str.as_deref())
        || (allow_svg && ct_str.as_deref() == Some("image/svg+xml")))
    {
        return Err("unsupported image content-type".into());
    }
    let bytes = resp.bytes().await?;
    Ok((ct_str, bytes))
}

fn within_scope(base_origin: &Url, target: &Url, cfg: &Config) -> bool {
    if cfg.host_only {
        base_origin.scheme() == target.scheme()
            && base_origin.host_str() == target.host_str()
            && base_origin.port_or_known_default() == target.port_or_known_default()
    } else {
        is_same_host(base_origin, target)
    }
}

fn rewrite_md_images(mut md: String, pairs: &[(String, String)]) -> String {
    for (src, rel) in pairs {
        let pattern = regex::escape(src);
        // Replace only in image markdown patterns: ![...](src)
        let re = Regex::new(&format!(r"!\[[^\]]*\]\({}\)", pattern)).unwrap();
        md = re
            .replace_all(&md, |caps: &regex::Captures| {
                let m = caps.get(0).unwrap().as_str();
                // Replace the URL inside parentheses
                m.replacen(src, rel, 1)
            })
            .to_string();
    }
    md
}
