use sea_orm::*;
use uuid::Uuid;

use super::dto::DeploymentHistoryQuery;
use super::entities::deployment::{
    ActiveModel as DeploymentActiveModel, Column as DeploymentColumn, Entity as DeploymentEntity,
    Model as DeploymentModel,
};
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;

pub struct DeploymentsRepository;

impl DeploymentsRepository {
    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: Uuid,
    ) -> Result<Option<DeploymentModel>, AppError> {
        DeploymentEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_project_id<C: ConnectionTrait>(
        db: &C,
        project_id: Uuid,
        query: DeploymentHistoryQuery,
    ) -> Result<PaginatedResponse<DeploymentModel>, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);

        let mut stmt = DeploymentEntity::find()
            .filter(DeploymentColumn::ProjectId.eq(project_id))
            .order_by_desc(DeploymentColumn::CreatedAt);

        if let Some(status) = query.status {
            stmt = stmt.filter(DeploymentColumn::Status.eq(status));
        }
        if let Some(branch) = query.branch {
            stmt = stmt.filter(DeploymentColumn::Branch.eq(branch));
        }

        let paginator = stmt.paginate(db, per_page);
        let total_items = paginator.num_items().await.map_err(AppError::from)?;
        let data = paginator
            .fetch_page(page - 1)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedResponse::new(data, page, per_page, total_items))
    }

    pub async fn find_running_by_project_id<C: ConnectionTrait>(
        db: &C,
        project_id: Uuid,
    ) -> Result<Option<DeploymentModel>, AppError> {
        DeploymentEntity::find()
            .filter(DeploymentColumn::ProjectId.eq(project_id))
            .filter(DeploymentColumn::Status.is_in(["Queued", "Building", "Deploying", "Running"]))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_last_success_by_project_id<C: ConnectionTrait>(
        db: &C,
        project_id: Uuid,
    ) -> Result<Option<DeploymentModel>, AppError> {
        DeploymentEntity::find()
            .filter(DeploymentColumn::ProjectId.eq(project_id))
            .filter(DeploymentColumn::Status.eq("Success"))
            .order_by_desc(DeploymentColumn::CreatedAt)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_deployment<C: ConnectionTrait>(
        db: &C,
        active_model: DeploymentActiveModel,
    ) -> Result<DeploymentModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_deployment<C: ConnectionTrait>(
        db: &C,
        active_model: DeploymentActiveModel,
    ) -> Result<DeploymentModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
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
        let result = DeploymentsRepository::find_by_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
