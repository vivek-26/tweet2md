use anyhow::Result;

use std::fs;

mod constants;
mod helper;
mod login;
mod save;

fn main() -> Result<()> {
    let twitter_cookie = fs::metadata(helper::twitter_cookie_file().unwrap())?;
    if twitter_cookie.is_file() {
        println!("twitter cookie file exists");
        match save::save_twitter_thread("https://twitter.com/emiko_dev/status/1686740343104970753")
        {
            Ok(_) => println!("save_twitter_thread succeeded"),
            Err(e) => println!("save_twitter_thread failed: {}", e),
        }
    } else {
        println!("twitter cookie file does not exist");
        let cookies = login::twitter_login()?;
        login::save_twitter_cookies(cookies)?;
    }

    Ok(())
}
