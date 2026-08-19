use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Type is required"))]
    pub type_name: String,
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Message is required"))]
    pub message: String,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NotificationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub is_read: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_notification_request_validation() {
        let req = CreateNotificationRequest {
            user_id: Uuid::new_v4(),
            type_name: "deployment_success".to_string(),
            title: "Deployment Succeeded".to_string(),
            message: "Project build #1 succeeded".to_string(),
            reference_id: None,
            reference_type: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_notification_request_empty_title_fails() {
        let req = CreateNotificationRequest {
            user_id: Uuid::new_v4(),
            type_name: "deployment_failed".to_string(),
            title: "".to_string(),
            message: "Build failed".to_string(),
            reference_id: None,
            reference_type: None,
        };
        assert!(req.validate().is_err());
    }
}
