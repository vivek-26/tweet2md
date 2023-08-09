use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod constants;
mod headless_chrome;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Login => {
            commands::twitter_login()?;
        }
        Command::Save { url } => {
            commands::save_twitter_thread(&url)?;
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
