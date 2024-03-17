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
    if !utils::is_steamid(steam_id) || utils::is_vanity_url(steam_id) {

        let mut steam_id = steam_id.to_string();

        if utils::is_vanity_url(&steam_id) {
            let vanity = steam_id.split("/").last().unwrap().to_owned();
            steam_id = vanity;
        }
        log::info!("vanity: {:?}", &steam_id);
                
        // check if vanity
        let vanity_id = utils::get_steamid_from_vanity(steam_id.to_string()).await;
        if vanity_id.is_err() {
            let err = Error{
                status: 400,
                error: "failed to get steam_id".to_string(),
            };
            return HttpResponse::Ok().json(err);
        }

        let summary = utils::get_player_summary(vanity_id.unwrap()).await;
        return create_profile_search_response(summary);
    } 

    // if just steam_id
    if !utils::is_steam_url(steam_id) && utils::is_steamid(steam_id){
        let summary = utils::get_player_summary(steam_id.parse::<i64>().unwrap()).await;
        return create_profile_search_response(summary); 
    }    

    // if is url and not an id
    let steam_id = utils::get_steamid_from_url(steam_id);
    let summary = utils::get_player_summary(steam_id).await;

    return create_profile_search_response(summary);
}

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
    
    let profile = Profile{
        steam_id: summary.steamid.parse::<i64>().unwrap(),
        username: summary.personaname.clone(),
        avatar_url: summary.avatarfull.clone(),
        online: summary.personastate == 1,
        country: summary.loccountrycode.clone().unwrap_or("".to_string()),
    };
    
    let steam_id_cookie = utils::create_cookie(profile.clone());

    let res = Error{
        status: 200,
        error: "".to_string(),
    };

    // send a refresh header
    HttpResponse::Ok()
        .cookie(steam_id_cookie)
        .json(res)
}

