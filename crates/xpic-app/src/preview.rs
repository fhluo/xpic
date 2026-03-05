use crate::image::Image;
use crate::spinner::Spinner;
use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{
    div, img, Action, App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable,
    FontWeight, Render, Size, Window,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use xpic::bing::ThumbnailParams;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
pub struct OpenPreview(pub usize);

pub struct Preview {
    focus_handle: FocusHandle,
    image: Arc<xpic::Image>,
    copyright: Option<xpic::Copyright>,
}

impl Focusable for Preview {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for Preview {}

impl Preview {
    pub fn new(image: Arc<xpic::Image>, cx: &mut Context<Self>) -> Self {
        let copyright = image
            .copyright_parsed
            .clone()
            .or_else(|| xpic::Copyright::parse(&image.copyright));

        Self {
            focus_handle: cx.focus_handle(),
            image,
            copyright,
        }
    }

    fn render_image(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let Size { width, height } = window.viewport_size();

        // Compute fixed display size for 16:9 aspect ratio.
        let max_w = width * 0.8;
        let max_h = height * 0.65;
        let aspect = 16.0_f32 / 9.0;
        let (w, h) = if max_w / max_h > aspect {
            (max_h * aspect, max_h)
        } else {
            (max_w, max_w / aspect)
        };

        let thumbnail_source = Image::new(&self.image.id)
            .width(theme.thumbnail_width)
            .height(theme.thumbnail_height)
            .no_padding()
            .source();

        let hd_source = Image::new(&self.image.id)
            .width(1920)
            .height(1080)
            .no_padding()
            .source();

        div()
            .relative()
            .w(w)
            .h(h)
            .rounded_lg()
            .overflow_hidden()
            .occlude()
            .child(img(thumbnail_source).size_full().rounded_lg())
            .child(
                img(hd_source)
                    .id("preview-hd")
                    .absolute()
                    .inset_0()
                    .size_full()
                    .rounded_lg()
                    .with_loading(|| Loading.into_any_element()),
            )
    }

    fn render_metadata(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .when(!self.image.title.is_empty(), |this| {
                this.child(
                    div()
                        .occlude()
                        .text_center()
                        .text_base()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.preview_title)
                        .child(self.image.title.clone()),
                )
            })
            .when_some(self.copyright.as_ref(), |this, copyright| {
                this.when(!copyright.description.is_empty(), |this| {
                    this.child(
                        div()
                            .occlude()
                            .text_center()
                            .text_sm()
                            .text_color(theme.preview_description)
                            .child(copyright.description.clone()),
                    )
                })
                .when(!copyright.copyright.is_empty(), |this| {
                    this.child(
                        div()
                            .occlude()
                            .text_center()
                            .text_xs()
                            .text_color(theme.preview_attribution)
                            .child(copyright.copyright.clone()),
                    )
                })
            })
    }
}

impl Render for Preview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

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
                    .gap_2()
                    .child(self.render_image(window, cx))
                    .child(self.render_metadata(cx)),
            )
    }
}

#[derive(IntoElement)]
struct Loading;

impl RenderOnce for Loading {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div().size_full().relative().child(
            div()
                .absolute()
                .bottom_2()
                .right_2()
                .flex()
                .items_center()
                .gap_1()
                .bg(theme.loading_bg)
                .rounded_md()
                .px_2()
                .py_1()
                .child(
                    Spinner::new()
                        .color(theme.loading_fg)
                        .duration(Duration::from_millis(800))
                        .size_4(),
                )
                .child(div().text_xs().text_color(theme.loading_fg).child("HD")),
        )
    }
}
