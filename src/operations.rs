use anyhow::Result;
use colored::Colorize;
use handlebars::Handlebars;

use crate::headless_chrome;
use crate::twitter_threads::TwitterThread;
use crate::util::{self, constants};

pub fn twitter_login() -> Result<()> {
    if twitter_cookie_file_exists() {
        util::print_info(format_args!("user is already logged in to twitter"));
        return Ok(());
    }

    util::print_info(format_args!("logging in to twitter"));
    let cookies = headless_chrome::twitter_login()?;
    let cookies_str = serde_json::to_string_pretty(&cookies)?;
    std::fs::write(constants::TWITTER_COOKIE_FILE.to_path_buf(), cookies_str)?;
    Ok(())
}

pub fn save_twitter_thread(tweet_url: &str) -> Result<()> {
    if !twitter_cookie_file_exists() {
        util::print_error(format_args!(
            "user not logged in, use {} to login",
            "tweet2md login".yellow().bold()
        ));
        return Ok(());
    }

    util::print_info(format_args!("fetching tweet {}", tweet_url));
    let cookie_file = std::fs::File::open(constants::TWITTER_COOKIE_FILE.to_path_buf())?;
    let twitter_thread = headless_chrome::fetch_twitter_thread(tweet_url, cookie_file)?;
    let thread: TwitterThread = twitter_thread.try_into()?;
    util::print_info(format_args!("tweet fetched successfully, rendering markdown"));

    // render markdown using handlebars
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("twitter_thread", "./src/template.hbs")?;
    let rendered_markdown = handlebars.render("twitter_thread", &thread)?;
    std::fs::write("./tweet.md", rendered_markdown)?;
    util::print_info(format_args!("markdown rendered successfully"));

    Ok(())
}

fn twitter_cookie_file_exists() -> bool {
    std::path::Path::new(constants::TWITTER_COOKIE_FILE.to_path_buf().as_path()).is_file()
}
