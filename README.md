# docrawl

Docs‑focused crawler that converts documentation sites to clean Markdown, mirrors the site’s path structure, and adds useful metadata — while staying polite and secure.

## Features

- Markdown output with YAML frontmatter (`title`, `source_url`, `fetched_at`, optional `security_flags`, `quarantined`)
- Path‑mirroring folder structure with `index.md` per directory
- Robots‑aware crawling (official matcher) and rate limiting
- Optional sitemap seeding from `/sitemap.xml` for broader coverage on `--all`
- Content extraction tuned for common docs frameworks with configurable selectors
- Same‑origin image download and Markdown relinking to local assets
- HTML/URL sanitization and prompt‑injection detection with quarantine
- Run manifest (`manifest.json`) and persistent visited‑URL cache

## Install

Prerequisites: Rust (stable toolchain).

```
cargo build --release
```

Binary: `target/release/docrawl`

## Library Usage (Embed in Your Tool)

You can use `docrawl` as a library and ship a single binary for your own CLI or service.

Add to Cargo.toml (using the Git URL until it’s published on crates.io):

```
[dependencies]
docrawl = { git = "https://github.com/neur0map/docrawl" }
url = "2"
tokio = { version = "1", features = ["full"] }
```

Minimal programmatic crawl:

```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = docrawl::CrawlConfig {
        base_url: url::Url::parse("https://example.com/docs")?,
        output_dir: std::path::PathBuf::from("./out"),
        user_agent: format!("mytool/{}", env!("CARGO_PKG_VERSION")),
        max_depth: Some(2),
        rate_limit_per_sec: 8,
        follow_sitemaps: false,
        concurrency: 8,
        timeout: None,
        resume: false,
        config: docrawl::Config { host_only: true, skip_assets: true, ..Default::default() },
    };
    let stats = docrawl::crawl(cfg).await?;
    eprintln!("pages={} assets={}", stats.pages, stats.assets);
    Ok(())
}
```

Notes:
- `docrawl::crawl` uses the default on-disk layout (same as the CLI) and returns simple `Stats`.
- You own the UX: wire your flags to `CrawlConfig` and `Config` to keep one binary.

## Quick Start

```
docrawl "https://example.com/docs"          # default depth=10
docrawl "https://example.com" --all         # full same‑origin site
docrawl "https://example.com" --depth 2     # start + links + their links
docrawl "https://example.com" -o ./export   # choose output root
docrawl "https://example.com/docs" --fast   # quick smoke test (no assets, higher rate)
```

## CLI

- `url` (positional): Absolute starting URL. Only same‑origin links are followed by default.
- `--all`: Unlimited depth. Crawls the whole same‑origin site (still honors robots).
- `--depth <n>`: Max link depth from the start page. Default is `10` when `--all` isn’t set.
- `-o, --output <path>`: Output root; a host‑named folder is created inside.
- `--concurrency <n>`: Number of parallel fetch workers (bounded). Default: `8`.
- `--rate <n>`: Global requests per second. Default: `2`.
- `--timeout-minutes <n>`: Graceful shutdown after N minutes. The crawler stops scheduling new work, drains active tasks, writes the manifest, and exits.
- `--resume`: Resume from the previous run using the persisted frontier (see Cache). Useful after a timeout or manual stop.
- `--host-only`: Restrict scope to the exact origin (scheme+host+port).
- `--external-assets`: Allow downloading images from other domains.
- `--allow-svg`: Permit saving SVG images.
- `--no-assets`: Skip image downloads (fastest).
- `--max-pages <n>`: Stop after writing this many pages.
- `--selector <CSS>`: Preferred CSS selector for content (repeatable).
- `--exclude <REGEX>`: Exclude URLs matching the regex (repeatable).
- `--fast`: Preset for quick crawls: raises `--rate`/`--concurrency` to at least 16 and implies `--no-assets`.

Depth is link‑hop depth (not URL path depth). Hub‑style homepages can expose many pages even at small depths.

## Configuration (optional)

Place `docrawl.config.json` in the working directory or in the output root. All fields are optional.

```
{
  "host_only": true,
  "external_assets": false,
  "allow_svg": false,
  "max_pages": 500,
  "selectors": [".theme-doc-markdown", ".article-content"],
  "exclude_patterns": ["\\.pdf$", "/private/"]
}
```

- `host_only`: Limit scope to exact origin (scheme+host+port). Default: same‑domain allowed.
- `external_assets`: Allow images from outside the site’s origin/domain. Default: false.
- `allow_svg`: Permit saving SVG images. Default: false.
- `max_pages`: Stop after writing this many pages.
- `selectors`: Preferred CSS selectors for the main content; tried before built‑ins.
- `exclude_patterns`: Regex patterns; if a canonical URL matches any, it’s skipped.

## Behavior

- Output root defaults to CWD; inside it a folder named after the host is created (e.g., `./out/example.com/`).
- Depth 0: only the start page. `--all` overrides `--depth`.
- Scope: same‑origin by default; exact origin if `host_only` is true.
- Robots: evaluated with the `robotstxt` matcher (agent‑specific allow/deny).
- Sitemaps: when `--all` is set, seeds from `/sitemap.xml` to front‑load coverage (non‑recursive for nested indexes).
- Extraction: tries common docs containers (Docusaurus, Sphinx, MkDocs) and falls back to `<main>`, `<article>`, `<body>`; you can supply `selectors` to override.
- Images: same‑origin images are downloaded into the site folder and Markdown links are rewritten to local relative paths. External or unsupported types are skipped unless enabled in config.

## Security

- Sanitization (pre‑conversion): removes active/embedded elements (`script`, `iframe`, `object`, `form`, `link`, `meta`, etc.), normalizes and neutralizes risky URLs.
- Markdown scan (post‑conversion): detects likely prompt‑injection phrases and risky patterns; flags are recorded in frontmatter under `security_flags`.
- Quarantine: if any flags are detected, the page file is still created but the body is replaced with a short notice explaining why it was skipped; frontmatter includes `quarantined: true`.
- Asset safety: only safe image types (`png`, `jpeg/jpg`, `gif`, `webp`, `bmp`) are downloaded by default. SVG and `data:` images are disabled unless allowed via config.

Example frontmatter with flags:

```
---
title: Example
source_url: https://example.com/page
fetched_at: 2025-01-01T00:00:00Z
quarantined: true
security_flags:
  - llm_ignore_previous
  - javascript_link
---
```

## Output

```
<output_root>/example.com/
├── index.md
├── guide/
│   └── index.md
├── assets/            # images saved under site paths
└── manifest.json
```

- Markdown files include YAML frontmatter. Images are referenced using relative paths.
- `manifest.json` records id, url, relative path, title, quarantined, `security_flags`, and timestamps for each saved page.

## Cache

- Location: `<output_root>/.docrawl_cache` (sled key–value store).
- Purpose: persist the visited set across runs so re‑runs don’t re‑fetch unchanged URLs.
- If the cache cannot be opened (e.g., read‑only filesystem), crawling proceeds with in‑memory dedupe only; Markdown output still works.
- Frontier persistence for resume: the crawler stores pending URLs in the cache. Use `--resume` to continue from where you left off (e.g., after `--timeout-minutes` or Ctrl‑C). If you don’t use `--resume`, the persisted frontier is cleared at the start and new seeds are used.

## Proxy & macOS Notes

- The HTTP client honors `HTTP_PROXY`/`HTTPS_PROXY` if present. If neither is set, system proxy detection is disabled to avoid rare macOS System Configuration panics.
- If you need a proxy, export it before running:
  - `export HTTPS_PROXY=http://user:pass@host:port`
  - `export HTTP_PROXY=http://user:pass@host:port`

## Development and Testing

- Build: `cargo build` or `cargo build --release`
- Try a small depth first: `docrawl "https://example.com/docs" --depth 2`
- Verify robots compliance: disallowed paths should be skipped.
- Tune extraction with `selectors` if needed for your site.
- Confirm quarantined pages render a notice and include `security_flags`.

## Roadmap

- Conditional GETs (ETag/Last‑Modified)
- Recursive sitemap index traversal
- CLI flags mirroring config fields
- Site‑specific selector presets
