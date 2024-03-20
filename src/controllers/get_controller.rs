use actix_web::{get, web, Responder, HttpResponse, HttpRequest};
use askama::Template;
use crate::views;
use crate::models;
use rand::Rng;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::PermanentRedirect().header("Location", "/dashboard").finish()
}

#[get("/dashboard{tail:.*}")]
pub async fn dashboard(req: HttpRequest) -> impl Responder {
    let template = views::DashboardTemplate;
    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/whatsnew")]
pub async fn whatsnew() -> impl Responder {
    views::WhatsNewTemplate.render().unwrap();
    HttpResponse::Ok().body(views::WhatsNewTemplate.render().unwrap())
}

#[get("/isaacnews")]
pub async fn isaacnews() -> impl Responder {

    let news = models::steam_api::steam_news::SteamNews::new().await;

    if news.is_err(){
        return HttpResponse::InternalServerError().body("Failed to get news");
    }

    let news = news.unwrap().appnews.newsitems;

    log::info!("news: {:?}", &news);

    let mut rng = rand::thread_rng();
    let arr_size = news.len();
    let mut random_windows = Vec::new();
    for _ in 0..arr_size{
        // 1-4
        let random = rng.gen_range(1..=4);
        random_windows.push(random);
    }


    let template = views::IsaacNewsTemplate{
        news,
        random_windows
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/isaacyoutube")]
pub async fn isaacyoutube() -> impl Responder {
    views::IsaacYoutubeTemplate.render().unwrap();
    HttpResponse::Ok().body(views::IsaacYoutubeTemplate.render().unwrap())
}
