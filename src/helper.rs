use std::path::PathBuf;

use crate::constants;

pub fn app_config_dir() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(path.join(constants::APP_CONFIG_DIR)),
        None => None,
    }
}

pub fn browser_data_dir() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(
            path.join(constants::APP_CONFIG_DIR).join(constants::HEADLESS_BROWSER_USER_DATA_DIR),
        ),
        None => None,
    }
}

pub fn twitter_cookie_file() -> Option<PathBuf> {
    match app_config_dir() {
        Some(path) => Some(path.join(constants::TWITTER_COOKIE_FILE)),
        None => None,
    }
}
