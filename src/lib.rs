//! A library that tells you when swas will upload new video
//! (Also lets you search his youtube videos and get metadata about his videos)

use std::collections::HashMap;
use tinyjson::JsonValue;


/// Data related to youtube channel

#[allow(dead_code)]
#[derive(Debug)]
pub struct ChannelData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub thumbnail_src: String,
    pub video_count: String,
    pub subscriber_count: String,
    pub verified: bool,
}

/// Data related to a video

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub duration: String,
    pub snippet: String,
    pub upload_date: String,
    pub thumbnail_src: String,
    pub views: String,
}
/// Data related to youtube playlist

#[allow(dead_code)]
#[derive(Debug)]
pub struct PlaylistData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail_src: String,
    pub video_count: String,
}
/// Data related to a video uploader

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoUploaderData {
    pub username: String,
    pub url: String,
    pub verified: bool,
}
/// Data related to a playlist uploader

#[allow(dead_code)]
#[derive(Debug)]
pub struct PlaylistUploaderData {
    pub username: String,
    pub url: String,
}
/// Packs all possible types of parsed youtube items

#[derive(Debug)]
pub enum Item {
    /// Encaptulates channels
    Channel(ChannelData),
    /// Encaptulates videos
    Video(VideoData, VideoUploaderData),
    /// Encaptulates playlists
    Playlist(PlaylistData, PlaylistUploaderData),
    /// Any other types(idk) go here
    Unknown,
}
/// Returns the scraped youtube result based on the query
/// # Example
/// ```
/// let resp = swas::raw_search_with_title("Code with swastik").expect("Error fetching data");
/// println!("{:?}", resp);
/// ```
pub fn raw_search_with_title(query: &str, page: u8) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!(
        "http://youtube-scrape.herokuapp.com/api/search?q={}&page={}",
        query, page.to_string()
    ))?;
    let text = resp.text()?;
    let parsed: JsonValue = text.parse().unwrap();
    Ok(parsed)
}

pub fn get_data(query: &str, page: u8) -> Vec<Item> {
    let resp = raw_search_with_title(query, page).expect("Error fetching data");
    let items: &Vec<_> = &resp["results"].get().expect("Error parsing data");
    let mut data: Vec<Item> = Vec::new();

    for item in items {
        let simplified: &HashMap<String, JsonValue> =
            item.get().expect("Error while unpacking data");
        if simplified.contains_key("channel") {
            let channel = ChannelData {
                id: item["channel"]["id"]
                    .get::<String>()
                    .expect("Error while unpacking channel id")
                    .clone(),
                title: item["channel"]["title"]
                    .get::<String>()
                    .expect("Error while unpacking channel name")
                    .clone(),
                url: item["channel"]["url"]
                    .get::<String>()
                    .expect("Error while unpacking channel url")
                    .clone(),
                snippet: item["channel"]["snippet"]
                    .get::<String>()
                    .expect("Error while unpacking snippet")
                    .clone(),
                thumbnail_src: item["channel"]["thumbnail_src"]
                    .get::<String>()
                    .expect("Error while unpacking thumbnail url")
                    .clone(),
                video_count: item["channel"]["video_count"]
                    .get::<String>()
                    .expect("Error while unpacking video count")
                    .clone(),
                subscriber_count: item["channel"]["subscriber_count"]
                    .get::<String>()
                    .expect("Error while unpacking subscriber count")
                    .clone(),
                verified: match item["channel"]["verified"] {
                    JsonValue::Boolean(v) => v,
                    _ => panic!("Error while unpacking channel verification status"),
                },
            };
            data.push(Item::Channel(channel));
        } else if simplified.contains_key("video") {
            let video = VideoData {
                id: item["video"]["id"]
                    .get::<String>()
                    .expect("Error while unpacking video id")
                    .clone(),
                title: item["video"]["title"]
                    .get::<String>()
                    .expect("Error while unpacking video title")
                    .clone(),
                url: item["video"]["url"]
                    .get::<String>()
                    .expect("Error while unpacking video url")
                    .clone(),
                duration: item["video"]["duration"]
                    .get::<String>()
                    .expect("Error while unpacking video duration")
                    .clone(),
                upload_date: item["video"]["upload_date"]
                    .get::<String>()
                    .expect("Error while unpacking video upload date")
                    .clone(),
                snippet: item["video"]["snippet"]
                    .get::<String>()
                    .expect("Error while unpacking snippet")
                    .clone(),
                thumbnail_src: item["video"]["thumbnail_src"]
                    .get::<String>()
                    .expect("Error while unpacking thumbnail url")
                    .clone(),
                views: item["video"]["views"]
                    .get::<String>()
                    .expect("Error while unpacking channel url")
                    .clone(),
            };
            let uploader = VideoUploaderData {
                username: item["uploader"]["username"]
                    .get::<String>()
                    .expect("Error while unpacking channel url")
                    .clone(),
                url: item["uploader"]["url"]
                    .get::<String>()
                    .expect("Error while unpacking channel url")
                    .clone(),
                verified: match item["uploader"]["verified"] {
                    JsonValue::Boolean(v) => v,
                    _ => panic!("Error while unpacking channel verification status"),
                },
            };
            data.push(Item::Video(video, uploader));
        } else if simplified.contains_key("playlist") {
            let playlist = PlaylistData {
                id: item["playlist"]["id"]
                    .get::<String>()
                    .expect("Error while unpacking playlist id")
                    .clone(),
                title: item["playlist"]["title"]
                    .get::<String>()
                    .expect("Error while unpacking playlist title")
                    .clone(),
                url: item["playlist"]["url"]
                    .get::<String>()
                    .expect("Error while unpacking playlist url")
                    .clone(),
                thumbnail_src: item["playlist"]["thumbnail_src"]
                    .get::<String>()
                    .expect("Error while unpacking thumbnail url")
                    .clone(),
                video_count: item["playlist"]["video_count"]
                    .get::<String>()
                    .expect("Error while unpacking video count")
                    .clone(),
            };
            let uploader = PlaylistUploaderData {
                username: item["uploader"]["username"]
                    .get::<String>()
                    .expect("Error while unpacking channel url")
                    .clone(),
                url: item["uploader"]["url"]
                    .get::<String>()
                    .expect("Error while unpacking channel url")
                    .clone(),
            };
            data.push(Item::Playlist(playlist, uploader));
        } else {
            data.push(Item::Unknown);
        }
    }
    data
}
