// src/main.rs
mod db;
mod ingest;
mod models;
use axum::{routing::get, Router};
use askama::Template;
use sqlx::PgPool;
use std::net::SocketAddr;
use askama_axum::IntoResponse; 

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    indicators: Vec<IndicatorDisplay>,
}
struct IndicatorDisplay {
    id: String,
    pattern: String,
    pattern_type: String,
    created: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let pool = crate::db::setup_pool().await?;

    // Build our application with a single route
    let app = Router::new()
        .route("/", get(move || index_handler(pool.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
async fn index_handler(pool: PgPool) -> impl IntoResponse {
    let rows = sqlx::query!(
        r#"SELECT id, pattern, pattern_type, created FROM indicators ORDER BY created DESC LIMIT 50"#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let indicators = rows.into_iter().map(|row| {
        let formatted_time = row.created.format(
            &time::format_description::well_known::Rfc3339
        ).unwrap_or_else(|_| "Invalid Date".to_string());

        IndicatorDisplay {
            id: row.id,
            pattern: row.pattern,
            pattern_type: row.pattern_type,
            created: formatted_time,
        }
    }).collect();

    IndexTemplate { indicators }
}