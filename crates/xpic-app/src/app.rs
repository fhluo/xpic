use crate::assets::Icon;
use crate::data;
use crate::gallery::Gallery;
use crate::market_selector::{ChangeMarket, MarketSelector};
use crate::search_bar::SearchBar;
use crate::theme::Theme;
use crate::theme_toggle::ThemeToggle;
use crate::title_bar::TitleBar;
use gpui::prelude::*;
use gpui::{div, img, px, Context, Entity, Render, Window};
use gpui_component::input::{InputEvent, InputState};
use gpui_component::scroll::ScrollableElement;
use xpic::bing::Market;

pub struct XpicApp {
    market: Market,

    search_input: Entity<InputState>,
    search_query: String,
}

impl XpicApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search wallpapers..."));

        cx.subscribe(&search_input, |this, input, event, cx| {
            if matches!(event, InputEvent::Change) {
                this.search_query = input.read(cx).value().to_string();
                cx.notify();
            }
        })
        .detach();

        XpicApp {
            market: Market::EN_US,
            search_input,
            search_query: String::new(),
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

    fn filtered_images(&self) -> Vec<&'static xpic::Image> {
        let images = data::images(self.market).unwrap_or_default();

        if self.search_query.is_empty() {
            return images.iter().collect();
        }

        let query = self.search_query.to_lowercase();

        images
            .iter()
            .filter(|img| {
                img.title.to_lowercase().contains(&query)
                    || img.copyright.to_lowercase().contains(&query)
            })
            .collect()
    }
}

impl Render for XpicApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let images = self.filtered_images();
        let is_empty = images.is_empty();

        div()
            .size_full()
            .flex()
            .flex_col()
            .relative()
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
            .child(
                div().flex_1().relative().overflow_hidden().child(
                    div()
                        .size_full()
                        .overflow_y_scrollbar()
                        .when(is_empty, |this| {
                            this.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .flex_1()
                                    .size_full()
                                    .text_color(theme.caption)
                                    .text_sm()
                                    .child("No wallpapers found"),
                            )
                        })
                        .when(!is_empty, |el| el.child(Gallery::new(images))),
                ),
            )
            .child(SearchBar::new(self.search_input.clone()))
    }
}
