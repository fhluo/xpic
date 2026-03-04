use crate::image::Image;
use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{div, img, Action, App, FocusHandle, Focusable, MouseButton, Size, Window};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use xpic::bing::ThumbnailParams;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
pub struct OpenPreview(pub usize);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
pub struct ClosePreview;

#[derive(Clone, IntoElement)]
pub struct Preview {
    focus_handle: FocusHandle,
    image: Arc<xpic::Image>,
}

impl Focusable for Preview {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Preview {
    pub fn new(image: Arc<xpic::Image>, focus_handle: FocusHandle) -> Self {
        Self {
            image,
            focus_handle,
        }
    }

    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        self.focus_handle.focus(window, cx);
    }
}

impl RenderOnce for Preview {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let Size { width, height } = window.viewport_size();

        div()
            .id("preview-backdrop")
            .track_focus(&self.focus_handle)
            .key_context("Preview")
            .absolute()
            .inset_0()
            .bg(theme.preview_backdrop)
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .occlude()
            .on_click(|_, window, cx: &mut App| {
                window.dispatch_action(Box::new(ClosePreview), cx);
            })
            .child(
                div()
                    .id("preview-content")
                    .flex()
                    .flex_col()
                    .flex_shrink_0()
                    .items_center()
                    .child(
                        img(Image::new(&self.image.id)
                            .width(theme.thumbnail_width)
                            .height(theme.thumbnail_height)
                            .no_padding()
                            .source())
                        .occlude()
                        .w(width * 0.8)
                        .max_h(height * 0.65)
                        .rounded_lg(),
                    ),
            )
    }
}
