use crate::{
    app::state::AppState,
    infrastructure::queue::{
        MessageHandler, QueueError, RabbitMqConsumer, events::build_logs::BuildLogsJobCreated,
    },
};
use async_trait::async_trait;
use axum::response::sse::Event;
use std::convert::Infallible;
use tokio::sync::mpsc::Sender;

pub struct LogStreamConsumer {}

impl LogStreamConsumer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(
        &self,
        app_state: AppState,
        deployment_id: String,
        tx: Sender<Result<Event, Infallible>>,
    ) {
        tokio::spawn(async move {
            let Some(rmq) = app_state.queue.rabbitmq() else {
                return;
            };

            let Ok(worker_channel) = rmq.open_channel().await else {
                return;
            };

            let handler = Consumer {
                deployment_id,
                tx: tx.clone(),
            };

            let Ok(_tag) = RabbitMqConsumer::start_consumer(
                &worker_channel,
                "forge.deployments.log.jobs",
                "forge.log_stream_consumer",
                10,
                handler,
            )
            .await
            else {
                return;
            };
            tx.closed().await;
        });
    }
}

struct Consumer {
    deployment_id: String,
    tx: Sender<Result<Event, Infallible>>,
}

#[async_trait]
impl MessageHandler<BuildLogsJobCreated> for Consumer {
    async fn handle(&self, message: BuildLogsJobCreated) -> Result<(), QueueError> {
        if self.deployment_id != message.payload.deployment_id {
            return Ok(());
        }
        let json = serde_json::to_string(&message.payload).unwrap_or_default();
        let event = Event::default().event("log").data(json);

        if self.tx.send(Ok(event)).await.is_err() {
            return Ok(());
        }

        if message.payload.is_end {
            let done_event = Event::default()
                .event("done")
                .data(r#"{"status":"finished"}"#);
            let _ = self.tx.send(Ok(done_event));
        }

        Ok(())
    }
}
