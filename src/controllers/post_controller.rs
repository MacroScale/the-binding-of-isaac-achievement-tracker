use actix_web::{web, post, HttpResponse, Responder};
use reqwest;
use serde_json;
use anyhow;
use crate::controllers::utils;
use crate::models::requests::dashboard;
use crate::models::responses::dashboard::Profile;
use crate::models::steam_api;
use crate::models::responses::error::Error;

#[post("/api/profile-search")]
pub async fn profile_search(req: web::Json<dashboard::ProfileSearch>) -> impl Responder {

    let steam_id = &req.steam_id;
    
    // check if vanity
    if !is_steamid(steam_id) || is_vanity_url(steam_id) {

        let mut steam_id = steam_id.to_string();

        if is_vanity_url(&steam_id) {
            let vanity = steam_id.split("/").last().unwrap().to_owned();
            steam_id = vanity;
        }
        log::info!("vanity: {:?}", &steam_id);
                
        // check if vanity
        let vanity_id = get_steamid_from_vanity(steam_id.to_string()).await;
        if vanity_id.is_err() {
            let err = Error{
                status: 400,
                error: "failed to get steam_id".to_string(),
            };
            return HttpResponse::Ok().json(err);
        }

        let summary = get_player_summary(vanity_id.unwrap()).await;
        return create_profile_search_response(summary);
    } 

    // if just steam_id
    if !is_steam_url(steam_id) && is_steamid(steam_id){
        let summary = get_player_summary(steam_id.parse::<i64>().unwrap()).await;
        return create_profile_search_response(summary); 
    }    

    // if is url and not an id
    let steam_id = get_steamid_from_url(steam_id);
    let summary = get_player_summary(steam_id).await;

    return create_profile_search_response(summary);
}



/*
 *
 * Helper functions
 *
* */


/*
fn update_app_state_profile_data(profile: Profile, app_state: web::Data<AppState>){
}
*/

fn create_profile_search_response(summary: anyhow::Result<steam_api::player_summaries::PlayerSummary>) -> HttpResponse{

    if summary.is_err() {
        let err = Error{
            status: 400,
            error: "failed to get player summary".to_string(),
        };
        return HttpResponse::Ok().json(err);
    }

    let summary = summary.unwrap();
    let summary: steam_api::player_summaries::Player = summary.response.players[0].clone();
    
    let res = Profile{
        status: 200,
        steam_id: summary.steamid.parse::<i64>().unwrap(),
        username: summary.personaname.clone(),
        avatar_url: summary.avatarfull.clone(),
        online: summary.personastate == 1,
        country: summary.loccountrycode.clone().unwrap_or("".to_string()),
        country_flag_url: format!("https://www.countryflags.io/{}/flat/64.png", summary.loccountrycode.clone().unwrap_or("".to_string())),
    };
    
    let steam_id_cookie = utils::create_cookie(res.clone());

    HttpResponse::Ok().cookie(steam_id_cookie).json(res)
}

async fn get_steamid_from_vanity(vanity: String) -> anyhow::Result<i64>{

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


async fn get_player_summary(steam_id: i64) -> anyhow::Result<steam_api::player_summaries::PlayerSummary>{

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

fn is_steam_url(url: &str) -> bool{
    url.starts_with("https://steamcommunity.com/")
}
fn is_vanity_url(url: &str) -> bool{
    url.starts_with("https://steamcommunity.com/id/")
}
fn is_steamid(url: &str) -> bool{
    let id = url.parse::<i64>();
    if id.is_ok() && url.len() == 17 {
        return true;
    }
    false
}
fn get_steamid_from_url(url: &str) -> i64{
    let id = url.split("/").last().unwrap().parse::<i64>().unwrap();
    id
}
