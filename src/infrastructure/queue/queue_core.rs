use super::connection::RabbitMq;
use super::error::QueueError;
use super::mock::MockMessagePublisher;
use super::publisher::RabbitMqPublisher;
use super::traits::MessagePublisher;
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

    pub fn rabbitmq(&self) -> Option<&RabbitMq> {
        match self {
            Self::RabbitMq(p) => Some(p.get_rabbitmq()),
            Self::Mock(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_publisher_mock_rabbitmq_accessor() {
        let mock = MockMessagePublisher::new();
        let queue = QueuePublisher::Mock(mock);
        assert!(queue.rabbitmq().is_none());
    }
}
