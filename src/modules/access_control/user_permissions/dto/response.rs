use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermissionResponse {
    pub permission_id: Uuid,
    pub user_id: Uuid,
}
