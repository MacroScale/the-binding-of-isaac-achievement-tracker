use actix_web::{web, post, HttpResponse, Responder, cookie};
use crate::models::requests::dashboard;
use crate::models::responses::error::Error;
use crate::models::steam_api::profile_search::ProfileSearch;
use crate::models::steam_api::player_info::PlayerInfo;
use crate::models::cookie::AchievementCookie;
use crate::models::app_data::AppData;

#[post("/api/profile-search")]
pub async fn profile_search(req: web::Json<dashboard::ProfileSearch>, app_data: web::Data<AppData>) -> impl Responder {

    let pool = &app_data.pool;

    let steam_id = &req.steam_id;
    let id = ProfileSearch::new(steam_id, pool).await;

    if id.is_err() {

        let id_err = id.err().unwrap().to_string();
        let err = Error{
            status: 500,
            message: id_err.clone(),
        };

        return HttpResponse::Ok().json(err);
    }

    let id = id.unwrap();
    
    let info = PlayerInfo::new(&id, pool).await;

    if info.is_err() {
        let info_err = info.err().unwrap().to_string();

        let err = Error{
            status: 500,
            message: info_err.clone(),
        };
        return HttpResponse::Ok().json(err);
    }

    let info = info.unwrap();

    let completed = info
        .player_achievements
        .playerstats
        .achievements
        .clone()
        .unwrap()
        .iter()
        .filter(|x| x.achieved == 1).count() as i32;

    let hours_played = info.player_game_data.clone().unwrap().playtime_forever / 60;

    let c = create_hours_played_cookie(&steam_id, &completed, &hours_played);

    return HttpResponse::Ok().cookie(c).json(info);
}

fn create_hours_played_cookie(steam_id: &str, completed: &i32, hours_played: &i64) -> cookie::Cookie<'static>{


    let completed = completed.clone();
    let steam_id = steam_id.clone().to_string();
    let hours_played = hours_played.clone();

    let percentage = (completed as f32 / 637.0) * 100.0;
    let percentage = percentage as i32;
    
    let cookie = AchievementCookie{
        steam_id,
        completed,
        percentage,
        hours_played,
    };

    let cookie_str = serde_json::to_string(&cookie).unwrap();

    let c = cookie::Cookie::build("achievement", cookie_str)
        .path("/")
        // CHANGE THIS TO TRUE WHEN IN PRODUCTION
        .secure(false)
        .http_only(true)
        .finish(); 

    return c;
}
