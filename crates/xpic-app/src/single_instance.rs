use const_format::concatc;
use sha2::{Digest, Sha256};
use std::env;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::{
    core::{BOOL, HSTRING, PWSTR},
    Win32::{
        Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HWND, LPARAM},
        System::Threading::{
            CreateMutexW, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowThreadProcessId, IsIconic, IsWindowVisible, SetForegroundWindow,
            ShowWindow, SW_RESTORE,
        },
    },
};

const DEFAULT_MUTEX_NAME: &str = concatc!("Xpic-", env!("CARGO_PKG_VERSION"));

fn mutex_name() -> String {
    let Ok(path) = env::current_exe() else {
        return DEFAULT_MUTEX_NAME.to_string();
    };

    let hash = hex::encode(Sha256::digest(path.to_string_lossy().as_bytes()));

    format!("{}-{}", DEFAULT_MUTEX_NAME, hash)
}

pub fn ensure_single_instance() -> bool {
    let mutex_name = HSTRING::from(mutex_name());

    unsafe {
        if let Err(_) = CreateMutexW(None, false, &mutex_name) {
            return true;
        }

        if GetLastError() == ERROR_ALREADY_EXISTS {
            activate_existing_instance();
            false
        } else {
            true
        }
    }
}

/// Wraps `EnumWindows` to accept a closure.
/// Returns `true` to continue enumeration, `false` to stop.
fn enumerate_windows<F>(mut f: F)
where
    F: FnMut(HWND) -> bool,
{
    unsafe extern "system" fn thunk<F>(handle: HWND, data: LPARAM) -> BOOL
    where
        F: FnMut(HWND) -> bool,
    {
        let f = unsafe { &mut *(data.0 as *mut F) };
        BOOL(if f(handle) { 1 } else { 0 })
    }

    unsafe {
        let _ = EnumWindows(Some(thunk::<F>), LPARAM(&raw mut f as isize));
    }
}

fn activate_existing_instance() {
    let Ok(target_path) = env::current_exe() else {
        return;
    };

    enumerate_windows(|handle| unsafe {
        if !IsWindowVisible(handle).as_bool() {
            return true;
        }

        let mut pid = 0u32;
        GetWindowThreadProcessId(handle, Some(&mut pid));
        if pid == 0 || pid == std::process::id() {
            return true;
        }

        let Some(path) = process_exe(pid) else {
            return true;
        };

        if path != target_path {
            return true;
        }

        if IsIconic(handle).as_bool() {
            let _ = ShowWindow(handle, SW_RESTORE);
        }
        let _ = SetForegroundWindow(handle);

        false
    });
}

fn process_exe(pid: u32) -> Option<PathBuf> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;

        let mut buf = [0u16; 260];
        let mut n = buf.len() as u32;

        let result =
            QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut n);
        let _ = CloseHandle(handle);
        result.ok()?;

        Some(PathBuf::from(OsString::from_wide(&buf[..n as usize])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_mutex_name() {
        println!("{}", mutex_name());
    }
}
