use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile{
    pub steam_id: i64,
    pub username: String,
    pub avatar_url: String,
    pub online: bool,
    pub country: String,
} 

impl Profile{
    pub fn new() -> Profile{
        Profile{
            steam_id: -1,
            username: "???".to_string(),
            avatar_url: "/static/profile/placeholder.png".to_string(),
            online: false,
            country: "".to_string(),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Mark{
    pub mark_name: String,
    pub mark_url: String,
} 

#[derive(Serialize, Deserialize)]
pub struct NextCharacter{
    pub status: i32, 
    pub char_icon_url: String, 
    pub completion_urls: Vec<Mark>,  
} 

#[derive(Serialize, Deserialize)]
pub struct Achievement{
    pub status: i32, 
    pub achievement_id: i32,
    pub achievement_page_url: String,
    pub date_unlocked: String,
    pub time_unlocked: String,
    pub time_zone: String,
}
