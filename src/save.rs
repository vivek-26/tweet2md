use anyhow::Result;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use headless_chrome::browser::tab::{RequestInterceptor, RequestPausedDecision};
use headless_chrome::browser::transport::{SessionId, Transport};
use headless_chrome::protocol::cdp::Fetch::{
    events::RequestPausedEvent, GetResponseBody, GetResponseBodyReturnObject, RequestPattern,
    RequestStage,
};
use headless_chrome::protocol::cdp::Network::{CookieParam, ResourceType};
use headless_chrome::{Browser, LaunchOptions};
use serde_json;

use std::sync::{Arc, Mutex};

use crate::constants;
use crate::helper;

pub fn save_twitter_thread(tweet_url: &str) -> Result<()> {
    let cookie_file = std::fs::File::open(helper::twitter_cookie_file().unwrap())?;
    let cookies: Vec<CookieParam> = serde_json::from_reader(cookie_file)?;

    let browser = Browser::new(LaunchOptions {
        headless: true,
        user_data_dir: helper::browser_data_dir(),
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.set_user_agent(constants::USER_AGENT, None, None)?;
    tab.set_cookies(cookies)?;

    let patterns = vec![RequestPattern {
        url_pattern: None,
        resource_Type: Some(ResourceType::Xhr),
        request_stage: Some(RequestStage::Response),
    }];
    tab.enable_fetch(Some(&patterns), None)?;

    let tweet_details_interceptor = Arc::new(TweetDetailInterceptor::new());
    let tweet_details = tweet_details_interceptor.clone();
    tab.enable_request_interception(tweet_details_interceptor)?;
    tab.navigate_to(tweet_url)?;
    tab.wait_until_navigated()?;
    tab.close(true)?;

    match tweet_details.response.lock().unwrap().take() {
        Some(Ok(response)) => {
            if response.base_64_encoded {
                let body = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD)
                    .decode(response.body)?;
                println!("Response body: {}", String::from_utf8_lossy(&body));
            } else {
                println!("Response body: {}", response.body);
            }
        }
        Some(Err(e)) => println!("Error getting response body: {}", e),
        None => println!("No response body"),
    }

    Ok(())
}

struct TweetDetailInterceptor {
    response: Arc<Mutex<Option<Result<GetResponseBodyReturnObject>>>>,
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
            println!("TweetDetailInterceptor: Not intercepting URL: {}", event.params.request.url);
            return RequestPausedDecision::Continue(None);
        }

        println!("TweetDetailInterceptor: Intercepting URL: {}", event.params.request.url);

        let response_maybe = transport.call_method_on_target(
            session_id,
            GetResponseBody { request_id: event.params.request_id },
        );
        *self.response.lock().unwrap() = Some(response_maybe);

        RequestPausedDecision::Continue(None)
    }
}
