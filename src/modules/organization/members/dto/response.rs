use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::organization_invitation::Model as InvitationModel;
use super::super::entities::organization_member::Model as MemberModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberResponse {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub email: Option<String>,
    pub role: String,
    pub joined_at: String,
}

impl MemberResponse {
    pub fn from_model(model: MemberModel, email: Option<String>) -> Self {
        Self {
            organization_id: model.organization_id,
            user_id: model.user_id,
            email,
            role: model.role,
            joined_at: model.joined_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub role: String,
    pub token: String,
    pub status: String,
    pub expires_at: String,
    pub created_at: String,
}

impl InvitationResponse {
    pub fn from_model(model: InvitationModel) -> Self {
        Self {
            id: model.id,
            organization_id: model.organization_id,
            email: model.email,
            role: model.role,
            token: model.token,
            status: model.status,
            expires_at: model.expires_at.to_rfc3339(),
            created_at: model.created_at.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_member_response_from_model() {
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now().into();
        let model = MemberModel {
            organization_id: org_id,
            user_id,
            role: "owner".to_string(),
            joined_at: now,
        };

        let res = MemberResponse::from_model(model, Some("owner@example.com".to_string()));
        assert_eq!(res.organization_id, org_id);
        assert_eq!(res.user_id, user_id);
        assert_eq!(res.role, "owner");
        assert_eq!(res.email, Some("owner@example.com".to_string()));
    }

    #[test]
    fn test_invitation_response_from_model() {
        let id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let now = Utc::now().into();
        let model = InvitationModel {
            id,
            organization_id: org_id,
            email: "invite@example.com".to_string(),
            role: "editor".to_string(),
            token: "tok_12345".to_string(),
            status: "pending".to_string(),
            expires_at: now,
            created_at: now,
            updated_at: now,
        };

        let res = InvitationResponse::from_model(model);
        assert_eq!(res.id, id);
        assert_eq!(res.email, "invite@example.com");
        assert_eq!(res.token, "tok_12345");
        assert_eq!(res.status, "pending");
    }
}
