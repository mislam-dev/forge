use sea_orm::*;
use uuid::Uuid;

use super::entities::project::{
    ActiveModel as ProjectActiveModel, Column as ProjectColumn, Entity as ProjectEntity,
    Model as ProjectModel,
};
use crate::shared::error::AppError;

pub struct ProjectsRepository;

impl ProjectsRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<ProjectModel>, AppError> {
        ProjectEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_org_id(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<ProjectModel>, AppError> {
        ProjectEntity::find()
            .filter(ProjectColumn::OrganizationId.eq(org_id))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_org_and_name(
        db: &DatabaseConnection,
        org_id: Uuid,
        name: &str,
    ) -> Result<Option<ProjectModel>, AppError> {
        ProjectEntity::find()
            .filter(ProjectColumn::OrganizationId.eq(org_id))
            .filter(ProjectColumn::Name.eq(name))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_project(
        db: &DatabaseConnection,
        active_model: ProjectActiveModel,
    ) -> Result<ProjectModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_project(
        db: &DatabaseConnection,
        active_model: ProjectActiveModel,
    ) -> Result<ProjectModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn delete_project(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<u64, AppError> {
        let res = ProjectEntity::delete_by_id(id).exec(db).await?;
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
    async fn test_find_by_id_empty_db() {
        let db = setup_mock_db();
        let result = ProjectsRepository::find_by_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_by_org_id_empty_db() {
        let db = setup_mock_db();
        let result = ProjectsRepository::find_by_org_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
