use gpui::Global;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub struct Config {
    pub cache_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| std::env::temp_dir())
                .join("Xpic")
                .join("cache"),
        }
    }
}

impl Global for Config {}

impl Config {
    pub fn image_cache(&self, url: impl AsRef<str>) -> PathBuf {
        let hash = Sha256::digest(url.as_ref().as_bytes());
        self.cache_dir.join(hex::encode(hash))
    }
}
