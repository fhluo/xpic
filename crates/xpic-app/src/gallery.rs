use crate::card::Card;
use crate::config::Config;
use crate::theme::Theme;
use crate::RUNTIME;
use anyhow::anyhow;
use gpui::prelude::*;
use gpui::{div, px, Action, App, ClipboardItem, SharedString, Window};
use gpui_component::menu::{ContextMenuExt, PopupMenuItem};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use xpic::bing::{ThumbnailParams, UrlBuilder};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
pub struct Refresh;

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
        let context_menu_index: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));

        let mut gallery = div()
            .id("gallery")
            .flex()
            .flex_wrap()
            .justify_center()
            .gap(px(theme.gallery_gap))
            .px(px(theme.gallery_padding_x))
            .py(px(theme.gallery_padding_y))
            .content_start();

        for (i, image) in self.images.iter().enumerate() {
            gallery = gallery.child(
                Card::new(&image.id)
                    .title(Self::display_title(image))
                    .context_menu_index(i, context_menu_index.clone())
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

        let images = self.images.clone();
        gallery.context_menu(move |menu, window, cx| {
            let index = context_menu_index.take();

            match index {
                Some(i) if i < images.len() => {
                    let image = &images[i];

                    menu.item(PopupMenuItem::new("Copy Title").on_click({
                        let title = image.title.clone();
                        move |_, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(title.clone()));
                        }
                    }))
                    .item(PopupMenuItem::new("Copy Copyright").on_click({
                        let copyright = image.copyright.clone();
                        move |_, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(copyright.clone()));
                        }
                    }))
                    .separator()
                    .submenu("Download", window, cx, {
                        let id = image.id.clone();
                        move |mut menu, _, _| {
                            let resolutions: &[(&str, Option<(u32, u32)>)] = &[
                                ("1920×1080", Some((1920, 1080))),
                                ("2560×1440", Some((2560, 1440))),
                                ("3840×2160", Some((3840, 2160))),
                                ("UHD", None),
                            ];

                            for &(label, resolution) in resolutions {
                                let id = id.clone();
                                menu = menu.item(PopupMenuItem::new(label).on_click(
                                    move |_, _, cx| {
                                        let _ = download(&id, resolution, cx);
                                    },
                                ));
                            }

                            menu
                        }
                    })
                }
                _ => menu.item(PopupMenuItem::new("Refresh").action(Box::new(Refresh))),
            }
        })
    }
}

fn download(id: &str, resolution: Option<(u32, u32)>, cx: &mut App) -> Result<(), anyhow::Error> {
    let mut builder = UrlBuilder::new(id);
    let mut id = xpic::ID::parse(id).ok_or_else(|| anyhow!("invalid ID"))?;
    id.uhd = true;

    if let Some((w, h)) = resolution {
        builder = builder.width(w).height(h).no_padding();

        id.uhd = false;
        id.width = Some(w as usize);
        id.height = Some(h as usize);
    }

    let url = builder.build()?;
    let cache_path = cx.global::<Config>().image_cache(&url);

    let dir = dirs::picture_dir()
        .unwrap_or_else(|| dirs::download_dir().unwrap_or_else(std::env::temp_dir));
    let filename = id.to_string();
    let receiver = cx.prompt_for_new_path(&dir, Some(&filename));

    RUNTIME.handle().spawn(async move {
        let save_path = receiver
            .await??
            .ok_or_else(|| anyhow!("failed to get save path"))?;

        if cache_path.exists() {
            return tokio::fs::copy(&cache_path, &save_path)
                .await
                .map(drop)
                .map_err(anyhow::Error::msg);
        }

        let data = reqwest::get(&url)
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        if let Some(dir) = cache_path.parent() {
            let _ = tokio::fs::create_dir_all(dir).await;
        }

        let _ = tokio::fs::write(&cache_path, &data).await;
        let _ = tokio::fs::write(&save_path, &data).await;

        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
