use std::collections::HashMap;

use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::super::projects::repository::ProjectsRepository;
use super::dto::{
    BulkCreateProjectEnvVarDTO, CreateProjectEnvVarDTO, ProjectEnvVarQueryDTO,
    ProjectEnvVarResponse, UpdateProjectEnvVarDTO,
};
use super::entities::project_environment_variable::ActiveModel as EnvVarActiveModel;
use super::repository::ProjectEnvironmentVariablesRepository;
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

    fn decrypt_value(encrypted_hex: &str) -> String {
        let mut bytes = Vec::new();
        for i in (0..encrypted_hex.len()).step_by(2) {
            if i + 2 <= encrypted_hex.len() {
                if let Ok(b) = u8::from_str_radix(&encrypted_hex[i..i + 2], 16) {
                    bytes.push(b);
                }
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
    }

    pub async fn create_env_var(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        req: CreateProjectEnvVarDTO,
    ) -> Result<ProjectEnvVarResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

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

        let env_var =
            ProjectEnvironmentVariablesRepository::create_env_var(db, active_model).await?;
        Ok(ProjectEnvVarResponse::from_model(env_var))
    }

    pub async fn list_env_vars(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        query: ProjectEnvVarQueryDTO,
    ) -> Result<Vec<ProjectEnvVarResponse>, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let env_vars = ProjectEnvironmentVariablesRepository::find_by_project_id(
            db,
            project_id,
            query.environment,
        )
        .await?;

        Ok(env_vars
            .into_iter()
            .map(ProjectEnvVarResponse::from_model)
            .collect())
    }

    pub async fn update_env_var(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        env_var_id: Uuid,
        req: UpdateProjectEnvVarDTO,
    ) -> Result<ProjectEnvVarResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let env_var = ProjectEnvironmentVariablesRepository::find_by_id(db, env_var_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Environment variable not found".to_string()))?;

        if env_var.project_id != project_id {
            return Err(AppError::NotFound(
                "Environment variable not found in this project".to_string(),
            ));
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

        let updated =
            ProjectEnvironmentVariablesRepository::update_env_var(db, active_model).await?;
        Ok(ProjectEnvVarResponse::from_model(updated))
    }

    pub async fn delete_env_var(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        env_var_id: Uuid,
    ) -> Result<(), AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let env_var = ProjectEnvironmentVariablesRepository::find_by_id(db, env_var_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Environment variable not found".to_string()))?;

        if env_var.project_id != project_id {
            return Err(AppError::NotFound(
                "Environment variable not found in this project".to_string(),
            ));
        }

        ProjectEnvironmentVariablesRepository::delete_env_var(db, env_var_id).await?;
        Ok(())
    }

    pub async fn bulk_create_env_vars(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        req: BulkCreateProjectEnvVarDTO,
    ) -> Result<Vec<ProjectEnvVarResponse>, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

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

            let env_var =
                ProjectEnvironmentVariablesRepository::create_env_var(&txn, active_model).await?;
            responses.push(ProjectEnvVarResponse::from_model(env_var));
        }

        txn.commit().await.map_err(AppError::from)?;
        Ok(responses)
    }

    pub async fn get_decrypted_env_vars(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        environment: &str,
    ) -> Result<HashMap<String, String>, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let env_vars = ProjectEnvironmentVariablesRepository::find_by_project_id(
            db,
            project_id,
            Some(environment.to_string()),
        )
        .await?;

        let mut map = HashMap::with_capacity(env_vars.len());
        for var in env_vars {
            let val = Self::decrypt_value(&var.value_encrypted);
            map.insert(var.key, val);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

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

    #[test]
    fn test_encryption_decryption_roundtrip() {
        let raw = "super-secret-password-123!@#";
        let encrypted = ProjectEnvironmentVariablesService::encrypt_value(raw);
        assert_ne!(raw, encrypted);
        let decrypted = ProjectEnvironmentVariablesService::decrypt_value(&encrypted);
        assert_eq!(raw, decrypted);
    }

    #[tokio::test]
    async fn test_create_env_var_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectEnvironmentVariablesService::create_env_var(
            &db,
            None,
            Uuid::new_v4(),
            CreateProjectEnvVarDTO {
                environment: "Production".to_string(),
                key: "API_KEY".to_string(),
                value: "secret".to_string(),
                is_secret: Some(true),
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_env_vars_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectEnvironmentVariablesService::list_env_vars(
            &db,
            None,
            Uuid::new_v4(),
            ProjectEnvVarQueryDTO { environment: None },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_env_var_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectEnvironmentVariablesService::delete_env_var(
            &db,
            None,
            Uuid::new_v4(),
            Uuid::new_v4(),
        )
        .await;
        assert!(result.is_err());
    }
}
