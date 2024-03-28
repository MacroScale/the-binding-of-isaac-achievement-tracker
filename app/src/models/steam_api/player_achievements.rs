use serde::{Serialize, Deserialize};
use anyhow;


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAchievements{
    pub playerstats: PlayerStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStats{
   pub steamID: Option<String>, 
   pub gameName: Option<String>,
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
    pub async fn new(steam_id: &i64) -> anyhow::Result<PlayerAchievements>{

        let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?appid=250900&key={}&steamid={}", STEAM_API_KEY, steam_id);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<PlayerAchievements>(&res_text);

        if res.is_err(){
            log::info!("Failed to get player achievements: {:?}", res.err());
            return Err(anyhow::Error::msg("Failed to get player achievements"));
        }

        let res = res.unwrap();
        log::info!("player achievement grabbed successfully");

        Ok(res)

    }
}
