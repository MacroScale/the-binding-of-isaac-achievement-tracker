use actix_web::{get, web, Responder, HttpResponse, HttpRequest, cookie, dev::ServiceRequest};
use serde_json;
use anyhow;
use crate::models::responses::dashboard::Profile;
use crate::models::cookie::AppCookie;
use crate::models::steam_api;

pub fn create_cookie(profile: Profile) -> cookie::Cookie<'static>{

    let app_cookie = AppCookie{
        steam_id: Some(profile.steam_id),
        player_summary: Some(profile),
    };

    let app_cookie_json = serde_json::to_string(&app_cookie).unwrap();

    let cook = cookie::Cookie::build("USER_DATA", app_cookie_json)
        .path("/")
        // CHANGE THIS TO TRUE WHEN IN PRODUCTION
        .secure(false)
        .http_only(true)
        .finish(); 
    return cook;
}

pub fn get_cookie(req: &HttpRequest) -> Option<AppCookie> {
    let cookie = req.cookie("USER_DATA");
    if cookie.is_none() {
        return None;
    }
    let user_data = serde_json::from_str::<AppCookie>(cookie.unwrap().value()).unwrap();

    return Some(user_data);
}

pub async fn update_player_summary(req: &HttpRequest) -> cookie::Cookie<'static>{
    
    let app_cookie = get_cookie(req);
    if app_cookie.is_none() {return create_cookie(Profile::new());}
    
    let app_cookie = app_cookie.unwrap();

    let summary = get_player_summary(app_cookie.steam_id.unwrap()).await;
    if summary.is_err() {return create_cookie(Profile::new());}

    let summary = summary.unwrap();
    let player_summary = summary.response.players[0].clone();
    
    let profile= Profile{
        steam_id: player_summary.steamid.parse::<i64>().unwrap(),
        username: player_summary.personaname,
        avatar_url: player_summary.avatarfull,
        country: player_summary.loccountrycode.clone().unwrap_or("MK".to_string()),
        online: player_summary.personastate == 1,
    };

    let app_cookie = AppCookie{
        steam_id: Some(profile.steam_id),
        player_summary: Some(profile),
    };

    let app_cookie_json = serde_json::to_string(&app_cookie).unwrap();
    
    let cook = cookie::Cookie::build("USER_DATA", app_cookie_json)
        .path("/")
        // CHANGE THIS TO TRUE WHEN IN PRODUCTION
        .secure(false)
        .http_only(true)
        .finish(); 
    return cook;
}


pub async fn get_steamid_from_vanity(vanity: String) -> anyhow::Result<i64>{

    let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");
    let url = format!("http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}", STEAM_API_KEY, vanity);

    //get fetch json
    let fetch = reqwest::get(&url).await?;
    let res_text = fetch.text().await?;
    let res = serde_json::from_str::<steam_api::profile_search::ProfileSearch>(&res_text)?;

    if res.response.success != 1 {
        return Err(anyhow::Error::msg("failed to get steam_id"));
    }

    let steam_id = res.response.steamid.expect("-1").parse::<i64>()?;

    Ok(steam_id)
}


pub async fn get_player_summary(steam_id: i64) -> anyhow::Result<steam_api::player_summaries::PlayerSummary>{

    let STEAM_API_KEY = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY must be set.");

    let url = format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", STEAM_API_KEY, steam_id);

    log::info!("url: {:?}", &url);

    //get fetch json
    let fetch = reqwest::get(&url).await?;
    let res_text = fetch.text().await?;
    let res = serde_json::from_str::<steam_api::player_summaries::PlayerSummary>(&res_text)?;

    if res.response.players.len() < 1 {
        return Err(anyhow::Error::msg("No player summary found"));
    }

    Ok(res)
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
pub fn get_steamid_from_url(url: &str) -> i64{
    let id = url.split("/").last().unwrap().parse::<i64>().unwrap();
    id
}
