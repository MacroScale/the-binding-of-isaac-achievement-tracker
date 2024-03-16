use actix_web::{get, web, Responder, HttpResponse};
use askama::Template;
use crate::views;
use crate::models::app_state::AppState;
use crate::models::responses::dashboard;

#[get("/")]
pub async fn index() -> impl Responder {
    views::DashboardTemplate.render().unwrap();
    HttpResponse::Ok().body(views::DashboardTemplate.render().unwrap())
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


/*
 * 
 * API
 *
 */
#[get("/api/app-state")]
pub async fn get_app_state(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_state)
}

#[get("/api/next-char")]
pub async fn next_char(app_state: web::Data<AppState>) -> impl Responder {

    log::info!("{:?}", app_state);

    let marks = vec![
        dashboard::Mark{
            mark_name: "mark_name".to_string(),
            mark_url: "mark_url".to_string()
        }
    ];

    HttpResponse::Ok().json(dashboard::NextCharacter{
        status: 200,
        char_icon_url: "char_icon_url".to_string(),
        completion_urls: marks 
    })
}
