use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::notification::Model as NotificationModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_name: String,
    pub title: String,
    pub message: String,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}

impl NotificationResponse {
    pub fn from_model(model: NotificationModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            type_name: model.r#type,
            title: model.title,
            message: model.message,
            reference_id: model.reference_id,
            reference_type: model.reference_type,
            is_read: model.is_read,
            created_at: model.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnreadCountResponse {
    pub count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_notification_response_from_model() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = NotificationModel {
            id,
            user_id,
            r#type: "deployment_success".to_string(),
            title: "Success".to_string(),
            message: "Build finished".to_string(),
            reference_id: None,
            reference_type: None,
            is_read: false,
            created_at: now,
        };

        let res = NotificationResponse::from_model(model);
        assert_eq!(res.id, id);
        assert_eq!(res.type_name, "deployment_success");
        assert!(!res.is_read);
    }
}
