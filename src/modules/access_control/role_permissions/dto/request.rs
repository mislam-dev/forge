use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignRolePermissionsDto {
    pub role_id: Uuid,

    #[validate(length(min = 1, message = "you must provide at least 1 value"))]
    pub permission_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RemoveRolePermissionsDto {
    pub role_id: Uuid,

    #[validate(length(min = 1, message = "you must provide at least 1 value"))]
    pub permission_ids: Vec<Uuid>,
}
