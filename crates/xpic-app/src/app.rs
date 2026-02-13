use gpui::prelude::*;
use gpui::{div, Context, IntoElement, Render, Window};
use gpui_component::TitleBar;

pub struct XpicApp;

impl XpicApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        XpicApp
    }
}

impl Render for XpicApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(TitleBar::new())
    }
}
