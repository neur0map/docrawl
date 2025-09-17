use std::collections::HashMap;

use url::Url;
use robotstxt::DefaultMatcher;

pub struct RobotsInfo { body: String }

impl RobotsInfo { pub fn default_allow() -> Self { Self { body: String::new() } } }

pub struct RobotsManager {
    by_origin: HashMap<String, RobotsInfo>,
    user_agent: String,
}

impl RobotsManager {
    pub fn new(user_agent: String) -> Self {
        Self { by_origin: HashMap::new(), user_agent }
    }

    pub async fn load_for(&mut self, client: &reqwest_middleware::ClientWithMiddleware, origin: &Url) {
        let key = origin.origin().unicode_serialization();
        if self.by_origin.contains_key(&key) { return; }
        let robots_url = match origin.join("/robots.txt") { Ok(u) => u, Err(_) => { self.by_origin.insert(key, RobotsInfo::default_allow()); return; } };
        let txt = match client.get(robots_url).send().await {
            Ok(resp) => {
                if !resp.status().is_success() { self.by_origin.insert(key, RobotsInfo::default_allow()); return; }
                match resp.text().await { Ok(t) => t, Err(_) => { self.by_origin.insert(key, RobotsInfo::default_allow()); return; } }
            }
            Err(_) => { self.by_origin.insert(key, RobotsInfo::default_allow()); return; }
        };
        self.by_origin.insert(key, RobotsInfo { body: txt });
    }

    pub fn allowed(&self, origin: &Url, path: &str) -> bool {
        let key = origin.origin().unicode_serialization();
        if let Some(info) = self.by_origin.get(&key) {
            let mut matcher = DefaultMatcher::default();
            let full = origin.join(path).unwrap_or_else(|_| origin.clone());
            matcher.one_agent_allowed_by_robots(&info.body, &self.user_agent, full.as_str())
        } else { true }
    }

    // no sitemap extraction here; handled by sitemap module
}
