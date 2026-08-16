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

        Ok(Self {
            db: Arc::new(db_connection),
            config: Arc::new(app_config),
        })
    }
}
