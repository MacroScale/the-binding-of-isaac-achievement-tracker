use serde::{Deserialize};
use sqlx::PgPool;
use crate::models::steam_api::player_summaries::PlayerSummary;
use crate::models::log::Log;
use anyhow;

#[derive(Deserialize, Debug)]
pub struct Response{
    pub success: i32,
    pub steamid: Option<String>,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileSearch{
    pub response: Response,
}

impl ProfileSearch{
    pub async fn new(steam_id: &str, pool: &PgPool) -> anyhow::Result<i64>{
       let id = validate_steam_id(steam_id).await; 

       if id.is_err(){
           let err = id.err().unwrap().to_string();
           Log::send_log(&steam_id, "profile_search", &err, pool).await;
           return Err(anyhow::anyhow!(err));
       }

       let id = id.unwrap();
       Log::send_log(&steam_id, "profile_search", "", pool).await;
       Ok(id)
    }
}



pub async fn validate_steam_id(steam_id: &str) -> anyhow::Result<i64> {
    // check if vanity
    if !is_steamid(steam_id) || is_vanity_url(steam_id) {

        let mut steam_id = steam_id.to_string();

        if is_vanity_url(&steam_id) {
            let vanity = steam_id.split("/").last().unwrap().to_owned();
            steam_id = vanity;
        }
                
        // check if vanity
        let vanity_id = get_steamid_from_vanity(steam_id.to_string()).await;
        if vanity_id.is_ok() {return Ok(vanity_id.unwrap());}
    } 

    // if just steam_id
    if !is_steam_url(steam_id) && is_steamid(steam_id){
        let summary = PlayerSummary::check(&steam_id.parse::<i64>().unwrap()).await;
        if summary.is_ok() {return Ok(steam_id.parse::<i64>().unwrap());}
    }    

    // if is url and not an id
    let steam_id = get_steamid_from_url(steam_id);
    if steam_id.is_ok() {
        let steam_id = steam_id.unwrap();
        let summary = PlayerSummary::check(&steam_id).await;
        if summary.is_ok() {return Ok(steam_id);}
    }

    return Err(anyhow::anyhow!("Invalid steam id"));
}

pub fn is_steam_url(url: &str) -> bool{
    url.starts_with("https://steamcommunity.com/")
}
pub fn is_vanity_url(url: &str) -> bool{
    url.starts_with("https://steamcommunity.com/id/")
}
pub fn is_steamid(url: &str) -> bool{
    let id = url.parse::<i64>();
    if id.is_ok() && url.len() == 17 {
        return true;
    }
    false
}
pub fn get_steamid_from_url(url: &str) -> anyhow::Result<i64>{
    let id = url.split("/").last().unwrap().parse::<i64>()?;
    Ok(id)
}

pub async fn get_steamid_from_vanity(vanity: String) -> anyhow::Result<i64>{

    let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");
    let url = format!("http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}", STEAM_API_KEY, vanity);

    //get fetch json
    let fetch = reqwest::get(&url).await?;
    let res_text = fetch.text().await?;
    let res = serde_json::from_str::<ProfileSearch>(&res_text)?;

    if res.response.success != 1 {
        return Err(anyhow::Error::msg("failed to get steam_id"));
    }

    let steam_id = res.response.steamid.expect("-1").parse::<i64>()?;

    Ok(steam_id)
}
