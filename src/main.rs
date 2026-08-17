mod db;
mod models;
mod routes;
mod errors;

use axum::{Router, http::Method};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

pub struct AppState {
    pub db: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:destajo.db".to_string());

    // Pool de conexiones SQLite
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Crear tablas si no existen
    db::init(&pool).await?;

    let state = Arc::new(AppState { db: pool });

    // CORS para consumir desde celular / frontend
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .merge(routes::trabajadores::router())
        .merge(routes::operaciones::router())
        .merge(routes::registros::router())
        .merge(routes::reportes::router())
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:3000";
    tracing::info!("Servidor corriendo en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
