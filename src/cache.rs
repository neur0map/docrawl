use sled::Db;
use serde::{Deserialize, Serialize};

pub struct Cache {
    db: Db,
    visited: sled::Tree,
    meta: sled::Tree,
    frontier: sled::Tree,
}

impl Cache {
    pub fn open(path: &std::path::Path) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let visited = db.open_tree("visited")?;
        let meta = db.open_tree("meta")?;
        let frontier = db.open_tree("frontier")?;
        Ok(Self { db, visited, meta, frontier })
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

impl Cache {
    pub fn add_frontier(&self, url: &str, depth: usize) -> sled::Result<()> {
        let mut buf = [0u8; 8];
        buf[..8].copy_from_slice(&(depth as u64).to_le_bytes());
        self.frontier.insert(url.as_bytes(), &buf[..])?;
        Ok(())
    }

    pub fn remove_frontier(&self, url: &str) -> sled::Result<()> {
        let _ = self.frontier.remove(url.as_bytes())?;
        Ok(())
    }

    pub fn list_frontier(&self) -> sled::Result<Vec<(String, usize)>> {
        let mut out = vec![];
        for kv in self.frontier.iter() {
            let (k, v) = kv?;
            let url = String::from_utf8_lossy(k.as_ref()).to_string();
            let mut arr = [0u8;8];
            let len = v.len().min(8);
            arr[..len].copy_from_slice(&v[..len]);
            let depth = u64::from_le_bytes(arr) as usize;
            out.push((url, depth));
        }
        Ok(out)
    }

    pub fn clear_frontier(&self) -> sled::Result<()> {
        self.frontier.clear()?;
        Ok(())
    }
}
