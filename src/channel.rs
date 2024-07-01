use regex::Regex;
use serde::Serialize;
use crate::{channel_info::{channel_video::ChannelVideo, ChannelInfo}, helper::{initial_data, parese_channel_metadata, parese_channel_videos}, playlist::crate_client, Playlist};


#[derive(Debug, Clone, PartialEq, Serialize)]

pub struct Channel {
    channel_info: ChannelInfo,
    videos: Vec<ChannelVideo>
}


impl Channel {
    pub async fn from_url(url: url::Url) -> crate::Result<Self> {
        let list_regex = [
            r"/c/([%\d\w_\-]+)(\/.*)?",
            r"/channel/([%\w\d_\-]+)(\/.*)?",
            r"/u/([%\d\w_\-]+)(\/.*)?",
            r"/user/([%\w\d_\-]+)(\/.*)?",
            r"/@([%\w\d_\-]+)(\/.*)?",
        ];
        for regex in list_regex {
            let re = Regex::new(regex).unwrap();
            let vec = re.captures(url.path());
            if vec.is_none() {
                continue;
            }
            let vec = vec.unwrap();
            let id_raw = vec.get(1);
            if id_raw.is_none() {
                continue;
            }
            let id = id_raw.unwrap().as_str();
            return Self::from_id(id).await
        }
        Err(crate::Error::BadIdFormat)
    }

    pub async fn from_id(id: &str) -> crate::Result<Self> {
        let client = crate_client().unwrap();
        let list_channel_url = [
            "/c/",
            "/channel/",
            "/u/",
            "/user/",
            "/@", 
        ];
        let mut page_raw = "".to_string();
        for url in list_channel_url {
            let url = format!("https://www.youtube.com{url}{id}/videos");
            let req = client.get(url.clone()).send().await?;
            if !req.status().is_success() {
                continue;
            }
            page_raw = req.text().await?;
            if page_raw.matches("Something went wrong").count() == 4 {
                page_raw = "".to_string();
                continue;
            }
            break;
        }
        if page_raw.is_empty() {
            return Err(crate::Error::BadIdFormat);
        }
        let init_obj = initial_data(&page_raw).unwrap();
        let channel_info = parese_channel_metadata(&init_obj)?;
        let channel_name = channel_info.title.clone();
        let videos = Self::get_videos(init_obj, channel_name).await?;
        Ok(Self { videos, channel_info })
    }

    pub(crate) async fn get_videos(init_obj: String, channel_name: String) -> crate::Result<Vec<ChannelVideo>> {
        let mut vec_videos = Vec::new();
        let mut init_obj = init_obj;
        loop {
            let (videos, continuation) = parese_channel_videos(&init_obj, channel_name.clone());
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

    pub fn channel_info(&self) -> ChannelInfo {
        self.channel_info.clone()
    }

    pub fn videos(&self) -> Vec<ChannelVideo> {
        self.videos.clone()
    }

}
