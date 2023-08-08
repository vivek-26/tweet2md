use anyhow::Result;
use clap::{Parser, Subcommand};

use std::fs;

mod constants;
mod login;
mod save;

fn main() -> Result<()> {
    let args = Cli::parse();
    let twitter_cookie = fs::metadata(constants::TWITTER_COOKIE_FILE.to_path_buf())?;

    match args.command {
        Command::Login => {
            if twitter_cookie.is_file() {
                println!("already logged in");
            } else {
                println!("logging in to twitter");
                let cookies = login::twitter_login()?;
                login::save_twitter_cookies(cookies)?;
            }
        }
        Command::Save { url } => {
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

#[derive(Debug, Parser)]
#[clap(
    name = "tweet2md",
    version = "0.1.0",
    author = "Vivek Kumar",
    about = "Converts a twitter thread to markdown"
)]
struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Login to twitter
    Login,

    /// Save a twitter thread
    #[clap(arg_required_else_help = true)]
    Save {
        /// URL of the twitter thread
        url: String,
    },
}
