use serde::{Serialize, Deserialize};
use anyhow;

use crate::models::youtube_api::video_list::VideoList;

#[derive(Serialize, Deserialize, Debug)]
pub struct Youtuber{
    pub channel_id: String,
    pub channel_name: String,
    pub latest_video_title: String,
    pub latest_video_thumbnail: String,
    pub latest_video_url: String,
}

impl Youtuber{
    pub async fn new(channel_id: &str) -> anyhow::Result<Youtuber>{
    
        let video_list = VideoList::new(channel_id).await?;

        let channel_id = video_list.items[0].snippet.channel_id.to_string();
        let channel_name = video_list.items[0].snippet.channel_title.to_string();
        let latest_video_title = video_list.items[0].snippet.title.to_string();
        let latest_video_thumbnail = video_list.items[0].snippet.thumbnails.default.url.to_string();
        let latest_video_url = format!("https://www.youtube.com/watch?v={}", video_list.items[0].id.video_id.to_string());

        let youtuber_obj = Youtuber{
            channel_id,
            channel_name,
            latest_video_title,
            latest_video_thumbnail,
            latest_video_url,
        };

        Ok(youtuber_obj)
    }
}
