use std::path::PathBuf;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "docrawl",
    version,
    about = "Docs-focused crawler: HTML to Markdown",
    long_about = "A simple, polite crawler for documentation sites.\n\nUSAGE:\n  docrawl <URL> [FLAGS]\n\nBASIC FLAGS:\n  --all                 Crawl the whole same-origin site (ignores --depth)\n  --depth <N>           Max link depth from the start page (default 10)\n  -o, --output <PATH>   Output root; a host-named folder is created inside\n\nPERFORMANCE FLAGS:\n  --concurrency <N>     Parallel fetch workers (default 8)\n  --rate <N>            Global requests per second (default 2)\n  --fast                Preset: raises rate/concurrency and skips assets\n  --no-assets           Skip downloading images/assets\n\nSCOPE & FILTER FLAGS:\n  --host-only           Restrict scope to exact origin (scheme+host+port)\n  --selector <CSS>      Preferred content selector (repeatable)\n  --exclude <REGEX>     Exclude URLs by regex (repeatable)\n\nLIMITS & RESUME:\n  --max-pages <N>       Stop after writing N pages\n  --timeout-minutes <N> Graceful shutdown after N minutes\n  --resume              Resume from previously persisted frontier\n",
    after_help = "EXAMPLES:\n  docrawl \"https://example.com/docs\"\n  docrawl \"https://example.com\" --all --fast\n  docrawl \"https://example.com\" --depth 2 --concurrency 12 --rate 10\n  docrawl \"https://example.com\" -o ./export --selector .main --exclude \\.(pdf|zip)$\n"
)]
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

    /// Skip downloading images/assets (fastest).
    #[arg(long = "no-assets", action = ArgAction::SetTrue)]
    pub no_assets: bool,

    /// Fast preset: higher rate/concurrency and no assets.
    #[arg(long, action = ArgAction::SetTrue)]
    pub fast: bool,
}

