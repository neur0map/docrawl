use std::collections::HashSet;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}};

use governor::{Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff; 
use reqwest_retry::RetryTransientMiddleware;
use tracing::{debug, info, warn};
use url::Url;
use tokio::sync::{Semaphore, RwLock, Mutex, Notify, mpsc};

use crate::extract::extract_page;
use crate::config::Config;
use crate::cache::{Cache, ResourceMeta};
use crate::sink::{Sink, FileSink, Stats};
use crate::robots::RobotsManager;
use crate::security::{is_safe_image_content_type, sanitize_html_for_md, sanitize_markdown};
use crate::sitemap::fetch_and_parse_sitemaps;
use crate::util::{is_same_host, site_name_from_url, normalize_url, path_for_asset, path_for_url, relpath};
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
}

pub async fn crawl(cfg: CrawlConfig) -> Result<Stats, Box<dyn std::error::Error>> {
    let base_origin = origin_of(&cfg.base_url)?;
    std::fs::create_dir_all(&cfg.output_dir)?;

    let client = build_client(&cfg.user_agent)?;

    let limiter = Arc::new(RateLimiter::direct(Quota::per_second(NonZeroU32::new(cfg.rate_limit_per_sec).unwrap())));

    let mut robots_loaded = RobotsManager::new(cfg.user_agent.clone());
    robots_loaded.load_for(&client, &base_origin).await;
    let robots = Arc::new(Mutex::new(robots_loaded));

    let visited: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
    let cache = Cache::open(&cfg.output_dir.join(".docrawl_cache")).ok().map(Arc::new);

    // Prepare file sink by default
    let host_dir = cfg.output_dir.join(site_name_from_url(&base_origin));
    let sink = Arc::new(FileSink::new(host_dir.clone()));

    // Optional sitemap seeding (collected first; scheduled below)
    let mut seeds: Vec<(Url, usize)> = vec![(cfg.base_url.clone(), 0)];
    if cfg.follow_sitemaps {
        let sitemap_urls = fetch_and_parse_sitemaps(&client, &base_origin).await;
        for u in sitemap_urls { if within_scope(&base_origin, &u, &cfg.config) { seeds.push((u, 0)); } }
    }

    // Precompile exclude patterns
    let exclude_res: Vec<Regex> = cfg
        .config
        .exclude_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let saved_pages = Arc::new(AtomicUsize::new(0));
    let saved_assets = Arc::new(AtomicUsize::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    let pb_shared = pb.clone();


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
            if let Some(maxd) = max_depth { if depth > maxd { debug!("Skipping {}: depth {} > max {}", url, depth, maxd); return; } }
            if !within_scope(&base, &url, &config) { debug!("Skipping {}: out of scope", url); return; }
            if stop_flag.load(Ordering::SeqCst) { debug!("Skipping {}: stop flag set", url); return; }

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
                if let Some(c) = &cache { let _ = c.remove_frontier(&canon); }
                {
                    let mut vis = visited.write().await;
                    if !vis.insert(canon.clone()) {
                        debug!("Already visited: {}", canon);
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                        return;
                    }
                }
                if let Some(c) = &cache {
                    let cache_result = c.check_and_mark_visited(&canon);
                    debug!("Cache check for {}: {:?}", canon, cache_result);
                    if !cache_result.unwrap_or(true) {
                        debug!("Already in cache, skipping: {}", canon);
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                        return;
                    }
                }
                // Exclude
                if exclude_res.iter().any(|re| re.is_match(canon.as_str())) {
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                    return;
                }
                // robots
                {
                    let rb = robots.lock().await;
                    if !rb.allowed(&base, url.path()) {
                        debug!("Blocked by robots.txt: {}", url);
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                        return;
                    }
                }

                // Prepare conditional headers from cache meta
                let mut req = client.get(url.clone());
                let cached_meta = cache.as_ref().and_then(|c| c.get_meta(&canon).ok()).flatten();
                if let Some(m) = &cached_meta {
                    if let Some(etag) = &m.etag { req = req.header(reqwest::header::IF_NONE_MATCH, etag); }
                    if let Some(lm) = &m.last_modified { req = req.header(reqwest::header::IF_MODIFIED_SINCE, lm); }
                }

                // Fetch
                let depth_msg = depth;
                pb_shared.set_message(format!("Fetching {} (depth {})", url, depth_msg));
                limiter.until_ready().await;
                let resp = match req.send().await { Ok(r) => r, Err(e) => { warn!(error=%e, "request failed");
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                    return; } };
                if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
                    debug!("Not modified: {}", url);
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                    return;
                }
                let resp = match resp.error_for_status() { Ok(r) => r, Err(e) => { warn!(error=%e, "bad status");
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                    return; } };

                let final_url = resp.url().clone();
                let content_type = resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("").to_lowercase();
                if !content_type.contains("text/html") && !content_type.contains("application/xhtml") {
                    debug!("Skipping non-HTML: {} [{}]", final_url, content_type);
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                    return;
                }
                let headers = resp.headers().clone();
                let body = match resp.text().await { Ok(t) => t, Err(e) => { warn!(error=%e, "read body failed");
                    pending_ctr.fetch_sub(1, Ordering::SeqCst);
                    if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                    return; } };

                // Save new conditional headers
                if let Some(c) = &cache {
                    let meta = ResourceMeta {
                        etag: headers.get(reqwest::header::ETAG).and_then(|v| v.to_str().ok()).map(|s| s.to_string()),
                        last_modified: headers.get(reqwest::header::LAST_MODIFIED).and_then(|v| v.to_str().ok()).map(|s| s.to_string()),
                    };
                    let _ = c.set_meta(&canon, &meta);
                }

                let page = extract_page(&final_url, &body, config.selectors.as_deref());
                let sanitized_html = sanitize_html_for_md(&final_url, &page.main_html);
                let mut md = html2md::parse_html(&sanitized_html);
                let (md_checked, security_flags) = sanitize_markdown(&md);
                md = md_checked;
                let out_path = path_for_url(&output_dir, &base, &final_url);

                let quarantined = !security_flags.is_empty();
                if !quarantined && !config.skip_assets {
                    // Download images and rewrite
                    let mut replace_pairs: Vec<(String, String)> = vec![];
                    for img in page.images.iter() {
                        if !config.external_assets && !within_scope(&base, img, &config) { continue; }
                        let asset_path = path_for_asset(&output_dir, &base, img);
                        limiter.until_ready().await;
                        match fetch_asset_checked(&client, img.clone(), config.allow_svg).await {
                            Ok((ct, bytes)) => {
                                if let Err(e) = sink.save_asset(img, &asset_path, ct.as_deref(), bytes).await {
                                    warn!(error=%e, "asset save failed: {}", img);
                                } else {
                                    saved_assets.fetch_add(1, Ordering::SeqCst);
                                }
                            }
                            Err(e) => { warn!(error=%e, "asset download failed: {}", img); }
                        }
                        if let Some(rel) = relpath(&out_path, &asset_path) {
                            replace_pairs.push((img.as_str().to_string(), rel.to_string_lossy().to_string()));
                        }
                    }
                    if !replace_pairs.is_empty() { md = rewrite_md_images(md, &replace_pairs); }
                }

                let body_to_write = if quarantined {
                    let reasons = security_flags.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n");
                    format!(
                        "Content skipped by docrawl due to detected security issues.\n\nDetected flags:\n{}\n\nReason: One or more patterns indicating potential prompt-injection, risky content, or unsafe assets were found. To protect downstream consumers, docrawl omitted page content.",
                        reasons
                    )
                } else { md };

                if let Err(e) = sink.save_page(&out_path, &page.title, &final_url, &body_to_write, &security_flags, quarantined).await {
                    warn!(error=%e, "save page failed");
                }
                let new_total = saved_pages.fetch_add(1, Ordering::SeqCst) + 1;
                if let Some(max) = config.max_pages { if new_total >= max { stop_flag_task.store(true, Ordering::SeqCst); } }

                info!("Saved {}", out_path.display());

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
                    for (u, d) in list { if let Ok(url) = Url::parse(&u) { enqueue(url, d); } }
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

    // Wait for all tasks; process discovered links as they arrive
    loop {
        let pending_count = pending.load(Ordering::SeqCst);
        debug!("Main loop: pending tasks = {}", pending_count);

        if pending_count == 0 {
            // Check if there are any messages in the channel before exiting
            match rx.try_recv() {
                Ok((u, d)) => {
                    debug!("Found pending message in channel: {} at depth {}", u, d);
                    enqueue(u, d);
                    continue;
                }
                Err(_) => {
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

    pb.finish_and_clear();
    let _ = sink.finalize().await;
    Ok(Stats { pages: saved_pages.load(Ordering::SeqCst), assets: saved_assets.load(Ordering::SeqCst) })
}

fn origin_of(u: &Url) -> Result<Url, Box<dyn std::error::Error>> {
    let mut o = u.clone();
    o.set_path("");
    o.set_query(None);
    o.set_fragment(None);
    Ok(o)
}

fn build_client(user_agent: &str) -> Result<reqwest_middleware::ClientWithMiddleware, Box<dyn std::error::Error>> {
    let mut builder = reqwest::Client::builder()
        .user_agent(user_agent.to_string())
        .redirect(reqwest::redirect::Policy::limited(10))
        .cookie_store(true)
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .tcp_keepalive(std::time::Duration::from_secs(30))
        .http2_adaptive_window(true)
        ;
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
    if !resp.status().is_success() { return Err(format!("non-success: {}", resp.status()).into()); }
    let ct_str = resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    if ct_str.as_deref().map(|s| s.eq_ignore_ascii_case("image/svg+xml")).unwrap_or(false) && !allow_svg {
        return Err("svg not allowed".into());
    }
    if !is_safe_image_content_type(ct_str.as_deref()) && !(allow_svg && ct_str.as_deref() == Some("image/svg+xml")) {
        return Err("unsupported image content-type".into());
    }
    let bytes = resp.bytes().await?;
    Ok((ct_str, bytes))
}

fn within_scope(base_origin: &Url, target: &Url, cfg: &Config) -> bool {
    if cfg.host_only {
        base_origin.scheme() == target.scheme() && base_origin.host_str() == target.host_str() && base_origin.port_or_known_default() == target.port_or_known_default()
    } else {
        is_same_host(base_origin, target)
    }
}

fn rewrite_md_images(mut md: String, pairs: &[(String, String)]) -> String {
    for (src, rel) in pairs {
        let pattern = regex::escape(src);
        // Replace only in image markdown patterns: ![...](src)
        let re = Regex::new(&format!(r"!\[[^\]]*\]\({}\)", pattern)).unwrap();
        md = re.replace_all(&md, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap().as_str();
            // Replace the URL inside parentheses
            m.replacen(src, rel, 1)
        }).to_string();
    }
    md
}
