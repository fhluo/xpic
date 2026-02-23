use crate::theme::Theme;
use gpui::{div, prelude::*, px, App, Entity, Styled, Window};
use gpui_component::input::{Input, InputState};
use gpui_component::Sizable;

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
                    .flex()
                    .items_center()
                    .pl_2()
                    .child(
                        div()
                            .font_family(Theme::icons_font())
                            .text_size(px(12.0))
                            .text_color(theme.caption)
                            .child("\u{E721}"),
                    )
                    .child(
                        Input::new(&self.input_state)
                            .small()
                            .py(px(14.))
                            .appearance(false),
                    )
                    .when(!self.input_state.read(cx).value().is_empty(), |this| {
                        this.child(
                            div()
                                .id("search-clear")
                                .cursor_pointer()
                                .pr_2()
                                .on_click(move |_, window, cx| {
                                    self.input_state.update(cx, |state, cx| {
                                        state.set_value("", window, cx);
                                    });
                                })
                                .child(
                                    div()
                                        .font_family(Theme::icons_font())
                                        .text_size(px(12.0))
                                        .text_color(theme.caption)
                                        .child("\u{E894}"),
                                ),
                        )
                    }),
            )
    }
}
