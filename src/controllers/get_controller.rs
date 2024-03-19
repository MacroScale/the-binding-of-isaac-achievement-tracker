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

    let template = views::IsaacNewsTemplate{
        slot1_title: "slot1_title".to_string(),
        slot1_sub_title: "slot1_sub_title".to_string(),
        slot1_content: "slot1_content".to_string(),
        slot1_win_num: 1,

        slot2_title: "slot2_title".to_string(),
        slot2_sub_title: "slot2_sub_title".to_string(),
        slot2_content: "slot2_content".to_string(),
        slot2_win_num: 2,

        slot3_title: "slot3_title".to_string(),
        slot3_sub_title: "slot3_sub_title".to_string(),
        slot3_content: "slot3_content".to_string(),
        slot3_win_num: 3,

        slot4_title: "slot4_title".to_string(),
        slot4_sub_title: "slot4_sub_title".to_string(),
        slot4_content: "slot4_content".to_string(),
        slot4_win_num: 4,

        slot5_title: "slot5_title".to_string(),
        slot5_sub_title: "slot5_sub_title".to_string(),
        slot5_content: "slot5_content".to_string(),
        slot5_win_num: 1
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/isaacyoutube")]
pub async fn isaacyoutube() -> impl Responder {
    views::IsaacYoutubeTemplate.render().unwrap();
    HttpResponse::Ok().body(views::IsaacYoutubeTemplate.render().unwrap())
}
