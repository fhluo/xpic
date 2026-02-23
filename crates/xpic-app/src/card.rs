use crate::image::Image;
use crate::spinner::Spinner;
use crate::theme::Theme;
use gpui::{
    div, img, prelude::*, App, ElementId, MouseButton, SharedString, StyleRefinement, Window,
};
use gpui_component::StyledExt;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use xpic::bing::{ThumbnailParams, ThumbnailQuery};

#[derive(IntoElement)]
pub struct Card {
    id: ElementId,
    title: Option<SharedString>,
    image: Image,
    /// Shared context menu index tracker.
    context_menu_index: Option<(usize, Rc<Cell<usize>>)>,
    style: StyleRefinement,
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ThumbnailParams for Card {
    fn query(&self) -> &ThumbnailQuery {
        self.image.query()
    }

    fn query_mut(&mut self) -> &mut ThumbnailQuery {
        self.image.query_mut()
    }
}

impl Card {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();

        Self {
            id: id.clone().into(),
            title: None,
            image: Image::new(id),
            context_menu_index: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the shared context menu index tracker for this card.
    pub fn context_menu_index(mut self, idx: usize, shared: Rc<Cell<usize>>) -> Self {
        self.context_menu_index = Some((idx, shared));
        self
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let img_source = self.image.source();
        let lightened_img_source = self.image.clone().lighten_level(0.1).source();

        let img_style = StyleRefinement::default().size_full().rounded_t_sm();
        let mut img_area_style = StyleRefinement::default();
        img_area_style.style().aspect_ratio = self.image.aspect_ratio();

        div()
            .id(self.id)
            .group("card")
            .flex()
            .flex_col()
            .overflow_hidden()
            .cursor_pointer()
            .refine_style(&self.style)
            .when_some(self.context_menu_index, |this, (idx, shared)| {
                this.on_mouse_down(MouseButton::Right, move |_, _, _| {
                    shared.set(idx);
                })
            })
            .child(
                div()
                    .relative()
                    .w_full()
                    .flex_shrink_0()
                    .refine_style(&img_area_style)
                    .child(
                        img(img_source)
                            .refine_style(&img_style)
                            .with_loading(|| Loading.into_any_element()),
                    )
                    .child(
                        img(lightened_img_source)
                            .absolute()
                            .top_0()
                            .left_0()
                            .invisible()
                            .refine_style(&img_style)
                            .group_hover("card", |s| s.visible()),
                    ),
            )
            .when_some(self.title, |card, title| {
                card.child(
                    div()
                        .bg(theme.secondary)
                        .border_x_1()
                        .border_b_1()
                        .border_color(theme.border)
                        .rounded_b_sm()
                        .px_2()
                        .py_1()
                        .text_xs()
                        .text_color(theme.caption)
                        .truncate()
                        .child(title)
                        .group_hover("card", |s| s.bg(theme.secondary_hover)),
                )
            })
    }
}

#[derive(IntoElement)]
struct Loading;

impl RenderOnce for Loading {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                Spinner::new()
                    .duration(Duration::from_secs_f64(0.5))
                    .size_6(),
            )
    }
}
