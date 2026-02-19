use std::time::Duration;

use gpui::{
    percentage, prelude::*, svg, Animation, AnimationExt, App, Hsla, StyleRefinement,
    Styled, Transformation, Window,
};
use gpui_component::StyledExt;

use crate::theme::Theme;

#[derive(IntoElement)]
pub struct Spinner {
    color: Option<Hsla>,
    duration: Duration,
    style: StyleRefinement,
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            color: None,
            duration: Duration::from_secs(1),
            style: StyleRefinement::default(),
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);

        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;

        self
    }
}

impl RenderOnce for Spinner {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let icon_color = self.color.unwrap_or_else(|| {
            if theme.is_dark() {
                gpui::white().opacity(0.5)
            } else {
                gpui::black().opacity(0.5)
            }
        });

        svg()
            .path("icons/loader-circle.svg")
            .text_color(icon_color)
            .refine_style(&self.style)
            .with_animation(
                "spinner",
                Animation::new(self.duration).repeat(),
                move |this, delta| {
                    this.with_transformation(Transformation::rotate(percentage(delta)))
                },
            )
    }
}
