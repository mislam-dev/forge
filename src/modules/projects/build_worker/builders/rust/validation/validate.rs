use crate::shared::error::AppError;
use std::path::Path;

pub struct Validation;

impl Validation {
    pub fn validate(project_source: &str) -> Result<(), AppError> {
        let path = Path::new(project_source);
        if !path.exists() {
            return Err(AppError::NotFound("Project source not found".to_string()));
        }

        let cargo_toml = path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(AppError::NotFound(
                "Cargo.toml not found in project root".to_string(),
            ));
        }

        Ok(())
    }
}
