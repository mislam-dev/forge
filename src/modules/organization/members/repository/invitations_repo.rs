use sea_orm::*;
use uuid::Uuid;

use super::super::entities::organization_invitations::{
    ActiveModel as InvitationActiveModel, Column as InvitationColumn, Entity as InvitationEntity,
    Model as InvitationModel,
};

use super::super::entities::sea_orm_active_enums::{
    OrganizationInvitationsStatus, OrganizationMemberRole,
};

use crate::shared::error::AppError;

pub struct OrganizationInvitationRepository;

impl OrganizationInvitationRepository {
    pub async fn find_by_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> Result<Option<InvitationModel>, AppError> {
        InvitationEntity::find()
            .filter(InvitationColumn::Token.eq(token))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_pending(
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

    pub async fn find_pending_by_email(
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

    pub async fn create(
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

    pub async fn accept(
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
