use lol_html::{element, html_content::ContentType, rewrite_str, RewriteStrSettings};
use scraper::{Html, Selector};
use soup::prelude::*;
use std::collections::HashMap;
use url::Url;

pub struct ExtractedPage {
    pub title: String,
    pub main_html: String,
    pub links: Vec<Url>,
    pub images: Vec<Url>,
}

pub fn extract_page(
    base: &Url,
    html: &str,
    preferred_selectors: Option<&[String]>,
) -> ExtractedPage {
    let doc = Html::parse_document(html);

    let mut title = extract_title(&doc).unwrap_or_else(|| "Untitled".to_string());
    let main_html_raw =
        extract_main_html(&doc, preferred_selectors).unwrap_or_else(|| html.to_string());
    let main_html_raw = clean_with_soup(&main_html_raw);
    if title.trim().is_empty() {
        title = "Untitled".to_string();
    }

    let links = extract_links(base, &doc);
    let images = extract_images(base, &doc);
    let anchor_fallbacks = compute_anchor_fallbacks(base, &doc);

    // Rewrite anchors without text to have a fallback text and normalize image src to absolute
    let main_html = rewrite_for_markdown(base, &main_html_raw, &anchor_fallbacks);

    ExtractedPage {
        title,
        main_html,
        links,
        images,
    }
}

fn extract_title(doc: &Html) -> Option<String> {
    let sel_title = Selector::parse("title").ok()?;
    if let Some(el) = doc.select(&sel_title).next() {
        let t = el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    // Fallback to first h1
    let sel_h1 = Selector::parse("h1").ok()?;
    if let Some(el) = doc.select(&sel_h1).next() {
        let t = el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    None
}

fn extract_main_html(doc: &Html, preferred: Option<&[String]>) -> Option<String> {
    if let Some(list) = preferred {
        for css in list.iter() {
            if let Ok(sel) = Selector::parse(css) {
                if let Some(el) = doc.select(&sel).next() {
                    return Some(el.inner_html());
                }
            }
        }
    }
    let candidates = [
        ".theme-doc-markdown",
        ".theme-doc-content",
        ".wy-nav-content",
        ".md-content",
        ".md-main__inner",
        ".markdown",
        "article",
        "main",
        "div[role=main]",
        "body",
    ];
    for css in candidates.iter() {
        if let Ok(sel) = Selector::parse(css) {
            if let Some(el) = doc.select(&sel).next() {
                // Prefer inner HTML so we don't wrap container markup itself
                return Some(el.inner_html());
            }
        }
    }
    None
}

fn clean_with_soup(html: &str) -> String {
    let soup = Soup::new(html);
    // Remove common non-content elements
    for tag in ["nav", "header", "footer", "aside"].iter() {
        let _maybe = soup.tag(*tag).find();
    }
    // If soup cannot serialize reliably, fall back to original HTML
    html.to_string()
}

fn extract_links(base: &Url, doc: &Html) -> Vec<Url> {
    let sel_a = match Selector::parse("a[href]") {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let mut links = vec![];
    for a in doc.select(&sel_a) {
        if let Some(href) = a.value().attr("href") {
            let href = href.trim();
            if href.is_empty() {
                continue;
            }
            if href.starts_with('#')
                || href.starts_with("mailto:")
                || href.starts_with("javascript:")
                || href.starts_with("tel:")
            {
                continue;
            }
            if let Ok(u) = base.join(href) {
                links.push(u);
            }
        }
    }
    links
}

fn extract_images(base: &Url, doc: &Html) -> Vec<Url> {
    let sel = match Selector::parse("img[src]") {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let mut out = vec![];
    for img in doc.select(&sel) {
        if let Some(src) = img.value().attr("src") {
            let src = src.trim();
            if src.is_empty() {
                continue;
            }
            if let Ok(u) = base.join(src) {
                out.push(u);
            }
        }
    }
    out
}

fn rewrite_for_markdown(
    base: &Url,
    html: &str,
    anchor_fallbacks: &HashMap<String, String>,
) -> String {
    let base_clone_a = base.clone();
    let base_clone_b = base.clone();
    let fallbacks = anchor_fallbacks.clone();
    let result = rewrite_str(
        html,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("a[href]", move |el| {
                    if let Some(h) = el.get_attribute("href") {
                        if let Ok(abs) = base_clone_a.join(h.trim()) {
                            if let Some(text) = fallbacks.get(abs.as_str()) {
                                el.set_inner_content(text, ContentType::Text);
                            }
                        }
                    }
                    Ok(())
                }),
                element!("img[src]", move |el| {
                    if let Some(src) = el.get_attribute("src") {
                        if let Ok(abs) = base_clone_b.join(src.trim()) {
                            el.set_attribute("src", abs.as_str()).ok();
                        }
                    }
                    // Ensure alt exists for better markdown output
                    if el.get_attribute("alt").is_none() {
                        if let Some(src) = el.get_attribute("src") {
                            let name = src.rsplit('/').next().unwrap_or("image");
                            el.set_attribute("alt", name).ok();
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

fn compute_anchor_fallbacks(base: &Url, doc: &Html) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let sel_a = match Selector::parse("a[href]") {
        Ok(s) => s,
        Err(_) => return map,
    };
    for a in doc.select(&sel_a) {
        let href = match a.value().attr("href") {
            Some(h) => h.trim(),
            None => continue,
        };
        if href.is_empty() {
            continue;
        }
        let text = a.text().collect::<Vec<_>>().join("").trim().to_string();
        if !text.is_empty() {
            continue;
        }
        let abs = match base.join(href) {
            Ok(u) => u,
            Err(_) => continue,
        };
        // Prefer title/aria-label, else host or last path segment
        let title = a.value().attr("title").map(|s| s.trim().to_string());
        let aria = a.value().attr("aria-label").map(|s| s.trim().to_string());
        let fallback = title
            .or(aria)
            .or_else(|| abs.host_str().map(|s| s.to_string()))
            .unwrap_or_else(|| abs.as_str().to_string());
        map.insert(abs.as_str().to_string(), fallback);
    }
    map
}
