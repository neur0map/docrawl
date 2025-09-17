use std::io::Cursor;
use std::collections::HashSet;
use sitemap::reader::{SiteMapEntity, SiteMapReader};
use url::Url;

pub async fn fetch_and_parse_sitemaps(client: &reqwest_middleware::ClientWithMiddleware, origin: &Url) -> Vec<Url> {
    let sitemap_url = match origin.join("/sitemap.xml") { Ok(u) => u, Err(_) => return vec![] };
    let mut seen: HashSet<Url> = HashSet::new();
    let mut out: Vec<Url> = vec![];
    fetch_sitemap_recursive(client, &sitemap_url, &mut seen, &mut out, 3);
    out
}

fn parse_sitemap_str(txt: &str, urls_out: &mut Vec<Url>, nested_out: &mut Vec<Url>) {
    let cursor = Cursor::new(txt.as_bytes());
    for entity in SiteMapReader::new(cursor) {
        match entity {
            SiteMapEntity::Url(url_entry) => { if let Some(u) = url_entry.loc.get_url() { urls_out.push(u); } }
            SiteMapEntity::SiteMap(smap) => { if let Some(u) = smap.loc.get_url() { nested_out.push(u); } }
            SiteMapEntity::Err(_) => {}
        }
    }
}

fn fetch_sitemap_recursive(
    client: &reqwest_middleware::ClientWithMiddleware,
    url: &Url,
    seen: &mut HashSet<Url>,
    out: &mut Vec<Url>,
    depth: usize,
) {
    if depth == 0 || !seen.insert(url.clone()) { return; }
    let rt = tokio::runtime::Handle::current();
    let txt = rt.block_on(async { match client.get(url.clone()).send().await { Ok(resp) if resp.status().is_success() => resp.text().await.unwrap_or_default(), _ => String::new(), } });
    let mut nested: Vec<Url> = vec![];
    parse_sitemap_str(&txt, out, &mut nested);
    for n in nested.into_iter() { fetch_sitemap_recursive(client, &n, seen, out, depth - 1); }
}
