use sea_orm::*;
use uuid::Uuid;

use super::dto::AssignProjectMemberDTO;
use super::entities::project_members::{
    ActiveModel as ProjectMemberActiveModel, Column as ProjectMemberColumn,
    Entity as ProjectMemberEntity, Model as ProjectMemberModel,
};
use crate::modules::users::entities::users::{
    Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::shared::error::AppError;

pub struct ProjectAssignmentsRepository;

impl ProjectAssignmentsRepository {
    pub async fn find_member(
        db: &DatabaseConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<ProjectMemberModel>, AppError> {
        ProjectMemberEntity::find()
            .filter(ProjectMemberColumn::ProjectId.eq(project_id))
            .filter(ProjectMemberColumn::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_members_by_project_id(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<Vec<(ProjectMemberModel, Option<UserModel>)>, AppError> {
        let members = ProjectMemberEntity::find()
            .filter(ProjectMemberColumn::ProjectId.eq(project_id))
            .all(db)
            .await?;

        let user_ids: Vec<Uuid> = members.iter().map(|m| m.user_id).collect();

        let users = if !user_ids.is_empty() {
            UserEntity::find()
                .filter(UserColumn::Id.is_in(user_ids))
                .all(db)
                .await?
        } else {
            vec![]
        };

        let mut result = Vec::with_capacity(members.len());
        for m in members {
            let u = users.iter().find(|user| user.id == m.user_id).cloned();
            result.push((m, u));
        }

        Ok(result)
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        project_id: Uuid,
        req: AssignProjectMemberDTO,
    ) -> Result<ProjectMemberModel, AppError> {
        let active_model = ProjectMemberActiveModel {
            project_id: Set(project_id),
            user_id: Set(req.user_id),
            role: Set(Some(req.role)),
            ..Default::default()
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = ProjectMemberEntity::delete_many()
            .filter(ProjectMemberColumn::ProjectId.eq(project_id))
            .filter(ProjectMemberColumn::UserId.eq(user_id))
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
    async fn test_find_member_empty_db() {
        let db = setup_mock_db();
        let result =
            ProjectAssignmentsRepository::find_member(&db, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
