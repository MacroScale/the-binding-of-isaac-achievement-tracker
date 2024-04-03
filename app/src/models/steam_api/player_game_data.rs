use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow;

use crate::models::log::Log;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game{
    pub appid: i32,
    pub playtime_forever: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub game_count: Option<i32>,
    pub games: Option<Vec<Game>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerGame{
    pub response: Response,
}

impl PlayerGame{

    pub async fn new(steam_id: &i64, pool: &PgPool) -> anyhow::Result<Option<Game>>{

        let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}", STEAM_API_KEY, steam_id);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<PlayerGame>(&res_text)?;


        if res.response.games.is_none(){
            Log::send_log(&steam_id.to_string(), "player_game_data", &res_text, pool).await;
            return Err(anyhow::Error::msg("Failed to get player game data"));
        }

        let tboi_data = res.response.games.as_ref().unwrap().iter().find(|x| x.appid == 250900);

        if tboi_data.is_none(){
            Log::send_log(&steam_id.to_string(), "player_game_data", &res_text, pool).await;
            return Err(anyhow::Error::msg("Failed to get player game data"));
        }

        Log::send_log(&steam_id.to_string(), "player_game_data", "", pool).await;

        Ok(tboi_data.cloned())
    }
}
