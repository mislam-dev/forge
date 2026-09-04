use sea_orm::*;
use uuid::Uuid;

use super::entities::project_repository::{
    ActiveModel as RepositoryActiveModel, Column as RepositoryColumn, Entity as RepositoryEntity,
    Model as RepositoryModel,
};
use crate::{
    modules::projects::repositories::{
        dto::{ConnectProjectRepositoryDTO, UpdateProjectRepositoryDTO},
        utils::ATService,
    },
    shared::error::AppError,
};

pub struct ProjectRepositoriesRepository;

impl ProjectRepositoriesRepository {
    pub async fn find_by_project_id(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<Option<RepositoryModel>, AppError> {
        RepositoryEntity::find()
            .filter(RepositoryColumn::ProjectId.eq(project_id))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn connect_repository(
        db: &DatabaseConnection,
        req: ConnectProjectRepositoryDTO,
        project_id: Uuid,
        token: String,
    ) -> Result<RepositoryModel, AppError> {
        let active_model = RepositoryActiveModel {
            project_id: Set(project_id),
            repository_url: Set(req.repository_url),
            auth_type: Set(req.auth_type.unwrap_or_else(|| "none".to_string())),
            access_token_encrypted: Set(token),
            default_branch: Set(Some(
                req.default_branch.unwrap_or_else(|| "main".to_string()),
            )),
            status: Set(Some("connected".to_string())),
            ..Default::default()
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_repository(
        db: &DatabaseConnection,
        repo: RepositoryModel,
        req: UpdateProjectRepositoryDTO,
    ) -> Result<RepositoryModel, AppError> {
        let mut active_model: RepositoryActiveModel = repo.into();

        if let Some(url) = req.repository_url {
            active_model.repository_url = Set(url);
        }
        if let Some(auth_type) = req.auth_type {
            active_model.auth_type = Set(auth_type);
        }
        if let Some(token) = req.access_token {
            active_model.access_token_encrypted = Set(ATService::encrypt(&token));
        }
        if let Some(branch) = req.default_branch {
            active_model.default_branch = Set(Some(branch));
        }

        active_model.update(db).await.map_err(AppError::from)
    }
    pub async fn disconnect_repository(
        db: &DatabaseConnection,
        repo: RepositoryModel,
    ) -> Result<RepositoryModel, AppError> {
        let mut active_model: RepositoryActiveModel = repo.into();
        active_model.status = Set(Some("disconnected".to_string()));
        active_model.access_token_encrypted = Set("".to_string());

        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn delete_by_project_id(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = RepositoryEntity::delete_many()
            .filter(RepositoryColumn::ProjectId.eq(project_id))
            .exec(db)
            .await?;
        Ok(res.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_find_by_project_id_empty_db() {
        let db = setup_mock_db();
        let result = ProjectRepositoriesRepository::find_by_project_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
