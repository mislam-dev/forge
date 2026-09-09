use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubError {
    #[error("Invalid Repo: {0}")]
    InvalidRepo(String),

    #[error("Authentication Failed: {0}")]
    AuthFailed(String),

    #[error("Cloning Failed: {0}")]
    CloningFailed(String),

    #[error("Not Found or No Access")]
    NotFoundOrNoAccess,

    #[error("No valid commit found in branch")]
    NoValidCommit,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Destination directory already exists: {0}")]
    DestinationAlreadyExists(String),

    #[error("Access Denied: {0}")]
    AccessDenied(String),
}
