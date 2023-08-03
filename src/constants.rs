// App config directory
pub const APP_CONFIG_DIR: &str = ".tweet2md";

// Headless browser user data directory
pub const HEADLESS_BROWSER_USER_DATA_DIR: &str = "chrome_data_dir";

// Twitter Login URL
pub const TWITTER_LOGIN_URL: &str = "https://twitter.com/i/flow/login";

// Twitter Logged In Selector
pub const TWITTER_LOGGED_IN_SELECTOR: &str = r#"div[aria-label="Home timeline"]"#;

// Twitter cookie file name
pub const TWITTER_COOKIE_FILE: &str = "twitter_cookies.json";

// Twitter TweetDetail GraphQL API
pub const TWITTER_TWEET_DETAIL_API: &str =
    "https://twitter.com/i/api/graphql/q94uRCEn65LZThakYcPT6g/TweetDetail";

// In Headless mode, the default user agent is "HeadlessChrome". This is easily detected by websites.
// We set the user agent to a common browser to avoid detection.
pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36";
