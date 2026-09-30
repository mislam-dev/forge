use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConnectProjectRepositoryDTO {
    #[validate(length(min = 5, message = "Repository URL must be valid"))]
    pub repository_url: String,

    pub access_token: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProjectRepositoryDTO {
    #[validate(length(min = 5, message = "Repository URL must be valid"))]
    pub repository_url: Option<String>,
    pub access_token: Option<String>,
    pub default_branch: Option<String>,
}

#[cfg(test)]
mod tests {}
