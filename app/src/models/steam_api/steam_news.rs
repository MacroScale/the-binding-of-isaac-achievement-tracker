use serde::{Serialize, Deserialize};
use anyhow;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsItem{
    pub gid: String,
    pub title: String,
    pub url: String,
    pub is_external_url: bool,
    pub author: String,
    pub contents: String,
    pub feedlabel: String,
    pub date: i64,
    pub feedname: String,
    pub feed_type: i32,
    pub appid: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppNews{
    pub appid: i32,
    pub newsitems: Vec<NewsItem>,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamNews{
    pub appnews: AppNews,
}

impl SteamNews{

    pub async fn new() -> anyhow::Result<SteamNews>{

        let url = "https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002/?appid=250900".to_string();

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<SteamNews>(&res_text)?;

        Ok(res)
    }
}
