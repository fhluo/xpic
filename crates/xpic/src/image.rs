use crate::bing;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub url: Url,
    pub date: String,
    pub title: String,
    pub copyright: String,
}

impl TryFrom<bing::Image> for Image {
    type Error = Box<dyn Error>;

    fn try_from(info: bing::Image) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"(?x)
(?P<title>.*?)
\s*\(
(?P<copyright>.*?)
\)
",
        )?;

        let captures = re.captures(&info.copyright).ok_or("")?;

        let r = Self {
            url: Url::parse("https://www.bing.com/")?.join(&info.url)?,
            date: info.start_date,
            title: captures["title"].to_string(),
            copyright: captures["copyright"].to_string(),
        };

        Ok(r)
    }
}

#[derive(Debug, PartialEq)]
pub struct ImageDetail {
    pub name: String,
    pub market: String,
    pub number: usize,
    pub uhd: bool,
    pub width: usize,
    pub height: usize,
    pub extension: String,
}

impl Default for ImageDetail {
    fn default() -> Self {
        Self {
            name: String::default(),
            market: String::default(),
            number: 0,
            uhd: false,
            width: 0,
            height: 0,
            extension: String::default(),
        }
    }
}

impl FromStr for ImageDetail {
    type Err = Box<dyn Error>;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
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
        )?;

        let r = match re.captures(id) {
            Some(captures) => {
                let uhd = captures.name("uhd").is_some();
                Self {
                    name: String::from(&captures["name"]),
                    market: String::from(&captures["market"]),
                    number: captures["number"].parse::<usize>()?,
                    uhd,
                    width: if uhd {
                        0
                    } else {
                        captures["width"].parse::<usize>()?
                    },
                    height: if uhd {
                        0
                    } else {
                        captures["height"].parse::<usize>()?
                    },
                    extension: String::from(&captures["extension"]),
                }
            }
            None => Default::default(),
        };

        Ok(r)
    }
}

impl Image {
    pub fn id(&self) -> Option<String> {
        self.url
            .query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, id)| id.into_owned())
    }

    pub fn detail(&self) -> Result<ImageDetail, Box<dyn Error>> {
        self.id().as_deref().unwrap_or("").parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_id() {
        let cases = vec![
            (
                "OHR.YosemiteFirefall_ROW8895162487_1920x1080.jpg",
                ImageDetail {
                    name: "YosemiteFirefall".to_string(),
                    market: "ROW".to_string(),
                    number: 8895162487,
                    width: 1920,
                    height: 1080,
                    extension: "jpg".to_string(),
                    ..Default::default()
                },
            ),
            (
                "OHR.HalfDomeYosemite_EN-US4890007214_UHD.jpg",
                ImageDetail {
                    name: "HalfDomeYosemite".to_string(),
                    market: "EN-US".to_string(),
                    number: 4890007214,
                    uhd: true,
                    extension: "jpg".to_string(),
                    ..Default::default()
                },
            ),
        ];

        for (id, expected) in cases {
            let image_detail: ImageDetail = id.parse().unwrap();
            assert_eq!(image_detail, expected);
        }
    }
}
