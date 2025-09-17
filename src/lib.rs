//! docrawl library API: programmatic crawling for documentation sites.
//!
//! Basic usage:
//! - Build a `CrawlConfig` and `Config`
//! - Call `crawl(cfg).await`
//!
//! This crate also ships a CLI (bin: `docrawl`) which wraps the library.

pub mod cache;
pub mod config;
pub mod crawler;
pub mod extract;
pub mod manifest;
pub mod robots;
pub mod save;
pub mod security;
pub mod sitemap;
pub mod util;
pub mod sink;

pub use crate::config::Config;
pub use crate::crawler::{crawl, CrawlConfig};
pub use crate::sink::{Sink, FileSink, Stats};
