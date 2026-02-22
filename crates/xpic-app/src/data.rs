use std::sync::LazyLock;
use xpic::bing::Market;
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
