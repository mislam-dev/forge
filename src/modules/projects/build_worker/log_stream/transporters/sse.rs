use super::super::log_item::LogItem;
use super::super::traits::LogStreamTransporter;
use crate::infrastructure::queue::QueuePublisher;
use crate::infrastructure::queue::events::build_logs::BuildLogsJobCreated;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct SseStreamTransporter {
    publisher: Arc<QueuePublisher>,
}

impl SseStreamTransporter {
    pub fn new(publisher: Arc<QueuePublisher>) -> Self {
        Self { publisher }
    }
}

#[async_trait]
impl LogStreamTransporter for SseStreamTransporter {
    async fn store(&self, _item: LogItem) {}
    async fn stream(&self, item: LogItem) {
        let job_event = BuildLogsJobCreated {
            id: Uuid::new_v4(),
            routing_key: format!(
                "forge.deployments.logs.{}",
                item.deployment_id.clone().to_string()
            ),
            payload: item,
        };
        if let Err(err) = self.publisher.publish(&job_event).await {
            tracing::error!(error = %err, "Failed to publish log item to RabbitMQ");
        }
        tracing::info!("successfully send message to rmq");
    }
}
