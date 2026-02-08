use crate::bing::Format;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none, BoolFromInt};
use std::{error::Error, fmt::Display};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Market {
    #[serde(rename = "de-DE")]
    DE_DE,
    #[serde(rename = "en-CA")]
    EN_CA,
    #[serde(rename = "en-GB")]
    EN_GB,
    #[serde(rename = "en-IN")]
    EN_IN,
    #[serde(rename = "en-US")]
    EN_US,
    #[serde(rename = "es-ES")]
    ES_ES,
    #[serde(rename = "fr-CA")]
    FR_CA,
    #[serde(rename = "fr-FR")]
    FR_FR,
    #[serde(rename = "it-IT")]
    IT_IT,
    #[serde(rename = "ja-JP")]
    JA_JP,
    #[serde(rename = "ko-KR")]
    KO_KR,
    #[serde(rename = "no-NO")]
    NO_NO,
    #[serde(rename = "pt-BR")]
    PT_BR,
    #[serde(rename = "zh-CN")]
    ZH_CN,
    /// Rest of the World
    #[serde(rename = "ROW")]
    ROW,
}

impl Market {
    pub fn all() -> &'static [Market] {
        &[
            Market::DE_DE,
            Market::EN_CA,
            Market::EN_GB,
            Market::EN_IN,
            Market::EN_US,
            Market::ES_ES,
            Market::FR_CA,
            Market::FR_FR,
            Market::IT_IT,
            Market::JA_JP,
            Market::KO_KR,
            Market::NO_NO,
            Market::PT_BR,
            Market::ZH_CN,
            Market::ROW,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Market::DE_DE => "de-DE",
            Market::EN_CA => "en-CA",
            Market::EN_GB => "en-GB",
            Market::EN_IN => "en-IN",
            Market::EN_US => "en-US",
            Market::ES_ES => "es-ES",
            Market::FR_CA => "fr-CA",
            Market::FR_FR => "fr-FR",
            Market::IT_IT => "it-IT",
            Market::JA_JP => "ja-JP",
            Market::KO_KR => "ko-KR",
            Market::NO_NO => "no-NO",
            Market::PT_BR => "pt-BR",
            Market::ZH_CN => "zh-CN",
            Market::ROW => "ROW",
        }
    }
}

impl Display for Market {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

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
