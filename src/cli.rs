use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = "tweet2md",
    version = "0.1.0",
    author = "Vivek Kumar",
    about = "Converts a twitter thread to markdown"
)]
pub struct Tweet2Md {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Login to twitter
    Login,

    /// Save a twitter thread
    #[clap(arg_required_else_help = true)]
    Save {
        /// URL of the twitter thread
        url: String,
    },
}
