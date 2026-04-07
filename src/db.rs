use anyhow::Result;
use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::config::Config;

pub async fn create_pg_pool(config: &Config) -> Result<PgPool> {
    let pool = PgPool::connect(&config.database_url).await?;
    Ok(pool)
}

pub async fn create_redis_client(config: &Config) -> Result<ConnectionManager> {
    let client = redis::Client::open(config.redis_url.as_str())?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}
