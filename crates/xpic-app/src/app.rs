use crate::assets::Icon;
use crate::theme::Theme;
use crate::theme_toggle::ThemeToggle;
use crate::title_bar::TitleBar;
use gpui::prelude::*;
use gpui::{div, img, px, Context, IntoElement, Render, Window};

pub struct XpicApp;

impl XpicApp {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        XpicApp
    }
}

impl Render for XpicApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div().child(
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
                        .child(ThemeToggle),
                ),
        )
    }
}
