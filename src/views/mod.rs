use askama::Template;
use crate::models::steam_api::steam_news::NewsItem;

#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
pub struct DashboardTemplate;

#[derive(Template)]
#[template(path = "whatsnew/whatsnew.html")]
pub struct WhatsNewTemplate;


#[derive(Template)]
#[template(path = "isaacnews/isaacnews.html")]
pub struct IsaacNewsTemplate{
    pub news: Vec<NewsItem>
}

#[derive(Template)]
#[template(path = "isaacyoutube/isaacyoutube.html")]
pub struct IsaacYoutubeTemplate;
