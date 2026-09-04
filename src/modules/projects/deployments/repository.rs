use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::DeploymentHistoryQuery;
use super::entities::deployment::{
    ActiveModel as DeploymentActiveModel, Column as DeploymentColumn, Entity as DeploymentEntity,
    Model as DeploymentModel,
};
use super::status::DeploymentStatus;
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
            .filter(DeploymentColumn::Status.is_in([
                DeploymentStatus::Queued.as_str(),
                DeploymentStatus::Building.as_str(),
                DeploymentStatus::Deploying.as_str(),
                DeploymentStatus::Running.as_str(),
            ]))
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
            .filter(DeploymentColumn::Status.eq(DeploymentStatus::Success.as_str()))
            .order_by_desc(DeploymentColumn::CreatedAt)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_deployment<C: ConnectionTrait>(
        db: &C,
        project_id: Uuid,
        triggered_by: Uuid,
        branch: String,
        commit_hash: String,
        status: DeploymentStatus,
    ) -> Result<DeploymentModel, AppError> {
        let active_model = DeploymentActiveModel {
            project_id: Set(project_id),
            triggered_by: Set(triggered_by),
            branch: Set(branch),
            commit_hash: Set(commit_hash),
            status: Set(status.as_str().to_string()),
            ..Default::default()
        };

        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_deployment<C: ConnectionTrait>(
        db: &C,
        deployment: DeploymentModel,
        target_status: DeploymentStatus,
        build_duration: Option<i32>,
        deploy_duration: Option<i32>,
        error_message: Option<String>,
    ) -> Result<DeploymentModel, AppError> {
        let mut active_model: DeploymentActiveModel = deployment.into();
        active_model.status = Set(target_status.as_str().to_string());
        active_model.updated_at = Set(Utc::now().into());

        if let Some(bd) = build_duration {
            active_model.build_duration = Set(Some(bd));
        }
        if let Some(dd) = deploy_duration {
            active_model.deploy_duration = Set(Some(dd));
        }
        if let Some(err) = error_message {
            active_model.error_message = Set(Some(err));
        }

        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn update_status<C: ConnectionTrait>(
        db: &C,
        deployment: DeploymentModel,
        target_status: DeploymentStatus,
        build_duration: Option<i32>,
        deploy_duration: Option<i32>,
        error_message: Option<String>,
    ) -> Result<DeploymentModel, AppError> {
        Self::update_deployment(
            db,
            deployment,
            target_status,
            build_duration,
            deploy_duration,
            error_message,
        )
        .await
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

    #[tokio::test]
    async fn test_create_deployment_success() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now().into();

        let expected_model = DeploymentModel {
            id,
            project_id,
            triggered_by: user_id,
            branch: "main".to_string(),
            commit_hash: "abc1234".to_string(),
            status: "Queued".to_string(),
            build_duration: None,
            deploy_duration: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![expected_model.clone()]])
            .into_connection();

        let result = DeploymentsRepository::create_deployment(
            &db,
            project_id,
            user_id,
            "main".to_string(),
            "abc1234".to_string(),
            DeploymentStatus::Queued,
        )
        .await;

        assert!(result.is_ok());
        let created = result.unwrap();
        assert_eq!(created.branch, "main");
        assert_eq!(created.commit_hash, "abc1234");
        assert_eq!(created.status, "Queued");
    }

    #[tokio::test]
    async fn test_update_status_success() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now().into();

        let initial_model = DeploymentModel {
            id,
            project_id,
            triggered_by: user_id,
            branch: "main".to_string(),
            commit_hash: "abc1234".to_string(),
            status: "Queued".to_string(),
            build_duration: None,
            deploy_duration: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let updated_model = DeploymentModel {
            id,
            project_id,
            triggered_by: user_id,
            branch: "main".to_string(),
            commit_hash: "abc1234".to_string(),
            status: "Building".to_string(),
            build_duration: Some(5000),
            deploy_duration: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![updated_model.clone()]])
            .into_connection();

        let result = DeploymentsRepository::update_status(
            &db,
            initial_model,
            DeploymentStatus::Building,
            Some(5000),
            None,
            None,
        )
        .await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.status, "Building");
        assert_eq!(updated.build_duration, Some(5000));
    }
}
