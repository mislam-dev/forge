use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{CreateProjectRequest, ProjectQuery, ProjectResponse, UpdateProjectRequest};
use super::entities::project::ActiveModel as ProjectActiveModel;
use super::repository::ProjectsRepository;
use super::super::permissions::role::ProjectRole;
use super::super::permissions::service::ProjectPermissionsService;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::shared::error::AppError;

pub struct ProjectsService;

impl ProjectsService {
    pub async fn create_project(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        req: CreateProjectRequest,
    ) -> Result<ProjectResponse, AppError> {
        let org_id = req.organization_id;

        if !is_system_admin {
            let role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
            match role {
                Some(r) if r >= OrgRole::Editor => {}
                _ => {
                    return Err(AppError::Forbidden(
                        "You must be an Organization Developer/Editor or higher to create a project".to_string(),
                    ));
                }
            }
        }

        if (ProjectsRepository::find_by_org_and_name(db, org_id, &req.name).await?).is_some() {
            return Err(AppError::Conflict(format!(
                "Project with name '{}' already exists in this organization",
                req.name
            )));
        }

        let now = Utc::now().into();
        let project_id = Uuid::new_v4();

        let active_model = ProjectActiveModel {
            id: Set(project_id),
            organization_id: Set(org_id),
            owner_id: Set(requester_id),
            name: Set(req.name),
            description: Set(req.description),
            project_type: Set(req.project_type),
            runtime: Set(req.runtime),
            port: Set(req.port.unwrap_or(3000)),
            health_check_url: Set(req.health_check_url.or_else(|| Some("/health".to_string()))),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let project = ProjectsRepository::create_project(db, active_model).await?;
        Ok(ProjectResponse::from_model(project))
    }

    pub async fn list_projects(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        query: ProjectQuery,
    ) -> Result<Vec<ProjectResponse>, AppError> {
        let projects = if let Some(org_id) = query.organization_id {
            if !is_system_admin {
                let role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                if role.is_none() {
                    return Err(AppError::Forbidden(
                        "You are not a member of this organization".to_string(),
                    ));
                }
            }
            ProjectsRepository::find_by_org_id(db, org_id).await?
        } else {
            vec![]
        };

        Ok(projects.into_iter().map(ProjectResponse::from_model).collect())
    }

    pub async fn get_project(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<ProjectResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Viewer,
            )
            .await?;
        }

        Ok(ProjectResponse::from_model(project))
    }

    pub async fn update_project(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: UpdateProjectRequest,
    ) -> Result<ProjectResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Admin,
            )
            .await?;
        }

        let mut active_model: ProjectActiveModel = project.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(new_name) = req.name {
            let org_id = active_model.organization_id.clone().unwrap();
            if let Some(existing) = ProjectsRepository::find_by_org_and_name(db, org_id, &new_name).await? {
                if existing.id != project_id {
                    return Err(AppError::Conflict(format!(
                        "Project with name '{}' already exists in this organization",
                        new_name
                    )));
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

        let updated = ProjectsRepository::update_project(db, active_model).await?;
        Ok(ProjectResponse::from_model(updated))
    }

    pub async fn delete_project(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<(), AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin && project.owner_id != requester_id {
            let org_role = OrgPermissionsService::resolve_org_role(db, project.organization_id, requester_id).await?;
            if org_role != Some(OrgRole::Owner) {
                return Err(AppError::Forbidden(
                    "Only the Project Owner or Organization Owner can delete this project".to_string(),
                ));
            }
        }

        ProjectsRepository::delete_project(db, project_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        let db = setup_mock_db();
        let project_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let result = ProjectsService::get_project(&db, requester_id, false, project_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_project_not_found() {
        let db = setup_mock_db();
        let project_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let result = ProjectsService::delete_project(&db, requester_id, false, project_id).await;
        assert!(result.is_err());
    }
}
