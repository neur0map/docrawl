use std::collections::{HashSet, VecDeque};
use std::num::NonZeroU32;
use std::path::PathBuf;

use governor::{Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff; 
use reqwest_retry::RetryTransientMiddleware;
use tracing::{debug, info, warn};
use url::Url;

use crate::extract::extract_page;
use crate::config::Config;
use crate::cache::Cache;
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
    pub config: Config,
}

pub async fn crawl(cfg: CrawlConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_origin = origin_of(&cfg.base_url)?;
    std::fs::create_dir_all(&cfg.output_dir)?;

    let client = build_client(&cfg.user_agent)?;

    let limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(cfg.rate_limit_per_sec).unwrap()));

    let mut robots = RobotsManager::new(cfg.user_agent.clone());
    robots.load_for(&client, &base_origin).await;

    let mut visited: HashSet<String> = HashSet::new();
    let cache = Cache::open(&cfg.output_dir.join(".docrawl_cache")).ok();
    let mut q: VecDeque<(Url, usize)> = VecDeque::new();

    q.push_back((cfg.base_url.clone(), 0));

    // Optional sitemap seeding
    if cfg.follow_sitemaps {
        let sitemap_urls = fetch_and_parse_sitemaps(&client, &base_origin).await;
        for u in sitemap_urls {
            if within_scope(&base_origin, &u, &cfg.config) {
                q.push_back((u, 0));
            }
        }
    }

    // Prepare manifest
    let host_dir = cfg.output_dir.join(site_name_from_url(&base_origin));
    let mut manifest = ManifestRecorder::new(host_dir.clone());

    // Precompile exclude patterns
    let exclude_res: Vec<Regex> = cfg
        .config
        .exclude_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let mut saved_pages: usize = 0;

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    while let Some((url, depth)) = q.pop_front() {
        let canon = normalize_url(&url);
        if !visited.insert(canon.clone()) { continue; }
        if let Some(c) = &cache { if !c.check_and_mark_visited(&canon).unwrap_or(true) { continue; } }

        // Exclude pattern matches
        if exclude_res.iter().any(|re| re.is_match(canon.as_str())) { continue; }

        // Depth check
        if let Some(maxd) = cfg.max_depth { if depth > maxd { continue; } }

        if !within_scope(&base_origin, &url, &cfg.config) { continue; }

        // robots
        if !robots.allowed(&base_origin, url.path()) { 
            debug!("Blocked by robots.txt: {}", url);
            continue; 
        }

        pb.set_message(format!("Fetching {} (depth {})", url, depth));
        limiter.until_ready().await;
        let resp = match client.get(url.clone()).send().await {
            Ok(r) => r,
            Err(e) => { warn!(error = %e, "request failed"); continue; }
        };
        let resp = match resp.error_for_status() {
            Ok(r) => r,
            Err(e) => { warn!(error = %e, "bad status"); continue; }
        };

        let final_url = resp.url().clone();
        let content_type = resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("").to_lowercase();
        if !content_type.contains("text/html") && !content_type.contains("application/xhtml") {
            debug!("Skipping non-HTML: {} [{}]", final_url, content_type);
            continue;
        }
        let body = match resp.text().await { Ok(t) => t, Err(e) => { warn!(error = %e, "read body failed"); continue; } };

        let page = extract_page(&final_url, &body, cfg.config.selectors.as_deref());
        let sanitized_html = sanitize_html_for_md(&final_url, &page.main_html);
        let mut md = html2md::parse_html(&sanitized_html);
        let (md_checked, security_flags) = sanitize_markdown(&md);
        md = md_checked;
        let out_path = path_for_url(&cfg.output_dir, &base_origin, &final_url);

        let quarantined = !security_flags.is_empty();
        if !quarantined {
            // Download same-origin images and rewrite markdown image links to local relative paths
            let mut replace_pairs: Vec<(String, String)> = vec![];
            for img in page.images.iter() {
                if !cfg.config.external_assets && !within_scope(&base_origin, img, &cfg.config) { continue; }
                let asset_path = path_for_asset(&cfg.output_dir, &base_origin, img);
                // politeness
                limiter.until_ready().await;
                if let Err(e) = download_asset(&client, img.clone(), &asset_path, cfg.config.allow_svg).await {
                    warn!(error=%e, "asset download failed: {}", img);
                    continue;
                }
                if let Some(rel) = relpath(&out_path, &asset_path) {
                    replace_pairs.push((img.as_str().to_string(), rel.to_string_lossy().to_string()));
                }
            }
            if !replace_pairs.is_empty() {
                md = rewrite_md_images(md, &replace_pairs);
            }
        }

        let body_to_write = if quarantined {
            let reasons = security_flags.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n");
            format!(
                "Content skipped by docrawl due to detected security issues.\n\nDetected flags:\n{}\n\nReason: One or more patterns indicating potential prompt-injection, risky content, or unsafe assets were found. To protect downstream consumers, docrawl omitted page content.",
                reasons
            )
        } else {
            md
        };

        if let Err(e) = write_markdown_with_frontmatter(&out_path, &page.title, final_url.as_str(), &body_to_write, &security_flags) {
            warn!(error = %e, "write file failed");
        }

        manifest.record(final_url.as_str(), &out_path, &page.title, quarantined, security_flags.clone());
        saved_pages += 1;
        if let Some(max) = cfg.config.max_pages { if saved_pages >= max { break; } }

        info!("Saved {}", out_path.display());

        // Enqueue links
        let next_depth = depth.saturating_add(1);
        for link in page.links {
            if !within_scope(&base_origin, &link, &cfg.config) { continue; }
            if let Some(maxd) = cfg.max_depth { if next_depth > maxd { continue; } }
            q.push_back((link, next_depth));
        }
    }

    pb.finish_and_clear();
    let _ = manifest.write();
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
