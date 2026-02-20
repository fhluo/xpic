#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::XpicApp;
use crate::assets::Assets;
use crate::config::Config;
use crate::theme::{apply_mica_theme, enable_mica_backdrop, Theme};
use gpui::{
    prelude::*, px, size, App, Bounds, Size, TitlebarOptions,
    WindowBackgroundAppearance, WindowBounds, WindowOptions,
};
use std::sync::LazyLock;

mod app;
mod assets;
mod card;
mod config;
mod data;
mod gallery;
mod image;
mod market_selector;
mod spinner;
mod theme;
mod theme_toggle;
mod title_bar;

pub static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to create Tokio runtime")
});

fn main() -> anyhow::Result<()> {
    let app = gpui_platform::application().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        LazyLock::force(&RUNTIME);
        cx.set_global(Config::default());
        open_main_window(cx);
    });

    Ok(())
}

fn open_main_window(cx: &App) {
    let bounds = Bounds::centered(None, size(px(1000.), px(625.)), cx);

    cx.spawn(async move |cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(Size {
                    width: px(400.),
                    height: Theme::DEFAULT_TITLE_BAR_HEIGHT,
                }),
                window_background: WindowBackgroundAppearance::MicaBackdrop,
                ..Default::default()
            },
            |window, cx| {
                enable_mica_backdrop(window);
                apply_mica_theme(window.appearance().into(), window, cx);

                cx.new(|cx| XpicApp::new(window, cx))
            },
        )?;

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}
