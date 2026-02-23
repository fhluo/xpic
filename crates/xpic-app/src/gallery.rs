use crate::card::Card;
use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{div, px, App, SharedString, Window};
use std::sync::Arc;
use xpic::bing::ThumbnailParams;

#[derive(IntoElement)]
pub struct Gallery {
    images: Vec<Arc<xpic::Image>>,
}

impl Gallery {
    pub fn new(images: Vec<Arc<xpic::Image>>) -> Self {
        Self { images }
    }

    fn display_title(image: &xpic::Image) -> SharedString {
        if image.title.is_empty() || image.title == "Info" {
            SharedString::from(image.copyright.clone())
        } else {
            SharedString::from(image.title.clone())
        }
    }
}

impl RenderOnce for Gallery {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let mut gallery = div()
            .id("gallery")
            .flex()
            .flex_wrap()
            .justify_center()
            .gap(px(theme.gallery_gap))
            .px(px(theme.gallery_padding_x))
            .py(px(theme.gallery_padding_y))
            .content_start();

        for image in &self.images {
            gallery = gallery.child(
                Card::new(&image.id)
                    .title(Self::display_title(image))
                    .w(px(theme.card_width))
                    .width(theme.thumbnail_width)
                    .height(theme.thumbnail_height)
                    .no_padding(),
            );
        }

        // Fill the last row with invisible spacers for left-alignment.
        let available = f32::from(window.viewport_size().width) - theme.gallery_padding_x * 2.0;
        let cols = ((available + theme.gallery_gap) / (theme.card_width + theme.gallery_gap))
            .floor()
            .max(1.0) as usize;
        let remainder = self.images.len() % cols;

        if remainder > 0 {
            for _ in 0..(cols - remainder) {
                gallery = gallery.child(div().w(px(theme.card_width)).h(px(0.)).invisible());
            }
        }

        gallery
    }
}
