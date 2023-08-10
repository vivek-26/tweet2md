use anyhow::Result;
use colored::Colorize;

use crate::constants;
use crate::headless_chrome;

pub fn twitter_login() -> Result<()> {
    if twitter_cookie_file_exists() {
        constants::print_info(format_args!("user is already logged in to twitter"));
        return Ok(());
    }

    constants::print_info(format_args!("logging in to twitter"));
    let cookies = headless_chrome::twitter_login()?;
    let cookies_str = serde_json::to_string_pretty(&cookies)?;
    std::fs::write(constants::TWITTER_COOKIE_FILE.to_path_buf(), cookies_str)?;
    Ok(())
}

pub fn save_twitter_thread(tweet_url: &str) -> Result<()> {
    if !twitter_cookie_file_exists() {
        constants::print_error(format_args!(
            "user not logged in, use {} to login",
            "tweet2md login".yellow().bold()
        ));
        return Ok(());
    }

    constants::print_info(format_args!("fetching tweet {}", tweet_url));
    let cookie_file = std::fs::File::open(constants::TWITTER_COOKIE_FILE.to_path_buf())?;
    let twitter_thread = headless_chrome::fetch_twitter_thread(tweet_url, cookie_file)?;
    constants::print_info(format_args!("tweet: {}", twitter_thread));
    Ok(())
}

fn twitter_cookie_file_exists() -> bool {
    std::path::Path::new(constants::TWITTER_COOKIE_FILE.to_path_buf().as_path()).is_file()
}
