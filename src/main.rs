use anyhow::Result;
use clap::Parser;

use std::fs;

mod cli;
mod constants;
mod login;
mod save;

fn main() -> Result<()> {
    let args = cli::Tweet2Md::parse();
    let twitter_cookie = fs::metadata(constants::TWITTER_COOKIE_FILE.to_path_buf())?;

    match args.command {
        cli::Command::Login => {
            if twitter_cookie.is_file() {
                println!("already logged in");
            } else {
                println!("logging in to twitter");
                let cookies = login::twitter_login()?;
                login::save_twitter_cookies(cookies)?;
            }
        }
        cli::Command::Save { url } => {
            if twitter_cookie.is_file() {
                match save::save_twitter_thread(&url) {
                    Ok(_) => println!("saved twitter thread"),
                    Err(err) => println!("failed to save twitter thread: {}", err),
                }
            } else {
                println!("user not logged in, use `tweet2md login` to login");
            }
        }
    }

    Ok(())
}
