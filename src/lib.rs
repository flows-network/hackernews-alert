use std::time::{SystemTime, UNIX_EPOCH};

use http_req::request;
use schedule_flows::schedule_cron_job;
use serde_derive::{Deserialize, Serialize};

use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};

#[no_mangle]
pub fn run() {
    // let keyword = std::env::var("KEYWORD").unwrap();

    // schedule_cron_job(String::from("30 * * * *"), String::from(keyword), callback);
    listen_to_channel("ham-5b68442", "general", callback)
}

// fn callback(keyword: Vec<u8>) {
fn callback(sm: SlackMessage) {
    // let query = String::from_utf8(keyword).unwrap();
    let query = sm.text;

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let url = format!("https://hn.algolia.com/api/v1/search_by_date?tags=story&query={query}&numericFilters=created_at_i>{timestamp}");

    let mut writer = Vec::new();
    let resp = request::get(url.clone(), &mut writer).unwrap();

    if resp.status_code().is_success() {
        let search: Search = serde_json::from_slice(&writer).unwrap();

        let hits = search.hits;
        let list = hits
            .iter()
            .filter_map(|hit| {
                let now = SystemTime::now();
                let dura = now.duration_since(UNIX_EPOCH).unwrap().as_secs() - 3600;

                if hit.created_at_i > dura as i64 {
                    let title = &hit.title;
                    let url = &hit.url.clone().unwrap_or_default();
                    let author = &hit.author;

                    Some(format!("* *{title}*<source|{url}> by {author}\n"))
                } else {
                    None
                }
            })
            .collect::<String>();

        let msg = format!("{}\nAbout {query}:\n{list}", url);
        send_message_to_channel("ham-5b68442", "general", msg);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub hits: Vec<Hit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub title: String,
    pub url: Option<String>,
    pub author: String,
    pub created_at_i: i64,
}
