use gpui::{hsla, px, rgba, size, App, Global, Hsla, Pixels, Size, Window, WindowAppearance};
use gpui_component::theme::{Theme as ComponentTheme, ThemeMode};
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
    pub cols: usize,
    pub rows: usize,
    pub card_width: f32,
    pub card_height: f32,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub gallery_gap: f32,
    pub gallery_padding_x: f32,
    pub gallery_padding_y: f32,
    pub foreground: Hsla,
    pub hover_bg: Hsla,
    pub active_bg: Hsla,
    pub border: Hsla,
    pub secondary: Hsla,
    pub secondary_hover: Hsla,
    pub muted: Hsla,
    pub caption: Hsla,
    pub danger: Hsla,
    pub danger_active: Hsla,
    pub danger_fg: Hsla,
}

impl Global for Theme {}

impl Theme {
    pub const DEFAULT_TITLE_BAR_HEIGHT: Pixels = px(34.);
    pub const DEFAULT_CONTROL_BUTTON_WIDTH: Pixels = px(46.);
    pub const DEFAULT_COLS: usize = 4;
    pub const DEFAULT_ROWS: usize = 3;
    pub const DEFAULT_CARD_WIDTH: f32 = 240.;
    pub const DEFAULT_CARD_HEIGHT: f32 = 160.;
    pub const DEFAULT_THUMBNAIL_WIDTH: u32 = 480;
    pub const DEFAULT_THUMBNAIL_HEIGHT: u32 = 270;
    pub const DEFAULT_GALLERY_GAP: f32 = 16.;
    pub const DEFAULT_GALLERY_PADDING_X: f32 = 60.;
    pub const DEFAULT_GALLERY_PADDING_Y: f32 = 16.;

    pub fn icons_font() -> &'static str {
        "Segoe Fluent Icons"
    }

    pub fn light() -> Self {
        Self {
            appearance: Appearance::Light,
            title_bar_height: Self::DEFAULT_TITLE_BAR_HEIGHT,
            control_button_width: Self::DEFAULT_CONTROL_BUTTON_WIDTH,
            cols: Self::DEFAULT_COLS,
            rows: Self::DEFAULT_ROWS,
            card_width: Self::DEFAULT_CARD_WIDTH,
            card_height: Self::DEFAULT_CARD_HEIGHT,
            thumbnail_width: Self::DEFAULT_THUMBNAIL_WIDTH,
            thumbnail_height: Self::DEFAULT_THUMBNAIL_HEIGHT,
            gallery_gap: Self::DEFAULT_GALLERY_GAP,
            gallery_padding_x: Self::DEFAULT_GALLERY_PADDING_X,
            gallery_padding_y: Self::DEFAULT_GALLERY_PADDING_Y,
            foreground: hsla(0., 0., 0.1, 1.0),
            hover_bg: hsla(0., 0., 0., 0.05),
            active_bg: hsla(0., 0., 0., 0.08),
            border: hsla(0., 0., 0., 0.08),
            secondary: hsla(0., 0., 0.96, 0.5),
            secondary_hover: hsla(0., 0., 1., 0.7),
            muted: hsla(0., 0., 0.94, 0.4),
            caption: hsla(0., 0., 0.3, 1.0),
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
            cols: Self::DEFAULT_COLS,
            rows: Self::DEFAULT_ROWS,
            card_width: Self::DEFAULT_CARD_WIDTH,
            card_height: Self::DEFAULT_CARD_HEIGHT,
            thumbnail_width: Self::DEFAULT_THUMBNAIL_WIDTH,
            thumbnail_height: Self::DEFAULT_THUMBNAIL_HEIGHT,
            gallery_gap: Self::DEFAULT_GALLERY_GAP,
            gallery_padding_x: Self::DEFAULT_GALLERY_PADDING_X,
            gallery_padding_y: Self::DEFAULT_GALLERY_PADDING_Y,
            foreground: hsla(0., 0., 0.95, 1.0),
            hover_bg: hsla(0., 0., 1.0, 0.08),
            active_bg: hsla(0., 0., 1.0, 0.12),
            border: hsla(0., 0., 1., 0.08),
            secondary: hsla(0., 0., 0.12, 0.5),
            secondary_hover: hsla(0., 0., 0.2, 0.6),
            muted: hsla(0., 0., 0.15, 0.4),
            caption: hsla(0., 0., 0.7, 1.0),
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
    sync_component_theme(mode, cx);

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

fn sync_component_theme(mode: Appearance, cx: &mut App) {
    let component_mode = match mode {
        Appearance::Dark => ThemeMode::Dark,
        Appearance::Light => ThemeMode::Light,
    };
    ComponentTheme::change(component_mode, None, cx);

    let theme = cx.global_mut::<ComponentTheme>();
    theme.shadow = false;

    match mode {
        Appearance::Dark => {
            theme.colors.background = hsla(0., 0., 0.05, 0.0);
            theme.colors.secondary = hsla(0., 0., 0.12, 0.5);
            theme.colors.muted = hsla(0., 0., 0.15, 0.4);
            theme.colors.popover = hsla(0., 0., 0.18, 1.0);
            theme.colors.border = hsla(0., 0., 1., 0.08);
        }
        Appearance::Light => {
            theme.colors.background = hsla(0., 0., 1., 0.0);
            theme.colors.secondary = hsla(0., 0., 0.96, 0.5);
            theme.colors.muted = hsla(0., 0., 0.94, 0.4);
            theme.colors.popover = hsla(0., 0., 0.96, 1.0);
            theme.colors.border = hsla(0., 0., 0., 0.08);
        }
    }
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
