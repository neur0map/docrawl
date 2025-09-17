use std::fs;
use std::io::Write;
use std::path::Path;

use crate::util::{ensure_parent_dir, now_rfc3339};

pub fn write_markdown_with_frontmatter(path: &Path, title: &str, url: &str, body_md: &str, security_flags: &[String]) -> std::io::Result<()> {
    ensure_parent_dir(path)?;

    let mut file = fs::File::create(path)?;
    writeln!(file, "---")?;
    writeln!(file, "title: {}", escape_yaml(title))?;
    writeln!(file, "source_url: {}", escape_yaml(url))?;
    writeln!(file, "fetched_at: {}", now_rfc3339())?;
    if !security_flags.is_empty() {
        writeln!(file, "quarantined: true")?;
    }
    if !security_flags.is_empty() {
        writeln!(file, "security_flags:")?;
        for f in security_flags {
            writeln!(file, "  - {}", escape_yaml(f))?;
        }
    }
    writeln!(file, "---\n")?;
    file.write_all(body_md.as_bytes())?;
    Ok(())
}

fn escape_yaml(s: &str) -> String {
    if s.chars().any(|c| c.is_whitespace()) { format!("\"{}\"", s.replace('"', "\\\"")) } else { s.to_string() }
}
