use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow;

use crate::models::steam_api::player_summaries::PlayerSummary; 
use crate::models::steam_api::player_achievements::PlayerAchievements;
use crate::models::steam_api::player_game_data::PlayerGame;
use crate::models::steam_api::player_game_data::Game;
use crate::models::character_unlocks::Character;
use crate::models::achievement_image::AchievementImg;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo{
    pub status: i32,
    pub message: String,
    pub player_summary: PlayerSummary,
    pub player_achievements: PlayerAchievements,
    pub player_game_data: Option<Game>,
    pub character_data: Vec<Character>,
    pub achievement_image_data: Vec<AchievementImg>,
}

impl PlayerInfo{
    pub async fn new(steam_id: &i64, pool: &PgPool) -> anyhow::Result<PlayerInfo>{

        let player_summary = PlayerSummary::new(steam_id, pool).await?;
        let player_achievements = PlayerAchievements::new(steam_id, pool).await?;
        let player_game_data = PlayerGame::new(steam_id, pool).await?;
        let character_data = Character::new();
        let achievement_image_data = AchievementImg::new();

        Ok(PlayerInfo{
            status: 200,
            message: "success".to_string(),
            player_summary,
            player_achievements,
            player_game_data,
            character_data,
            achievement_image_data
        })
    }
}
