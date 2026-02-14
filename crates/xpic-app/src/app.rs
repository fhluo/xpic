use gpui::prelude::*;
use gpui::{div, Context, IntoElement, Render, Window};

use crate::title_bar::TitleBar;

pub struct XpicApp;

impl XpicApp {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        XpicApp
    }
}

impl Render for XpicApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(TitleBar::new())
    }
}
