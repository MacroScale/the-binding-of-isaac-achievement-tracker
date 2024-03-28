use serde::{Serialize, Deserialize};
use anyhow;


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

    pub async fn new(steam_id: &i64) -> anyhow::Result<PlayerSummary>{

        let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", STEAM_API_KEY, steam_id);

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
