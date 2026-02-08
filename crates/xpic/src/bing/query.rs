use crate::bing::{Format, Market};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none, BoolFromInt};
use std::error::Error;

#[skip_serializing_none]
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Query {
    /// Response format
    pub format: Option<Format>,

    #[serde(rename = "idx")]
    pub index: usize,

    #[serde(rename = "n")]
    pub number: usize,

    #[serde(rename = "mkt")]
    pub market: Option<Market>,

    #[serde_as(as = "Option<BoolFromInt>")]
    pub uhd: Option<bool>,
}

impl Query {
    pub fn new(format: Option<Format>, index: usize, number: usize) -> Self {
        Self {
            format,
            index,
            number,
            ..Self::default()
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            format: Some(Format::JSON),
            index: 0,
            number: 8,
            market: Some(Market::EN_US),
            uhd: Some(true),
        }
    }
}

#[derive(Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "startdate")]
    pub start_date: String,

    #[serde(rename = "fullstartdate")]
    pub full_start_date: String,

    #[serde(rename = "enddate")]
    pub end_date: String,

    pub url: String,

    #[serde(rename = "urlbase")]
    pub url_base: String,

    pub copyright: String,

    #[serde(rename = "copyrightlink")]
    pub copyright_link: String,

    pub title: String,

    #[serde(rename = "quiz")]
    pub quiz_link: String,

    #[serde(rename = "wp")]
    pub wallpaper: bool,

    #[serde(rename = "hsh")]
    pub hash: String,

    #[serde(rename = "hs")]
    pub hotspots: Vec<Value>,
}

#[derive(Deserialize)]
struct ImagesResponse {
    images: Vec<ImageInfo>,
}

pub async fn query(query: Query) -> Result<Vec<ImageInfo>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    // Home Page Image Archive
    let request = client
        .get("https://global.bing.com/HPImageArchive.aspx")
        .query(&query)
        .build()?;

    let resp = client.execute(request).await?;

    if !resp.status().is_success() {
        return Err(format!("failed to get images response: {}", resp.status()).into());
    }

    let images = resp.json::<ImagesResponse>().await?.images;

    Ok(images)
}
