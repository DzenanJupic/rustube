use std::collections::HashMap;

use reqwest::Client;
use serde::Serialize;
use crate::{fetcher::{recommended_cookies, recommended_headers}, helper::{initial_data, parese_playlist_metadata, parese_playlist_videos}, playlist_info::{playlist_video::PlaylistVideo, req_json::ContinuationReq, PlaylistInfo}};



#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Playlist {
    playlist_info: PlaylistInfo,
    videos: Vec<PlaylistVideo>,
}

pub(crate) fn crate_client() -> crate::Result<reqwest::Client> {
    let cookie_jar = recommended_cookies();
    let headers = recommended_headers();

    let client = Client::builder()
        .default_headers(headers)
        .cookie_provider(std::sync::Arc::new(cookie_jar))
        .build()?;
    Ok(client)
}

impl Playlist {

    pub(crate) async fn get_videos(init_obj: String) -> crate::Result<Vec<PlaylistVideo>> {
        let mut vec_videos = Vec::new();
        let mut init_obj = init_obj;
        loop {
            let (videos, continuation) = parese_playlist_videos(&init_obj);
            vec_videos.extend(videos);
            if continuation.is_none() {
                break;
            }
            let continuation = continuation.unwrap();
            let init_obj_r = Playlist::get_from_continuation(&continuation).await;
            if init_obj_r.is_err() { break; }
            init_obj = init_obj_r.unwrap();
        }
        Ok(vec_videos)
    }

    pub async fn from_id(id: &str) -> crate::Result<Self> {
        let client = crate_client().unwrap();
        let req = client.get(format!("https://www.youtube.com/playlist?list={}", id)).send().await?;
        if !req.status().is_client_error() {
            return Err(crate::Error::BadIdFormat);
        }
        let body = req.text().await?;
        let init_obj = initial_data(&body);
        if init_obj.is_none() {
            return Err(crate::Error::Custom("Failed to parse initial data".into()));
        }
        let init_obj = init_obj.unwrap();
        let playlist_info = parese_playlist_metadata(&init_obj)?;
        let vec_videos = Playlist::get_videos(init_obj).await?;
        Ok(Self { videos: vec_videos, playlist_info })
    }

    pub async fn from_url(url: &url::Url) -> crate::Result<Self> {
        let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let id_raw = hash_query.get("list");
        if id_raw.is_none() {
            return Err(crate::Error::BadIdFormat);
        }
        let id = id_raw.unwrap();
        Self::from_id(id).await
    }

    pub(crate) async fn get_from_continuation(continuation: &str) -> crate::Result<String> {
        let body = ContinuationReq::new(continuation);
        let client = crate_client().unwrap();
        let req = client.post("https://www.youtube.com/youtubei/v1/browse?prettyPrint=false")
        .json(&body).send().await?;
        let body = req.text().await?;
        Ok(body)
    }


    pub fn playlist_info(&self) -> PlaylistInfo {
        self.playlist_info.clone()
    }

    pub fn videos(&self) -> Vec<PlaylistVideo> {
        self.videos.clone()
    }

}