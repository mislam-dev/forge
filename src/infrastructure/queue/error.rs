pub use amqprs::error::Error as RabbitMqError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("AMQP broker connection failure: {0}")]
    ConnectionError(String),

    #[error("AMQP channel error: {0}")]
    ChannelError(String),

    #[error("AMQP exchange error: {0}")]
    ExchangeError(String),

    #[error("Failed to serialize message payload: {0}")]
    SerializeError(String),

    #[error("Message publishing rejected by broker (NACK)")]
    PublishedNackedError,

    #[error("Failed to declare queue topology: {0}")]
    TopologyError(String),

    #[error("AMQP operation timed out: {0}")]
    TimeoutError(String),

    #[error("AMQP internal driver error: {0}")]
    DriverError(String),
}

impl From<RabbitMqError> for QueueError {
    fn from(err: RabbitMqError) -> Self {
        QueueError::DriverError(err.to_string())
    }
}
