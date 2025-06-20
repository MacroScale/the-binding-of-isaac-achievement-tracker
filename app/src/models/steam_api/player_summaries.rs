use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow;

use crate::models::log::Log;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player{
    pub steamid: String,
    pub communityvisibilitystate: i32,
    pub profilestate: i32,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub avatarhash: String,
    pub lastlogoff: Option<i64>,
    pub personastate: i32,
    pub realname: Option<String>,
    pub primaryclanid: Option<String>,
    pub timecreated: Option<i64>,
    pub personastateflags: Option<i32>,
    pub loccountrycode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummary{
    pub response: Response,
}

impl PlayerSummary{

    pub async fn new(steam_id: &i64, pool: &PgPool) -> anyhow::Result<PlayerSummary>{

        let steam_api_key = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", steam_api_key, steam_id);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<PlayerSummary>(&res_text);

        if res.is_err(){
            Log::send_log(&steam_id.to_string(), "player_summary", &res.err().unwrap().to_string(), pool).await;
            return Err(anyhow::Error::msg("Failed to get player summary"));
        }

        let res = res.unwrap();

        Log::send_log(&steam_id.to_string(), "player_summary", "", pool).await;

        Ok(res)
    }
    

    pub async fn check(steam_id: &i64) -> anyhow::Result<PlayerSummary>{

        let steam_api_key = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", steam_api_key, steam_id);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<PlayerSummary>(&res_text);

        if res.is_err(){
            log::info!("Failed to get player summary: {:?}", res.err());
            return Err(anyhow::Error::msg("Failed to get player summary"));
        }

        let res = res.unwrap();

        Ok(res)
    }

}
