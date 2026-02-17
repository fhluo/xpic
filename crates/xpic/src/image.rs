use crate::bing;
use crate::bing::Market;
use anyhow::anyhow;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub url: Url,

    pub start_date: String,
    pub full_start_date: String,
    pub end_date: String,

    pub id: String,
    #[serde(skip)]
    pub id_parsed: Option<ID>,

    pub copyright: String,
    #[serde(skip)]
    pub copyright_parsed: Option<Copyright>,
    pub copyright_link: Url,

    pub title: String,
    pub quiz_link: Url,
    pub wallpaper: bool,
    pub hash: String,
}

impl Image {
    pub fn parse(image: bing::Image) -> Result<Self, anyhow::Error> {
        let bing::Image {
            start_date,
            full_start_date,
            end_date,
            url,
            copyright,
            copyright_link,
            title,
            quiz_link,
            wallpaper,
            hash,
            ..
        } = image;

        let base = Url::parse(bing::BASE_URL)?;

        let url = base.join(&url)?;

        let id = url
            .query_pairs()
            .find_map(|(key, id)| {
                if key == "id" {
                    Some(id.into_owned())
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("missing id"))?;

        Ok(Image {
            url,
            start_date,
            full_start_date,
            end_date,
            id_parsed: ID::parse(&id),
            id,
            copyright_parsed: Copyright::parse(&copyright),
            copyright_link: base.join(&copyright_link)?,
            copyright,
            title,
            quiz_link: base.join(&quiz_link)?,
            wallpaper,
            hash,
        })
    }

    /// Returns a [`UrlBuilder`](bing::UrlBuilder) for this image's thumbnail.
    pub fn url_builder(&self) -> bing::UrlBuilder {
        bing::UrlBuilder::new(&self.id)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Copyright {
    pub description: String,
    pub copyright: String,
}

static COPYRIGHT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
(?P<description>.*?)
\s*\(
(?P<copyright>.*?)
\)
",
    )
    .unwrap()
});

impl Copyright {
    pub fn parse(s: impl AsRef<str>) -> Option<Self> {
        if let Some(captures) = COPYRIGHT_REGEX.captures(s.as_ref())
            && let (Some(description), Some(copyright)) =
                (captures.name("description"), captures.name("copyright"))
        {
            Some(Copyright {
                description: description.as_str().to_owned(),
                copyright: copyright.as_str().to_owned(),
            })
        } else {
            None
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ID {
    pub name: String,
    pub market: Option<Market>,
    pub number: usize,
    pub uhd: bool,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub extension: String,
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ID {
            name,
            market,
            number,
            uhd,
            width,
            height,
            extension,
        } = self;

        let market = if let Some(market) = market {
            market.code().to_ascii_uppercase()
        } else {
            "ROW".to_owned()
        };

        if *uhd {
            return write!(f, "OHR.{name}_{market}{number}_UHD.{extension}");
        }

        if let (Some(width), Some(height)) = (width, height) {
            write!(
                f,
                "OHR.{name}_{market}{number}_{width}x{height}.{extension}"
            )
        } else {
            Err(std::fmt::Error)
        }
    }
}

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
^OHR
\.
(?P<name>\w+)
_
(?P<market>ROW|\w{2}-\w{2})
(?P<number>\d+)
_
(
(?P<width>\d+)x(?P<height>\d+)
|
(?P<uhd>UHD)
)
\.
(?P<extension>\w+)$",
    )
    .unwrap()
});

impl ID {
    pub fn parse(id: impl AsRef<str>) -> Option<Self> {
        let captures = ID_REGEX.captures(id.as_ref())?;

        let uhd = captures.name("uhd").is_some();

        let id = Self {
            name: captures.name("name")?.as_str().to_owned(),
            market: captures.name("market")?.as_str().parse::<Market>().ok(),
            number: captures.name("number")?.as_str().parse::<usize>().ok()?,
            uhd,
            width: if uhd {
                None
            } else {
                Some(captures.name("width")?.as_str().parse::<usize>().ok()?)
            },
            height: if uhd {
                None
            } else {
                Some(captures.name("height")?.as_str().parse::<usize>().ok()?)
            },
            extension: captures.name("extension")?.as_str().to_owned(),
        };

        Some(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bing::Market;

    #[test]
    fn test_id() {
        let test_cases = vec![
            (
                "OHR.YosemiteFirefall_ROW8895162487_1920x1080.jpg",
                ID {
                    name: "YosemiteFirefall".to_string(),
                    market: None,
                    number: 8895162487,
                    width: Some(1920),
                    height: Some(1080),
                    extension: "jpg".to_string(),
                    ..Default::default()
                },
            ),
            (
                "OHR.HalfDomeYosemite_EN-US4890007214_UHD.jpg",
                ID {
                    name: "HalfDomeYosemite".to_string(),
                    market: Some(Market::EN_US),
                    number: 4890007214,
                    uhd: true,
                    extension: "jpg".to_string(),
                    ..Default::default()
                },
            ),
        ];

        for (id, expected) in test_cases {
            let parsed = ID::parse(id).expect("failed to parse id");
            assert_eq!(parsed, expected);
            assert_eq!(parsed.to_string(), id);
        }
    }
}
