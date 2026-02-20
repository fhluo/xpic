use crate::theme::Theme;
use gpui::{div, prelude::*, px, App, Entity, Styled, Window};
use gpui_component::input::{Input, InputState};
use gpui_component::{IconName, Sizable};

#[derive(IntoElement)]
pub struct SearchBar {
    input_state: Entity<InputState>,
}

impl SearchBar {
    pub fn new(state: Entity<InputState>) -> Self {
        SearchBar { input_state: state }
    }
}

impl RenderOnce for SearchBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .absolute()
            .top(px(8.))
            .left_0()
            .right_0()
            .flex()
            .justify_center()
            .child(
                div()
                    .occlude()
                    .w(px(280.))
                    .bg(theme.secondary)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border.opacity(0.5))
                    .child(
                        Input::new(&self.input_state)
                            .prefix(
                                div()
                                    .child(
                                        gpui_component::Icon::new(IconName::Search)
                                            .xsmall()
                                            .text_color(theme.caption),
                                    )
                                    .mr_0p5(),
                            )
                            .small()
                            .py(px(14.))
                            .cleanable(true)
                            .appearance(false),
                    ),
            )
    }
}
