use crate::image::Image;
use gpui::{div, prelude::*, App, ElementId, StyleRefinement, Window};
use gpui_component::StyledExt;
use xpic::bing::ThumbnailParams;

#[derive(IntoElement)]
pub struct Card {
    id: ElementId,
    image: Image,
    style: StyleRefinement,
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Card {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();

        Self {
            id: id.clone().into(),
            image: Image::new(id),
            style: StyleRefinement::default(),
        }
    }
}

impl ThumbnailParams for Card {
    fn query_mut(&mut self) -> &mut xpic::bing::ThumbnailQuery {
        self.image.query_mut()
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let lightened = self.image.clone().lighten_level(0.1);
        let style = StyleRefinement::default().size_full().rounded_sm();

        div()
            .id(self.id)
            .group("card")
            .relative()
            .overflow_hidden()
            .refine_style(&self.style)
            .rounded_sm()
            .child(self.image.into_element().refine_style(&style))
            .child(
                lightened
                    .into_element()
                    .absolute()
                    .top_0()
                    .left_0()
                    .invisible()
                    .refine_style(&style)
                    .group_hover("card", |s| s.visible()),
            )
    }
}
