use crate::imports::*;

pub fn get_home_dir() -> PathBuf {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            dirs::data_local_dir().unwrap()
        } else {
            dirs::home_dir().unwrap()
        }
    }
}

/// Get the default application directory.
pub fn get_app_dir() -> PathBuf {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            get_home_dir().join("sparkle")
        } else {
            get_home_dir().join(".sparkle")
        }
    }
}
