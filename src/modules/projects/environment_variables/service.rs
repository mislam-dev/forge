use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{BulkCreateEnvVarRequest, CreateEnvVarRequest, EnvVarQuery, EnvVarResponse, UpdateEnvVarRequest};
use super::entities::project_environment_variable::ActiveModel as EnvVarActiveModel;
use super::repository::ProjectEnvironmentVariablesRepository;
use super::super::permissions::role::ProjectRole;
use super::super::permissions::service::ProjectPermissionsService;
use super::super::projects::repository::ProjectsRepository;
use crate::shared::error::AppError;

pub struct ProjectEnvironmentVariablesService;

impl ProjectEnvironmentVariablesService {
    pub fn validate_posix_key(key: &str) -> Result<(), AppError> {
        if key.is_empty() {
            return Err(AppError::BadRequest("Key cannot be empty".to_string()));
        }
        let bytes = key.as_bytes();
        let first = bytes[0];
        if !(first.is_ascii_uppercase() || first == b'_') {
            return Err(AppError::BadRequest(format!(
                "Environment variable key '{}' does not match POSIX standard (must start with uppercase letter or underscore)",
                key
            )));
        }
        for &b in &bytes[1..] {
            if !(b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_') {
                return Err(AppError::BadRequest(format!(
                    "Environment variable key '{}' does not match POSIX standard (uppercase letters, numbers, and underscores)",
                    key
                )));
            }
        }
        Ok(())
    }

    fn encrypt_value(raw_value: &str) -> String {
        raw_value.bytes().map(|b| format!("{:02x}", b)).collect()
    }

    pub async fn create_env_var(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: CreateEnvVarRequest,
    ) -> Result<EnvVarResponse, AppError> {
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

        Self::validate_posix_key(&req.key)?;

        if (ProjectEnvironmentVariablesRepository::find_by_project_env_key(
            db,
            project_id,
            &req.environment,
            &req.key,
        )
        .await?)
            .is_some()
        {
            return Err(AppError::Conflict(format!(
                "Environment variable '{}' already exists in {} environment",
                req.key, req.environment
            )));
        }

        let now = Utc::now().into();
        let active_model = EnvVarActiveModel {
            id: Set(Uuid::new_v4()),
            project_id: Set(project_id),
            environment: Set(req.environment),
            key: Set(req.key),
            value_encrypted: Set(Self::encrypt_value(&req.value)),
            is_secret: Set(Some(req.is_secret.unwrap_or(true))),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let env_var = ProjectEnvironmentVariablesRepository::create_env_var(db, active_model).await?;
        Ok(EnvVarResponse::from_model(env_var))
    }

    pub async fn list_env_vars(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        query: EnvVarQuery,
    ) -> Result<Vec<EnvVarResponse>, AppError> {
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

        let env_vars = ProjectEnvironmentVariablesRepository::find_by_project_id(
            db,
            project_id,
            query.environment,
        )
        .await?;

        Ok(env_vars.into_iter().map(EnvVarResponse::from_model).collect())
    }

    pub async fn update_env_var(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        env_var_id: Uuid,
        req: UpdateEnvVarRequest,
    ) -> Result<EnvVarResponse, AppError> {
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

        let env_var = ProjectEnvironmentVariablesRepository::find_by_id(db, env_var_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Environment variable not found".to_string()))?;

        if env_var.project_id != project_id {
            return Err(AppError::NotFound("Environment variable not found in this project".to_string()));
        }

        let mut active_model: EnvVarActiveModel = env_var.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(val) = req.value {
            active_model.value_encrypted = Set(Self::encrypt_value(&val));
        }
        if let Some(secret) = req.is_secret {
            active_model.is_secret = Set(Some(secret));
        }

        let updated = ProjectEnvironmentVariablesRepository::update_env_var(db, active_model).await?;
        Ok(EnvVarResponse::from_model(updated))
    }

    pub async fn delete_env_var(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        env_var_id: Uuid,
    ) -> Result<(), AppError> {
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

        let env_var = ProjectEnvironmentVariablesRepository::find_by_id(db, env_var_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Environment variable not found".to_string()))?;

        if env_var.project_id != project_id {
            return Err(AppError::NotFound("Environment variable not found in this project".to_string()));
        }

        ProjectEnvironmentVariablesRepository::delete_env_var(db, env_var_id).await?;
        Ok(())
    }

    pub async fn bulk_create_env_vars(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: BulkCreateEnvVarRequest,
    ) -> Result<Vec<EnvVarResponse>, AppError> {
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

        for item in &req.vars {
            Self::validate_posix_key(&item.key)?;
        }

        let txn = db.begin().await.map_err(AppError::from)?;
        let mut responses = Vec::with_capacity(req.vars.len());

        for item in req.vars {
            if (ProjectEnvironmentVariablesRepository::find_by_project_env_key(
                &txn,
                project_id,
                &req.environment,
                &item.key,
            )
            .await?)
                .is_some()
            {
                txn.rollback().await.map_err(AppError::from)?;
                return Err(AppError::Conflict(format!(
                    "Environment variable '{}' already exists in {} environment",
                    item.key, req.environment
                )));
            }

            let now = Utc::now().into();
            let active_model = EnvVarActiveModel {
                id: Set(Uuid::new_v4()),
                project_id: Set(project_id),
                environment: Set(req.environment.clone()),
                key: Set(item.key),
                value_encrypted: Set(Self::encrypt_value(&item.value)),
                is_secret: Set(Some(item.is_secret.unwrap_or(true))),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let env_var = ProjectEnvironmentVariablesRepository::create_env_var(&txn, active_model).await?;
            responses.push(EnvVarResponse::from_model(env_var));
        }

        txn.commit().await.map_err(AppError::from)?;
        Ok(responses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_posix_key_valid() {
        assert!(ProjectEnvironmentVariablesService::validate_posix_key("DATABASE_URL").is_ok());
        assert!(ProjectEnvironmentVariablesService::validate_posix_key("PORT").is_ok());
        assert!(ProjectEnvironmentVariablesService::validate_posix_key("_INTERNAL_VAR1").is_ok());
    }

    #[test]
    fn test_validate_posix_key_invalid() {
        assert!(ProjectEnvironmentVariablesService::validate_posix_key("database-url").is_err());
        assert!(ProjectEnvironmentVariablesService::validate_posix_key("1INVALID").is_err());
        assert!(ProjectEnvironmentVariablesService::validate_posix_key("key with spaces").is_err());
    }
}
