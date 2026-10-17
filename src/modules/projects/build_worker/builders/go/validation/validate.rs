use crate::shared::error::{AppError, codes};
use std::path::Path;

pub struct Validation;

impl Validation {
    pub fn validate(project_source: &str) -> Result<(), AppError> {
        let path = Path::new(project_source);
        if !path.exists() {
            return Err(AppError::not_found(
                codes::projects::PRJ_FILE_001,
                "Project source not found",
            ));
        }

        let go_mod = path.join("go.mod");
        if !go_mod.exists() {
            return Err(AppError::not_found(
                codes::projects::PRJ_FILE_001,
                "go.mod not found in project root",
            ));
        }

        Ok(())
    }
}
