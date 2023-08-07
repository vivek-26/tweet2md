use anyhow::Result;
use headless_chrome::protocol::cdp::Network::Cookie;
use headless_chrome::{Browser, LaunchOptions};

use std::time::Duration;

use crate::constants;

pub fn twitter_login() -> Result<Vec<Cookie>> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        user_data_dir: Some(constants::HEADLESS_BROWSER_USER_DATA_DIR.to_path_buf()),
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.navigate_to(constants::TWITTER_LOGIN_URL)?;

    tab.wait_for_element_with_custom_timeout(
        constants::TWITTER_LOGGED_IN_SELECTOR,
        Duration::from_secs(120),
    )?;

    let cookies = tab.get_cookies()?;
    tab.close(true)?;

    Ok(cookies)
}

pub fn save_twitter_cookies(cookies: Vec<Cookie>) -> Result<()> {
    let cookies_str = serde_json::to_string_pretty(&cookies)?;
    std::fs::write(constants::TWITTER_COOKIE_FILE.to_path_buf(), cookies_str)?;
    Ok(())
}
