use std::path::PathBuf;

mod cli;
mod crawler;
mod extract;
mod robots;
mod sitemap;
mod save;
mod util;
mod config;
mod cache;
mod manifest;
mod security;

use clap::Parser;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    let args = cli::Args::parse();

    let base_url = match url::Url::parse(&args.url) {
        Ok(u) => u,
        Err(e) => {
            error!(error = %e, "Invalid URL");
            std::process::exit(2);
        }
    };

    let output_root: PathBuf = args
        .output
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let max_depth = if args.all { None } else { Some(args.depth.unwrap_or(10)) };

    let mut cfgfile = config::load_config(&std::env::current_dir().unwrap(), &output_root);
    // CLI overrides config
    if args.host_only { cfgfile.host_only = true; }
    if args.external_assets { cfgfile.external_assets = true; }
    if args.allow_svg { cfgfile.allow_svg = true; }
    if let Some(m) = args.max_pages { cfgfile.max_pages = Some(m); }
    if !args.selectors.is_empty() { cfgfile.selectors = Some(args.selectors.clone()); }
    if !args.exclude_patterns.is_empty() { cfgfile.exclude_patterns = args.exclude_patterns.clone(); }

    let cfg = crawler::CrawlConfig {
        base_url,
        output_dir: output_root,
        user_agent: format!("docrawl/{}", env!("CARGO_PKG_VERSION")),
        max_depth,
        rate_limit_per_sec: args.rate.unwrap_or(2),
        follow_sitemaps: args.all,
        concurrency: args.concurrency.unwrap_or(8).max(1),
        config: cfgfile,
    };

    info!("Starting crawl");
    if let Err(e) = crawler::crawl(cfg).await {
        error!(error = %e, "Crawl failed");
        std::process::exit(1);
    }

    info!("Crawl complete");
}
