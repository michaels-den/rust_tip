use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn setup_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in your environment or .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5) 
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    println!("Database connection established.");
    
    Ok(pool)
}

pub async fn check_health(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1").execute(pool).await.is_ok()
}