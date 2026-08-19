use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::project_environment_variable::Model as EnvVarModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVarResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub environment: String,
    pub key: String,
    pub value: String,
    pub is_secret: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl EnvVarResponse {
    pub fn from_model(model: EnvVarModel) -> Self {
        Self {
            id: model.id,
            project_id: model.project_id,
            environment: model.environment,
            key: model.key,
            value: "••••••••".to_string(), // Always masked in public API responses
            is_secret: model.is_secret.unwrap_or(true),
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
    fn test_env_var_response_masks_value() {
        let id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = EnvVarModel {
            id,
            project_id,
            environment: "Production".to_string(),
            key: "API_SECRET_KEY".to_string(),
            value_encrypted: "super_secret_unmasked_value".to_string(),
            is_secret: Some(true),
            created_at: now,
            updated_at: now,
        };

        let res = EnvVarResponse::from_model(model);
        assert_eq!(res.value, "••••••••");
        assert_ne!(res.value, "super_secret_unmasked_value");
    }
}
