use lol_html::{element, rewrite_str, RewriteStrSettings};
use regex::Regex;
use url::Url;

// Sanitize HTML before converting to Markdown.
// - Remove active content and risky elements
// - Neutralize javascript/data URIs in anchors and images
pub fn sanitize_html_for_md(base: &Url, html: &str) -> String {
    let base_a = base.clone();
    let base_i = base.clone();
    let result = rewrite_str(
        html,
        RewriteStrSettings {
            element_content_handlers: vec![
                // Remove active or metadata elements outright
                element!(
                    "script, style, noscript, iframe, object, embed, form, input, button, textarea, select, option, link, meta, base, video, audio, svg",
                    |el| {
                        el.remove();
                        Ok(())
                    }
                ),
                // Anchors: neutralize javascript/data URIs
                element!("a[href]", move |el| {
                    if let Some(href) = el.get_attribute("href") {
                        let h = href.trim();
                        let lower = h.to_ascii_lowercase();
                        if lower.starts_with("javascript:") || lower.starts_with("data:") || lower.starts_with("vbscript:") {
                            el.set_attribute("href", "#").ok();
                        } else if let Ok(abs) = base_a.join(h) {
                            // Normalize to absolute to help later markdown rewriting
                            el.set_attribute("href", abs.as_str()).ok();
                        }
                    }
                    Ok(())
                }),
                // Images: drop data URIs; normalize to absolute
                element!("img[src]", move |el| {
                    if let Some(src) = el.get_attribute("src") {
                        let s = src.trim();
                        let lower = s.to_ascii_lowercase();
                        if lower.starts_with("data:") || lower.starts_with("javascript:") || lower.starts_with("vbscript:") {
                            el.remove();
                        } else if let Ok(abs) = base_i.join(s) {
                            el.set_attribute("src", abs.as_str()).ok();
                        }
                    }
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    );
    match result {
        Ok(s) => s,
        Err(_) => html.to_string(),
    }
}

// Scan markdown for risky patterns and return (possibly modified md, flags)
pub fn sanitize_markdown(md: &str) -> (String, Vec<String>) {
    let mut flags: Vec<String> = vec![];
    let mut out = md.to_string();

    let patterns = vec![
        (
            Regex::new(r"(?i)ignore (all|any|previous) (instructions|directives)").unwrap(),
            "llm_ignore_previous",
        ),
        (
            Regex::new(r"(?i)you are (chatgpt|an? ai|a large language model)").unwrap(),
            "llm_role_override",
        ),
        (
            Regex::new(r"(?i)begin (system|assistant|user) prompt").unwrap(),
            "llm_prompt_block",
        ),
        (
            Regex::new(r"(?i)```\s*(system|assistant|user)\b").unwrap(),
            "llm_fenced_role_block",
        ),
        (
            Regex::new(r"(?i)<\s*(script|iframe|object|embed)\b").unwrap(),
            "raw_html_active",
        ),
        (
            Regex::new(r"(?i)javascript:\S+").unwrap(),
            "javascript_link",
        ),
        (
            Regex::new(r"(?i)data:[^;]+;base64,[A-Za-z0-9+/=]{100,}").unwrap(),
            "large_base64_blob",
        ),
    ];

    for (re, label) in patterns.iter() {
        if re.is_match(&out) {
            flags.push((*label).to_string());
        }
    }

    // Optional minimal neutralization: break "```system" markers to avoid accidental role parsing in some tooling
    let re_fence = Regex::new(r"```\s*(system|assistant|user)\b").unwrap();
    if re_fence.is_match(&out) {
        out = re_fence.replace_all(&out, "```_redacted_role").to_string();
    }

    (out, flags)
}

pub fn is_safe_image_content_type(ct: Option<&str>) -> bool {
    let t = ct.map(|s| {
        s.split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase()
    });
    matches!(
        t.as_deref(),
        Some("image/png" | "image/jpeg" | "image/jpg" | "image/gif" | "image/webp" | "image/bmp")
    )
}
