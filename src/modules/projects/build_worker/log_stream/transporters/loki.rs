use crate::infrastructure::logging::loki_client::LokiClient;

use super::super::log_item::LogItem;
use super::super::traits::LogStreamTransporter;

use async_trait::async_trait;
pub struct LokiStreamTransporter;

impl LokiStreamTransporter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LogStreamTransporter for LokiStreamTransporter {
    async fn store(&self, item: LogItem) {
        let loki_client = LokiClient::new("http://127.0.0.1:3100/loki/api/v1".to_string());
        let deployment_id = item.deployment_id.clone();
        let r = loki_client.push(&[item]).await;
        if let Err(e) = r {
            tracing::error!(
                "[LokiStreamTransporter]: failed to push log to loki for {}. error: {:?}",
                deployment_id,
                e
            );
            return;
        };

        tracing::info!(
            "[LokiStreamTransporter]: successfully pushed log to loki for {}",
            deployment_id
        );
    }
    async fn stream(&self, _item: LogItem) {}
}
