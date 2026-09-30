use super::super::projects::repository::ProjectsRepository;
use super::dto::{
    ConnectProjectRepositoryDTO, ProjectRepositoryResponse, UpdateProjectRepositoryDTO,
};
use super::repository::ProjectRepositoriesRepository;
use crate::modules::projects::repositories::utils::ATService;
use crate::shared::error::AppError;
use sea_orm::*;
use uuid::Uuid;

pub struct ProjectRepositoriesService;

impl ProjectRepositoriesService {
    pub async fn connect_repository(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        req: ConnectProjectRepositoryDTO,
    ) -> Result<ProjectRepositoryResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if (ProjectRepositoriesRepository::find_by_project_id(db, project_id).await?).is_some() {
            let mut req_dto = UpdateProjectRepositoryDTO {
                access_token: None,
                default_branch: Some("main".to_string()),
                repository_url: Some(req.repository_url),
            };
            if let Some(token) = req.access_token {
                req_dto.access_token = Some(token);
            }

            if let Some(branch) = req.default_branch {
                req_dto.default_branch = Some(branch);
            }

            let a = Self::update_repository(db, org_id, project_id, req_dto).await?;

            return Ok(a);
        }

        let encrypted_token = req
            .access_token
            .as_ref()
            .map(|t| ATService::encrypt(t))
            .unwrap_or_default();

        let repository =
            ProjectRepositoriesRepository::connect_repository(db, req, project_id, encrypted_token)
                .await?;
        Ok(ProjectRepositoryResponse::from_model(repository))
    }

    pub async fn get_repository(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
    ) -> Result<ProjectRepositoryResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("No repository connected to this project".to_string())
            })?;

        Ok(ProjectRepositoryResponse::from_model(repo))
    }
    pub async fn get_repository_by_id_internal(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<ProjectRepositoryResponse, AppError> {
        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("No repository connected to this project".to_string())
            })?;

        Ok(ProjectRepositoryResponse::from_model(repo))
    }

    pub async fn get_repository_by_project_id_internal(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<Option<ProjectRepositoryResponse>, AppError> {
        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id).await?;

        Ok(repo.map(|r| ProjectRepositoryResponse::from_model(r)))
    }

    pub async fn update_repository(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        req: UpdateProjectRepositoryDTO,
    ) -> Result<ProjectRepositoryResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("No repository connected to this project".to_string())
            })?;

        let updated = ProjectRepositoriesRepository::update_repository(db, repo, req).await?;
        Ok(ProjectRepositoryResponse::from_model(updated))
    }

    pub async fn disconnect_repository(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
    ) -> Result<(), AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("No repository connected to this project".to_string())
            })?;

        ProjectRepositoriesRepository::disconnect_repository(db, repo).await?;
        Ok(())
    }

    pub async fn get_decrypted_token(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
    ) -> Result<Option<String>, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id).await?;
        Ok(repo.map(|r| ATService::decrypt(&r.access_token_encrypted)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_get_repo_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectRepositoriesService::get_repository(&db, None, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disconnect_repo_project_not_found() {
        let db = setup_mock_db();
        let result =
            ProjectRepositoriesService::disconnect_repository(&db, None, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
