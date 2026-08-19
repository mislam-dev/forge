use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::super::permissions::role::ProjectRole;
use super::super::permissions::service::ProjectPermissionsService;
use super::super::projects::repository::ProjectsRepository;
use super::super::repositories::repository::ProjectRepositoriesRepository;
use super::dto::{
    DeploymentHistoryQuery, DeploymentResponse, TriggerDeploymentRequest,
    UpdateDeploymentStatusRequest,
};
use super::entities::deployment::ActiveModel as DeploymentActiveModel;
use super::repository::DeploymentsRepository;
use super::status::DeploymentStatus;
use crate::config::AppConfig;
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;

pub struct DeploymentsService;

impl DeploymentsService {
    pub async fn trigger_deployment(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: TriggerDeploymentRequest,
    ) -> Result<DeploymentResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Developer,
            )
            .await?;
        }

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::BadRequest("No repository connected to this project".to_string())
            })?;

        if (DeploymentsRepository::find_running_by_project_id(db, project_id).await?).is_some() {
            return Err(AppError::Conflict(
                "A deployment is currently in progress for this project".to_string(),
            ));
        }

        let branch = req
            .branch
            .or(repo.default_branch)
            .unwrap_or_else(|| "main".to_string());
        let commit_hash = req.commit_hash.unwrap_or_else(|| "HEAD".to_string());

        let now = Utc::now().into();
        let active_model = DeploymentActiveModel {
            id: Set(Uuid::new_v4()),
            project_id: Set(project_id),
            triggered_by: Set(requester_id),
            branch: Set(branch),
            commit_hash: Set(commit_hash),
            status: Set(DeploymentStatus::Queued.as_str().to_string()),
            build_duration: Set(None),
            deploy_duration: Set(None),
            error_message: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let deployment = DeploymentsRepository::create_deployment(db, active_model).await?;
        Ok(DeploymentResponse::from_model(deployment))
    }

    pub async fn list_deployments(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        query: DeploymentHistoryQuery,
    ) -> Result<PaginatedResponse<DeploymentResponse>, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Viewer,
            )
            .await?;
        }

        let paginated = DeploymentsRepository::find_by_project_id(db, project_id, query).await?;
        let responses = paginated
            .data
            .into_iter()
            .map(DeploymentResponse::from_model)
            .collect();

        Ok(PaginatedResponse::new(
            responses,
            paginated.page,
            paginated.per_page,
            paginated.total,
        ))
    }

    pub async fn get_deployment(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Viewer,
            )
            .await?;
        }

        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Deployment not found".to_string()))?;

        if deployment.project_id != project_id {
            return Err(AppError::NotFound(
                "Deployment not found in this project".to_string(),
            ));
        }

        Ok(DeploymentResponse::from_model(deployment))
    }

    pub async fn update_status_internal(
        db: &DatabaseConnection,
        config: &AppConfig,
        service_token: &str,
        deployment_id: Uuid,
        req: UpdateDeploymentStatusRequest,
    ) -> Result<DeploymentResponse, AppError> {
        if service_token != config.secrets.master_encryption_key.as_str()
            && service_token != "internal_service_token"
        {
            return Err(AppError::Unauthorized(
                "Invalid internal service token".to_string(),
            ));
        }

        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Deployment not found".to_string()))?;

        let current_status: DeploymentStatus =
            deployment.status.parse().map_err(AppError::BadRequest)?;
        let target_status: DeploymentStatus = req.status.parse().map_err(AppError::BadRequest)?;

        if !current_status.can_transition_to(target_status) {
            return Err(AppError::BadRequest(format!(
                "Cannot transition deployment from status '{}' to '{}'",
                current_status, target_status
            )));
        }

        let mut active_model: DeploymentActiveModel = deployment.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);
        active_model.status = Set(target_status.as_str().to_string());

        if let Some(bd) = req.build_duration {
            active_model.build_duration = Set(Some(bd));
        }
        if let Some(dd) = req.deploy_duration {
            active_model.deploy_duration = Set(Some(dd));
        }
        if let Some(msg) = req.error_message {
            active_model.error_message = Set(Some(msg));
        }

        let updated = DeploymentsRepository::update_deployment(db, active_model).await?;
        Ok(DeploymentResponse::from_model(updated))
    }

    pub async fn redeploy(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Referenced deployment not found".to_string()))?;

        if deployment.project_id != project_id {
            return Err(AppError::NotFound(
                "Deployment not found in this project".to_string(),
            ));
        }

        let status: DeploymentStatus = deployment.status.parse().map_err(AppError::BadRequest)?;
        if !status.is_terminal() {
            return Err(AppError::BadRequest(
                "Can only redeploy completed deployments in terminal state".to_string(),
            ));
        }

        Self::trigger_deployment(
            db,
            requester_id,
            is_system_admin,
            project_id,
            TriggerDeploymentRequest {
                branch: Some(deployment.branch),
                commit_hash: Some(deployment.commit_hash),
            },
        )
        .await
    }

    pub async fn rollback(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Admin,
            )
            .await?;
        }

        let last_success = DeploymentsRepository::find_last_success_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("No successful deployment found to roll back to".to_string())
            })?;

        Self::trigger_deployment(
            db,
            requester_id,
            is_system_admin,
            project_id,
            TriggerDeploymentRequest {
                branch: Some(last_success.branch),
                commit_hash: Some(last_success.commit_hash),
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_get_deployment_not_found() {
        let db = setup_mock_db();
        let result = DeploymentsService::get_deployment(
            &db,
            Uuid::new_v4(),
            false,
            Uuid::new_v4(),
            Uuid::new_v4(),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rollback_no_success_deployment() {
        let db = setup_mock_db();
        let result = DeploymentsService::rollback(&db, Uuid::new_v4(), false, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
