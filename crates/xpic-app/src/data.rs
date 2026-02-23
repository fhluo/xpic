use ahash::AHashSet;
use chrono::{Duration, Utc};
use serde::Serialize;
use std::borrow::Borrow;
use std::path::Path;
use std::sync::{Arc, LazyLock};
use xpic::bing::{Market, QueryParams};
use xpic::Image;

macro_rules! data {
    ($($market:ident => $filename:literal),* $(,)?) => {
        $(
            static $market: LazyLock<Vec<Image>> = LazyLock::new(|| {
                serde_json::from_str(include_str!(concat!("../../../data/", $filename)))
                    .expect(concat!("embedded data should be valid JSON: ", $filename))
            });
        )*

        pub const AVAILABLE_MARKETS: &'static [Market] = &[
            $(Market::$market,)*
        ];

        pub fn embedded(market: Market) -> &'static [Image] {
            match market {
                $(Market::$market => &$market,)*
                _ => &[],
            }
        }
    };
}

data! {
    DE_DE => "de-DE.json",
    EN_CA => "en-CA.json",
    EN_GB => "en-GB.json",
    EN_IN => "en-IN.json",
    EN_US => "en-US.json",
    ES_ES => "es-ES.json",
    FR_CA => "fr-CA.json",
    FR_FR => "fr-FR.json",
    IT_IT => "it-IT.json",
    JA_JP => "ja-JP.json",
    PT_BR => "pt-BR.json",
    ZH_CN => "zh-CN.json",
}

pub async fn load(path: impl AsRef<Path>) -> anyhow::Result<Vec<Image>> {
    serde_json::from_slice(&tokio::fs::read(&path).await?).map_err(anyhow::Error::msg)
}

/// Merges two image lists, deduplicating by `id`. Items from `new` take priority over `existing`.
/// The result is sorted by `start_date` descending.
pub fn merge<T>(existing: &[T], new: &[T]) -> Vec<T>
where
    T: Borrow<Image> + Clone,
{
    let mut seen = AHashSet::new();
    let mut result = Vec::with_capacity(existing.len() + new.len());

    for img in new.iter().chain(existing.iter()) {
        if seen.insert(img.borrow().id.clone()) {
            result.push(img.clone());
        }
    }

    result.sort_by(|a, b| b.borrow().start_date.cmp(&a.borrow().start_date));
    result
}

pub fn to_arc(images: &[Image]) -> Vec<Arc<Image>> {
    images.iter().cloned().map(Arc::new).collect()
}

pub fn into_arc(images: Vec<Image>) -> Vec<Arc<Image>> {
    images.into_iter().map(Arc::new).collect()
}

pub async fn save(path: impl AsRef<Path>, images: &[impl Serialize]) -> anyhow::Result<()> {
    if let Some(dir) = path.as_ref().parent() {
        let _ = tokio::fs::create_dir_all(dir).await.ok();
    }

    tokio::fs::write(path, serde_json::to_vec_pretty(images)?)
        .await
        .map_err(anyhow::Error::msg)
}

/// Returns `true` if the latest image is older than `max_age`, or the list is empty.
pub fn is_stale(images: &[impl Borrow<Image>], max_age: Duration) -> bool {
    images
        .iter()
        .max_by_key(|&img| img.borrow().full_start_date)
        .is_none_or(|latest| {
            Utc::now().signed_duration_since(latest.borrow().full_start_date) > max_age
        })
}

pub async fn fetch(market: Market) -> anyhow::Result<Vec<Image>> {
    xpic::list_images().market(market).send().await
}

pub async fn fetch_remote(market: Market) -> anyhow::Result<Vec<Image>> {
    let url = format!(
        "https://raw.githubusercontent.com/fhluo/xpic/main/data/{}.json",
        market.code()
    );

    Ok(reqwest::get(&url)
        .await?
        .error_for_status()?
        .json::<Vec<Image>>()
        .await?)
}
