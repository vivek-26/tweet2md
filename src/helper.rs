use std::path::PathBuf;

use crate::constants;

pub fn app_config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|path| path.join(constants::APP_CONFIG_DIR))
}

pub fn browser_data_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|path| {
        path.join(constants::APP_CONFIG_DIR).join(constants::HEADLESS_BROWSER_USER_DATA_DIR)
    })
}

pub fn twitter_cookie_file() -> Option<PathBuf> {
    app_config_dir().map(|path| path.join(constants::TWITTER_COOKIE_FILE))
}
