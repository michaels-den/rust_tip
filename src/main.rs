#![allow(warnings)]
// src/main.rs
mod db;
mod ingest;
mod models;
mod utils;
use axum::{routing::get, Router, extract::Query};
use askama::Template;
use sqlx::PgPool;
use std::net::SocketAddr;
use askama_axum::IntoResponse;
use serde::Deserialize;


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    indicators: Vec<IndicatorDisplay>,
    search_query: String,
    search_results: Vec<IndicatorDisplay>,
}

struct IndicatorDisplay {
    id: String,
    pattern: String,
    pattern_type: String,
    created: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let pool = crate::db::setup_pool().await?;

    // Clone the pool for use in multiple route handlers
    let pool_index = pool.clone();
    let pool_search = pool.clone();

    // Build our application with routes for index and search
    let app = Router::new()
        .route("/", get(move || index_handler(pool_index)))
        .route("/search", get(move |q: Query<SearchQuery>| search_handler(pool_search, q)));

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

    IndexTemplate { 
        indicators,
        search_query: String::new(),
        search_results: Vec::new(),
    }
}

async fn search_handler(pool: PgPool, Query(params): Query<SearchQuery>) -> impl IntoResponse {
    let query = params.q.unwrap_or_default();

    let all_rows = sqlx::query!(
        r#"SELECT id, pattern, pattern_type, created FROM indicators ORDER BY created DESC LIMIT 50"#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let indicators = all_rows.into_iter().map(|row| {
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

    let search_rows = sqlx::query!(
        r#"SELECT id, pattern, pattern_type, created FROM indicators WHERE pattern ILIKE $1 ORDER BY created DESC LIMIT 100"#,
        format!("%{}%", query)
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let search_results = search_rows.into_iter().map(|row| {
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

    IndexTemplate {
        indicators,
        search_query: query,
        search_results,
    }
}