use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use crate::database::database;

pub struct AppData{
    pub pool: PgPool,
}

impl AppData{
    pub async fn new() -> AppData{
        let pool = database::db_init().await.unwrap();
        AppData{pool}  
    }
}
