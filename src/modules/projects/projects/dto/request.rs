use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::modules::projects::projects::entities::sea_orm_active_enums::{
    ProjectRuntime, ProjectStatus, ProjectTypes,
};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateProjectDTO {
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectTypes,
    pub runtime: ProjectRuntime,
    pub port: Option<i32>,
    pub health_check_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProjectDTO {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_type: Option<ProjectTypes>,
    pub runtime: Option<ProjectRuntime>,
    pub port: Option<i32>,
    pub health_check_url: Option<String>,
    pub status: Option<ProjectStatus>,
}
