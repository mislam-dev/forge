use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{AddTeamMemberRequest, TeamMemberResponse, UpdateTeamMemberRoleRequest};
use super::entities::team_member::ActiveModel as TeamMemberActiveModel;
use super::repository::TeamMembersRepository;
use super::role::TeamRole;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::modules::teams::teams::repository::TeamsRepository;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;

pub struct TeamMembersService;

impl TeamMembersService {
    pub async fn add_member(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
        req: AddTeamMemberRequest,
    ) -> Result<TeamMemberResponse, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        let org_id = team.organization_id.ok_or_else(|| {
            AppError::BadRequest("Team does not belong to a valid organization".to_string())
        })?;

        // Check requester permissions (Org Admin/Owner, Team Admin, or System Admin)
        if !is_system_admin {
            let is_allowed = {
                let org_role =
                    OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                let is_org_admin = matches!(org_role, Some(OrgRole::Admin) | Some(OrgRole::Owner));

                let team_member =
                    TeamMembersRepository::find_member(db, team_id, requester_id).await?;
                let is_team_admin =
                    matches!(team_member.as_ref().map(|m| m.role.as_str()), Some("admin"));

                is_org_admin || is_team_admin
            };

            if !is_allowed {
                return Err(AppError::Forbidden(
                    "Only Organization Admins/Owners or Team Admins can add members to a team"
                        .to_string(),
                ));
            }
        }

        // Verify target user is a member of the parent organization!
        let target_org_role =
            OrgPermissionsService::resolve_org_role(db, org_id, req.user_id).await?;
        if target_org_role.is_none() {
            return Err(AppError::BadRequest(
                "Target user is not a member of the parent organization".to_string(),
            ));
        }

        // Check if user is already a member of the team
        if (TeamMembersRepository::find_member(db, team_id, req.user_id).await?).is_some() {
            return Err(AppError::Conflict(
                "User is already a member of this team".to_string(),
            ));
        }

        // Validate role string
        let role: TeamRole = req.role.parse().map_err(AppError::BadRequest)?;

        let now = Utc::now().into();
        let active_model = TeamMemberActiveModel {
            team_id: Set(team_id),
            user_id: Set(req.user_id),
            role: Set(role.as_str().to_string()),
            joined_at: Set(now),
        };

        let member = TeamMembersRepository::add_member(db, active_model).await?;
        let user = UserRepository::find_by_id(db, req.user_id).await?;

        Ok(TeamMemberResponse::from_model(member, user))
    }

    pub async fn list_members(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
    ) -> Result<Vec<TeamMemberResponse>, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if !is_system_admin {
            if let Some(org_id) = team.organization_id {
                let org_role =
                    OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                let team_member =
                    TeamMembersRepository::find_member(db, team_id, requester_id).await?;

                if org_role.is_none() && team_member.is_none() {
                    return Err(AppError::Forbidden(
                        "You are not authorized to list members of this team".to_string(),
                    ));
                }
            }
        }

        let members_with_users =
            TeamMembersRepository::find_members_by_team_id(db, team_id).await?;
        let mut responses = Vec::with_capacity(members_with_users.len());

        for (member, user) in members_with_users {
            responses.push(TeamMemberResponse::from_model(member, user));
        }

        Ok(responses)
    }

    pub async fn update_member_role(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
        target_user_id: Uuid,
        req: UpdateTeamMemberRoleRequest,
    ) -> Result<TeamMemberResponse, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if !is_system_admin {
            let is_allowed = if let Some(org_id) = team.organization_id {
                let org_role =
                    OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                let is_org_admin = matches!(org_role, Some(OrgRole::Admin) | Some(OrgRole::Owner));

                let team_member =
                    TeamMembersRepository::find_member(db, team_id, requester_id).await?;
                let is_team_admin =
                    matches!(team_member.as_ref().map(|m| m.role.as_str()), Some("admin"));

                is_org_admin || is_team_admin
            } else {
                false
            };

            if !is_allowed {
                return Err(AppError::Forbidden(
                    "Only Organization Admins/Owners or Team Admins can update team member roles"
                        .to_string(),
                ));
            }
        }

        let member = TeamMembersRepository::find_member(db, team_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in team".to_string()))?;

        let role: TeamRole = req.role.parse().map_err(AppError::BadRequest)?;

        let mut active_model: TeamMemberActiveModel = member.into();
        active_model.role = Set(role.as_str().to_string());

        let updated_member = TeamMembersRepository::update_member(db, active_model).await?;
        let user = UserRepository::find_by_id(db, target_user_id).await?;

        Ok(TeamMemberResponse::from_model(updated_member, user))
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<(), AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if !is_system_admin {
            let is_allowed = if let Some(org_id) = team.organization_id {
                let org_role =
                    OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                let is_org_admin = matches!(org_role, Some(OrgRole::Admin) | Some(OrgRole::Owner));

                let team_member =
                    TeamMembersRepository::find_member(db, team_id, requester_id).await?;
                let is_team_admin =
                    matches!(team_member.as_ref().map(|m| m.role.as_str()), Some("admin"));

                is_org_admin || is_team_admin
            } else {
                false
            };

            if !is_allowed {
                return Err(AppError::Forbidden(
                    "Only Organization Admins/Owners or Team Admins can remove team members"
                        .to_string(),
                ));
            }
        }

        let member = TeamMembersRepository::find_member(db, team_id, target_user_id).await?;
        if member.is_none() {
            return Err(AppError::NotFound("Member not found in team".to_string()));
        }

        TeamMembersRepository::remove_member(db, team_id, target_user_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_list_members_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let result = TeamMembersService::list_members(&db, requester_id, false, team_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_member_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let result =
            TeamMembersService::remove_member(&db, requester_id, false, team_id, target_user_id)
                .await;
        assert!(result.is_err());
    }
}
