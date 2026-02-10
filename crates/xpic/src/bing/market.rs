use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, EnumString, VariantArray};

/// Market and language codes.
#[allow(non_camel_case_types)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    EnumCount,
    VariantArray,
    ValueEnum,
    EnumString,
)]
#[strum(ascii_case_insensitive)]
pub enum Market {
    #[serde(rename = "da-DK")]
    #[strum(serialize = "da-DK")]
    DA_DK,

    #[serde(rename = "de-AT")]
    #[strum(serialize = "de-AT")]
    DE_AT,

    #[serde(rename = "de-CH")]
    #[strum(serialize = "de-CH")]
    DE_CH,

    #[serde(rename = "de-DE")]
    #[strum(serialize = "de-DE")]
    DE_DE,

    #[serde(rename = "en-AU")]
    #[strum(serialize = "en-AU")]
    EN_AU,

    #[serde(rename = "en-CA")]
    #[strum(serialize = "en-CA")]
    EN_CA,

    #[serde(rename = "en-GB")]
    #[strum(serialize = "en-GB")]
    EN_GB,

    #[serde(rename = "en-ID")]
    #[strum(serialize = "en-ID")]
    EN_ID,

    #[serde(rename = "en-IN")]
    #[strum(serialize = "en-IN")]
    EN_IN,

    #[serde(rename = "en-MY")]
    #[strum(serialize = "en-MY")]
    EN_MY,

    #[serde(rename = "en-NZ")]
    #[strum(serialize = "en-NZ")]
    EN_NZ,

    #[serde(rename = "en-PH")]
    #[strum(serialize = "en-PH")]
    EN_PH,

    #[serde(rename = "en-US")]
    #[strum(serialize = "en-US")]
    EN_US,

    #[serde(rename = "en-ZA")]
    #[strum(serialize = "en-ZA")]
    EN_ZA,

    #[serde(rename = "es-AR")]
    #[strum(serialize = "es-AR")]
    ES_AR,

    #[serde(rename = "es-CL")]
    #[strum(serialize = "es-CL")]
    ES_CL,

    #[serde(rename = "es-ES")]
    #[strum(serialize = "es-ES")]
    ES_ES,

    #[serde(rename = "es-MX")]
    #[strum(serialize = "es-MX")]
    ES_MX,

    #[serde(rename = "es-US")]
    #[strum(serialize = "es-US")]
    ES_US,

    #[serde(rename = "fi-FI")]
    #[strum(serialize = "fi-FI")]
    FI_FI,

    #[serde(rename = "fr-BE")]
    #[strum(serialize = "fr-BE")]
    FR_BE,

    #[serde(rename = "fr-CA")]
    #[strum(serialize = "fr-CA")]
    FR_CA,

    #[serde(rename = "fr-CH")]
    #[strum(serialize = "fr-CH")]
    FR_CH,

    #[serde(rename = "fr-FR")]
    #[strum(serialize = "fr-FR")]
    FR_FR,

    #[serde(rename = "it-IT")]
    #[strum(serialize = "it-IT")]
    IT_IT,

    #[serde(rename = "ja-JP")]
    #[strum(serialize = "ja-JP")]
    JA_JP,

    #[serde(rename = "ko-KR")]
    #[strum(serialize = "ko-KR")]
    KO_KR,

    #[serde(rename = "nl-BE")]
    #[strum(serialize = "nl-BE")]
    NL_BE,

    #[serde(rename = "nl-NL")]
    #[strum(serialize = "nl-NL")]
    NL_NL,

    #[serde(rename = "no-NO")]
    #[strum(serialize = "no-NO")]
    NO_NO,

    #[serde(rename = "pl-PL")]
    #[strum(serialize = "pl-PL")]
    PL_PL,

    #[serde(rename = "pt-BR")]
    #[strum(serialize = "pt-BR")]
    PT_BR,

    #[serde(rename = "ru-RU")]
    #[strum(serialize = "ru-RU")]
    RU_RU,

    #[serde(rename = "sv-SE")]
    #[strum(serialize = "sv-SE")]
    SV_SE,

    #[serde(rename = "tr-TR")]
    #[strum(serialize = "tr-TR")]
    TR_TR,

    #[serde(rename = "zh-CN")]
    #[strum(serialize = "zh-CN")]
    ZH_CN,

    #[serde(rename = "zh-HK")]
    #[strum(serialize = "zh-HK")]
    ZH_HK,

    #[serde(rename = "zh-TW")]
    #[strum(serialize = "zh-TW")]
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

    #[test]
    fn test_from_str() {
        // Case-insensitive parsing
        assert_eq!("en-us".parse::<Market>(), Ok(Market::EN_US));
        assert_eq!("EN-US".parse::<Market>(), Ok(Market::EN_US));
        assert_eq!("En-Us".parse::<Market>(), Ok(Market::EN_US));

        assert_eq!("zh-cn".parse::<Market>(), Ok(Market::ZH_CN));
        assert_eq!("ZH-CN".parse::<Market>(), Ok(Market::ZH_CN));

        // Invalid market
        assert!("invalid".parse::<Market>().is_err());
        assert!("xx-XX".parse::<Market>().is_err());
    }
}
