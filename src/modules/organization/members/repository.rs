use sea_orm::*;
use uuid::Uuid;

use super::entities::organization_invitations::{
    ActiveModel as InvitationActiveModel, Column as InvitationColumn, Entity as InvitationEntity,
    Model as InvitationModel,
};
use super::entities::organization_members::{
    ActiveModel as MemberActiveModel, Column as MemberColumn, Entity as MemberEntity,
    Model as MemberModel,
};
use super::entities::sea_orm_active_enums::{
    OrganizationInvitationsStatus, OrganizationMemberRole,
};
use crate::modules::users::entities::users::{
    Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
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

    pub async fn count_owners(db: &DatabaseConnection, org_id: Uuid) -> Result<u64, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::Role.eq(OrganizationMemberRole::Owner))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_members(db: &DatabaseConnection, org_id: Uuid) -> Result<u64, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<MemberModel>, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
        role: OrganizationMemberRole,
    ) -> Result<MemberModel, AppError> {
        Self::add_member_with_txn(db, org_id, user_id, role).await
    }

    pub async fn add_member_with_txn<C>(
        db: &C,
        org_id: Uuid,
        user_id: Uuid,
        role: OrganizationMemberRole,
    ) -> Result<MemberModel, AppError>
    where
        C: ConnectionTrait,
    {
        let active_model = MemberActiveModel {
            organization_id: Set(org_id),
            user_id: Set(user_id),
            role: Set(Some(role)),
            joined_at: Set(chrono::Utc::now().into()),
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_member_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
        new_role: OrganizationMemberRole,
    ) -> Result<MemberModel, AppError> {
        let member = Self::find_member(db, org_id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in organization".to_string()))?;

        let mut active: MemberActiveModel = member.into();
        active.role = Set(Some(new_role));
        active.update(db).await.map_err(AppError::from)
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
            .filter(InvitationColumn::Status.eq(OrganizationInvitationsStatus::Pending))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_pending_invitation_by_email(
        db: &DatabaseConnection,
        org_id: Uuid,
        email: &str,
    ) -> Result<Option<InvitationModel>, AppError> {
        InvitationEntity::find()
            .filter(InvitationColumn::OrganizationId.eq(org_id))
            .filter(InvitationColumn::Email.eq(email))
            .filter(InvitationColumn::Status.eq(OrganizationInvitationsStatus::Pending))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_invitation(
        db: &DatabaseConnection,
        org_id: Uuid,
        email: String,
        role: OrganizationMemberRole,
        token: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<InvitationModel, AppError> {
        let now = chrono::Utc::now();
        let active_model = InvitationActiveModel {
            organization_id: Set(org_id),
            email: Set(email),
            role: Set(Some(role)),
            token: Set(token),
            status: Set(Some(OrganizationInvitationsStatus::Pending)),
            expires_at: Set(expires_at.into()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn accept_invitation(
        db: &DatabaseConnection,
        invitation_id: Uuid,
    ) -> Result<InvitationModel, AppError> {
        let invite = InvitationEntity::find_by_id(invitation_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".to_string()))?;

        let mut active: InvitationActiveModel = invite.into();
        active.status = Set(Some(OrganizationInvitationsStatus::Accepted));
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(db).await.map_err(AppError::from)
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
        let result =
            OrganizationMembersRepository::find_invitation_by_token(&db, "tok_nonexistent").await;
        assert!(result.is_err());
    }
}
