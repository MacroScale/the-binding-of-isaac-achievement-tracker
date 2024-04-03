use serde::{Serialize, Deserialize};
use sqlx::types::chrono;
use sqlx::FromRow;
use chrono::NaiveDateTime;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Log{
    pub id: i32,
    pub steam_id: String,
    pub action: String,
    pub error: String,
    pub timestamp: NaiveDateTime,
}

impl Log{
    pub async fn send_log(steam_id: &str, action: &str, error: &str, pool: &sqlx::PgPool){

        dotenv().ok();

        let schema = env::var("DATABASE_SCHEMA").expect("SCHEMA must be set");
    
        let query = format!("
            INSERT INTO {}.action_log
            (steam_id, action, error) 
            VALUES ($1, $2, $3);
        ", schema);

        sqlx::query(&query)
            .bind(steam_id)
            .bind(action)
            .bind(error)
            .execute(pool)
            .await
            .expect("Failed to send log");
    }
}
