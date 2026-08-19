use sea_orm::*;
use uuid::Uuid;

use super::entities::project_repository::{
    ActiveModel as RepositoryActiveModel, Column as RepositoryColumn, Entity as RepositoryEntity,
    Model as RepositoryModel,
};
use crate::shared::error::AppError;

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
        active_model: RepositoryActiveModel,
    ) -> Result<RepositoryModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_repository(
        db: &DatabaseConnection,
        active_model: RepositoryActiveModel,
    ) -> Result<RepositoryModel, AppError> {
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
