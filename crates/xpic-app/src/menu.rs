use crate::config::Config;
use crate::image::fetch_cached;
use crate::wallpaper;
use crate::RUNTIME;
use anyhow::anyhow;
use gpui::{App, ClipboardItem, Context, ImageFormat, SharedString, Window};
use gpui_component::menu::{PopupMenu, PopupMenuItem};
use xpic::bing::{ThumbnailParams, UrlBuilder};

pub fn copy(label: impl Into<SharedString>, text: impl Into<String>) -> PopupMenuItem {
    let text = text.into();
    PopupMenuItem::new(label).on_click(move |_, _, cx| {
        cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
    })
}

pub fn copy_image(id: impl Into<String>) -> PopupMenuItem {
    let id = id.into();

    PopupMenuItem::new("Copy Image").on_click(move |_, _, cx| {
        let url = UrlBuilder::new(&id).build().expect("URL should be valid");
        let cache_path = cx.global::<Config>().image_cache(&url);
        let handle = RUNTIME.handle().clone();

        cx.spawn(async move |cx| {
            let bytes = handle
                .spawn(async move { fetch_cached(&url, &cache_path).await })
                .await??;

            cx.update(|cx| {
                let image = gpui::Image::from_bytes(ImageFormat::Jpeg, bytes);
                cx.write_to_clipboard(ClipboardItem::new_image(&image));
            });

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    })
}

const RESOLUTIONS: &[(&str, Option<(u32, u32)>)] = &[
    ("1920×1080", Some((1920, 1080))),
    ("2560×1440", Some((2560, 1440))),
    ("3840×2160", Some((3840, 2160))),
    ("UHD", None),
];

pub fn download_submenu(
    id: impl Into<String>,
) -> impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static {
    let id = id.into();
    move |mut menu, _, _| {
        for &(label, resolution) in RESOLUTIONS {
            let id = id.clone();
            menu = menu.item(PopupMenuItem::new(label).on_click(move |_, _, cx| {
                let _ = download(&id, resolution, cx);
            }));
        }
        menu
    }
}

pub fn download(
    id: &str,
    resolution: Option<(u32, u32)>,
    cx: &mut App,
) -> Result<(), anyhow::Error> {
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

        let data = fetch_cached(&url, &cache_path).await?;
        tokio::fs::write(&save_path, &data).await?;

        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

pub fn set_wallpaper(id: impl Into<String>) -> PopupMenuItem {
    let id = id.into();

    PopupMenuItem::new("Set as Wallpaper").on_click(move |_, _, cx| {
        let url = UrlBuilder::new(&id).build().expect("URL should be valid");
        let cache_path = cx.global::<Config>().image_cache(&url);

        RUNTIME.handle().spawn(async move {
            if let Err(err) = fetch_cached(&url, &cache_path).await {
                eprintln!("failed to fetch image: {err}");
                return;
            }

            if let Err(err) = wallpaper::set_wallpaper(&cache_path) {
                eprintln!("failed to set wallpaper: {err}");
            }
        });
    })
}
