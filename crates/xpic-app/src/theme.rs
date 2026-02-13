use gpui::{App, Window};
use gpui_component::{Theme, ThemeMode};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::core::BOOL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::{
    DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE,
};
use windows::Win32::UI::Controls::MARGINS;

pub fn apply_mica_theme(mode: ThemeMode, window: &mut Window, cx: &mut App) {
    Theme::change(mode, None, cx);

    let theme = cx.global_mut::<Theme>();

    theme.colors.background = theme.colors.background.alpha(0.0);
    theme.colors.title_bar = theme.colors.title_bar.alpha(0.0);

    if let Ok(handle) = window.window_handle()
        && let RawWindowHandle::Win32(handle) = handle.as_raw()
    {
        let dark: BOOL = theme.is_dark().into();
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
