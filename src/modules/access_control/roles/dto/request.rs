use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RoleCreateDto {
    #[validate(length(min = 1, message = "Role key is required."))]
    pub key: String,

    #[validate(length(min = 1, message = "Role value is required."))]
    pub value: String,

    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    #[validate(length(min = 1, message = "Role key is required."))]
    pub key: Option<String>,

    #[validate(length(min = 1, message = "Role value is required."))]
    pub value: Option<String>,

    pub description: Option<String>,
}
