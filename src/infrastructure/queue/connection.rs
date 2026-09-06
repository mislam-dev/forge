use core::fmt;

use amqprs::channel::Channel;
use amqprs::connection::{Connection, OpenConnectionArguments};

use super::config::RabbitMqConfig;
use super::error::QueueError;

#[derive(Clone)]
pub struct RabbitMq {
    connection: Connection,
    publisher_channel: Channel,
}

impl RabbitMq {
    pub async fn connect(config: &RabbitMqConfig) -> Result<Self, QueueError> {
        let mut args = OpenConnectionArguments::new(
            &config.host,
            config.port,
            &config.username,
            &config.password,
        );
        args.virtual_host(&config.virtual_host);

        let connection = Connection::open(&args).await?;
        let publisher_channel = connection.open_channel(None).await?;

        Ok(Self {
            connection,
            publisher_channel,
        })
    }

    pub async fn open_channel(&self) -> Result<Channel, QueueError> {
        self.connection
            .open_channel(None)
            .await
            .map_err(|e| QueueError::ChannelError(e.to_string()))
    }

    pub async fn close(self) -> Result<(), QueueError> {
        self.connection
            .close()
            .await
            .map_err(|e| QueueError::ChannelError(e.to_string()))
    }

    pub fn is_alive(&self) -> bool {
        self.connection.is_open()
    }

    pub fn get_publisher_channel(&self) -> &Channel {
        &self.publisher_channel
    }
}

impl fmt::Debug for RabbitMq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RabbitMq")
            .field("is_alive", &self.is_alive())
            .finish()
    }
}
