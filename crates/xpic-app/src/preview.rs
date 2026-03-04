use crate::image::Image;
use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{div, img, App, Size, Window};
use std::sync::Arc;
use xpic::bing::ThumbnailParams;

#[derive(Debug, Clone, IntoElement)]
pub struct Preview {
    image: Arc<xpic::Image>,
}

impl Preview {
    pub fn new(image: Arc<xpic::Image>) -> Self {
        Self { image }
    }
}

impl RenderOnce for Preview {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let Size { width, height } = window.viewport_size();

        div().flex().flex_col().items_center().child(
            img(Image::new(&self.image.id)
                .width(theme.thumbnail_width)
                .height(theme.thumbnail_height)
                .no_padding()
                .source())
            .w(width * 0.8)
            .max_h(height * 0.65)
            .rounded_lg(),
        )
    }
}
