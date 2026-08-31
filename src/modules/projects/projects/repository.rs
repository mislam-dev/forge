use sea_orm::*;
use uuid::Uuid;

use super::entities::projects::{
    ActiveModel as ProjectActiveModel, Column as ProjectColumn, Entity as ProjectEntity,
    Model as ProjectModel,
};
use crate::{
    modules::projects::projects::{
        dto::CreateProjectDTO, entities::sea_orm_active_enums::ProjectStatus,
    },
    shared::error::AppError,
};

pub struct ProjectsRepository;

impl ProjectsRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<ProjectModel>, AppError> {
        ProjectEntity::find()
            .filter(ProjectColumn::Id.eq(id))
            .filter(ProjectColumn::OrganizationId.is_null())
            .one(db)
            .await
            .map_err(AppError::from)
    }
    pub async fn find_by_id_with_org(
        db: &DatabaseConnection,
        id: Uuid,
        org_id: Uuid,
    ) -> Result<Option<ProjectModel>, AppError> {
        ProjectEntity::find()
            .filter(ProjectColumn::Id.eq(id))
            .filter(ProjectColumn::OrganizationId.eq(org_id))
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
    pub async fn find_by_owner_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<ProjectModel>, AppError> {
        ProjectEntity::find()
            .filter(ProjectColumn::OwnerId.eq(user_id))
            .all(db)
            .await
            .map_err(AppError::from)
    }
    pub async fn find_by_owner_and_name(
        db: &DatabaseConnection,
        owner_id: Uuid,
        name: &str,
    ) -> Result<Option<ProjectModel>, AppError> {
        ProjectEntity::find()
            .filter(ProjectColumn::OwnerId.eq(owner_id))
            .filter(ProjectColumn::Name.eq(name))
            .one(db)
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
        requester_id: Uuid,
        org_id: Option<Uuid>,
        req: CreateProjectDTO,
    ) -> Result<ProjectModel, AppError> {
        // todo: work on this
        let active_model = ProjectActiveModel {
            owner_id: Set(requester_id),
            name: Set(req.name),
            description: Set(req.description),
            project_type: Set(req.project_type),
            runtime: Set(req.runtime),
            port: Set(req.port.unwrap_or(3000)),
            health_check_url: Set(req.health_check_url.or_else(|| Some("/health".to_string()))),
            status: Set(ProjectStatus::Active),
            organization_id: Set(org_id),
            ..Default::default()
        };

        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_project(
        db: &DatabaseConnection,
        active_model: ProjectActiveModel,
    ) -> Result<ProjectModel, AppError> {
        // todo: work on this
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn delete_project(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
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
