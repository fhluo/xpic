use crate::theme::Appearance;
use gpui::{Bounds, Global, Pixels};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use xpic::bing::Market;

const APP_NAME: &str = "Xpic";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub cache_dir: PathBuf,
    pub data_dir: PathBuf,

    pub market: Market,
    pub appearance: Appearance,
    pub window_bounds: Option<Bounds<Pixels>>,
}

impl Default for Config {
    fn default() -> Self {
        let base = dirs::data_local_dir()
            .unwrap_or_else(|| std::env::temp_dir())
            .join("Xpic");

        Self {
            cache_dir: base.join("cache"),
            data_dir: base.join("data"),
            market: Market::EN_US,
            appearance: Appearance::Dark,
            window_bounds: None,
        }
    }
}

impl Global for Config {}

impl Config {
    pub fn load() -> Self {
        confy::load(APP_NAME, None).unwrap_or_default()
    }

    pub fn save(&self) {
        if let Err(err) = confy::store(APP_NAME, None, self) {
            eprintln!("Failed to save config: {err}");
        }
    }

    pub fn image_cache(&self, url: impl AsRef<str>) -> PathBuf {
        let hash = Sha256::digest(url.as_ref().as_bytes());
        self.cache_dir.join(hex::encode(hash))
    }

    pub fn data_path(&self, market: Market) -> PathBuf {
        self.data_dir.join(format!("{}.json", market.code()))
    }
}
