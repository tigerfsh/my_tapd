mod config;
mod db;
mod domain;
mod error;
mod repository;
mod service;
mod api;
mod ws;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::router::{AppState, create_router};
use crate::config::Config;
use crate::db::{create_pg_pool, create_redis_client};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "my_tapd=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Initialize database connection pools
    let pg_pool = create_pg_pool(&config).await?;
    tracing::info!("PostgreSQL connection pool initialized");

    let redis = create_redis_client(&config).await?;
    tracing::info!("Redis connection manager initialized");

    // Build application state and router
    let state = AppState {
        pg_pool,
        redis,
        jwt_secret: config.jwt_secret.clone(),
    };
    let app = create_router(state);

    // Start server
    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
