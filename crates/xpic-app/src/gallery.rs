use crate::card::Card;
use gpui::prelude::*;
use gpui::{div, px, App, SharedString, Window};
use xpic::bing::ThumbnailParams;

const DEFAULT_CARD_WIDTH: f32 = 240.;
const DEFAULT_GAP: f32 = 16.;
const DEFAULT_PADDING_X: f32 = 24.;

#[derive(IntoElement)]
pub struct Gallery {
    images: Vec<&'static xpic::Image>,
    card_width: f32,
    gap: f32,
    padding_x: f32,
}

impl Gallery {
    pub fn new(images: Vec<&'static xpic::Image>) -> Self {
        Self {
            images,
            card_width: DEFAULT_CARD_WIDTH,
            gap: DEFAULT_GAP,
            padding_x: DEFAULT_PADDING_X,
        }
    }

    pub fn card_width(mut self, width: f32) -> Self {
        self.card_width = width;

        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;

        self
    }

    pub fn padding_x(mut self, padding: f32) -> Self {
        self.padding_x = padding;

        self
    }

    fn display_title(image: &xpic::Image) -> SharedString {
        if image.title.is_empty() || image.title == "Info" {
            SharedString::from(image.copyright.clone())
        } else {
            SharedString::from(image.title.clone())
        }
    }

    fn compute_cols(&self, viewport_width: f32) -> usize {
        let available = viewport_width - self.padding_x * 2.0;

        ((available + self.gap) / (self.card_width + self.gap))
            .floor()
            .max(1.0) as usize
    }
}

impl RenderOnce for Gallery {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut gallery = div()
            .id("gallery")
            .flex()
            .flex_wrap()
            .justify_center()
            .gap(px(self.gap))
            .px(px(self.padding_x))
            .py_6()
            .content_start();

        for image in &self.images {
            gallery = gallery.child(
                Card::new(&image.id)
                    .title(Self::display_title(image))
                    .w(px(self.card_width))
                    .width(240 * 2)
                    .height(135 * 2)
                    .no_padding(),
            );
        }

        // Fill the last row with invisible spacers for left-alignment.
        let cols = self.compute_cols(f32::from(window.viewport_size().width));
        let remainder = self.images.len() % cols;

        if remainder > 0 {
            for _ in 0..(cols - remainder) {
                gallery = gallery.child(div().w(px(self.card_width)).h(px(0.)).invisible());
            }
        }

        gallery
    }
}
