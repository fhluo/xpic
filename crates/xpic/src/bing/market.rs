use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, VariantArray};

/// Market and language codes.
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, EnumIter, EnumCount, VariantArray)]
pub enum Market {
    #[serde(rename = "da-DK")]
    DA_DK,
    #[serde(rename = "de-AT")]
    DE_AT,
    #[serde(rename = "de-CH")]
    DE_CH,
    #[serde(rename = "de-DE")]
    DE_DE,
    #[serde(rename = "en-AU")]
    EN_AU,
    #[serde(rename = "en-CA")]
    EN_CA,
    #[serde(rename = "en-GB")]
    EN_GB,
    #[serde(rename = "en-ID")]
    EN_ID,
    #[serde(rename = "en-IN")]
    EN_IN,
    #[serde(rename = "en-MY")]
    EN_MY,
    #[serde(rename = "en-NZ")]
    EN_NZ,
    #[serde(rename = "en-PH")]
    EN_PH,
    #[serde(rename = "en-US")]
    EN_US,
    #[serde(rename = "en-ZA")]
    EN_ZA,
    #[serde(rename = "es-AR")]
    ES_AR,
    #[serde(rename = "es-CL")]
    ES_CL,
    #[serde(rename = "es-ES")]
    ES_ES,
    #[serde(rename = "es-MX")]
    ES_MX,
    #[serde(rename = "es-US")]
    ES_US,
    #[serde(rename = "fi-FI")]
    FI_FI,
    #[serde(rename = "fr-BE")]
    FR_BE,
    #[serde(rename = "fr-CA")]
    FR_CA,
    #[serde(rename = "fr-CH")]
    FR_CH,
    #[serde(rename = "fr-FR")]
    FR_FR,
    #[serde(rename = "it-IT")]
    IT_IT,
    #[serde(rename = "ja-JP")]
    JA_JP,
    #[serde(rename = "ko-KR")]
    KO_KR,
    #[serde(rename = "nl-BE")]
    NL_BE,
    #[serde(rename = "nl-NL")]
    NL_NL,
    #[serde(rename = "no-NO")]
    NO_NO,
    #[serde(rename = "pl-PL")]
    PL_PL,
    #[serde(rename = "pt-BR")]
    PT_BR,
    #[serde(rename = "ru-RU")]
    RU_RU,
    #[serde(rename = "sv-SE")]
    SV_SE,
    #[serde(rename = "tr-TR")]
    TR_TR,
    #[serde(rename = "zh-CN")]
    ZH_CN,
    #[serde(rename = "zh-HK")]
    ZH_HK,
    #[serde(rename = "zh-TW")]
    ZH_TW,
}

impl Market {
    pub fn code(&self) -> &'static str {
        match self {
            Market::DA_DK => "da-DK",
            Market::DE_AT => "de-AT",
            Market::DE_CH => "de-CH",
            Market::DE_DE => "de-DE",
            Market::EN_AU => "en-AU",
            Market::EN_CA => "en-CA",
            Market::EN_GB => "en-GB",
            Market::EN_ID => "en-ID",
            Market::EN_IN => "en-IN",
            Market::EN_MY => "en-MY",
            Market::EN_NZ => "en-NZ",
            Market::EN_PH => "en-PH",
            Market::EN_US => "en-US",
            Market::EN_ZA => "en-ZA",
            Market::ES_AR => "es-AR",
            Market::ES_CL => "es-CL",
            Market::ES_ES => "es-ES",
            Market::ES_MX => "es-MX",
            Market::ES_US => "es-US",
            Market::FI_FI => "fi-FI",
            Market::FR_BE => "fr-BE",
            Market::FR_CA => "fr-CA",
            Market::FR_CH => "fr-CH",
            Market::FR_FR => "fr-FR",
            Market::IT_IT => "it-IT",
            Market::JA_JP => "ja-JP",
            Market::KO_KR => "ko-KR",
            Market::NL_BE => "nl-BE",
            Market::NL_NL => "nl-NL",
            Market::NO_NO => "no-NO",
            Market::PL_PL => "pl-PL",
            Market::PT_BR => "pt-BR",
            Market::RU_RU => "ru-RU",
            Market::SV_SE => "sv-SE",
            Market::TR_TR => "tr-TR",
            Market::ZH_CN => "zh-CN",
            Market::ZH_HK => "zh-HK",
            Market::ZH_TW => "zh-TW",
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
