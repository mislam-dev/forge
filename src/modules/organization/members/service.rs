use super::dto::request::{InviteMemberRequest, UpdateMemberRoleRequest};
use super::dto::response::{InvitationResponse, MemberResponse};
use super::entities::sea_orm_active_enums::{
    OrganizationInvitationsStatus, OrganizationMemberRole,
};
use super::repository::OrganizationMembersRepository;
use crate::modules::organization::orgs::repository::OrganizationRepository;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::shared::error::AppError;
use chrono::{Duration, Utc};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use validator::Validate;

pub struct OrganizationMembersService;

impl OrganizationMembersService {
    pub async fn invite_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        requester_id: Uuid,
        is_system_admin: bool,
        req: InviteMemberRequest,
    ) -> Result<InvitationResponse, AppError> {
        req.validate().map_err(AppError::from)?;

        let _org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        OrgPermissionsService::verify_org_role(
            db,
            org_id,
            requester_id,
            OrganizationMemberRole::Admin,
            is_system_admin,
        )
        .await?;

        let member_role: OrganizationMemberRole = req.role.parse().map_err(AppError::BadRequest)?;

        let target_email = if let Some(email) = req.email {
            email
        } else if let Some(user_id) = req.user_id {
            let user = OrganizationMembersRepository::find_user_by_id(db, user_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Target user not found".to_string()))?;
            user.email
        } else {
            return Err(AppError::BadRequest(
                "Either email or user_id must be provided".to_string(),
            ));
        };

        if let Some(user) =
            OrganizationMembersRepository::find_user_by_email(db, &target_email).await?
        {
            if OrganizationMembersRepository::find_member(db, org_id, user.id)
                .await?
                .is_some()
            {
                return Err(AppError::Conflict(
                    "User is already a member of this organization".to_string(),
                ));
            }
        }

        if OrganizationMembersRepository::find_pending_invitation_by_email(
            db,
            org_id,
            &target_email,
        )
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

        let created_invite = OrganizationMembersRepository::create_invitation(
            db,
            org_id,
            target_email,
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
        let invite = OrganizationMembersRepository::find_invitation_by_token(db, token)
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

        let user = OrganizationMembersRepository::find_user_by_id(db, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

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

        OrganizationMembersRepository::accept_invitation(db, invite.id).await?;

        Ok(MemberResponse::from_model(member, Some(user.email)))
    }

    pub async fn list_members(
        db: &DatabaseConnection,
        org_id: Uuid,
        requester_id: Uuid,
        is_system_admin: bool,
    ) -> Result<Vec<MemberResponse>, AppError> {
        let _org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        OrgPermissionsService::verify_org_role(
            db,
            org_id,
            requester_id,
            OrgRole::Viewer,
            is_system_admin,
        )
        .await?;

        let members_with_users =
            OrganizationMembersRepository::find_members_by_org_id(db, org_id).await?;

        let res = members_with_users
            .into_iter()
            .map(|(m, u)| {
                let email = u.map(|usr| usr.email);
                MemberResponse::from_model(m, email)
            })
            .collect();

        Ok(res)
    }

    pub async fn list_invitations(
        db: &DatabaseConnection,
        org_id: Uuid,
        requester_id: Uuid,
        is_system_admin: bool,
    ) -> Result<Vec<InvitationResponse>, AppError> {
        let _org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        OrgPermissionsService::verify_org_role(
            db,
            org_id,
            requester_id,
            OrgRole::Admin,
            is_system_admin,
        )
        .await?;

        let invites = OrganizationMembersRepository::find_pending_invitations(db, org_id).await?;
        Ok(invites
            .into_iter()
            .map(InvitationResponse::from_model)
            .collect())
    }

    pub async fn update_member_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        target_user_id: Uuid,
        requester_id: Uuid,
        is_system_admin: bool,
        req: UpdateMemberRoleRequest,
    ) -> Result<MemberResponse, AppError> {
        req.validate().map_err(AppError::from)?;

        let requester_role = OrgPermissionsService::verify_org_role(
            db,
            org_id,
            requester_id,
            OrganizationMemberRole::Admin,
            is_system_admin,
        )
        .await?;
        let new_member_role: OrganizationMemberRole = req.role.parse().map_err(AppError::BadRequest)?;

        let target_member = OrganizationMembersRepository::find_member(db, org_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in organization".to_string()))?;

        let target_role = target_member
            .role
            .unwrap_or(OrganizationMemberRole::Viewer);

        if !is_system_admin && requester_role == OrganizationMemberRole::Admin && target_role == OrganizationMemberRole::Owner {
            return Err(AppError::Forbidden(
                "Admin cannot modify the Organization Owner role".to_string(),
            ));
        }

        if target_role == OrganizationMemberRole::Owner && new_member_role != OrganizationMemberRole::Owner {
            let owner_count = OrganizationMembersRepository::count_owners(db, org_id).await?;
            if owner_count <= 1 {
                return Err(AppError::Conflict(
                    "Cannot demote the sole Owner of an organization".to_string(),
                ));
            }
        }

        let updated = OrganizationMembersRepository::update_member_role(
            db,
            org_id,
            target_user_id,
            new_member_role,
        )
        .await?;

        let target_user =
            OrganizationMembersRepository::find_user_by_id(db, target_user_id).await?;
        let email = target_user.map(|u| u.email);

        Ok(MemberResponse::from_model(updated, email))
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        target_user_id: Uuid,
        requester_id: Uuid,
        is_system_admin: bool,
    ) -> Result<(), AppError> {
        let requester_role = OrgPermissionsService::verify_org_role(
            db,
            org_id,
            requester_id,
            OrganizationMemberRole::Admin,
            is_system_admin,
        )
        .await?;

        let target_member = OrganizationMembersRepository::find_member(db, org_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in organization".to_string()))?;

        let target_role = target_member
            .role
            .unwrap_or(OrganizationMemberRole::Viewer);

        if !is_system_admin && requester_role == OrganizationMemberRole::Admin && target_role == OrganizationMemberRole::Owner {
            return Err(AppError::Forbidden(
                "Admin cannot remove the Organization Owner".to_string(),
            ));
        }

        if target_role == OrganizationMemberRole::Owner {
            let owner_count = OrganizationMembersRepository::count_owners(db, org_id).await?;
            if owner_count <= 1 {
                return Err(AppError::Conflict(
                    "Cannot remove the sole Owner of an organization".to_string(),
                ));
            }
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
        let user_id = Uuid::new_v4();
        let result = OrganizationMembersService::list_members(&db, org_id, user_id, false).await;
        assert!(result.is_err());
    }
}
