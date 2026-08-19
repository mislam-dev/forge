use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::team::Model as TeamModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamResponse {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub name: String,
    pub descriptions: Option<String>,
    pub member_count: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl TeamResponse {
    pub fn from_model(model: TeamModel, member_count: u64) -> Self {
        Self {
            id: model.id,
            organization_id: model.organization_id,
            name: model.name,
            descriptions: model.descriptions,
            member_count,
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
    fn test_team_response_from_model() {
        let id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let now = Utc::now().into();
        let model = TeamModel {
            id,
            organization_id: Some(org_id),
            name: "DevOps".to_string(),
            descriptions: Some("Infra team".to_string()),
            created_at: now,
            updated_at: now,
        };

        let res = TeamResponse::from_model(model, 5);
        assert_eq!(res.id, id);
        assert_eq!(res.organization_id, Some(org_id));
        assert_eq!(res.name, "DevOps");
        assert_eq!(res.member_count, 5);
    }
}
