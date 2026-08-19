use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::organization::orgs::entities::organization::Model as OrganizationModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub owner_user_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

impl OrganizationResponse {
    pub fn from_model(model: OrganizationModel, owner_user_id: Option<Uuid>) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            logo_url: model.logo_url,
            owner_user_id,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_organization_response_from_model() {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let now = Utc::now().into();
        let model = OrganizationModel {
            id,
            name: "Acme Corp".to_string(),
            slug: "acme-corp".to_string(),
            description: Some("Test".to_string()),
            logo_url: None,
            created_at: now,
            updated_at: now,
        };

        let res = OrganizationResponse::from_model(model, Some(owner_id));
        assert_eq!(res.id, id);
        assert_eq!(res.name, "Acme Corp");
        assert_eq!(res.slug, "acme-corp");
        assert_eq!(res.owner_user_id, Some(owner_id));
    }
}
