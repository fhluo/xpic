use std::path::Path;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
        },
        UI::Shell::{DesktopWallpaper, IDesktopWallpaper},
    },
};

/// Sets the desktop wallpaper on all monitors using the `IDesktopWallpaper` COM interface.
pub fn set_wallpaper(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = HSTRING::from(path.as_ref().as_os_str());

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let wallpaper: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
        wallpaper.SetWallpaper(PCWSTR::null(), &path)?;

        CoUninitialize();
    };

    Ok(())
}
