use sea_orm::*;
use uuid::Uuid;

use super::entities::project_environment_variable::{
    ActiveModel as EnvVarActiveModel, Column as EnvVarColumn, Entity as EnvVarEntity,
    Model as EnvVarModel,
};
use crate::shared::error::AppError;

pub struct ProjectEnvironmentVariablesRepository;

impl ProjectEnvironmentVariablesRepository {
    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: Uuid,
    ) -> Result<Option<EnvVarModel>, AppError> {
        EnvVarEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_project_id<C: ConnectionTrait>(
        db: &C,
        project_id: Uuid,
        environment: Option<String>,
    ) -> Result<Vec<EnvVarModel>, AppError> {
        let mut query = EnvVarEntity::find().filter(EnvVarColumn::ProjectId.eq(project_id));
        if let Some(env) = environment {
            query = query.filter(EnvVarColumn::Environment.eq(env));
        }

        query.all(db).await.map_err(AppError::from)
    }

    pub async fn find_by_project_env_key<C: ConnectionTrait>(
        db: &C,
        project_id: Uuid,
        environment: &str,
        key: &str,
    ) -> Result<Option<EnvVarModel>, AppError> {
        EnvVarEntity::find()
            .filter(EnvVarColumn::ProjectId.eq(project_id))
            .filter(EnvVarColumn::Environment.eq(environment))
            .filter(EnvVarColumn::Key.eq(key))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_env_var<C: ConnectionTrait>(
        db: &C,
        active_model: EnvVarActiveModel,
    ) -> Result<EnvVarModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_env_var<C: ConnectionTrait>(
        db: &C,
        active_model: EnvVarActiveModel,
    ) -> Result<EnvVarModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn delete_env_var<C: ConnectionTrait>(db: &C, id: Uuid) -> Result<u64, AppError> {
        let res = EnvVarEntity::delete_by_id(id).exec(db).await?;
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
        let result = ProjectEnvironmentVariablesRepository::find_by_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
