use actix_web::{web, post, HttpResponse, Responder};
use reqwest;
use crate::models::app_state::AppState;
use crate::models::requests::dashboard::ProfileSearch;
use crate::models::responses::dashboard::Profile;
use crate::models::responses::error::Error;

#[post("/api/profile-search")]
pub async fn profile_search(req: web::Json<ProfileSearch>, app_state: web::Data<AppState>) -> impl Responder {

    let steam_id = &req.steam_id;
    let steam_id = steam_id.parse::<i64>();

    if steam_id.is_err() {
        //could be username or vanity url at this point
    }

    let steam_id = steam_id.unwrap();

    // is id, should be 17 digits
    if steam_id.to_string().len() != 17 {
        let res = Error{status: 400, error: "Invalid steam_id".to_string()};
        return HttpResponse::Ok().json(res);
    }

    //let res = format!("searching for profile with steam_id: {}", steam_id);
    HttpResponse::Ok().json("pass")
}
