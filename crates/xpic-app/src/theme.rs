use gpui::{hsla, px, rgba, App, Global, Hsla, Pixels, Window, WindowAppearance};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::core::BOOL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::{
    DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE,
};
use windows::Win32::UI::Controls::MARGINS;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Appearance {
    Light,
    Dark,
}

impl From<WindowAppearance> for Appearance {
    fn from(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub appearance: Appearance,
    pub title_bar_height: Pixels,
    pub control_button_width: Pixels,
    pub foreground: Hsla,
    pub hover_bg: Hsla,
    pub active_bg: Hsla,
    pub danger: Hsla,
    pub danger_active: Hsla,
    pub danger_fg: Hsla,
}

impl Global for Theme {}

impl Theme {
    pub const DEFAULT_TITLE_BAR_HEIGHT: Pixels = px(34.);
    pub const DEFAULT_CONTROL_BUTTON_WIDTH: Pixels = px(46.);

    pub fn icons_font() -> &'static str {
        "Segoe Fluent Icons"
    }

    pub fn light() -> Self {
        Self {
            appearance: Appearance::Light,
            title_bar_height: Self::DEFAULT_TITLE_BAR_HEIGHT,
            control_button_width: Self::DEFAULT_CONTROL_BUTTON_WIDTH,
            foreground: hsla(0., 0., 0.1, 1.0),
            hover_bg: hsla(0., 0., 0., 0.05),
            active_bg: hsla(0., 0., 0., 0.08),
            danger: rgba(0xe81123ff).into(),
            danger_active: rgba(0xe81123cc).into(),
            danger_fg: gpui::white(),
        }
    }

    pub fn dark() -> Self {
        Self {
            appearance: Appearance::Dark,
            title_bar_height: Self::DEFAULT_TITLE_BAR_HEIGHT,
            control_button_width: Self::DEFAULT_CONTROL_BUTTON_WIDTH,
            foreground: hsla(0., 0., 0.95, 1.0),
            hover_bg: hsla(0., 0., 1.0, 0.08),
            active_bg: hsla(0., 0., 1.0, 0.12),
            danger: Hsla::from(rgba(0xe81123ff)),
            danger_active: Hsla::from(rgba(0xe81123cc)),
            danger_fg: gpui::white(),
        }
    }

    pub fn is_dark(&self) -> bool {
        self.appearance == Appearance::Dark
    }
}

pub fn apply_mica_theme(mode: Appearance, window: &mut Window, cx: &mut App) {
    let theme = match mode {
        Appearance::Light => Theme::light(),
        Appearance::Dark => Theme::dark(),
    };

    let is_dark = theme.is_dark();
    cx.set_global(theme);

    if let Ok(handle) = window.window_handle()
        && let RawWindowHandle::Win32(handle) = handle.as_raw()
    {
        let dark: BOOL = is_dark.into();
        unsafe {
            let _ = DwmSetWindowAttribute(
                HWND(handle.hwnd.get() as _),
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &dark as *const _ as _,
                size_of::<BOOL>() as _,
            );
        }
    }

    window.refresh();
}

pub fn enable_mica_backdrop(window: &mut Window) {
    if let Ok(handle) = window.window_handle()
        && let RawWindowHandle::Win32(handle) = handle.as_raw()
    {
        unsafe {
            let _ = DwmExtendFrameIntoClientArea(
                HWND(handle.hwnd.get() as _),
                &MARGINS {
                    cxLeftWidth: 0,
                    cxRightWidth: 0,
                    cyTopHeight: 1,
                    cyBottomHeight: 0,
                },
            );
        }
    }

    window.refresh();
}
