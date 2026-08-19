use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{config::AppConfig, database::connect_db};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub async fn new() -> Result<AppState, Box<dyn std::error::Error>> {
        let app_config = AppConfig::load()?;

        let db_connection = connect_db(&app_config.infra.db)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        Ok(Self::from_parts(db_connection, app_config))
    }

    pub fn from_parts(db: DatabaseConnection, config: AppConfig) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
        }
    }

    pub fn mock(config: AppConfig) -> Self {
        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres).into_connection();
        Self::from_parts(db, config)
    }
}
