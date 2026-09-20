use crate::infrastructure::github::{GithubRepo, RepoCheckDto, RepoCloneDto};
use crate::shared::error::AppError;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RepoCloning {
    pat_token: Option<String>,
    repo_url: String,
    branch: String,
    destination: PathBuf,
}

impl RepoCloning {
    pub fn new(
        pat_token: Option<String>,
        repo_url: String,
        branch: String,
        destination: PathBuf,
    ) -> Self {
        Self {
            pat_token: pat_token.map(|s| s.to_string()),
            branch,
            repo_url,
            destination,
        }
    }

    async fn check_repo_existence(&self) -> Result<bool, AppError> {
        let repo = GithubRepo::check(&RepoCheckDto {
            url: self.repo_url.clone(),
            token: self.pat_token.clone(),
        })
        .await
        .map_err(|e| {
            AppError::InternalServerError(format!("repo didn't exist: {}", e.to_string()))
        })?;

        Ok(repo)
    }

    async fn download_repo_files(&self) -> Result<(), AppError> {
        let dto = RepoCloneDto {
            url: self.repo_url.clone(),
            branch: self.branch.clone(),
            destination: self.destination.clone(),
            token: self.pat_token.clone(),
        };

        GithubRepo::clone(dto).map_err(|e| {
            AppError::InternalServerError(format!("Failed to clone repo: {}", e.to_string()))
        })?;
        Ok(())
    }

    pub async fn clone(&self) -> Result<(), AppError> {
        self.check_repo_existence().await?;
        self.download_repo_files().await?;

        Ok(())
    }
}
