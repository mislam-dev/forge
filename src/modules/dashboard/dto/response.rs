use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentSummaryItem {
    pub id: Uuid,
    pub project_id: Uuid,
    pub branch: String,
    pub commit_hash: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgDashboardResponse {
    pub org_id: Uuid,
    pub members_count: u64,
    pub projects_count: u64,
    pub teams_count: u64,
    pub deployments_count: u64,
    pub success_rate: f64,
    pub active_deployments_count: u64,
    pub recent_deployments: Vec<DeploymentSummaryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDashboardResponse {
    pub assigned_projects_count: u64,
    pub deployments_triggered_count: u64,
    pub org_memberships_count: u64,
    pub recent_activity: Vec<DeploymentSummaryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDashboardResponse {
    pub total_organizations: u64,
    pub total_users: u64,
    pub total_projects: u64,
    pub total_deployments: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_org_dashboard_response_serialization() {
        let res = OrgDashboardResponse {
            org_id: Uuid::new_v4(),
            members_count: 5,
            projects_count: 2,
            teams_count: 1,
            deployments_count: 20,
            success_rate: 0.95,
            active_deployments_count: 1,
            recent_deployments: vec![],
        };
        let json = serde_json::to_string(&res).unwrap();
        assert!(json.contains("0.95"));
    }
}
