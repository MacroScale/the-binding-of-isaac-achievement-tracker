use serde::{Serialize, Deserialize};
use anyhow;


#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailDefault{
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail{
   pub default: ThumbnailDefault, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet{
   pub publishedAt: String, 
   pub channelId: String,
   pub title: String,
   pub description: String,
   pub thumbnails: Thumbnail,
   pub channelTitle: String,
   pub liveBroadcastContent: String,
   pub publishTime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoID{
    pub kind: String, 
    pub videoId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video{
    pub kind: String, 
    pub etag: String,
    pub id: VideoID,
    pub snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoList{
    pub items: Vec<Video>,
}

impl VideoList{
    pub async fn new(channel_id: &str) -> anyhow::Result<VideoList>{

        let youtube_api_key = std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set.");

        let url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&channelId={}&type=video&order=date&maxResults=1&key={}", channel_id, youtube_api_key);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<VideoList>(&res_text)?;

        Ok(res)
    }
}
