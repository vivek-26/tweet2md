use handlebars::Handlebars;

use crate::headless_chrome;
use crate::twitter_threads::{TwitterThread, THREAD_MARKDOWN_TEMPLATE};
use crate::util::{self, constants};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("user is not logged in")]
    UserNotLoggedIn,

    #[error(transparent)]
    HeadlessBrowser(#[from] headless_chrome::Error),

    #[error("could not serialize cookies: {0}")]
    CookieSerialization(#[source] serde_json::Error),

    #[error("could not write cookies to disk: {0}")]
    CookieWrite(#[source] std::io::Error),

    #[error("could not read cookies from disk: {0}")]
    CookieRead(#[from] std::io::Error),

    #[error("could not parse thread response: {0}")]
    ParseThreadResponse(#[from] serde_json::Error),

    #[error("could not render markdown template: {0}")]
    RenderMarkdownTemplate(#[from] handlebars::RenderError),

    #[error("could not write markdown to disk: {0}")]
    MarkdownWrite(#[source] std::io::Error),
}

pub fn twitter_login() -> Result<(), Error> {
    if util::twitter_cookie_file_exists() {
        util::print_info(format_args!("twitter cookie file already exists"));
        return Ok(());
    }

    util::print_info(format_args!("logging in to twitter"));
    let cookies = headless_chrome::twitter_login().map_err(Error::HeadlessBrowser)?;
    util::print_info(format_args!("fetched cookies successfully"));

    let cookies_str = serde_json::to_string_pretty(&cookies).map_err(Error::CookieSerialization)?;
    std::fs::write(constants::TWITTER_COOKIE_FILE.to_path_buf(), cookies_str)
        .map_err(Error::CookieWrite)?;

    Ok(())
}

pub fn save_twitter_thread(tweet_url: &str, path: &str) -> Result<(), Error> {
    if !util::twitter_cookie_file_exists() {
        return Err(Error::UserNotLoggedIn);
    }

    util::print_info(format_args!("fetching tweet {}", tweet_url));

    let cookie_file = std::fs::File::open(constants::TWITTER_COOKIE_FILE.to_path_buf())
        .map_err(Error::CookieRead)?;
    let twitter_thread = headless_chrome::fetch_twitter_thread(tweet_url, cookie_file)
        .map_err(Error::HeadlessBrowser)?;
    let thread: TwitterThread = twitter_thread.try_into().map_err(Error::ParseThreadResponse)?;

    util::print_info(format_args!("tweet fetched successfully"));

    // render markdown using handlebars
    util::print_info(format_args!("rendering markdown"));

    let handlebars = Handlebars::new();
    let rendered_markdown = handlebars
        .render_template(THREAD_MARKDOWN_TEMPLATE, &thread)
        .map_err(Error::RenderMarkdownTemplate)?;

    util::print_info(format_args!("markdown rendered successfully"));

    std::fs::write(path, rendered_markdown).map_err(Error::MarkdownWrite)?;

    Ok(())
}
