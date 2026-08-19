use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::project::Model as ProjectModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
    pub runtime: String,
    pub port: i32,
    pub health_check_url: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl ProjectResponse {
    pub fn from_model(model: ProjectModel) -> Self {
        Self {
            id: model.id,
            organization_id: model.organization_id,
            owner_id: model.owner_id,
            name: model.name,
            description: model.description,
            project_type: model.project_type,
            runtime: model.runtime,
            port: model.port,
            health_check_url: model.health_check_url,
            status: model.status,
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
    fn test_project_response_from_model() {
        let id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = ProjectModel {
            id,
            organization_id: org_id,
            owner_id,
            name: "Forge API".to_string(),
            description: Some("Core API".to_string()),
            project_type: "repo".to_string(),
            runtime: "Rust".to_string(),
            port: 3000,
            health_check_url: Some("/health".to_string()),
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        };

        let res = ProjectResponse::from_model(model);
        assert_eq!(res.id, id);
        assert_eq!(res.organization_id, org_id);
        assert_eq!(res.name, "Forge API");
        assert_eq!(res.port, 3000);
    }
}
