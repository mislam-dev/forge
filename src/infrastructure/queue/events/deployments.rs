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
    pub triggered_by: Uuid,
}

impl RabbitMqMessage for DeploymentJobCreated {
    fn exchange(&self) -> &'static str {
        "forge.deployments"
    }

    fn routing_key(&self) -> &'static str {
        "job.build"
    }

    fn message_type(&self) -> &'static str {
        "deployment.job.created"
    }
}
