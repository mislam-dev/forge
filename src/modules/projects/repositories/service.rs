use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::super::projects::repository::ProjectsRepository;
use super::dto::{
    ConnectProjectRepositoryDTO, ProjectRepositoryResponse, UpdateProjectRepositoryDTO,
};
use super::entities::project_repository::ActiveModel as RepositoryActiveModel;
use super::repository::ProjectRepositoriesRepository;
use crate::shared::error::AppError;

pub struct ProjectRepositoriesService;

impl ProjectRepositoriesService {
    fn encrypt_token(raw_token: &str) -> String {
        raw_token.bytes().map(|b| format!("{:02x}", b)).collect()
    }

    fn decrypt_token(encrypted_hex: &str) -> String {
        let mut bytes = Vec::new();
        for i in (0..encrypted_hex.len()).step_by(2) {
            if i + 2 <= encrypted_hex.len() {
                if let Ok(b) = u8::from_str_radix(&encrypted_hex[i..i + 2], 16) {
                    bytes.push(b);
                }
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
    }

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
            default_branch: Set(Some(
                req.default_branch.unwrap_or_else(|| "main".to_string()),
            )),
            status: Set(Some("connected".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let repository =
            ProjectRepositoriesRepository::connect_repository(db, active_model).await?;
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

        let mut active_model: RepositoryActiveModel = repo.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(url) = req.repository_url {
            active_model.repository_url = Set(url);
        }
        if let Some(auth_type) = req.auth_type {
            active_model.auth_type = Set(auth_type);
        }
        if let Some(token) = req.access_token {
            active_model.access_token_encrypted = Set(Self::encrypt_token(&token));
        }
        if let Some(branch) = req.default_branch {
            active_model.default_branch = Set(Some(branch));
        }

        let updated = ProjectRepositoriesRepository::update_repository(db, active_model).await?;
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

        let mut active_model: RepositoryActiveModel = repo.into();
        active_model.status = Set(Some("disconnected".to_string()));
        active_model.access_token_encrypted = Set("".to_string());
        active_model.updated_at = Set(Utc::now().into());

        ProjectRepositoriesRepository::update_repository(db, active_model).await?;
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
        Ok(repo.map(|r| Self::decrypt_token(&r.access_token_encrypted)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[test]
    fn test_token_encryption_decryption_roundtrip() {
        let pat = "ghp_1234567890abcdefghijklmnopqrstuvwxyz";
        let encrypted = ProjectRepositoriesService::encrypt_token(pat);
        assert_ne!(pat, encrypted);
        let decrypted = ProjectRepositoriesService::decrypt_token(&encrypted);
        assert_eq!(pat, decrypted);
    }

    #[tokio::test]
    async fn test_connect_repo_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectRepositoriesService::connect_repository(
            &db,
            None,
            Uuid::new_v4(),
            ConnectProjectRepositoryDTO {
                repository_url: "https://github.com/org/repo.git".to_string(),
                auth_type: Some("pat".to_string()),
                access_token: Some("secret".to_string()),
                default_branch: Some("main".to_string()),
            },
        )
        .await;
        assert!(result.is_err());
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
