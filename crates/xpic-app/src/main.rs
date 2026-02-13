use crate::app::XpicApp;
use crate::assets::Assets;
use gpui::{prelude::*, px, size, App, Application, Bounds, WindowBounds, WindowOptions};
use gpui_component::{Root, TitleBar};

mod app;
mod assets;

fn main() -> anyhow::Result<()> {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        open_main_window(cx);
    });

    Ok(())
}

fn open_main_window(cx: &App) {
    let bounds = Bounds::centered(None, size(px(1000.), px(625.)), cx);

    cx.spawn(async move |cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| XpicApp::new(window, cx));

                cx.new(|cx| Root::new(view, window, cx))
            },
        )?;

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}
