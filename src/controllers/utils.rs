use actix_web::{get, web, Responder, HttpResponse, HttpRequest, cookie, dev::ServiceRequest};
use serde_json;
use anyhow;
use crate::models::responses::dashboard::Profile;
use crate::models::cookie::AppCookie;

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
