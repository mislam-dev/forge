use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::queue::traits::RabbitMqMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentJobCreated {
    pub deployment_id: Uuid,
    pub project_id: Uuid,
    pub repository_url: String,
    pub commit_hash: String,
    pub branch: String,
    pub triggered_id: Uuid,
}

impl RabbitMqMessage for DeploymentJobCreated {
    fn routing_key() -> &'static str {
        "forge.deployment"
    }

    fn message_type() -> &'static str {
        "job.build"
    }

    fn exchange() -> &'static str {
        "deployments.job.created"
    }
}
