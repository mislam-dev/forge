use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PermissionCreateDto {
    #[validate(length(min = 1, message = "Permission key is required."))]
    pub key: String,

    #[validate(length(min = 1, message = "Permission value is required."))]
    pub value: String,

    pub descriptions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PermissionUpdateDto {
    #[validate(length(min = 1, message = "Permission key is required."))]
    pub key: Option<String>,

    #[validate(length(min = 1, message = "Permission value is required."))]
    pub value: Option<String>,

    pub descriptions: Option<String>,
}
