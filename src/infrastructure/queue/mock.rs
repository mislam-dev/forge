use std::sync::Arc;

use super::error::QueueError;
use super::traits::{MessagePublisher, RabbitMqMessage};
use tokio::sync::Mutex;

#[derive(Clone, Default, Debug)]
pub struct MockMessagePublisher {
    pub published: Arc<Mutex<Vec<(String, String, Vec<u8>)>>>,
}

impl MockMessagePublisher {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn count(&self) -> usize {
        self.published.lock().await.len()
    }

    pub async fn clear(&self) {
        self.published.lock().await.clear();
    }
}

impl MessagePublisher for MockMessagePublisher {
    async fn publish<M: RabbitMqMessage>(&self, message: &M) -> Result<(), QueueError> {
        let payload =
            serde_json::to_vec(message).map_err(|e| QueueError::SerializeError(e.to_string()))?;

        self.published.lock().await.push((
            M::exchange().to_string(),
            M::routing_key().to_string(),
            payload,
        ));

        Ok(())
    }
}
