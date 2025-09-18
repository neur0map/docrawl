use std::path::PathBuf;

mod cli;

use clap::Parser;
use tracing::error;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".to_string());
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(match filter.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::WARN,
        })
        .init();

    let args = cli::Args::parse();

    let base_url = match url::Url::parse(&args.url) {
        Ok(u) => u,
        Err(e) => {
            error!(error = %e, "Invalid URL");
            std::process::exit(2);
        }
    };

    let mut cfgfile = docrawl::Config::default();
    if args.host_only {
        cfgfile.host_only = true;
    }
    if args.external_assets {
        cfgfile.external_assets = true;
    }
    if args.allow_svg {
        cfgfile.allow_svg = true;
    }
    if args.no_assets {
        cfgfile.skip_assets = true;
    }
    if let Some(m) = args.max_pages {
        cfgfile.max_pages = Some(m);
    }
    if !args.selectors.is_empty() {
        cfgfile.selectors = Some(args.selectors.clone());
    }
    if !args.exclude_patterns.is_empty() {
        cfgfile.exclude_patterns = args.exclude_patterns.clone();
    }

    // Fast preset with more aggressive defaults
    let rate = args.rate.unwrap_or(if args.fast { 50 } else { 10 });
    let concurrency = args
        .concurrency
        .unwrap_or(if args.fast { 32 } else { 16 })
        .max(1);
    if args.fast {
        cfgfile.skip_assets = true;
        cfgfile.external_assets = false;
        cfgfile.allow_svg = false;
    }

    let output_root: PathBuf = args
        .output
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let max_depth = if args.all {
        None
    } else {
        Some(args.depth.unwrap_or(10))
    };

    let cfg = docrawl::CrawlConfig {
        base_url,
        output_dir: output_root,
        user_agent: format!("docrawl/{}", env!("CARGO_PKG_VERSION")),
        max_depth,
        rate_limit_per_sec: rate,
        follow_sitemaps: args.all,
        concurrency,
        timeout: args
            .timeout_minutes
            .map(|m| std::time::Duration::from_secs(m.saturating_mul(60))),
        resume: args.resume,
        config: cfgfile,
    };

    println!("Starting crawl...");
    if let Err(e) = docrawl::crawl(cfg).await {
        error!(error = %e, "Crawl failed");
        std::process::exit(1);
    }
}
