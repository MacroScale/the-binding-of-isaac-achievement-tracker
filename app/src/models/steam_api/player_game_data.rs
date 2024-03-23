use serde::{Serialize, Deserialize};
use anyhow;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game{
    pub appid: i32,
    pub playtime_forever: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub game_count: i32,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerGame{
    pub response: Response,
}

impl PlayerGame{

    pub async fn new(steam_id: &i64) -> anyhow::Result<Option<Game>>{

        let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

        let url = format!("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}", STEAM_API_KEY, steam_id);

        log::info!("url: {:?}", &url);

        //get fetch json
        let fetch = reqwest::get(&url).await?;
        let res_text = fetch.text().await?;
        let res = serde_json::from_str::<PlayerGame>(&res_text)?;

        let tboi_data = res.response.games.iter().find(|x| x.appid == 250900);

        Ok(tboi_data.cloned())
    }
}
