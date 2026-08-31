use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{CreateProjectDTO, ProjectResponse, UpdateProjectDTO};
use super::entities::projects::ActiveModel as ProjectActiveModel;
use super::repository::ProjectsRepository;
use crate::shared::error::AppError;

pub struct ProjectsService;

impl ProjectsService {
    pub async fn create_project(
        db: &DatabaseConnection,
        requester_id: Uuid,
        org_id: Option<Uuid>,
        dto: CreateProjectDTO,
    ) -> Result<ProjectResponse, AppError> {
        if let Some(org_id) = org_id {
            if (ProjectsRepository::find_by_org_and_name(db, org_id, &dto.name).await?).is_some() {
                return Err(AppError::Conflict(format!(
                    "Project with name '{}' already exists in this organization",
                    dto.name
                )));
            }
        } else {
            if (ProjectsRepository::find_by_owner_and_name(db, requester_id, &dto.name).await?)
                .is_some()
            {
                return Err(AppError::Conflict(format!(
                    "Project with name '{}' already exists",
                    dto.name
                )));
            }
        }

        let project = ProjectsRepository::create_project(db, requester_id, org_id, dto).await?;
        Ok(ProjectResponse::from_model(project))
    }

    pub async fn list_projects(
        db: &DatabaseConnection,
        user_id: Uuid,
        org_id: Option<Uuid>,
    ) -> Result<Vec<ProjectResponse>, AppError> {
        let projects = if let Some(org_id) = org_id {
            ProjectsRepository::find_by_org_id(db, org_id).await?
        } else {
            ProjectsRepository::find_by_owner_id(db, user_id).await?
        };

        Ok(projects
            .into_iter()
            .map(ProjectResponse::from_model)
            .collect())
    }

    pub async fn get_project(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        requester_id: Uuid,
        project_id: Uuid,
    ) -> Result<ProjectResponse, AppError> {
        let project = if let Some(org_id) = org_id {
            ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?
        } else {
            let p = ProjectsRepository::find_by_id(db, project_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;
            if p.owner_id != requester_id {
                return Err(AppError::Forbidden(
                    "You are not authorized to access this project".to_string(),
                ));
            }
            p
        };

        Ok(ProjectResponse::from_model(project))
    }

    pub async fn update_project(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        requester_id: Uuid,
        project_id: Uuid,
        req: UpdateProjectDTO,
    ) -> Result<ProjectResponse, AppError> {
        let project = if let Some(org_id) = org_id {
            ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?
        } else {
            ProjectsRepository::find_by_id(db, project_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?
        };

        let mut active_model: ProjectActiveModel = project.clone().into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(new_name) = req.name {
            if let Some(org_id) = project.organization_id {
                if let Some(existing) =
                    ProjectsRepository::find_by_org_and_name(db, org_id, &new_name).await?
                {
                    if existing.id != project_id {
                        return Err(AppError::Conflict(format!(
                            "Project with name '{}' already exists in this organization",
                            new_name
                        )));
                    }
                }
            } else {
                if let Some(existing) =
                    ProjectsRepository::find_by_owner_and_name(db, requester_id, &new_name).await?
                {
                    if existing.id != project_id {
                        return Err(AppError::Conflict(format!(
                            "Project with name '{}' already exists!",
                            new_name
                        )));
                    }
                }
            }
            active_model.name = Set(new_name);
        }

        if let Some(desc) = req.description {
            active_model.description = Set(Some(desc));
        }
        if let Some(ptype) = req.project_type {
            active_model.project_type = Set(ptype);
        }
        if let Some(rt) = req.runtime {
            active_model.runtime = Set(rt);
        }
        if let Some(port) = req.port {
            active_model.port = Set(port);
        }
        if let Some(hurl) = req.health_check_url {
            active_model.health_check_url = Set(Some(hurl));
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        if org_id.is_none() && project.owner_id != requester_id {
            return Err(AppError::Forbidden(
                "You are not authorized to update this project".to_string(),
            ));
        }

        let updated = ProjectsRepository::update_project(db, active_model).await?;
        Ok(ProjectResponse::from_model(updated))
    }

    pub async fn delete_project(
        db: &DatabaseConnection,
        requester_id: Uuid,
        org_id: Option<Uuid>,
        project_id: Uuid,
    ) -> Result<(), AppError> {
        if let Some(org_id) = org_id {
            ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?
        } else {
            let p = ProjectsRepository::find_by_id(db, project_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;
            if p.owner_id != requester_id {
                return Err(AppError::Forbidden(
                    "You are not authorized to delete this project".to_string(),
                ));
            }
            p
        };

        // todo: check if other thing is running

        ProjectsRepository::delete_project(db, project_id).await?;
        Ok(())
    }
}
