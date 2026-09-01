use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::project_repository::Model as RepositoryModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectRepositoryResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub repository_url: String,
    pub auth_type: String,
    pub access_token: String,
    pub default_branch: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl ProjectRepositoryResponse {
    pub fn from_model(model: RepositoryModel) -> Self {
        Self {
            id: model.id,
            project_id: model.project_id,
            repository_url: model.repository_url,
            auth_type: model.auth_type,
            access_token: "••••••••".to_string(), // Always masked in public responses
            default_branch: model.default_branch.unwrap_or_else(|| "main".to_string()),
            status: model.status.unwrap_or_else(|| "connected".to_string()),
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
    fn test_repository_response_masks_access_token() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = RepositoryModel {
            id,
            project_id,
            repository_url: "https://github.com/test/repo".to_string(),
            auth_type: "pat".to_string(),
            access_token_encrypted: "encrypted_secret_payload".to_string(),
            default_branch: Some("main".to_string()),
            status: Some("connected".to_string()),
            created_at: now,
            updated_at: now,
        };

        let res = ProjectRepositoryResponse::from_model(model);
        assert_eq!(res.access_token, "••••••••");
        assert_ne!(res.access_token, "encrypted_secret_payload");
    }
}
