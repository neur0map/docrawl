use sled::Db;
use serde::{Deserialize, Serialize};

pub struct Cache {
    db: Db,
    visited: sled::Tree,
    meta: sled::Tree,
}

impl Cache {
    pub fn open(path: &std::path::Path) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let visited = db.open_tree("visited")?;
        let meta = db.open_tree("meta")?;
        Ok(Self { db, visited, meta })
    }

    // returns true if was not present and inserted (i.e., first visit)
    pub fn check_and_mark_visited(&self, url: &str) -> sled::Result<bool> {
        let key = url.as_bytes();
        match self.visited.insert(key, &[1u8])? {
            Some(_) => Ok(false),
            None => Ok(true),
        }
    }

    pub fn get_meta(&self, url: &str) -> sled::Result<Option<ResourceMeta>> {
        if let Some(v) = self.meta.get(url.as_bytes())? {
            let m: ResourceMeta = serde_json::from_slice(&v).unwrap_or_default();
            Ok(Some(m))
        } else { Ok(None) }
    }

    pub fn set_meta(&self, url: &str, meta: &ResourceMeta) -> sled::Result<()> {
        let bytes = serde_json::to_vec(meta).unwrap_or_default();
        self.meta.insert(url.as_bytes(), bytes)?;
        Ok(())
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        let _ = self.db.flush();
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceMeta {
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}
