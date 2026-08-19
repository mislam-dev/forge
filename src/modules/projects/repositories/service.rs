use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{ConnectRepositoryRequest, RepositoryResponse, UpdateRepositoryRequest};
use super::entities::project_repository::ActiveModel as RepositoryActiveModel;
use super::repository::ProjectRepositoriesRepository;
use super::super::permissions::role::ProjectRole;
use super::super::permissions::service::ProjectPermissionsService;
use super::super::projects::repository::ProjectsRepository;
use crate::shared::error::AppError;

pub struct ProjectRepositoriesService;

impl ProjectRepositoriesService {
    fn encrypt_token(raw_token: &str) -> String {
        raw_token.bytes().map(|b| format!("{:02x}", b)).collect()
    }

    pub async fn connect_repository(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: ConnectRepositoryRequest,
    ) -> Result<RepositoryResponse, AppError> {
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

        if (ProjectRepositoriesRepository::find_by_project_id(db, project_id).await?).is_some() {
            return Err(AppError::Conflict(
                "A repository is already connected to this project".to_string(),
            ));
        }

        let auth_type = req.auth_type.unwrap_or_else(|| "none".to_string());
        let encrypted_token = req
            .access_token
            .as_ref()
            .map(|t| Self::encrypt_token(t))
            .unwrap_or_default();

        let now = Utc::now().into();
        let active_model = RepositoryActiveModel {
            id: Set(Uuid::new_v4()),
            project_id: Set(project_id),
            repository_url: Set(req.repository_url),
            auth_type: Set(auth_type),
            access_token_encrypted: Set(encrypted_token),
            default_branch: Set(Some(req.default_branch.unwrap_or_else(|| "main".to_string()))),
            status: Set(Some("connected".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let repository = ProjectRepositoriesRepository::connect_repository(db, active_model).await?;
        Ok(RepositoryResponse::from_model(repository))
    }

    pub async fn get_repository(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<RepositoryResponse, AppError> {
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

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("No repository connected to this project".to_string()))?;

        Ok(RepositoryResponse::from_model(repo))
    }

    pub async fn update_repository(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: UpdateRepositoryRequest,
    ) -> Result<RepositoryResponse, AppError> {
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

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("No repository connected to this project".to_string()))?;

        let mut active_model: RepositoryActiveModel = repo.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(url) = req.repository_url {
            active_model.repository_url = Set(url);
        }
        if let Some(atype) = req.auth_type {
            active_model.auth_type = Set(atype);
        }
        if let Some(token) = req.access_token {
            active_model.access_token_encrypted = Set(Self::encrypt_token(&token));
        }
        if let Some(branch) = req.default_branch {
            active_model.default_branch = Set(Some(branch));
        }

        let updated = ProjectRepositoriesRepository::update_repository(db, active_model).await?;
        Ok(RepositoryResponse::from_model(updated))
    }

    pub async fn disconnect_repository(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<(), AppError> {
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

        ProjectRepositoriesRepository::delete_by_project_id(db, project_id).await?;
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
    async fn test_get_repository_not_found() {
        let db = setup_mock_db();
        let result = ProjectRepositoriesService::get_repository(&db, Uuid::new_v4(), false, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disconnect_repository_not_found() {
        let db = setup_mock_db();
        let result = ProjectRepositoriesService::disconnect_repository(&db, Uuid::new_v4(), false, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
