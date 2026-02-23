use crate::theme::Appearance;
use gpui::Global;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use xpic::bing::Market;

pub struct Config {
    pub cache_dir: PathBuf,
    pub data_dir: PathBuf,
    pub appearance: Appearance,
}

impl Default for Config {
    fn default() -> Self {
        let base = dirs::data_local_dir()
            .unwrap_or_else(|| std::env::temp_dir())
            .join("Xpic");

        Self {
            cache_dir: base.join("cache"),
            data_dir: base.join("data"),
            appearance: Appearance::Dark,
        }
    }
}

impl Global for Config {}

impl Config {
    pub fn image_cache(&self, url: impl AsRef<str>) -> PathBuf {
        let hash = Sha256::digest(url.as_ref().as_bytes());
        self.cache_dir.join(hex::encode(hash))
    }

    pub fn data_path(&self, market: Market) -> PathBuf {
        self.data_dir.join(format!("{}.json", market.code()))
    }
}
