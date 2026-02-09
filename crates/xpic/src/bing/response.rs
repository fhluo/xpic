use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub images: Vec<Image>,
    pub tooltips: Option<Tooltips>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
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

    #[serde(rename = "drk")]
    pub dark: Option<isize>,

    #[serde(rename = "top")]
    pub top: Option<isize>,

    #[serde(rename = "bot")]
    pub bottom: Option<isize>,

    #[serde(rename = "hs")]
    pub hotspots: Option<Vec<Value>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltips {
    pub loading: Option<String>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub walle: Option<String>,
    pub walls: Option<String>,
}
