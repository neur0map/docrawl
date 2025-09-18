use std::path::{Path, PathBuf};
use std::sync::Mutex;

use async_trait::async_trait;
use bytes::Bytes;
use url::Url;

use crate::manifest::ManifestRecorder;
use crate::save::write_markdown_with_frontmatter;
use crate::util::ensure_parent_dir;

#[derive(Debug, Default, Clone, Copy)]
pub struct Stats {
    pub pages: usize,
    pub assets: usize,
}

#[async_trait]
pub trait Sink: Send + Sync {
    async fn save_asset(
        &self,
        url: &Url,
        dest: &Path,
        content_type: Option<&str>,
        bytes: Bytes,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn save_page(
        &self,
        path: &Path,
        title: &str,
        url: &Url,
        markdown: &str,
        security_flags: &[String],
        quarantined: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn finalize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

pub struct FileSink {
    root_host_dir: PathBuf,
    manifest: Mutex<ManifestRecorder>,
}

impl FileSink {
    pub fn new(root_host_dir: PathBuf) -> Self {
        let manifest = ManifestRecorder::new(root_host_dir.clone());
        Self {
            root_host_dir,
            manifest: Mutex::new(manifest),
        }
    }
}

#[async_trait]
impl Sink for FileSink {
    async fn save_asset(
        &self,
        _url: &Url,
        dest: &Path,
        _content_type: Option<&str>,
        bytes: Bytes,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Validate that the destination is within our root directory
        if !dest.starts_with(&self.root_host_dir) {
            return Err(format!(
                "Destination path {:?} is outside root directory {:?}",
                dest, self.root_host_dir
            )
            .into());
        }

        ensure_parent_dir(dest)?;
        std::fs::write(dest, &bytes)?;
        Ok(())
    }

    async fn save_page(
        &self,
        path: &Path,
        title: &str,
        url: &Url,
        markdown: &str,
        security_flags: &[String],
        quarantined: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Validate that the path is within our root directory
        if !path.starts_with(&self.root_host_dir) {
            return Err(format!(
                "Page path {:?} is outside root directory {:?}",
                path, self.root_host_dir
            )
            .into());
        }

        write_markdown_with_frontmatter(path, title, url.as_str(), markdown, security_flags)?;
        let mut man = self.manifest.lock().unwrap();
        man.record(
            url.as_str(),
            path,
            title,
            quarantined,
            security_flags.to_vec(),
        );
        Ok(())
    }

    async fn finalize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let man = self.manifest.lock().unwrap();
        man.write()?;
        Ok(())
    }
}
