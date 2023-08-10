use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use headless_chrome::{
    browser::{
        tab::{RequestInterceptor, RequestPausedDecision},
        transport::{SessionId, Transport},
    },
    protocol::cdp::{
        Fetch::{
            events::RequestPausedEvent, GetResponseBody, GetResponseBodyReturnObject,
            RequestPattern, RequestStage,
        },
        Network::{Cookie, CookieParam, ResourceType},
    },
    Browser, LaunchOptions,
};
use parking_lot::Mutex;

use crate::util::{self, constants};
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("headless browser error: {0}")]
    Browser(#[from] anyhow::Error),

    #[error("cookie deserialization error: {0}")]
    CookieDeserialize(#[from] serde_json::Error),

    #[error("could not intercept twitter api to fetch tweet details")]
    RequestInterception,

    #[error("response base64 decode error: {0}")]
    ResponseDecode(#[from] base64::DecodeError),
}

pub fn twitter_login() -> Result<Vec<Cookie>, Error> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        user_data_dir: Some(constants::HEADLESS_BROWSER_USER_DATA_DIR.to_path_buf()),
        ..Default::default()
    })
    .map_err(Error::Browser)?;

    let tab = browser.new_tab()?;
    tab.navigate_to(constants::TWITTER_LOGIN_URL).map_err(Error::Browser)?;

    tab.wait_for_element_with_custom_timeout(
        constants::TWITTER_LOGGED_IN_SELECTOR,
        Duration::from_secs(60),
    )
    .map_err(Error::Browser)?;

    let cookies = tab.get_cookies().map_err(Error::Browser)?;
    tab.close(true).map_err(Error::Browser)?;

    Ok(cookies)
}

pub fn fetch_twitter_thread(tweet_url: &str, cookie_file: File) -> Result<String, Error> {
    let cookies: Vec<CookieParam> =
        serde_json::from_reader(cookie_file).map_err(Error::CookieDeserialize)?;

    let browser = Browser::new(LaunchOptions {
        headless: true,
        user_data_dir: Some(constants::HEADLESS_BROWSER_USER_DATA_DIR.to_path_buf()),
        ..Default::default()
    })
    .map_err(Error::Browser)?;

    let tab = browser.new_tab().map_err(Error::Browser)?;
    tab.set_user_agent(constants::USER_AGENT, None, None).map_err(Error::Browser)?;
    tab.set_cookies(cookies).map_err(Error::Browser)?;

    let patterns = vec![RequestPattern {
        url_pattern: None,
        resource_Type: Some(ResourceType::Xhr),
        request_stage: Some(RequestStage::Response),
    }];
    tab.enable_fetch(Some(&patterns), None).map_err(Error::Browser)?;

    let tweet_details_interceptor = Arc::new(TweetDetailInterceptor::new());
    tab.enable_request_interception(tweet_details_interceptor.clone()).map_err(Error::Browser)?;
    tab.navigate_to(tweet_url).map_err(Error::Browser)?;
    tab.wait_until_navigated().map_err(Error::Browser)?;
    tab.close(true).map_err(Error::Browser)?;

    let response = match tweet_details_interceptor.response.lock().take() {
        Some(Ok(response)) => {
            if response.base_64_encoded {
                let decoded_body =
                    engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD)
                        .decode(response.body)
                        .map_err(Error::ResponseDecode)?;
                Ok(String::from_utf8_lossy(&decoded_body).into_owned())
            } else {
                Ok(response.body)
            }
        }
        Some(Err(e)) => Err(e),
        None => Err(Error::RequestInterception),
    };

    response
}

struct TweetDetailInterceptor {
    response: Arc<Mutex<Option<Result<GetResponseBodyReturnObject, Error>>>>,
}

impl TweetDetailInterceptor {
    fn new() -> Self {
        TweetDetailInterceptor { response: Arc::new(Mutex::new(None)) }
    }
}

impl RequestInterceptor for TweetDetailInterceptor {
    fn intercept(
        &self,
        transport: Arc<Transport>,
        session_id: SessionId,
        event: RequestPausedEvent,
    ) -> RequestPausedDecision {
        if !event.params.request.url.contains(constants::TWITTER_TWEET_DETAIL_API) {
            // TODO: This is a bit spammy, but useful for debugging. Maybe add a debug flag?
            util::print_info(format_args!("not intercepting url: {}", event.params.request.url));
            return RequestPausedDecision::Continue(None);
        }

        util::print_info(format_args!(
            "intercepted tweet detail api: {}",
            event.params.request.url
        ));

        let response_maybe = transport
            .call_method_on_target(
                session_id,
                GetResponseBody { request_id: event.params.request_id },
            )
            .map_err(Error::Browser);

        *self.response.lock() = Some(response_maybe);

        RequestPausedDecision::Continue(None)
    }
}
