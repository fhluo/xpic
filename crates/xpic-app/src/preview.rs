use crate::image::Image;
use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{
    div, img, Action, App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, Render,
    Size, Window,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use xpic::bing::ThumbnailParams;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
pub struct OpenPreview(pub usize);

pub struct Preview {
    focus_handle: FocusHandle,
    image: Arc<xpic::Image>,
}

impl Focusable for Preview {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for Preview {}

impl Preview {
    pub fn new(image: Arc<xpic::Image>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            image,
        }
    }
}

impl Render for Preview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .on_click(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
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
