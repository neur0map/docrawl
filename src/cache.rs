use sled::Db;

pub struct Cache {
    db: Db,
    visited: sled::Tree,
}

impl Cache {
    pub fn open(path: &std::path::Path) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let visited = db.open_tree("visited")?;
        Ok(Self { db, visited })
    }

    // returns true if was not present and inserted (i.e., first visit)
    pub fn check_and_mark_visited(&self, url: &str) -> sled::Result<bool> {
        let key = url.as_bytes();
        match self.visited.insert(key, &[1u8])? {
            Some(_) => Ok(false),
            None => Ok(true),
        }
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        let _ = self.db.flush();
    }
}
