use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = docrawl::CrawlConfig {
        base_url: url::Url::parse("https://example.com/docs")?,
        output_dir: PathBuf::from("./out"),
        user_agent: format!("mytool/{}", env!("CARGO_PKG_VERSION")),
        max_depth: Some(2),
        rate_limit_per_sec: 8,
        follow_sitemaps: false,
        concurrency: 8,
        timeout: None,
        resume: false,
        config: docrawl::Config { host_only: true, skip_assets: true, ..Default::default() },
    };
    docrawl::crawl(cfg).await?;
    Ok(())
}

