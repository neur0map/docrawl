use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Parser, Debug, Clone)]
#[command(name = "docrawl", version, about = "Docs-focused crawler: HTML → Markdown")] 
pub struct Args {
    /// The starting URL to crawl
    pub url: String,

    /// Crawl entire site (overrides --depth)
    #[arg(long, action = ArgAction::SetTrue)]
    pub all: bool,

    /// Maximum crawl depth (0 = only the start page). Defaults to 10 when `--all` is not set.
    #[arg(long)]
    pub depth: Option<usize>,

    /// Output directory root (site folder is created inside)
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Concurrent fetch workers (bounded). Defaults to 8.
    #[arg(long)]
    pub concurrency: Option<usize>,

    /// Requests per second (global rate limit). Defaults to 2.
    #[arg(long)]
    pub rate: Option<u32>,

    /// Restrict scope to exact origin (scheme+host+port). Default: same-domain.
    #[arg(long, action = ArgAction::SetTrue)]
    pub host_only: bool,

    /// Allow downloading images from other domains.
    #[arg(long, action = ArgAction::SetTrue)]
    pub external_assets: bool,

    /// Permit saving SVG images.
    #[arg(long, action = ArgAction::SetTrue)]
    pub allow_svg: bool,

    /// Stop after writing this many pages.
    #[arg(long)]
    pub max_pages: Option<usize>,

    /// Preferred CSS selectors for main content (can be repeated).
    #[arg(long = "selector")]
    pub selectors: Vec<String>,

    /// Regex patterns to exclude URLs (can be repeated).
    #[arg(long = "exclude")]
    pub exclude_patterns: Vec<String>,

    /// Stop the crawl after N minutes (graceful shutdown).
    #[arg(long = "timeout-minutes")]
    pub timeout_minutes: Option<u64>,

    /// Resume from previous run using the persisted frontier in the cache.
    #[arg(long, action = ArgAction::SetTrue)]
    pub resume: bool,
}
