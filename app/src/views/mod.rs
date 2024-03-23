use askama::Template;
use crate::models::steam_api::steam_news::NewsItem;

#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
pub struct DashboardTemplate{
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
    pub profile_loaded: bool
}

#[derive(Template)]
#[template(path = "whatsnew/whatsnew.html")]
pub struct WhatsNewTemplate{
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
    pub profile_loaded: bool
}


#[derive(Template)]
#[template(path = "isaacnews/isaacnews.html")]
pub struct IsaacNewsTemplate{
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
    pub profile_loaded: bool,
    pub news: Vec<NewsItem>,
    pub random_windows: Vec<i32>
}

#[derive(Template)]
#[template(path = "isaacyoutube/isaacyoutube.html")]
pub struct IsaacYoutubeTemplate{
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
    pub profile_loaded: bool
}

#[derive(Template)]
#[template(path = "faqs/faqs.html")]
pub struct FaqsTemplate{
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
    pub profile_loaded: bool
}

#[derive(Template)]
#[template(path = "contact/contact.html")]
pub struct ContactTemplate{
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
    pub profile_loaded: bool
}
