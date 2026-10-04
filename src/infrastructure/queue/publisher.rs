use super::connection::RabbitMq;
use super::error::QueueError;
use super::traits::{MessagePublisher, RabbitMqMessage};
use amqprs::BasicProperties;
use amqprs::channel::BasicPublishArguments;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RabbitMqPublisher {
    rabbitmq: RabbitMq,
}

impl RabbitMqPublisher {
    pub fn new(rabbitmq: RabbitMq) -> Self {
        Self { rabbitmq }
    }

    pub fn get_rabbitmq(&self) -> &RabbitMq {
        &self.rabbitmq
    }
}

impl MessagePublisher for RabbitMqPublisher {
    async fn publish<M: RabbitMqMessage>(&self, message: &M) -> Result<(), QueueError> {
        let payload =
            serde_json::to_vec(message).map_err(|e| QueueError::SerializeError(e.to_string()))?;
        let message_id = Uuid::new_v4().to_string();
        let mut properties = BasicProperties::default();
        properties
            .with_content_type(message.content_type())
            .with_delivery_mode(message.delivery_mode())
            .with_message_id(&message_id)
            .with_timestamp(Utc::now().timestamp() as u64);
        if let Some(expiration) = message.expiration() {
            properties.with_expiration(expiration);
        }

        let properties = properties.finish();

        let publish_args = BasicPublishArguments::new(message.exchange(), message.routing_key());

        let channel = self.rabbitmq.get_publisher_channel();

        channel
            .basic_publish(properties, payload, publish_args)
            .await
            .map_err(|e| QueueError::DriverError(e.to_string()))?;

        Ok(())
    }
}
