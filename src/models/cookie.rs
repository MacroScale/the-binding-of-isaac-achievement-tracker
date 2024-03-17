use serde::{Serialize, Deserialize};
use crate::models::responses::dashboard::Profile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppCookie{
    pub steam_id: Option<i64>,  
    pub player_summary: Option<Profile>,
    //pub player_achievements: Option<steam_api::player_achievements::PlayerAchievements>,
}

impl AppCookie{
    pub fn new() -> AppCookie{
        AppCookie{
            steam_id: None,
            player_summary: None,
            //player_achievements: None,
        }
    }
}


