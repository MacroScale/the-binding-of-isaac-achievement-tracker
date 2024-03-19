use serde::{Serialize, Deserialize};
use anyhow;

use crate::models::steam_api::player_summaries::PlayerSummary; 
use crate::models::steam_api::player_achievements::PlayerAchievements;
use crate::models::character_unlocks::Character;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo{
    pub status: i32,
    pub message: String,
    pub player_summary: PlayerSummary,
    pub player_achievements: PlayerAchievements,
    pub character_data: Vec<Character>,
}

impl PlayerInfo{
    pub async fn new(steam_id: &i64) -> anyhow::Result<PlayerInfo>{
        let player_summary = PlayerSummary::new(steam_id).await?;
        let player_achievements = PlayerAchievements::new(steam_id).await?;
        let character_data = Character::new();

        Ok(PlayerInfo{
            status: 200,
            message: "success".to_string(),
            player_summary,
            player_achievements,
            character_data,
        })
    }
}
