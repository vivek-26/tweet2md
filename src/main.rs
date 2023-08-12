use anyhow::Result;
use clap::{Parser, Subcommand};

mod headless_chrome;
mod operations;
mod twitter_threads;
mod util;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Login => {
            operations::twitter_login()?;
        }
        Command::Save { url, path } => {
            operations::save_twitter_thread(&url, &path)?;
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
        #[clap(short = 'u', long = "url")]
        url: String,

        /// Path to save the markdown file
        #[clap(short = 'p', long = "path", default_value = "./thread.md")]
        path: String,
    },
}
