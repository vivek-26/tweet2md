use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

mod headless_chrome;
mod operations;
mod twitter_threads;
mod util;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Login => match operations::twitter_login() {
            Ok(_) => util::print_success(format_args!("logged in successfully")),
            Err(err) => util::print_error(format_args!("login failed: {}", err)),
        },
        Command::Save { url, path } => match operations::save_twitter_thread(&url, &path) {
            Ok(_) => util::print_success(format_args!("thread saved to {}", path)),
            Err(err) => match err {
                operations::Error::UserNotLoggedIn => util::print_error(format_args!(
                    "{}, use {} to login",
                    err,
                    "tweet2md login".yellow().bold()
                )),
                _ => util::print_error(format_args!("failed to save thread: {}", err)),
            },
        },
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
        #[clap(short = 'u', long = "url")]
        url: String,

        /// Path to save the markdown file
        #[clap(short = 'p', long = "path", default_value = "./thread.md")]
        path: String,
    },
}
