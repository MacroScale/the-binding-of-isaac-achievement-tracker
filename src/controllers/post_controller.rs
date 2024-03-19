use actix_web::{web, post, HttpResponse, Responder};
use crate::models::requests::dashboard;
use crate::models::responses::error::Error;
use crate::models::steam_api::profile_search::ProfileSearch;
use crate::models::steam_api::player_info::PlayerInfo;

#[post("/api/profile-search")]
pub async fn profile_search(req: web::Json<dashboard::ProfileSearch>) -> impl Responder {

    let steam_id = &req.steam_id;
    let id = ProfileSearch::new(steam_id).await;

    if id.is_err() {
        let err = Error{
            status: 500,
            message: "error".to_string(),
        };
        return HttpResponse::Ok().json(err);
    }

    let id = id.unwrap();

    let info = PlayerInfo::new(&id).await;

    if info.is_err() {
        let err = Error{
            status: 500,
            message: "error".to_string(),
        };
        return HttpResponse::Ok().json(err);
    }

    let info = info.unwrap();

    return HttpResponse::Ok().json(info);
}
