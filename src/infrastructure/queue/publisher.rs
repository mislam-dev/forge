use amqprs::BasicProperties;
use amqprs::channel::BasicPublishArguments;
use chrono::Utc;
use uuid::Uuid;

use super::connection::RabbitMq;
use super::error::QueueError;
use super::traits::{MessagePublisher, RabbitMqMessage};

pub struct RabbitMqPublisher {
    rabbitmq: RabbitMq,
}

impl RabbitMqPublisher {
    pub fn new(rabbitmq: RabbitMq) -> Self {
        Self { rabbitmq }
    }
}

impl MessagePublisher for RabbitMqPublisher {
    async fn publish<M: RabbitMqMessage>(&self, message: &M) -> Result<(), QueueError> {
        let payload =
            serde_json::to_vec(message).map_err(|e| QueueError::SerializeError(e.to_string()))?;

        let properties = BasicProperties::default()
            .with_content_type(message.content_type())
            .with_delivery_mode(2)
            .with_message_id(&Uuid::new_v4().to_string())
            .with_timestamp(Utc::now().timestamp() as u64)
            .finish();

        let publish_args = BasicPublishArguments::new(M::exchange(), M::routing_key());

        tracing::debug!(
            exchange = M::exchange(),
            routing_key = M::routing_key(),
            message_type = M::message_type(),
            "Publishing AMQP message"
        );

        let channel = self.rabbitmq.get_publisher_channel();

        channel
            .basic_publish(properties, payload, publish_args)
            .await
            .map_err(|e| QueueError::DriverError(e.to_string()))?;

        Ok(())
    }
}
