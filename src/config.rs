use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub host_only: bool,
    #[serde(default)]
    pub external_assets: bool,
    #[serde(default)]
    pub allow_svg: bool,
    #[serde(default)]
    pub skip_assets: bool,
    #[serde(default)]
    pub max_pages: Option<usize>,
    #[serde(default)]
    pub selectors: Option<Vec<String>>, // preferred CSS selectors for main content
    #[serde(default)]
    pub exclude_patterns: Vec<String>, // regex strings
}

// Default is derived; serde #[serde(default)] supplies field defaults

pub fn load_config(working_dir: &Path, output_root: &Path) -> Config {
    let candidates = [
        working_dir.join("docrawl.config.json"),
        output_root.join("docrawl.config.json"),
    ];
    for path in candidates.iter() {
        if path.exists() {
            if let Ok(txt) = fs::read_to_string(path) {
                if let Ok(cfg) = serde_json::from_str::<Config>(&txt) {
                    return cfg;
                }
            }
        }
    }
    Config::default()
}
