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

    let cfgfile = config::load_config(&std::env::current_dir().unwrap(), &output_root);

    let cfg = crawler::CrawlConfig {
        base_url,
        output_dir: output_root,
        user_agent: format!("docrawl/{}", env!("CARGO_PKG_VERSION")),
        max_depth,
        rate_limit_per_sec: 2,
        follow_sitemaps: args.all,
        config: cfgfile,
    };

    info!("Starting crawl");
    if let Err(e) = crawler::crawl(cfg).await {
        error!(error = %e, "Crawl failed");
        std::process::exit(1);
    }

    info!("Crawl complete");
}
