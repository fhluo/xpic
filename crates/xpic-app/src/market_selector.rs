use crate::data::AVAILABLE_MARKETS;
use crate::fluent_icon_button::FluentIconButton;
use gpui::prelude::*;
use gpui::{Action, App, Corner, Window};
use gpui_component::menu::{DropdownMenu, PopupMenuItem};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xpic::bing::Market;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
pub struct ChangeMarket(pub String);

fn market_display_name(market: Market) -> &'static str {
    match market {
        Market::DE_DE => "Deutsch",
        Market::EN_CA => "English (Canada)",
        Market::EN_GB => "English (UK)",
        Market::EN_IN => "English (India)",
        Market::EN_US => "English (US)",
        Market::ES_ES => "Español",
        Market::FR_CA => "Français (Canada)",
        Market::FR_FR => "Français",
        Market::IT_IT => "Italiano",
        Market::JA_JP => "日本語",
        Market::PT_BR => "Português",
        Market::ZH_CN => "中文",
        _ => market.code(),
    }
}

#[derive(IntoElement)]
pub struct MarketSelector {
    selected: Market,
}

impl MarketSelector {
    pub fn new(selected: Market) -> Self {
        Self { selected }
    }
}

impl RenderOnce for MarketSelector {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        FluentIconButton::new("market-selector", "\u{E774}").dropdown_menu_with_anchor(
            Corner::TopRight,
            move |menu, _, _| {
                let mut menu = menu;

                for &market in AVAILABLE_MARKETS {
                    menu = menu.item(
                        PopupMenuItem::new(market_display_name(market))
                            .checked(market == self.selected)
                            .action(Box::new(ChangeMarket(market.code().to_string()))),
                    );
                }

                menu
            },
        )
    }
}
