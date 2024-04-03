use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow;

use crate::models::log::Log;



#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAchievements{
    pub playerstats: PlayerStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStats{
   #[serde(rename = "steamID")]
   pub steam_id: Option<String>, 
   #[serde(rename = "gameName")]
   pub game_name: Option<String>,
   pub error: Option<String>,
   pub success: Option<bool>,
   pub achievements: Option<Vec<Achievement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Achievement{
    pub apiname: String,
    pub achieved: i32,
    pub unlocktime: i32,
}

impl PlayerAchievements{
    pub async fn new(steam_id: &i64, pool: &PgPool) -> anyhow::Result<PlayerAchievements>{

        let steam_api_key = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?appid=250900&key={}&steamid={}", steam_api_key, steam_id);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<PlayerAchievements>(&res_text);

        if res.is_err(){
            Log::send_log(&steam_id.to_string(), "player_achievements", &res.err().unwrap().to_string(), &pool).await;
            return Err(anyhow::Error::msg("Failed to get player achievements"));
        }

        let res = res.unwrap();

        Log::send_log(&steam_id.to_string(), "player_achievements", "", &pool).await;

        Ok(res)

    }
}
