use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{
    div, px, App, Div, ElementId, InteractiveElement, Interactivity,
    SharedString, Stateful, StyleRefinement, Styled, Window,
};
use gpui_component::menu::DropdownMenu;
use gpui_component::Selectable;

#[derive(IntoElement)]
pub struct FluentIconButton {
    base: Stateful<Div>,
    icon: SharedString,
    selected: bool,
}

impl FluentIconButton {
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        Self {
            base: div().id(id),
            icon: icon.into(),
            selected: false,
        }
    }
}

impl Styled for FluentIconButton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for FluentIconButton {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl Selectable for FluentIconButton {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;

        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl DropdownMenu for FluentIconButton {}

impl RenderOnce for FluentIconButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        self.base
            .flex()
            .items_center()
            .justify_center()
            .size(px(26.0))
            .rounded(px(6.0))
            .cursor_pointer()
            .hover(|s| s.bg(theme.hover_bg))
            .active(|s| s.bg(theme.active_bg))
            .when(self.selected, |this| this.bg(theme.active_bg))
            .child(
                div()
                    .font_family(Theme::icons_font())
                    .text_size(px(14.0))
                    .text_color(theme.foreground)
                    .child(self.icon),
            )
    }
}
