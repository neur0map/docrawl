use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Serialize)]
pub struct PageRecord {
    pub id: String,
    pub url: String,
    pub path: String,
    pub title: String,
    pub quarantined: bool,
    pub security_flags: Vec<String>,
    pub fetched_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Manifest {
    pub run_id: String,
    pub started_at: DateTime<Utc>,
    pub pages: Vec<PageRecord>,
}

pub struct ManifestRecorder {
    out_dir_host: PathBuf,
    manifest: Manifest,
}

impl ManifestRecorder {
    pub fn new(out_dir_host: PathBuf) -> Self {
        Self {
            out_dir_host,
            manifest: Manifest {
                run_id: Uuid::new_v4().to_string(),
                started_at: Utc::now(),
                pages: vec![],
            },
        }
    }

    pub fn record(
        &mut self,
        url: &str,
        path: &Path,
        title: &str,
        quarantined: bool,
        security_flags: Vec<String>,
    ) {
        let rec = PageRecord {
            id: Uuid::new_v4().to_string(),
            url: url.to_string(),
            path: path
                .strip_prefix(&self.out_dir_host)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string_lossy().to_string()),
            title: title.to_string(),
            quarantined,
            security_flags,
            fetched_at: Utc::now(),
        };
        self.manifest.pages.push(rec);
    }

    pub fn write(&self) -> std::io::Result<()> {
        let path = self.out_dir_host.join("manifest.json");
        let json =
            serde_json::to_string_pretty(&self.manifest).unwrap_or_else(|_| "{}".to_string());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, json)
    }
}
