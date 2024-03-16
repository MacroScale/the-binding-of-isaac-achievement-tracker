use serde::Serialize;
use std::sync::Mutex;
use std::fmt;

#[derive(Serialize)]
pub struct AppState{
    pub steam_id: Mutex<i64>,
    pub is_tainted: Mutex<bool>,
    pub current_character_id: Mutex<i32>,
    pub current_character_icon_url: Mutex<String>,
    pub current_achievement_id: Mutex<i32>, 
    pub current_achievement_url: Mutex<String>,
} 

impl AppState{
    pub fn new() -> AppState{
        AppState{
            steam_id: Mutex::new(-1),
            is_tainted: Mutex::new(false),
            current_character_id: Mutex::new(1),
            current_character_icon_url: Mutex::new("/static/character wheel/character icons/isaac.png".to_string()),
            current_achievement_id: Mutex::new(1),
            current_achievement_url: Mutex::new("/static/achievements/achievements/achievement_magdalene.png".to_string()),
        }
    }
}

impl fmt::Debug for AppState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nsteam_id: {:?} \n", self.steam_id.lock().unwrap())?;
        write!(f, "is_tainted: {:?} \n", self.is_tainted.lock().unwrap())?;
        write!(f, "current_character_id: {:?}\n", self.current_character_id.lock().unwrap())?;
        write!(f, "current_character_icon_url: {:?}\n", self.current_character_icon_url.lock().unwrap())?;
        write!(f, "current_achievement_id: {:?}\n", self.current_achievement_id.lock().unwrap())?;
        write!(f, "current_achievement_url: {:?}\n", self.current_achievement_url.lock().unwrap())?;
        Ok(())
    }
}
