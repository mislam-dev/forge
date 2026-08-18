use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignUserPermissionsDto {
    pub user_id: Uuid,
    pub permission_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RemoveUserPermissionsDto {
    pub user_id: Uuid,
    pub permission_ids: Vec<Uuid>,
}
