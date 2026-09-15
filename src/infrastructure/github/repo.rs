use std::path::Path;

use git2::{Cred, ErrorCode, FetchOptions, RemoteCallbacks, build::RepoBuilder};
use reqwest::Client;
use url::Url;

use super::error::GithubError;

#[derive(Debug)]
pub struct RepoCloneDto {
    pub url: String,
    pub token: Option<String>,
    pub destination: String,
    pub branch: String,
}

pub struct RepoCheckDto {
    pub url: String,
    pub token: Option<String>,
}

pub struct GithubRepo;

impl GithubRepo {
    pub async fn check(dto: &RepoCheckDto) -> Result<bool, GithubError> {
        let (owner, name) = Self::get_name_owner_from_url(&dto.url)
            .map_err(|e| GithubError::InvalidRepo(e.to_string()))?;

        let url = format!("https://api.github.com/repos/{}/{}", owner, name);

        let client = Client::builder()
            .user_agent("forge-app")
            .build()
            .map_err(|e| GithubError::InternalError(e.to_string()))?;

        let mut req = client.get(url);

        if let Some(token) = &dto.token {
            if token.clone() != "".to_string() {
                req = req.bearer_auth(token);
            }
        }

        let response = req
            .send()
            .await
            .map_err(|e| GithubError::InternalError(e.to_string()))?;

        return match response.status().as_u16() {
            200 => Ok(true),
            404 => Ok(false),
            403 | 401 => Err(GithubError::AuthFailed("Authentication Failed".to_string())),
            _ => Err(GithubError::InternalError("Internal Error".to_string())),
        };
    }

    pub fn clone(dto: RepoCloneDto) -> Result<(), GithubError> {
        let destination_path = Path::new(&dto.destination);

        if destination_path.exists() {
            return Err(GithubError::DestinationAlreadyExists(
                "Destination already exist".to_string(),
            ));
        }

        let mut callbacks = RemoteCallbacks::new();

        if let Some(token) = dto.token {
            callbacks.credentials(move |_, _, _| Cred::userpass_plaintext("oauth2", &token));
        }

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_opts);
        builder.branch(&dto.branch);

        builder
            .clone(&dto.url, destination_path)
            .map_err(Self::map_error)?;

        Ok(())
    }

    fn map_error(err: git2::Error) -> GithubError {
        tracing::error!("Git Error: {err}");
        match err.code() {
            ErrorCode::NotFound => GithubError::NotFoundOrNoAccess,
            ErrorCode::Auth => GithubError::AuthFailed("Authentication Failed".to_string()),
            ErrorCode::Exists => GithubError::DestinationAlreadyExists(
                "Destination directory already exists".to_string(),
            ),
            ErrorCode::BareRepo => GithubError::InvalidRepo(err.to_string()),
            ErrorCode::InvalidSpec => GithubError::InvalidRepo(err.to_string()),
            ErrorCode::User => GithubError::AccessDenied("Invalid Token or No Access".to_string()),
            _ => GithubError::InternalError(err.to_string()),
        }
    }

    fn get_name_owner_from_url(url: &str) -> Result<(String, String), GithubError> {
        let url = Url::parse(url).map_err(|e| GithubError::InvalidRepo(e.to_string()))?;
        let path = url.path().trim_start_matches('/');

        let mut parts = path.split('/');

        let owner = parts
            .next()
            .map(|s| s.to_string())
            .ok_or(GithubError::InvalidRepo("Invalid URL".to_string()))?;
        let name = parts
            .next()
            .map(|s| s.trim_end_matches(".git").to_string())
            .ok_or(GithubError::InvalidRepo("Invalid URL".to_string()))?;

        if owner.is_empty() || name.is_empty() {
            return Err(GithubError::InvalidRepo("Invalid URL".to_string()));
        }

        Ok((owner, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name_owner_from_url_valid() {
        let (owner, name) =
            GithubRepo::get_name_owner_from_url("https://github.com/octocat/Hello-World").unwrap();
        assert_eq!(owner, "octocat");
        assert_eq!(name, "Hello-World");
    }

    #[test]
    fn test_get_name_owner_from_url_with_git_suffix() {
        let (owner, name) =
            GithubRepo::get_name_owner_from_url("https://github.com/octocat/Hello-World.git")
                .unwrap();
        assert_eq!(owner, "octocat");
        assert_eq!(name, "Hello-World");
    }

    #[test]
    fn test_get_name_owner_from_url_invalid() {
        assert!(GithubRepo::get_name_owner_from_url("not-a-valid-url").is_err());
        assert!(GithubRepo::get_name_owner_from_url("https://github.com/").is_err());
        assert!(GithubRepo::get_name_owner_from_url("https://github.com/octocat").is_err());
    }

    #[test]
    fn test_map_error_variants() {
        let not_found = git2::Error::from_str("not found");
        let mapped = GithubRepo::map_error(not_found);
        assert!(matches!(mapped, GithubError::InternalError(_)));
    }

    #[test]
    fn test_clone_destination_already_exists() {
        let temp_dir = std::env::temp_dir().join(format!("forge_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();

        let dto = RepoCloneDto {
            url: "https://github.com/octocat/Hello-World".to_string(),
            token: None,
            destination: temp_dir.to_str().unwrap().to_string(),
            branch: "main".to_string(),
        };

        let result = GithubRepo::clone(dto);
        assert!(matches!(
            result,
            Err(GithubError::DestinationAlreadyExists(_))
        ));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
