use amqprs::FieldTable;
use amqprs::channel::{
    ExchangeDeclareArguments, ExchangeType, QueueBindArguments, QueueDeclareArguments,
};

use super::connection::RabbitMq;
use super::error::QueueError;

pub struct RabbitMqTopology;

impl RabbitMqTopology {
    pub async fn setup(rabbitmq: &RabbitMq) -> Result<(), QueueError> {
        let channel = rabbitmq.open_channel().await?;

        tracing::info!("Declaring RabbitMQ exchange and queue topologies...");

        channel
            .exchange_declare(
                ExchangeDeclareArguments::new(
                    "forge.deployments",
                    &ExchangeType::Direct.to_string(),
                )
                .durable(true)
                .finish(),
            )
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        channel
            .exchange_declare(
                ExchangeDeclareArguments::new(
                    "forge.deployments.dlx",
                    &ExchangeType::Direct.to_string(),
                )
                .durable(true)
                .finish(),
            )
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        channel
            .exchange_declare(
                ExchangeDeclareArguments::new("forge.logs", &ExchangeType::Topic.to_string())
                    .durable(true)
                    .finish(),
            )
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        // dead letter queue
        channel
            .queue_declare(
                QueueDeclareArguments::new("forge.deployments.dead-letter")
                    .durable(true)
                    .finish(),
            )
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        channel
            .queue_bind(QueueBindArguments::new(
                "forge.deployments.dead-letter",
                "forge.deployments.dlx",
                "job.dead-letter",
            ))
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        // 3. Deployment Jobs Queue (Quorum + DLX)
        let mut job_args = FieldTable::new();

        job_args.insert(
            "x-queue-type"
                .try_into()
                .map_err(|e| QueueError::TopologyError(format!("{e}")))?,
            amqprs::FieldValue::S(
                "quorum"
                    .try_into()
                    .map_err(|e| QueueError::TopologyError(format!("{e}")))?,
            ),
        );

        let mut queue_args = QueueDeclareArguments::new("forge.deployments.jobs");
        queue_args.durable(true).arguments(job_args);

        channel
            .queue_declare(queue_args.finish())
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        channel
            .queue_bind(QueueBindArguments::new(
                "forge.deployments.jobs",
                "forge.deployments",
                "job.build",
            ))
            .await
            .map_err(|e| QueueError::TopologyError(e.to_string()))?;

        Ok(())
    }
}
