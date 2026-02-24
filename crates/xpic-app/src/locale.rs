use crate::data;
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{locale, DataLocale, Locale, LocaleFallbacker};
use xpic::bing::Market;

fn system_locale() -> Locale {
    sys_locale::get_locale()
        .unwrap_or_else(|| "en".to_string())
        .parse::<Locale>()
        .unwrap_or_else(|_| locale!("en"))
}

trait LocaleCandidate {
    fn locale(&self) -> Option<Locale>;
    fn data_locale(&self) -> Option<DataLocale> {
        self.locale().map(DataLocale::from)
    }
}

impl LocaleCandidate for Market {
    fn locale(&self) -> Option<Locale> {
        self.code().parse::<Locale>().ok()
    }
}

/// Negotiates the best match from a set of candidates using ICU locale fallback.
fn negotiate<T: LocaleCandidate>(candidates: &[T]) -> Option<&T> {
    let mut config = LocaleFallbackConfig::default();
    config.priority = LocaleFallbackPriority::Language;

    let mut iter = LocaleFallbacker::new()
        .for_config(config)
        .fallback_for(system_locale().into());

    loop {
        let locale = iter.get();
        if locale.is_unknown() {
            return None;
        }

        if let Some(candidate) = candidates
            .iter()
            .find(|c| c.data_locale().as_ref() == Some(locale))
        {
            return Some(candidate);
        }

        iter.step();
    }
}

/// Detects the best default market from the system locale.
pub fn default_market() -> Market {
    negotiate(data::AVAILABLE_MARKETS)
        .copied()
        .unwrap_or(Market::EN_US)
}

/// Maps a Bing market to the closest available UI locale.
pub fn from_market(market: Market) -> &'static str {
    match market {
        Market::ZH_CN => "zh-CN",
        _ => "en",
    }
}

/// Sets the locale from a market selection.
pub fn set_from_market(market: Market) {
    rust_i18n::set_locale(from_market(market));
}
