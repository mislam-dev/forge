use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::projects::projects::entities::sea_orm_active_enums::{
    ProjectRuntime, ProjectStatus, ProjectTypes,
};

use super::super::entities::projects::Model as ProjectModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectTypes,
    pub runtime: ProjectRuntime,
    pub port: i32,
    pub health_check_url: Option<String>,
    pub status: ProjectStatus,
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
