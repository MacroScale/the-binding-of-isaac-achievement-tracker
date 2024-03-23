use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AchievementCookie{
    pub steam_id: String,
    pub completed: i32,
    pub percentage: i32,
    pub hours_played: i64,
}
