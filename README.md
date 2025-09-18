# docrawl

A documentation-focused web crawler that converts sites to clean Markdown while preserving structure and staying polite.

## Installation

```bash
# Install from crates.io
cargo install docrawl

# Or build from source
git clone https://github.com/neur0map/docrawl
cd docrawl
cargo build --release
```

## Quick Start

```bash
docrawl "https://docs.rust-lang.org"          # crawl with default depth
docrawl "https://docs.python.org" --all       # full site crawl
docrawl "https://react.dev" --depth 2         # shallow crawl
docrawl "https://nextjs.org/docs" --fast      # quick scan without assets
```

## Key Features

- **Documentation-optimized extraction** - Built-in selectors for Docusaurus, MkDocs, Sphinx, Next.js docs
- **Clean Markdown output** - Preserves code blocks, tables, and formatting with YAML frontmatter metadata
- **Path-mirroring structure** - Maintains original URL hierarchy as folders with `index.md` files
- **Polite crawling** - Respects robots.txt, rate limits, and sitemap hints
- **Security-first** - Sanitizes content, detects prompt injections, quarantines suspicious pages
- **Resumable sessions** - Persistent cache allows stopping and continuing crawls

## Why docrawl?

Unlike general-purpose crawlers, docrawl is purpose-built for documentation:

| Tool | Purpose | Output | Documentation Support |
|------|---------|--------|----------------------|
| **wget/curl** | File downloading | Raw HTML | No extraction |
| **httrack** | Website mirroring | Full HTML site | No Markdown conversion |
| **scrapy** | Web scraping framework | Custom formats | Requires coding |
| **docrawl** | Documentation crawler | Clean Markdown | Auto-detects docs frameworks |

docrawl combines crawling, extraction, and conversion in a single tool optimized for technical documentation.

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
docrawl = "0.1"
tokio = { version = "1", features = ["full"] }
```

Minimal example:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = docrawl::CrawlConfig {
        base_url: url::Url::parse("https://example.com/docs")?,
        output_dir: std::path::PathBuf::from("./out"),
        max_depth: Some(3),
        ..Default::default()
    };
    let stats = docrawl::crawl(cfg).await?;
    println!("Crawled {} pages", stats.pages);
    Ok(())
}
```

## CLI Options

| Option | Description | Default |
|--------|-------------|---------|
| `--depth <n>` | Maximum crawl depth | 10 |
| `--all` | Crawl entire site | - |
| `--output <dir>` | Output directory | Current dir |
| `--rate <n>` | Requests per second | 2 |
| `--concurrency <n>` | Parallel workers | 8 |
| `--selector <css>` | Custom content selector | Auto-detect |
| `--fast` | Quick mode (no assets, rate=16) | - |
| `--resume` | Continue previous crawl | - |

## Configuration

Optional `docrawl.config.json`:

```json
{
  "selectors": [".content", "article"],
  "exclude_patterns": ["\\.pdf$", "/api/"],
  "max_pages": 1000,
  "host_only": true
}
```

## Output Structure

```
output/
└── example.com/
    ├── index.md
    ├── guide/
    │   └── index.md
    ├── assets/
    │   └── images/
    └── manifest.json
```

Each Markdown file includes frontmatter:

```yaml
---
title: Page Title
source_url: https://example.com/page
fetched_at: 2025-01-18T12:00:00Z
---
```

## License

MIT