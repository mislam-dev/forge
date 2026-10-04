use serde::{Deserialize, Serialize};

use crate::{
    infrastructure::queue::RabbitMqMessage, modules::projects::build_worker::log_stream::LogItem,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildLogsJobCreated {
    pub id: uuid::Uuid,
    pub payload: LogItem,
    pub routing_key: String,
}

impl RabbitMqMessage for BuildLogsJobCreated {
    fn exchange(&self) -> &'static str {
        "forge.deployments.logs"
    }

    fn routing_key(&self) -> &str {
        self.routing_key.as_str() // format!("forge.deployments.logs.{}", deployment_id)
    }

    fn message_type(&self) -> &'static str {
        "build.stream"
    }
    fn delivery_mode(&self) -> u8 {
        1
    }

    fn expiration(&self) -> Option<&'static str> {
        Some("30000")
    }
}
