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
}
