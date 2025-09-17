use std::io::Cursor;
use sitemap::reader::{SiteMapEntity, SiteMapReader};
use url::Url;

pub async fn fetch_and_parse_sitemaps(client: &reqwest_middleware::ClientWithMiddleware, origin: &Url) -> Vec<Url> {
    let mut urls: Vec<Url> = vec![];
    let sitemap_url = match origin.join("/sitemap.xml") { Ok(u) => u, Err(_) => return urls };
    let txt = match client.get(sitemap_url).send().await {
        Ok(resp) if resp.status().is_success() => resp.text().await.unwrap_or_default(),
        _ => return urls,
    };
    parse_sitemap_str(&txt, &mut urls);
    urls
}

fn parse_sitemap_str(txt: &str, out: &mut Vec<Url>) {
    let cursor = Cursor::new(txt.as_bytes());
    for entity in SiteMapReader::new(cursor) {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                if let Some(u) = url_entry.loc.get_url() { out.push(u); }
            }
            SiteMapEntity::SiteMap(smap) => {
                if let Some(u) = smap.loc.get_url() { out.push(u); }
            }
            SiteMapEntity::Err(_) => {}
        }
    }
}
