use serde::Serialize;

#[derive(Serialize)]
pub struct Profile{
    pub status: i32,
    pub steam_id: i64,
    pub username: String,
    pub avatar_url: String,
    pub online: bool,
    pub country: String,
    pub country_flag_url: String,
} 

#[derive(Serialize)]
pub struct Mark{
    pub mark_name: String,
    pub mark_url: String,
} 

#[derive(Serialize)]
pub struct NextCharacter{
    pub status: i32, 
    pub char_icon_url: String, 
    pub completion_urls: Vec<Mark>,  
} 

#[derive(Serialize)]
pub struct Achievement{
    pub status: i32, 
    pub achievement_id: i32,
    pub achievement_page_url: String,
    pub date_unlocked: String,
    pub time_unlocked: String,
    pub time_zone: String,
}
