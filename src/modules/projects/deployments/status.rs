use std::fmt;
use std::str::FromStr;

pub use super::entities::sea_orm_active_enums::DeploymentStatus;

impl DeploymentStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, DeploymentStatus::Failed | DeploymentStatus::Success)
    }

    pub fn is_transient(&self) -> bool {
        !self.is_terminal()
    }

    pub fn can_transition_to(&self, next: &DeploymentStatus) -> bool {
        if self.is_terminal() {
            return false; // Terminal states are strictly immutable
        }
        match (self, next) {
            (DeploymentStatus::Queued, DeploymentStatus::Building) => true,
            (DeploymentStatus::Queued, DeploymentStatus::Failed) => true,
            (DeploymentStatus::Building, DeploymentStatus::Deploying) => true,
            (DeploymentStatus::Building, DeploymentStatus::Failed) => true,
            (DeploymentStatus::Deploying, DeploymentStatus::Running) => true,
            (DeploymentStatus::Deploying, DeploymentStatus::Failed) => true,
            (DeploymentStatus::Running, DeploymentStatus::Success) => true,
            (DeploymentStatus::Running, DeploymentStatus::Failed) => true,
            (current, target) if current == target => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DeploymentStatus::Queued => "Queued",
            DeploymentStatus::Building => "Building",
            DeploymentStatus::Deploying => "Deploying",
            DeploymentStatus::Running => "Running",
            DeploymentStatus::Failed => "Failed",
            DeploymentStatus::Success => "Success",
        }
    }
}

impl fmt::Display for DeploymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for DeploymentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Queued" | "queued" => Ok(DeploymentStatus::Queued),
            "Building" | "building" => Ok(DeploymentStatus::Building),
            "Deploying" | "deploying" => Ok(DeploymentStatus::Deploying),
            "Running" | "running" => Ok(DeploymentStatus::Running),
            "Failed" | "failed" => Ok(DeploymentStatus::Failed),
            "Success" | "success" => Ok(DeploymentStatus::Success),
            _ => Err(format!("Invalid deployment status: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_status_terminal_states() {
        assert!(DeploymentStatus::Success.is_terminal());
        assert!(DeploymentStatus::Failed.is_terminal());
        assert!(!DeploymentStatus::Building.is_terminal());
        assert!(!DeploymentStatus::Queued.is_terminal());
    }

    #[test]
    fn test_deployment_status_state_transitions() {
        assert!(DeploymentStatus::Queued.can_transition_to(&DeploymentStatus::Building));
        assert!(DeploymentStatus::Building.can_transition_to(&DeploymentStatus::Deploying));
        assert!(DeploymentStatus::Deploying.can_transition_to(&DeploymentStatus::Running));
        assert!(DeploymentStatus::Running.can_transition_to(&DeploymentStatus::Success));

        // Terminal states cannot transition out
        assert!(!DeploymentStatus::Success.can_transition_to(&DeploymentStatus::Building));
        assert!(!DeploymentStatus::Failed.can_transition_to(&DeploymentStatus::Running));
    }

    #[test]
    fn test_deployment_status_parsing() {
        assert_eq!(
            "Queued".parse::<DeploymentStatus>().unwrap(),
            DeploymentStatus::Queued
        );
        assert_eq!(
            "Running".parse::<DeploymentStatus>().unwrap(),
            DeploymentStatus::Running
        );
        assert_eq!(
            "Failed".parse::<DeploymentStatus>().unwrap(),
            DeploymentStatus::Failed
        );
        assert!("invalid".parse::<DeploymentStatus>().is_err());
    }
}
