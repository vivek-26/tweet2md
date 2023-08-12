use serde::de::Error as SerdeError;
use serde::Serialize;

use std::convert::TryFrom;

#[derive(Debug, Serialize)]
pub struct TwitterThread {
    pub tweet: Tweet,
    pub thread: Vec<Tweet>,
}

#[derive(Debug, Serialize)]
pub struct Tweet {
    pub id: String,
    pub index: u8,
    pub author: Author,
    pub text: String,
    pub created_at: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub name: String,
    pub handle: String,
    pub url: String,
}

pub type RawThreadResponse = serde_json::Value;

impl TryFrom<RawThreadResponse> for TwitterThread {
    type Error = serde_json::Error;

    fn try_from(value: RawThreadResponse) -> Result<TwitterThread, Self::Error> {
        let tweet_entries = value["data"]["threaded_conversation_with_injections_v2"]
            ["instructions"][0]["entries"]
            .as_array()
            .ok_or(SerdeError::custom("tweet entries missing or not an array"))?;

        // first tweet is the original tweet
        let first_tweet = tweet_entries[0]
            .as_object()
            .ok_or(SerdeError::custom("first tweet entry missing or not an object"))?;

        let tweet_data = &first_tweet["content"]["itemContent"]["tweet_results"]["result"];
        let tweet = parse_tweet(tweet_data, 1)?;

        // the rest of the tweets are the thread
        let thread_items = tweet_entries[1]["content"]["items"]
            .as_array()
            .ok_or(SerdeError::custom("thread items missing or not an array"))?;

        let thread = thread_items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let tweet_data = &item["item"]["itemContent"]["tweet_results"]["result"];
                parse_tweet(tweet_data, index as u8 + 2)
            })
            .collect::<Result<Vec<Tweet>, serde_json::Error>>()?;

        Ok(TwitterThread { tweet, thread })
    }
}

fn parse_tweet(tweet_obj: &serde_json::Value, index: u8) -> Result<Tweet, serde_json::Error> {
    let raw_tweet_object =
        tweet_obj.as_object().ok_or(SerdeError::custom("tweet object missing or not an object"))?;

    if raw_tweet_object["__typename"]
        .as_str()
        .ok_or(SerdeError::custom("__typename missing or not a string"))?
        != "Tweet"
    {
        return Err(SerdeError::custom("expected __typeName to be Tweet"));
    }

    let author_handle = raw_tweet_object["core"]["user_results"]["result"]["legacy"]["screen_name"]
        .as_str()
        .ok_or(SerdeError::custom(
            "core.user_results.result.legacy.screen_name missing or not a string",
        ))?
        .to_string();

    let tweet_id = raw_tweet_object["rest_id"]
        .as_str()
        .ok_or(SerdeError::custom("rest_id missing or not a string"))?
        .to_string();

    let author = Author {
        name: raw_tweet_object["core"]["user_results"]["result"]["legacy"]["name"]
            .as_str()
            .ok_or(SerdeError::custom(
                "core.user_results.result.legacy.name missing or not a string",
            ))?
            .to_string(),
        handle: author_handle.clone(),
        url: format!("https://twitter.com/{}", author_handle),
    };

    let tweet = Tweet {
        id: tweet_id.clone(),
        index,
        author,
        text: raw_tweet_object["legacy"]["full_text"]
            .as_str()
            .ok_or(SerdeError::custom("legacy.full_text missing or not a string"))?
            .to_string(),
        created_at: raw_tweet_object["legacy"]["created_at"]
            .as_str()
            .ok_or(SerdeError::custom("legacy.created_at missing or not a string"))?
            .to_string(),
        url: format!("https://twitter.com/{}/status/{}", author_handle, tweet_id),
    };

    Ok(tweet)
}
