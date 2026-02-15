use std::sync::LazyLock;
use xpic::bing::Market;
use xpic::Image;

macro_rules! data {
    ($($market:ident => $file:literal),* $(,)?) => {
        pub static MARKET_DATA: LazyLock<Vec<(Market, Vec<Image>)>> = LazyLock::new(|| {
            vec![
                $(
                    (Market::$market, serde_json::from_str(include_str!(concat!("../../../data/", $file))).unwrap()),
                )*
            ]
        });

        pub const AVAILABLE_MARKETS: &'static [Market] = &[
            $(Market::$market,)*
        ];
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

pub fn images(market: Market) -> Option<&'static [Image]> {
    MARKET_DATA
        .iter()
        .find(|(m, _)| *m == market)
        .map(|(_, images)| images.as_slice())
}
