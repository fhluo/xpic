use crate::assets::Icon;
use crate::data;
use crate::gallery::Gallery;
use crate::market_selector::{ChangeMarket, MarketSelector};
use crate::theme::Theme;
use crate::theme_toggle::ThemeToggle;
use crate::title_bar::TitleBar;
use gpui::prelude::*;
use gpui::{div, img, px, Context, Render, Window};
use xpic::bing::Market;

pub struct XpicApp {
    market: Market,
}

impl XpicApp {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        XpicApp {
            market: Market::EN_US,
        }
    }

    fn on_change_market(
        &mut self,
        ChangeMarket(market_code): &ChangeMarket,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Ok(market) = market_code.parse::<Market>()
            && self.market != market
        {
            self.market = market;
            cx.notify();
        }
    }
}

impl Render for XpicApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let images = data::images(self.market).unwrap_or_default();

        div()
            .size_full()
            .flex()
            .flex_col()
            .on_action(cx.listener(Self::on_change_market))
            .child(
                TitleBar::new()
                    .child(
                        div()
                            .id("title-bar-content")
                            .flex()
                            .flex_row()
                            .items_center()
                            .w_full()
                            .child(
                                div()
                                    .flex_none()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .ml_2()
                                    .size(px(18.0))
                                    .child(img(Icon::AppIcon).size_full()),
                            )
                            .child(
                                div()
                                    .ml_1p5()
                                    .text_size(px(12.0))
                                    .text_color(theme.foreground)
                                    .child("Xpic"),
                            ),
                    )
                    .child(
                        div()
                            .id("title-bar-actions")
                            .flex()
                            .flex_row()
                            .items_center()
                            .mr_1p5()
                            .h(theme.title_bar_height)
                            .child(MarketSelector::new(self.market))
                            .child(ThemeToggle),
                    ),
            )
            .child(Gallery::new(images))
    }
}
