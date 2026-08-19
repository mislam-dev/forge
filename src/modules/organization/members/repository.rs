use sea_orm::*;
use uuid::Uuid;

use super::entities::organization_invitation::{
    ActiveModel as InvitationActiveModel, Column as InvitationColumn, Entity as InvitationEntity,
    Model as InvitationModel,
};
use super::entities::organization_member::{
    ActiveModel as MemberActiveModel, Column as MemberColumn, Entity as MemberEntity,
    Model as MemberModel,
};
use crate::modules::users::entities::users::{Column as UserColumn, Entity as UserEntity, Model as UserModel};
use crate::shared::error::AppError;

pub struct OrganizationMembersRepository;

impl OrganizationMembersRepository {
    pub async fn find_members_by_org_id(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<(MemberModel, Option<UserModel>)>, AppError> {
        let members = MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
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

    pub async fn find_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<MemberModel>, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_owners(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<u64, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::Role.eq("owner"))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        active_model: MemberActiveModel,
    ) -> Result<MemberModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_member(
        db: &DatabaseConnection,
        active_model: MemberActiveModel,
    ) -> Result<MemberModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = MemberEntity::delete_many()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::UserId.eq(user_id))
            .exec(db)
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn find_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<UserModel>, AppError> {
        UserEntity::find()
            .filter(UserColumn::Email.eq(email))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_user_by_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Option<UserModel>, AppError> {
        UserEntity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_invitation_by_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> Result<Option<InvitationModel>, AppError> {
        InvitationEntity::find()
            .filter(InvitationColumn::Token.eq(token))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_pending_invitations(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<InvitationModel>, AppError> {
        InvitationEntity::find()
            .filter(InvitationColumn::OrganizationId.eq(org_id))
            .filter(InvitationColumn::Status.eq("pending"))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_invitation(
        db: &DatabaseConnection,
        active_model: InvitationActiveModel,
    ) -> Result<InvitationModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_invitation(
        db: &DatabaseConnection,
        active_model: InvitationActiveModel,
    ) -> Result<InvitationModel, AppError> {
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
    async fn test_find_member_empty_db() {
        let db = setup_mock_db();
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let result = OrganizationMembersRepository::find_member(&db, org_id, user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_invitation_by_token_empty_db() {
        let db = setup_mock_db();
        let result = OrganizationMembersRepository::find_invitation_by_token(&db, "tok_nonexistent").await;
        assert!(result.is_err());
    }
}
