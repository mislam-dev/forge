use sea_orm::*;
use uuid::Uuid;

use super::super::super::projects::repository::ProjectsRepository;
use super::dto::{AssignProjectMemberDTO, ProjectMemberResponse};
use super::repository::ProjectAssignmentsRepository;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;

pub struct ProjectAssignmentsService;

impl ProjectAssignmentsService {
    pub async fn assign_member(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        req: AssignProjectMemberDTO,
    ) -> Result<ProjectMemberResponse, AppError> {
        let project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let _user = UserRepository::find_by_id(db, req.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Target user not found".to_string()))?;

        if let Some(org_id) = org_id {
            let org_role = OrgPermissionsService::resolve_org_role(db, org_id, req.user_id).await?;
            if org_role.is_none() {
                return Err(AppError::BadRequest(
                    "Target user is not a member of the parent organization".to_string(),
                ));
            }
        }

        if req.user_id == project.owner_id {
            return Err(AppError::Conflict(
                "User is already the owner of this project".to_string(),
            ));
        }

        // to do work on this project
        if (ProjectAssignmentsRepository::find_member(db, project_id, req.user_id).await?).is_some()
        {
            return Err(AppError::Conflict(
                "User is already assigned to this project".to_string(),
            ));
        }

        let member = ProjectAssignmentsRepository::add_member(db, project_id, req).await?;

        Ok(ProjectMemberResponse::from_model(member))
    }

    pub async fn list_members(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
    ) -> Result<Vec<ProjectMemberResponse>, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let members =
            ProjectAssignmentsRepository::find_members_by_project_id(db, project_id).await?;
        Ok(members
            .into_iter()
            .map(|(m, _u)| ProjectMemberResponse::from_model(m))
            .collect())
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<(), AppError> {
        let project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if project.owner_id == target_user_id {
            return Err(AppError::BadRequest(
                "Cannot remove the Project Owner from the project".to_string(),
            ));
        }

        let member =
            ProjectAssignmentsRepository::find_member(db, project_id, target_user_id).await?;
        if member.is_none() {
            return Err(AppError::NotFound(
                "User is not assigned to this project".to_string(),
            ));
        }

        ProjectAssignmentsRepository::remove_member(db, project_id, target_user_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::entities::sea_orm_active_enums::ProjectMembersRole;
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_assign_member_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectAssignmentsService::assign_member(
            &db,
            None,
            Uuid::new_v4(),
            AssignProjectMemberDTO {
                user_id: Uuid::new_v4(),
                role: ProjectMembersRole::Developer,
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_members_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectAssignmentsService::list_members(&db, None, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_member_project_not_found() {
        let db = setup_mock_db();
        let result =
            ProjectAssignmentsService::remove_member(&db, None, Uuid::new_v4(), Uuid::new_v4())
                .await;
        assert!(result.is_err());
    }
}
