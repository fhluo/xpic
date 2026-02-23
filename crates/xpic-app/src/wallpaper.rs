use std::path::Path;

use windows::{
    core::{HSTRING, PCWSTR},
    Storage::StorageFile,
    System::UserProfile::LockScreen,
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
    }

    let result = unsafe {
        CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
            .and_then(|wallpaper: IDesktopWallpaper| wallpaper.SetWallpaper(PCWSTR::null(), &path))
    };

    unsafe {
        CoUninitialize();
    }

    result.map_err(anyhow::Error::msg)
}

/// Sets the lock screen image using the WinRT `LockScreen` API.
pub async fn set_lock_screen(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = HSTRING::from(path.as_ref().as_os_str());

    {
        let file = StorageFile::GetFileFromPathAsync(&path)?.await?;
        LockScreen::SetImageFileAsync(&file)?
    }
    .await?;

    Ok(())
}
