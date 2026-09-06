use super::error::QueueError;
use super::traits::MessagePublisher;

use super::mock::MockMessagePublisher;
use super::publisher::RabbitMqPublisher;
use super::traits::RabbitMqMessage;

#[derive(Clone, Debug)]
pub enum QueuePublisher {
    RabbitMq(RabbitMqPublisher),
    Mock(MockMessagePublisher),
}

impl QueuePublisher {
    pub async fn publish<M: RabbitMqMessage>(&self, message: &M) -> Result<(), QueueError> {
        match self {
            Self::RabbitMq(q) => q.publish(message).await,
            Self::Mock(q) => q.publish(message).await,
        }
    }
}
