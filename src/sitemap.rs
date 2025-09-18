use sitemap::reader::{SiteMapEntity, SiteMapReader};
use std::collections::{HashSet, VecDeque};
use std::io::Cursor;
use url::Url;

pub async fn fetch_and_parse_sitemaps(
    client: &reqwest_middleware::ClientWithMiddleware,
    origin: &Url,
) -> Vec<Url> {
    let start = match origin.join("/sitemap.xml") {
        Ok(u) => u,
        Err(_) => return vec![],
    };
    let mut seen: HashSet<Url> = HashSet::new();
    let mut out: Vec<Url> = vec![];
    let mut q: VecDeque<(Url, usize)> = VecDeque::new();
    q.push_back((start, 3));
    while let Some((url, depth)) = q.pop_front() {
        if depth == 0 || !seen.insert(url.clone()) {
            continue;
        }
        let txt = match client.get(url.clone()).send().await {
            Ok(resp) if resp.status().is_success() => resp.text().await.unwrap_or_default(),
            _ => continue,
        };
        let mut nested: Vec<Url> = vec![];
        parse_sitemap_str(&txt, &mut out, &mut nested);
        for n in nested {
            q.push_back((n, depth - 1));
        }
    }
    out
}

fn parse_sitemap_str(txt: &str, urls_out: &mut Vec<Url>, nested_out: &mut Vec<Url>) {
    let cursor = Cursor::new(txt.as_bytes());
    for entity in SiteMapReader::new(cursor) {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                if let Some(u) = url_entry.loc.get_url() {
                    urls_out.push(u);
                }
            }
            SiteMapEntity::SiteMap(smap) => {
                if let Some(u) = smap.loc.get_url() {
                    nested_out.push(u);
                }
            }
            SiteMapEntity::Err(_) => {}
        }
    }
}

// iterative approach above; no recursion needed
