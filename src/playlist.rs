use reqwest::Client;
use serde::{ser::SerializeTuple, Serialize};
use crate::{fetcher::{recommended_cookies, recommended_headers}, helper::{initial_data, parese_playlist_metadata, parese_playlist_videos}, playlist_info::{playlist_video::PlaylistVideo, req_json::ContinuationReq, PlaylistInfo}};



#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    playlist_info: PlaylistInfo,
    videos: Vec<PlaylistVideo>,
}

impl Serialize for Playlist {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: serde::Serializer {
        let mut map = serializer.serialize_tuple(self.videos.len())?;
        for i in &self.videos {
            map.serialize_element(&i)?;
        }
        map.end()
    }
}

fn crate_client() -> crate::Result<reqwest::Client> {
    let cookie_jar = recommended_cookies();
    let headers = recommended_headers();

    let client = Client::builder()
        .default_headers(headers)
        .cookie_provider(std::sync::Arc::new(cookie_jar))
        .build()?;
    Ok(client)
}

impl Playlist {

    pub async fn get(id: &str) -> crate::Result<Self> {
        let client = crate_client().unwrap();
        let req = client.get(format!("https://www.youtube.com/playlist?list={}", id)).send().await?.error_for_status()?;
        let body = req.text().await?;
        let init_obj = initial_data(&body);
        let mut vec_videos = Vec::new();
        if init_obj.is_none() {
            return Err(crate::Error::Custom("Failed to parse initial data".into()));
        }
        let mut init_obj = init_obj.unwrap();
        let playlist_info = parese_playlist_metadata(&init_obj)?;
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
        Ok(Self { videos: vec_videos, playlist_info })
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