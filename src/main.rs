use anyhow::Result;

use std::fs;

mod constants;
mod helper;
mod login;

fn main() -> Result<()> {
    let twitter_cookie = fs::metadata(helper::twitter_cookie_file().unwrap())?;
    if twitter_cookie.is_file() {
        println!("twitter cookie file exists");
    } else {
        println!("twitter cookie file does not exist");
        let cookies = login::twitter_login()?;
        login::save_twitter_cookies(cookies)?;
    }

    Ok(())
}
