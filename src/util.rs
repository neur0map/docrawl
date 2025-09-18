use std::path::{Path, PathBuf};

use url::Url;

pub fn normalize_url(u: &Url) -> String {
    // Manual, conservative normalization: drop fragment, default ports, normalize path
    let mut clone = u.clone();
    clone.set_fragment(None);
    if (clone.scheme() == "http" && clone.port() == Some(80)) || (clone.scheme() == "https" && clone.port() == Some(443)) {
        clone.set_port(None).ok();
    }
    // Remove trailing slash except for root
    let mut path = clone.path().to_string();
    if path.len() > 1 && path.ends_with('/') { path.pop(); }
    clone.set_path(&path);
    // Remove empty query
    if clone.query().map(|q| q.trim().is_empty()).unwrap_or(false) { clone.set_query(None); }
    clone.as_str().to_string()
}

pub fn is_same_host(a: &Url, b: &Url) -> bool {
    a.domain() == b.domain() && a.scheme() == b.scheme()
}

pub fn site_name_from_url(u: &Url) -> String {
    u.host_str().unwrap_or("site").to_string()
}

fn sanitize_segment(seg: &str) -> String {
    let mut s = seg.trim().replace(['\\', '/', ':', '*', '?', '"', '<', '>', '|'], "-");
    s = s.replace([' ', '\t', '\n'], "-");
    // Collapse repeated '-'
    while s.contains("--") { s = s.replace("--", "-"); }
    s.trim_matches('-').to_string()
}

pub fn path_for_url(output_root: &Path, base: &Url, target: &Url) -> PathBuf {
    let base_host = site_name_from_url(base);
    let mut rel = PathBuf::new();
    // Ensure host folder exists one level up (already in main), we only build inside it
    let path = target.path();
    let mut segments: Vec<&str> = path.split('/')
        .filter(|s| !s.is_empty())
        .collect();

    // Determine filename
    let (mut file_stem, ext): (String, &str) = if path.ends_with('/') || path.is_empty() || segments.is_empty() {
        // Directory or root path: keep directory segments and use index.md
        if !segments.is_empty() {
            rel.extend(segments.iter().map(|s| sanitize_segment(s)));
        }
        ("index".to_string(), "md")
    } else if let Some(last) = segments.last() {
        if last.ends_with(".html") || last.ends_with(".htm") {
            let stem = last.trim_end_matches(".html").trim_end_matches(".htm");
            segments.pop();
            rel.extend(segments.iter().map(|s| sanitize_segment(s)));
            (sanitize_segment(stem), "md")
        } else if last.contains('.') {
            // looks like a file with other extension → treat as a directory index
            rel.extend(segments.iter().map(|s| sanitize_segment(s)));
            ("index".to_string(), "md")
        } else {
            rel.extend(segments.iter().map(|s| sanitize_segment(s)));
            ("index".to_string(), "md")
        }
    } else {
        ("index".to_string(), "md")
    };

    // Encode query into file name if present (to avoid collisions)
    if let Some(q) = target.query() {
        use xxhash_rust::xxh3::xxh3_64;
        let h = xxh3_64(q.as_bytes());
        file_stem.push_str(&format!("__q_{:x}", h));
    }

    let file_name = format!("{}.{}", file_stem, ext);

    output_root
        .join(base_host)
        .join(rel)
        .join(file_name)
}

pub fn now_rfc3339() -> String {
    let t = chrono::Utc::now();
    t.to_rfc3339()
}

pub fn ensure_parent_dir(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
    Ok(())
}

pub fn path_for_asset(output_root: &Path, base: &Url, asset: &Url) -> PathBuf {
    let base_host = site_name_from_url(base);
    let mut rel = PathBuf::new();
    let path = asset.path();
    let segments: Vec<&str> = path.split('/')
        .filter(|s| !s.is_empty())
        .collect();

    // If ends with '/', generate a name from hash
    let file_name = if let Some(last) = segments.last() {
        if last.ends_with('/') || last.is_empty() { None } else { Some(last.to_string()) }
    } else { None };

    rel.extend(segments.into_iter().map(|s| sanitize_segment(s)));

    let final_name = match file_name {
        Some(n) => sanitize_segment(&n),
        None => {
            use xxhash_rust::xxh3::xxh3_64;
            let h = xxh3_64(asset.as_str().as_bytes());
            format!("asset_{:x}", h)
        }
    };

    output_root.join(base_host).join(rel).with_file_name(final_name)
}

pub fn relpath(from: &Path, to: &Path) -> Option<PathBuf> {
    pathdiff::diff_paths(to, from.parent().unwrap_or_else(|| Path::new(".")))
}
