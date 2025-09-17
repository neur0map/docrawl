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
use crate::manifest::ManifestRecorder;
use crate::robots::RobotsManager;
use crate::save::write_markdown_with_frontmatter;
use crate::security::{is_safe_image_content_type, sanitize_html_for_md, sanitize_markdown};
use crate::sitemap::fetch_and_parse_sitemaps;
use crate::util::{is_same_host, site_name_from_url, normalize_url, path_for_asset, path_for_url, relpath};
use regex::Regex;

pub struct CrawlConfig {
    pub base_url: Url,
    pub output_dir: PathBuf,
    pub user_agent: String,
    pub max_depth: Option<usize>,
    pub rate_limit_per_sec: u32,
    pub follow_sitemaps: bool,
    pub concurrency: usize,
    pub config: Config,
}

pub async fn crawl(cfg: CrawlConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_origin = origin_of(&cfg.base_url)?;
    std::fs::create_dir_all(&cfg.output_dir)?;

    let client = build_client(&cfg.user_agent)?;

    let limiter = Arc::new(RateLimiter::direct(Quota::per_second(NonZeroU32::new(cfg.rate_limit_per_sec).unwrap())));

    let mut robots_loaded = RobotsManager::new(cfg.user_agent.clone());
    robots_loaded.load_for(&client, &base_origin).await;
    let robots = Arc::new(Mutex::new(robots_loaded));

    let visited: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
    let cache = Cache::open(&cfg.output_dir.join(".docrawl_cache")).ok().map(Arc::new);

    // Prepare manifest
    let host_dir = cfg.output_dir.join(site_name_from_url(&base_origin));
    let manifest = Arc::new(Mutex::new(ManifestRecorder::new(host_dir.clone())));

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
        let manifest = manifest.clone();
        let output_dir = output_dir.clone();
        let saved_pages = saved_pages.clone();
        let semaphore = semaphore.clone();
        let pending_ctr = pending.clone();
        let notify = notify.clone();
        let config = config.clone();
        let tx_outer = tx.clone();
        move |url: Url, depth: usize| {
            // Depth/scope quick check
            if let Some(maxd) = max_depth { if depth > maxd { return; } }
            if !within_scope(&base, &url, &config) { return; }
            if stop_flag.load(Ordering::SeqCst) { return; }

            pending_ctr.fetch_add(1, Ordering::SeqCst);
            let visited = visited.clone();
            let cache = cache.clone();
            let exclude_res = exclude_res.clone();
            let base = base.clone();
            let robots = robots.clone();
            let limiter = limiter.clone();
            let client = client.clone();
            let manifest = manifest.clone();
            let output_dir = output_dir.clone();
            let saved_pages = saved_pages.clone();
            let semaphore = semaphore.clone();
            let pending_ctr = pending_ctr.clone();
            let notify = notify.clone();
            let config = config.clone();
            let pb_shared = pb_shared.clone();
            let stop_flag_task = stop_flag.clone();
            let tx = tx_outer.clone();
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                // Deduplicate
                let canon = normalize_url(&url);
                {
                    let mut vis = visited.write().await;
                    if !vis.insert(canon.clone()) {
                        pending_ctr.fetch_sub(1, Ordering::SeqCst);
                        if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
                        return;
                    }
                }
                if let Some(c) = &cache {
                    if !c.check_and_mark_visited(&canon).unwrap_or(true) {
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
                if !quarantined {
                    // Download images and rewrite
                    let mut replace_pairs: Vec<(String, String)> = vec![];
                    for img in page.images.iter() {
                        if !config.external_assets && !within_scope(&base, img, &config) { continue; }
                        let asset_path = path_for_asset(&output_dir, &base, img);
                        limiter.until_ready().await;
                        if let Err(e) = download_asset(&client, img.clone(), &asset_path, config.allow_svg).await {
                            warn!(error=%e, "asset download failed: {}", img);
                            continue;
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

                if let Err(e) = write_markdown_with_frontmatter(&out_path, &page.title, final_url.as_str(), &body_to_write, &security_flags) {
                    warn!(error=%e, "write file failed");
                }

                {
                    let mut man = manifest.lock().await;
                    man.record(final_url.as_str(), &out_path, &page.title, quarantined, security_flags.clone());
                }
                let new_total = saved_pages.fetch_add(1, Ordering::SeqCst) + 1;
                if let Some(max) = config.max_pages { if new_total >= max { stop_flag_task.store(true, Ordering::SeqCst); } }

                info!("Saved {}", out_path.display());

                // Enqueue links
                let next_depth = depth.saturating_add(1);
                if !stop_flag_task.load(Ordering::SeqCst) {
                    for link in page.links {
                        let _ = tx.send((link, next_depth));
                    }
                }

                pending_ctr.fetch_sub(1, Ordering::SeqCst);
                if pending_ctr.load(Ordering::SeqCst) == 0 { notify.notify_waiters(); }
            });
        }
    };

    // Seed work
    for (u, d) in seeds { enqueue(u, d); }

    // Wait for all tasks; process discovered links as they arrive
    while pending.load(Ordering::SeqCst) > 0 {
        tokio::select! {
            _ = notify.notified() => { /* check pending again */ },
            maybe = rx.recv() => {
                if let Some((u,d)) = maybe { enqueue(u,d); }
            }
        }
    }

    pb.finish_and_clear();
    let _ = manifest.lock().await.write();
    Ok(())
}

fn origin_of(u: &Url) -> Result<Url, Box<dyn std::error::Error>> {
    let mut o = u.clone();
    o.set_path("");
    o.set_query(None);
    o.set_fragment(None);
    Ok(o)
}

fn build_client(user_agent: &str) -> Result<reqwest_middleware::ClientWithMiddleware, Box<dyn std::error::Error>> {
    let base = reqwest::Client::builder()
        .user_agent(user_agent.to_string())
        .redirect(reqwest::redirect::Policy::limited(10))
        .cookie_store(true)
        .build()?;

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(base)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    Ok(client)
}

async fn download_asset(client: &reqwest_middleware::ClientWithMiddleware, url: Url, dest: &std::path::Path, allow_svg: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dest.exists() { return Ok(()); }
    if let Some(parent) = dest.parent() { std::fs::create_dir_all(parent)?; }
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() { return Err(format!("non-success: {}", resp.status()).into()); }
    let ct = resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok());
    // Permit svg only when allowed via config
    if ct.map(|s| s.to_ascii_lowercase()) == Some("image/svg+xml".to_string()) && !allow_svg {
        return Err("svg not allowed".into());
    }
    if !is_safe_image_content_type(ct) && !(allow_svg && ct == Some("image/svg+xml")) {
        return Err("unsupported image content-type".into());
    }
    let bytes = resp.bytes().await?;
    std::fs::write(dest, &bytes)?;
    Ok(())
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
