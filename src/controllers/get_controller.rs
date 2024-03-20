use actix_web::{get, web, Responder, HttpResponse, HttpRequest};
use askama::Template;
use crate::views;
use crate::models;

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

    let template = views::IsaacNewsTemplate{
        news
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/isaacyoutube")]
pub async fn isaacyoutube() -> impl Responder {
    views::IsaacYoutubeTemplate.render().unwrap();
    HttpResponse::Ok().body(views::IsaacYoutubeTemplate.render().unwrap())
}
