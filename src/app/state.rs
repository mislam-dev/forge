use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::infrastructure::queue::{
    MockMessagePublisher, QueuePublisher, RabbitMq, RabbitMqConfig, RabbitMqPublisher,
};
use crate::{config::AppConfig, database::connect_db};
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<AppConfig>,
    pub queue: QueuePublisher,
}

impl AppState {
    pub async fn new() -> Result<AppState, Box<dyn std::error::Error>> {
        let app_config = AppConfig::load()?;

        let db_connection = connect_db(&app_config.infra.db)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let rmq_config = RabbitMqConfig::from_env();
        let rabbitmq = RabbitMq::connect(&rmq_config).await?;

        let queue = QueuePublisher::RabbitMq(RabbitMqPublisher::new(rabbitmq));
        Ok(Self::from_parts(db_connection, app_config, queue))
    }

    pub fn from_parts(db: DatabaseConnection, config: AppConfig, queue: QueuePublisher) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
            queue,
        }
    }

    pub fn mock(config: AppConfig) -> Self {
        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres).into_connection();
        let queue = QueuePublisher::Mock(MockMessagePublisher::new());
        Self::from_parts(db, config, queue)
    }
}
