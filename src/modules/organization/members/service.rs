use super::dto::request::{InviteMemberRequest, UpdateMemberRoleRequest};
use super::dto::response::{InvitationResponse, MemberResponse};
use super::entities::sea_orm_active_enums::{
    OrganizationInvitationsStatus, OrganizationMemberRole,
};
use super::repository::{OrganizationInvitationRepository, OrganizationMembersRepository};
use crate::modules::organization::orgs::repository::OrganizationRepository;
use crate::modules::users::service::UserService;
use crate::shared::error::AppError;
use chrono::{Duration, Utc};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use uuid::Uuid;

pub struct OrganizationMembersService;

impl OrganizationMembersService {
    pub async fn invite_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        dto: InviteMemberRequest,
    ) -> Result<InvitationResponse, AppError> {
        let _org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let member_role: OrganizationMemberRole = dto.role.parse().map_err(AppError::BadRequest)?;

        let user = UserService::find_by_email(db, &dto.email).await?;

        if OrganizationMembersRepository::find_member(db, org_id, user.id)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict(
                "User is already a member of this organization".to_string(),
            ));
        }

        if OrganizationInvitationRepository::find_pending_by_email(db, org_id, &dto.email)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict(
                "A pending invitation already exists for this email".to_string(),
            ));
        }

        let token = format!("tok_{}", Uuid::new_v4().simple());
        let now = Utc::now();
        let expires_at = now + Duration::days(7);

        let created_invite = OrganizationInvitationRepository::create(
            db,
            org_id,
            dto.email,
            member_role,
            token,
            expires_at,
        )
        .await?;

        Ok(InvitationResponse::from_model(created_invite))
    }

    pub async fn accept_invitation(
        db: &DatabaseConnection,
        token: &str,
        user_id: Uuid,
    ) -> Result<MemberResponse, AppError> {
        let invite = OrganizationInvitationRepository::find_by_token(db, token)
            .await?
            .ok_or_else(|| AppError::NotFound("Invalid or expired invitation token".to_string()))?;

        if invite.status != Some(OrganizationInvitationsStatus::Pending) {
            return Err(AppError::BadRequest(
                "Invitation has already been used or cancelled".to_string(),
            ));
        }

        let now = Utc::now();
        if invite.expires_at.with_timezone(&Utc) < now {
            return Err(AppError::BadRequest(
                "Invitation token has expired".to_string(),
            ));
        }

        let user = UserService::find_one(db, user_id).await?;

        if OrganizationMembersRepository::find_member(db, invite.organization_id, user_id)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict(
                "User is already a member of this organization".to_string(),
            ));
        }

        let role = invite.role.unwrap_or(OrganizationMemberRole::Viewer);
        let member =
            OrganizationMembersRepository::add_member(db, invite.organization_id, user_id, role)
                .await?;

        OrganizationInvitationRepository::accept(db, invite.id).await?;

        Ok(MemberResponse::from_model(member, Some(user.email)))
    }

    pub async fn list_members(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<MemberResponse>, AppError> {
        let _org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let members = OrganizationMembersRepository::find_members_by_org_id(db, org_id).await?;

        let user_ids: Vec<Uuid> = members.iter().map(|m| m.user_id).collect();
        let users = UserService::find_ids(db, user_ids).await?;

        let user_email_map: HashMap<Uuid, String> =
            users.into_iter().map(|u| (u.id, u.email)).collect();

        let member_data = members
            .into_iter()
            .map(|m| {
                let email = user_email_map.get(&m.user_id).cloned();
                MemberResponse::from_model(m, email)
            })
            .collect();
        Ok(member_data)
    }

    pub async fn list_invitations(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<InvitationResponse>, AppError> {
        let _org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let invites = OrganizationInvitationRepository::find_pending(db, org_id).await?;
        Ok(invites
            .into_iter()
            .map(InvitationResponse::from_model)
            .collect())
    }

    pub async fn update_member_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        target_user_id: Uuid,
        dto: UpdateMemberRoleRequest,
    ) -> Result<MemberResponse, AppError> {
        let new_role: OrganizationMemberRole = dto.role.parse().map_err(AppError::BadRequest)?;

        let target_member = OrganizationMembersRepository::find_member(db, org_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in organization".to_string()))?;

        let target_member_role = target_member.role.unwrap_or(OrganizationMemberRole::Viewer);

        // ! currently allowing only 1 owner for each organization
        if target_member_role == OrganizationMemberRole::Owner
            || new_role == OrganizationMemberRole::Owner
        {
            return Err(AppError::Forbidden(
                "Owner cannot be modified or removed from the Organization".to_string(),
            ));
        }

        let updated =
            OrganizationMembersRepository::update_member_role(db, org_id, target_user_id, new_role)
                .await?;

        let target_user = UserService::find_one(db, target_user_id).await?;

        Ok(MemberResponse::from_model(updated, Some(target_user.email)))
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<(), AppError> {
        let target_member = OrganizationMembersRepository::find_member(db, org_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in organization".to_string()))?;

        let target_role = target_member.role.unwrap_or(OrganizationMemberRole::Viewer);

        if target_role == OrganizationMemberRole::Owner {
            return Err(AppError::Forbidden(
                "Owner cannot be modified or removed from the Organization".to_string(),
            ));
        }

        OrganizationMembersRepository::remove_member(db, org_id, target_user_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_accept_invitation_invalid_token() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result =
            OrganizationMembersService::accept_invitation(&db, "invalid_tok", user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_members_org_not_found() {
        let db = setup_mock_db();
        let org_id = Uuid::new_v4();
        let result = OrganizationMembersService::list_members(&db, org_id).await;
        assert!(result.is_err());
    }
}
