use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::deployment::Model as DeploymentModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub triggered_by: Uuid,
    pub branch: String,
    pub commit_hash: String,
    pub status: String,
    pub build_duration: Option<i32>,
    pub deploy_duration: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl DeploymentResponse {
    pub fn from_model(model: DeploymentModel) -> Self {
        Self {
            id: model.id,
            project_id: model.project_id,
            triggered_by: model.triggered_by,
            branch: model.branch,
            commit_hash: model.commit_hash,
            status: model.status,
            build_duration: model.build_duration,
            deploy_duration: model.deploy_duration,
            error_message: model.error_message,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_deployment_response_from_model() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let triggered_by = Uuid::new_v4();
        let now = Utc::now().into();

        let model = DeploymentModel {
            id,
            project_id,
            triggered_by,
            branch: "main".to_string(),
            commit_hash: "9f8e7d6".to_string(),
            status: "Queued".to_string(),
            build_duration: None,
            deploy_duration: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let res = DeploymentResponse::from_model(model);
        assert_eq!(res.id, id);
        assert_eq!(res.status, "Queued");
        assert_eq!(res.branch, "main");
    }
}
