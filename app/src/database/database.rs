use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;
use sqlx::migrate::MigrateDatabase;
use sqlx::pool::Pool;

use dotenv::dotenv;
use std::env;

pub async fn db_init() -> Result<Pool<Postgres>, sqlx::Error> {

    dotenv().ok(); 

    let host = env::var("DATABASE_HOST").expect("HOST must be set");
    let port = env::var("DATABASE_PORT").expect("PORT must be set");
    let user = env::var("DATABASE_USER").expect("USER must be set");
    let password = env::var("DATABASE_PASSWORD").expect("PASSWORD must be set");
    let database = env::var("DATABASE_NAME").expect("DATABASE must be set");
    let schema = env::var("DATABASE_SCHEMA").expect("SCHEMA must be set");

    let db_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, database);

    log::info!("Database URL: {}", db_url);
     
    log::info!("Connecting to database");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
} 
