use gpui::{
    div, img, prelude::*, px, App, StyleRefinement, Styled, Window, WindowControlArea,
};

use crate::assets::Icon;
use crate::theme::Theme;

#[derive(IntoElement)]
pub struct TitleBar;

impl TitleBar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("title-bar")
            .flex()
            .flex_row()
            .w_full()
            .h(theme.title_bar_height)
            .window_control_area(WindowControlArea::Drag)
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
                            .ml(px(8.0))
                            .size(px(18.0))
                            .child(img(Icon::AppIcon).size_full()),
                    )
                    .child(
                        div()
                            .ml(px(6.0))
                            .text_size(px(12.0))
                            .text_color(theme.foreground)
                            .child("Xpic"),
                    ),
            )
            .child(WindowControls)
    }
}

#[derive(IntoElement)]
struct WindowControls;

impl RenderOnce for WindowControls {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("window-controls")
            .font_family(Theme::icons_font())
            .flex()
            .flex_row()
            .justify_center()
            .h(theme.title_bar_height)
            .child(CaptionButton::Minimize)
            .child(if window.is_maximized() {
                CaptionButton::Restore
            } else {
                CaptionButton::Maximize
            })
            .child(CaptionButton::Close)
    }
}

#[derive(IntoElement)]
enum CaptionButton {
    Close,
    Minimize,
    Maximize,
    Restore,
}

impl CaptionButton {
    fn id(&self) -> &'static str {
        match self {
            Self::Close => "close",
            Self::Minimize => "minimize",
            Self::Maximize => "maximize",
            Self::Restore => "restore",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Close => "\u{e8bb}",
            Self::Minimize => "\u{e921}",
            Self::Maximize => "\u{e922}",
            Self::Restore => "\u{e923}",
        }
    }

    fn control_area(&self) -> WindowControlArea {
        match self {
            Self::Close => WindowControlArea::Close,
            Self::Minimize => WindowControlArea::Min,
            Self::Maximize | Self::Restore => WindowControlArea::Max,
        }
    }

    fn hover_style(&self, theme: &Theme) -> impl FnOnce(StyleRefinement) -> StyleRefinement {
        let (bg, fg) = match self {
            Self::Close => (theme.danger, theme.danger_fg),
            _ => (theme.hover_bg, theme.foreground),
        };

        move |s| s.bg(bg).text_color(fg)
    }

    fn active_style(&self, theme: &Theme) -> impl FnOnce(StyleRefinement) -> StyleRefinement {
        let (bg, fg) = match self {
            Self::Close => (theme.danger_active, theme.danger_fg),
            _ => (theme.active_bg, theme.foreground),
        };

        move |s| s.bg(bg).text_color(fg)
    }
}

impl RenderOnce for CaptionButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id(self.id())
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .w(theme.control_button_width)
            .h_full()
            .text_size(px(10.0))
            .text_color(theme.foreground)
            .hover(self.hover_style(theme))
            .active(self.active_style(theme))
            .window_control_area(self.control_area())
            .child(self.icon())
    }
}
