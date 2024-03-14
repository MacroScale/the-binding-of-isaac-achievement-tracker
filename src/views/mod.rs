use askama::Template;

#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
pub struct DashboardTemplate;

#[derive(Template)]
#[template(path = "whatsnew/whatsnew.html")]
pub struct WhatsNewTemplate;


#[derive(Template)]
#[template(path = "isaacnews/isaacnews.html")]
pub struct IsaacNewsTemplate{
    pub slot1_title: String,
    pub slot1_sub_title: String,
    pub slot1_content: String,
    pub slot1_win_num: i32,

    pub slot2_title: String,
    pub slot2_sub_title: String,
    pub slot2_content: String,
    pub slot2_win_num: i32,

    pub slot3_title: String,
    pub slot3_sub_title: String,
    pub slot3_content: String,
    pub slot3_win_num: i32,

    pub slot4_title: String,
    pub slot4_sub_title: String,
    pub slot4_content: String,
    pub slot4_win_num: i32,

    pub slot5_title: String,
    pub slot5_sub_title: String,
    pub slot5_content: String,
    pub slot5_win_num: i32
}


#[derive(Template)]
#[template(path = "isaacyoutube/isaacyoutube.html")]
pub struct IsaacYoutubeTemplate;
