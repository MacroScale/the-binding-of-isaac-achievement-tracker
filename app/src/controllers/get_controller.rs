use actix_web::{get, Responder, HttpResponse, HttpRequest};
use askama::Template;
use crate::views;
use crate::models;
use rand::Rng;

use crate::models::responses::error::Error;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header(
            ("Location", "/dashboard")
        ).finish()
}

#[get("/dashboard{tail:.*}")]
pub async fn dashboard(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }


    let cookie = req.cookie("achievement");
    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();
        let steam_id = &cookie.steam_id;
        let location = format!("/dashboard?steam_id={}", steam_id);

        if location != req.uri().to_string(){
            return HttpResponse::TemporaryRedirect()
                .append_header(
                    ("Location", location)
                ).finish();
        }
    }

    let template = views::DashboardTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/whatsnew")]
pub async fn whatsnew(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }

    let template = views::WhatsNewTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded

    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/isaacnews")]
pub async fn isaacnews(req: HttpRequest) -> impl Responder {


    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }

    let news = models::steam_api::steam_news::SteamNews::new().await;

    if news.is_err(){
        return HttpResponse::InternalServerError().body("Failed to get news");
    }

    let news = news.unwrap().appnews.newsitems;

    let mut rng = rand::thread_rng();
    let arr_size = news.len();
    let mut random_windows = Vec::new();
    for _ in 0..arr_size{
        // 1-4
        let random = rng.gen_range(1..=4);
        random_windows.push(random);
    }

    let template = views::IsaacNewsTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded,
        news,
        random_windows
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/isaacyoutube")]
pub async fn isaacyoutube(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }

    let template = views::IsaacYoutubeTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/contact")]
pub async fn contact(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }

    let template = views::ContactTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded,
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/faqs")]
pub async fn faqs(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }

    let template = views::FaqsTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded,
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/api/isaacyoutubers")]
pub async fn api_isaacyoutubers() -> impl Responder {

    let youtubers = models::youtube_api::isaac_youtubers::IsaacYoutubers::new().await;

    if youtubers.is_err(){

        let error = Error{
            status: 500,
            message: "Failed to get youtubers".to_string()
        };

        log::error!("Failed to get youtubers: {:?}", youtubers.err());

        return HttpResponse::Ok().json(error);
    }

    let youtubers = youtubers.unwrap();

    HttpResponse::Ok().json(youtubers)
}


pub async fn not_found(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("achievement");

    let mut completed = 0;
    let mut percentage = 0;
    let mut hours_played = 0;
    let mut profile_loaded = false;

    if cookie.is_some(){
        let cookie = cookie.unwrap();
        let cookie_str = cookie.value();
        let cookie: models::cookie::AchievementCookie = serde_json::from_str(cookie_str).unwrap();

        completed = cookie.completed;
        percentage = cookie.percentage;
        hours_played = cookie.hours_played;
        profile_loaded = true;
    }

    let template = views::ErrorTemplate{
        completed,
        percentage,
        hours_played,
        profile_loaded,
    };

    HttpResponse::Ok().body(template.render().unwrap())
}
