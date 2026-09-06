use amqprs::{
    BasicProperties, Deliver,
    channel::{
        BasicAckArguments, BasicConsumeArguments, BasicNackArguments, BasicQosArguments, Channel,
    },
    consumer::AsyncConsumer,
};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::infrastructure::queue::error::QueueError;

use super::traits::MessageHandler;

pub struct RabbitMqConsumer;

struct ConsumerAdapter<M, H>
where
    M: DeserializeOwned + Send + 'static,
    H: MessageHandler<M>,
{
    handler: H,
    _marker: PhantomData<M>,
}

#[async_trait]
impl<M, H> AsyncConsumer for ConsumerAdapter<M, H>
where
    M: DeserializeOwned + Send + 'static,
    H: MessageHandler<M>,
{
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let delivery_tag = deliver.delivery_tag();

        match serde_json::from_slice::<M>(&content) {
            Ok(message) => match self.handler.handle(message).await {
                Ok(_) => {
                    tracing::debug!(delivery_tag, "Message processed successfully. Sending ACK.");
                    let _ = channel
                        .basic_ack(BasicAckArguments::new(delivery_tag, false))
                        .await;
                }
                Err(err) => {
                    tracing::error!(
                        error = %err,
                        delivery_tag,
                        "Handler failed. Sending NACK without requeue (routed to DLX)."
                    );
                    let _ = channel
                        .basic_nack(BasicNackArguments::new(delivery_tag, false, false))
                        .await;
                }
            },
            Err(err) => {
                tracing::error!(
                    error = %err,
                    delivery_tag,
                    "Malformed payload. Rejecting message to DLX."
                );
                let _ = channel
                    .basic_nack(BasicNackArguments::new(delivery_tag, false, false))
                    .await;
            }
        }
    }
}

impl RabbitMqConsumer {
    pub async fn start_consumer<M, H>(
        channel: &Channel,
        queue: &str,
        consumer_tag: &str,
        prefetch_count: u16,
        handler: H,
    ) -> Result<String, QueueError>
    where
        M: DeserializeOwned + Send + 'static,
        H: MessageHandler<M> + 'static,
    {
        let qos_args = BasicQosArguments::new(0, prefetch_count, false);

        channel
            .basic_qos(qos_args)
            .await
            .map_err(|e| QueueError::ChannelError(e.to_string()))?;

        let adapter = ConsumerAdapter {
            handler,
            _marker: PhantomData,
        };

        let mut consume_args = BasicConsumeArguments::new(queue, consumer_tag);

        consume_args.manual_ack(true);

        let registered_tag = channel
            .basic_consume(adapter, consume_args.finish())
            .await
            .map_err(|e| QueueError::DriverError(e.to_string()))?;

        Ok(registered_tag)
    }
}
