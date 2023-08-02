use anyhow::Result;
use headless_chrome::protocol::cdp::Network::Cookie;
use headless_chrome::{Browser, LaunchOptions};
use serde_json;

use std::time::Duration;

use crate::{constants, helper};

pub fn twitter_login() -> Result<Vec<Cookie>> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        user_data_dir: helper::browser_data_dir(),
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.navigate_to(constants::TWITTER_LOGIN_URL)?;

    tab.wait_for_element_with_custom_timeout(
        constants::TWITTER_LOGGED_IN_SELECTOR,
        Duration::from_secs(120),
    )?;

    let cookies = tab.get_cookies()?;
    tab.close(false)?;

    Ok(cookies)
}

pub fn save_twitter_cookies(cookies: Vec<Cookie>) -> Result<()> {
    let cookies_str = serde_json::to_string_pretty(&cookies)?;
    match helper::twitter_cookie_file() {
        Some(path) => {
            std::fs::write(path, cookies_str)?;
            Ok(())
        }
        None => panic!("Could not find home directory"),
    }
}
