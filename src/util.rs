use colored::Colorize;

pub fn print_info(args: std::fmt::Arguments) {
    println!("{} {}", "ⓘ".green().bold(), args);
}

pub fn print_success(args: std::fmt::Arguments) {
    println!("{} {}", "✔".green().bold(), args);
}

pub fn print_error(args: std::fmt::Arguments) {
    println!("{} {}", "❌".red().bold(), args);
}

pub fn twitter_cookie_file_exists() -> bool {
    std::path::Path::new(constants::TWITTER_COOKIE_FILE.to_path_buf().as_path()).is_file()
}

pub mod constants {
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        pub static ref HOME_DIR: PathBuf = match dirs::home_dir() {
            Some(path) => path,
            None => panic!("failed to get home directory"),
        };

        // App config directory
        pub static ref APP_CONFIG_DIR: PathBuf = HOME_DIR.join(".tweet2md");

        // Headless browser user data directory
        pub static ref HEADLESS_BROWSER_USER_DATA_DIR: PathBuf = APP_CONFIG_DIR.join("chrome_data_dir");

        // Twitter cookie file name
        pub static ref TWITTER_COOKIE_FILE: PathBuf = APP_CONFIG_DIR.join("twitter_cookies.json");
    }

    // Twitter Login URL
    pub const TWITTER_LOGIN_URL: &str = "https://twitter.com/i/flow/login";

    // Twitter Logged In Selector
    pub const TWITTER_LOGGED_IN_SELECTOR: &str = r#"div[aria-label="Home timeline"]"#;

    // Twitter TweetDetail GraphQL API
    pub const TWITTER_TWEET_DETAIL_API: &str =
        "https://twitter.com/i/api/graphql/q94uRCEn65LZThakYcPT6g/TweetDetail";

    // In Headless mode, the default user agent is "HeadlessChrome". This is easily detected by websites.
    // We set the user agent to a common browser to avoid detection.
    pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36";
}
