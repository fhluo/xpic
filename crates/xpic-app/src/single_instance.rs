use const_format::concatc;
use sha2::{Digest, Sha256};
use std::env;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::CreateMutexW,
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
            false
        } else {
            true
        }
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
