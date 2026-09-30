use super::super::entities::project_repositories::Model as RepositoryModel;
use super::super::entities::sea_orm_active_enums::ProjectRepositoryStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectRepositoryResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub repository_url: String,
    pub access_token: String,
    pub default_branch: String,
    pub status: ProjectRepositoryStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl ProjectRepositoryResponse {
    pub fn from_model(model: RepositoryModel) -> Self {
        Self {
            id: model.id,
            project_id: model.project_id,
            repository_url: model.repository_url,
            access_token: "••••••••".to_string(), // Always masked in public responses
            default_branch: model.default_branch.unwrap_or("main".to_string()),
            status: model
                .status
                .unwrap_or(ProjectRepositoryStatus::Disconnected),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}
