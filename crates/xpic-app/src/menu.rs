use crate::config::Config;
use crate::RUNTIME;
use anyhow::anyhow;
use gpui::{App, ClipboardItem, Context, SharedString, Window};
use gpui_component::menu::{PopupMenu, PopupMenuItem};
use xpic::bing::{ThumbnailParams, UrlBuilder};

pub fn copy_item(label: impl Into<SharedString>, text: impl Into<String>) -> PopupMenuItem {
    let text = text.into();
    PopupMenuItem::new(label).on_click(move |_, _, cx| {
        cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
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
