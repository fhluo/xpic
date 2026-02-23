use crate::config::Config;
use crate::theme::{apply_mica_theme, Appearance, Theme};
use gpui::{div, prelude::*, px, App, Window};

#[derive(IntoElement)]
pub struct ThemeToggle;

impl RenderOnce for ThemeToggle {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("theme-toggle")
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .size(px(26.0))
            .rounded(px(6.0))
            .cursor_pointer()
            .hover(|s| s.bg(theme.hover_bg))
            .active(|s| s.bg(theme.active_bg))
            .child(
                div()
                    .font_family(Theme::icons_font())
                    .text_size(px(14.0))
                    .text_color(theme.foreground)
                    .child(if theme.is_dark() {
                        "\u{E706}"
                    } else {
                        "\u{E708}"
                    }),
            )
            .on_click(|_, window, cx| {
                let appearance = match cx.global::<Theme>().appearance {
                    Appearance::Light => Appearance::Dark,
                    Appearance::Dark => Appearance::Light,
                };
                cx.global_mut::<Config>().appearance = appearance;

                apply_mica_theme(appearance, window, cx);
            })
    }
}
