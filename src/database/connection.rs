use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub db_url: String,
}

pub async fn connect_db(config: &DbConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(config.db_url.clone());
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout_secs))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
        .sqlx_logging(cfg!(debug_assertions));

    let max_retries = 5;
    let mut retries = 0;

    loop {
        match Database::connect(opt.clone()).await {
            Ok(db) => {
                tracing::info!("Successfully db connected!");
                return Ok(db);
            }
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    tracing::error!(
                        "Failed to connect to database after {} attempts",
                        max_retries
                    );
                    return Err(e);
                }
                let backoff_secs = (2u64).pow(retries).min(30);
                tracing::warn!(
                    retries,
                    max_retries,
                    backoff_secs,
                    error = %e,
                    "Database connection failed. Retrying",
                );
                tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
            }
        }
    }
}
