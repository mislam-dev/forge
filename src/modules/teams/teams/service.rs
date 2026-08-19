use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{CreateTeamRequest, TeamQuery, TeamResponse, UpdateTeamRequest};
use super::entities::team::ActiveModel as TeamActiveModel;
use super::repository::TeamsRepository;
use super::super::members::entities::team_member::ActiveModel as TeamMemberActiveModel;
use super::super::members::repository::TeamMembersRepository;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::shared::error::AppError;

pub struct TeamsService;

impl TeamsService {
    pub async fn create_team(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        req: CreateTeamRequest,
    ) -> Result<TeamResponse, AppError> {
        let org_id = req.organization_id;

        // Check if requester is a member of the parent organization
        if !is_system_admin {
            let role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
            if role.is_none() {
                return Err(AppError::Forbidden(
                    "You are not a member of this organization".to_string(),
                ));
            }
        }

        // Check name uniqueness within the organization
        if (TeamsRepository::find_by_org_and_name(db, org_id, &req.name).await?).is_some() {
            return Err(AppError::Conflict(format!(
                "Team with name '{}' already exists in this organization",
                req.name
            )));
        }

        let now = Utc::now().into();
        let team_id = Uuid::new_v4();

        let active_model = TeamActiveModel {
            id: Set(team_id),
            organization_id: Set(Some(org_id)),
            name: Set(req.name),
            descriptions: Set(req.descriptions),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let team = TeamsRepository::create_team(db, active_model).await?;

        // Automatically add creator as Team Admin
        let member_active_model = TeamMemberActiveModel {
            team_id: Set(team_id),
            user_id: Set(requester_id),
            role: Set("admin".to_string()),
            joined_at: Set(now),
        };
        let _ = TeamMembersRepository::add_member(db, member_active_model).await?;

        Ok(TeamResponse::from_model(team, 1))
    }

    pub async fn list_teams(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        query: TeamQuery,
    ) -> Result<Vec<TeamResponse>, AppError> {
        let teams = if let Some(org_id) = query.organization_id {
            if !is_system_admin {
                let role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                if role.is_none() {
                    return Err(AppError::Forbidden(
                        "You are not a member of this organization".to_string(),
                    ));
                }
            }
            TeamsRepository::find_by_org_id(db, org_id).await?
        } else {
            TeamsRepository::find_teams_by_user_id(db, requester_id).await?
        };

        let mut responses = Vec::with_capacity(teams.len());
        for team in teams {
            let count = TeamsRepository::count_members(db, team.id).await?;
            responses.push(TeamResponse::from_model(team, count));
        }

        Ok(responses)
    }

    pub async fn get_team_by_id(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
    ) -> Result<TeamResponse, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if !is_system_admin {
            if let Some(org_id) = team.organization_id {
                let role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                if role.is_none() {
                    return Err(AppError::Forbidden(
                        "You are not authorized to view this team".to_string(),
                    ));
                }
            }
        }

        let count = TeamsRepository::count_members(db, team.id).await?;
        Ok(TeamResponse::from_model(team, count))
    }

    pub async fn update_team(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
        req: UpdateTeamRequest,
    ) -> Result<TeamResponse, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if !is_system_admin {
            let is_allowed = if let Some(org_id) = team.organization_id {
                let org_role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                let is_org_admin = matches!(org_role, Some(OrgRole::Admin) | Some(OrgRole::Owner));

                let team_member = TeamMembersRepository::find_member(db, team_id, requester_id).await?;
                let is_team_admin = matches!(team_member.as_ref().map(|m| m.role.as_str()), Some("admin"));

                is_org_admin || is_team_admin
            } else {
                false
            };

            if !is_allowed {
                return Err(AppError::Forbidden(
                    "Only Organization Admins or Team Admins can update this team".to_string(),
                ));
            }
        }

        let mut active_model: TeamActiveModel = team.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(new_name) = req.name {
            if let Some(org_id) = active_model.organization_id.clone().unwrap() {
                if let Some(existing) = TeamsRepository::find_by_org_and_name(db, org_id, &new_name).await? {
                    if existing.id != team_id {
                        return Err(AppError::Conflict(format!(
                            "Team with name '{}' already exists in this organization",
                            new_name
                        )));
                    }
                }
            }
            active_model.name = Set(new_name);
        }

        if let Some(desc) = req.descriptions {
            active_model.descriptions = Set(Some(desc));
        }

        let updated = TeamsRepository::update_team(db, active_model).await?;
        let count = TeamsRepository::count_members(db, team_id).await?;
        Ok(TeamResponse::from_model(updated, count))
    }

    pub async fn delete_team(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        team_id: Uuid,
    ) -> Result<(), AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if !is_system_admin {
            let is_allowed = if let Some(org_id) = team.organization_id {
                let org_role = OrgPermissionsService::resolve_org_role(db, org_id, requester_id).await?;
                matches!(org_role, Some(OrgRole::Admin) | Some(OrgRole::Owner))
            } else {
                false
            };

            if !is_allowed {
                return Err(AppError::Forbidden(
                    "Only Organization Admins/Owners can delete teams".to_string(),
                ));
            }
        }

        TeamsRepository::delete_team(db, team_id).await?;
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
    async fn test_get_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let result = TeamsService::get_team_by_id(&db, requester_id, false, team_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let result = TeamsService::delete_team(&db, requester_id, false, team_id).await;
        assert!(result.is_err());
    }
}
