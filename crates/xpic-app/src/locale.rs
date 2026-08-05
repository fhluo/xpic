use crate::data;
use arc_swap::ArcSwap;
use fluent_templates::{LanguageIdentifier, Loader, langid, static_loader};
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{DataLocale, Locale, LocaleFallbacker, locale};
use std::sync::{Arc, LazyLock};
use xpic::bing::Market;

static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
    };
}

static CURRENT_LOCALE: LazyLock<ArcSwap<LanguageIdentifier>> =
    LazyLock::new(|| ArcSwap::from_pointee(langid!("en")));

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

/// Looks up a message in the currently selected UI locale.
pub fn lookup(key: &str) -> String {
    let locale = CURRENT_LOCALE.load_full();
    LOCALES.lookup(&locale, key)
}

/// Sets the UI locale from a language identifier string.
pub fn set_locale(lang: &str) {
    let locale = lang
        .parse::<LanguageIdentifier>()
        .unwrap_or_else(|_| langid!("en"));
    CURRENT_LOCALE.store(Arc::new(locale));
}

/// Sets the locale from a market selection.
pub fn set_from_market(market: Market) {
    set_locale(from_market(market));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_locales() {
        set_locale("en");
        assert_eq!(lookup("search-placeholder"), "Search wallpapers...");
        assert_eq!(lookup("save-as"), "Save As...");

        set_locale("zh-CN");
        assert_eq!(lookup("search-placeholder"), "搜索壁纸…");
        assert_eq!(lookup("save-as"), "保存为…");
    }

    #[test]
    fn unsupported_locale_fallback() {
        set_locale("fr");
        assert_eq!(lookup("no-wallpapers-found"), "No wallpapers found");
    }
}
