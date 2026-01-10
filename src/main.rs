// src/main.rs
mod db;
mod ingest;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); 

    let pool = db::setup_pool().await?;
    
    if db::check_health(&pool).await {
        println!("Database connection verified.");
    }

    sqlx::migrate!("./migrations").run(&pool).await?;
    ingest::run_binary_defense_ingest(&pool).await?;

    Ok(())
}