use actix_web::{get, web, Responder, HttpResponse, HttpRequest};
use askama::Template;
use crate::views;
use crate::models::responses::dashboard;
use crate::controllers::utils;
use crate::models::cookie::AppCookie;


#[get("/")]
pub async fn index(req: HttpRequest) -> impl Responder {
    
    let user_data = utils::get_cookie(&req);

    let template;

    if user_data.is_none() {
        template = views::DashboardTemplate{
            profile_username: "???".to_string(),
            profile_avatar: "/static/profile/placeholder.png".to_string(),
            profile_country: "MK".to_string(),
            profile_online_status: "/static/profile/offline.png".to_string(),
            achievement_date_unlocked: "???".to_string(),
            achievement_time_unlocked: "???".to_string(),
            achievement_timezone: "???".to_string()
        };

        HttpResponse::Ok().body(template.render().unwrap())
    }
    else{
        let updated_cookie = utils::update_player_summary(&req).await;
        //get data from cookie
        
        let app_cookie: AppCookie = serde_json::from_str(&updated_cookie.value()).unwrap();

        let player_summary = app_cookie.player_summary.unwrap();

        let player_online_status;
        if player_summary.online {player_online_status="/static/profile/online.png"} 
        else {player_online_status="/static/profile/offline.png"};
        
        log::info!("player_online_status: {:?}", player_online_status);

        template = views::DashboardTemplate{
            profile_username: player_summary.username.to_string(),
            profile_avatar: player_summary.avatar_url.to_string(),
            profile_country: player_summary.country.to_string(),
            profile_online_status: player_online_status.to_string(),
            achievement_date_unlocked: "achievement_date_unlocked".to_string(),
            achievement_time_unlocked: "achievement_time_unlocked".to_string(),
            achievement_timezone: "achievement_timezone".to_string()
        };

        HttpResponse::Ok().cookie(updated_cookie).body(template.render().unwrap())
    }

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

#[get("/api/get-cookie-data")]
pub async fn get_cookie_data(req: HttpRequest) -> impl Responder {

    let user_data = utils::get_cookie(&req);  

    if user_data.is_none() {
        return HttpResponse::Ok().json(AppCookie::new())
    }

    HttpResponse::Ok().json(user_data)
}

#[get("/api/next-char")]
pub async fn next_char() -> impl Responder {

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
