#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::XpicApp;
use crate::assets::Assets;
use crate::theme::{apply_mica_theme, enable_mica_backdrop, Theme};
use gpui::{
    prelude::*, px, size, App, Application, Bounds, Size,
    TitlebarOptions, WindowBackgroundAppearance, WindowBounds, WindowOptions,
};

mod app;
mod assets;
mod theme;
mod title_bar;

fn main() -> anyhow::Result<()> {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
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
