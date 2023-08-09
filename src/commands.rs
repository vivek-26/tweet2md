use anyhow::Result;

use crate::constants;
use crate::headless_chrome;

pub fn twitter_login() -> Result<()> {
    if twitter_cookie_exists()? {
        println!("already logged in");
        return Ok(());
    }

    println!("logging in to twitter");
    let cookies = headless_chrome::twitter_login()?;
    let cookies_str = serde_json::to_string_pretty(&cookies)?;
    std::fs::write(constants::TWITTER_COOKIE_FILE.to_path_buf(), cookies_str)?;
    Ok(())
}

pub fn save_twitter_thread(tweet_url: &str) -> Result<()> {
    if !twitter_cookie_exists()? {
        println!("user not logged in, use `tweet2md login` to login");
        return Ok(());
    }

    println!("fetching: {}", tweet_url);
    let cookie_file = std::fs::File::open(constants::TWITTER_COOKIE_FILE.to_path_buf())?;
    let twitter_thread = headless_chrome::fetch_twitter_thread(tweet_url, cookie_file)?;
    println!("{}", twitter_thread);
    Ok(())
}

fn twitter_cookie_exists() -> Result<bool> {
    let twitter_cookie = std::fs::metadata(constants::TWITTER_COOKIE_FILE.to_path_buf())?;
    Ok(twitter_cookie.is_file())
}
