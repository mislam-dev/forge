use super::error::QueueError;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait MessageHandler<M>: Send + Sync
where
    M: Send + 'static,
{
    async fn handle(&self, message: M) -> Result<(), QueueError>;
}

pub trait RabbitMqMessage: Serialize + Send + Sync {
    fn exchange(&self) -> &'static str;
    fn routing_key(&self) -> &str;
    fn message_type(&self) -> &'static str;

    fn content_type(&self) -> &'static str {
        "application/json"
    }
    fn delivery_mode(&self) -> u8 {
        2
    }
    fn expiration(&self) -> Option<&'static str> {
        None
    }
}

pub trait MessagePublisher: Send + Sync {
    async fn publish<M: RabbitMqMessage>(&self, message: &M) -> Result<(), QueueError>;
}
