use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, VariantArray};

/// Market and language codes.
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, EnumIter, EnumCount, VariantArray)]
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
}

impl Market {
    pub fn code(&self) -> &'static str {
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
        }
    }
}

impl Display for Market {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_market() {
        #[derive(Debug, Serialize, Deserialize)]
        struct Query {
            #[serde(rename = "mkt")]
            market: Market,
        }

        fn query(market: Market) -> Query {
            Query { market }
        }

        for market in Market::iter() {
            assert_eq!(
                serde_urlencoded::to_string(query(market)).unwrap(),
                format!("mkt={}", market.code())
            );
        }
    }
}
