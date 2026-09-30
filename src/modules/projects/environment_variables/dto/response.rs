use super::super::entities::sea_orm_active_enums::ProjectEnvironmentVariablesEnvironment as EnvironmentEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::project_environment_variables::Model as EnvVarModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectEnvVarResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub environment: EnvironmentEnum,
    pub key: String,
    pub value: String,
    pub is_secret: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl ProjectEnvVarResponse {
    pub fn from_model(model: EnvVarModel) -> Self {
        Self {
            id: model.id,
            project_id: model.project_id,
            environment: model.environment,
            key: model.key,
            value: model.value_encrypted,
            is_secret: model.is_secret.unwrap_or(false),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_env_var_response_masks_secret_value() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = EnvVarModel {
            id,
            project_id,
            environment: EnvironmentEnum::Development,
            key: "API_SECRET_KEY".to_string(),
            value_encrypted: "super_secret_unmasked_value".to_string(),
            is_secret: Some(true),
            created_at: now,
            updated_at: now,
        };

        let res = ProjectEnvVarResponse::from_model(model);
        assert_eq!(res.value, "••••••••");
        assert_ne!(res.value, "super_secret_unmasked_value");
        assert!(res.is_secret);
    }

    #[test]
    fn test_env_var_response_reveals_non_secret_value() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = EnvVarModel {
            id,
            project_id,
            environment: EnvironmentEnum::Development,
            key: "PORT".to_string(),
            value_encrypted: "8080".to_string(),
            is_secret: Some(false),
            created_at: now,
            updated_at: now,
        };

        let res = ProjectEnvVarResponse::from_model(model);
        assert_eq!(res.value, "8080");
        assert!(!res.is_secret);
    }
}
