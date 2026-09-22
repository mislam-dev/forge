use crate::shared::error::AppError;
use std::path::Path;

pub struct Validation;

impl Validation {
    pub fn validate(project_source: &str) -> Result<(), AppError> {
        let path = Path::new(project_source);
        if !path.exists() {
            return Err(AppError::NotFound("Project source not found".to_string()));
        }

        let has_reqs = path.join("requirements.txt").exists();
        let has_pyproject = path.join("pyproject.toml").exists();
        let has_main = path.join("main.py").exists() || path.join("app.py").exists();

        if !has_reqs && !has_pyproject && !has_main {
            return Err(AppError::NotFound(
                "Neither requirements.txt, pyproject.toml, main.py, nor app.py found in Python project root".to_string(),
            ));
        }

        Ok(())
    }
}
