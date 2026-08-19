use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectRole {
    Viewer = 1,
    Developer = 2,
    Admin = 3,
    Owner = 4,
}

impl ProjectRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectRole::Viewer => "viewer",
            ProjectRole::Developer => "developer",
            ProjectRole::Admin => "admin",
            ProjectRole::Owner => "owner",
        }
    }
}

impl fmt::Display for ProjectRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ProjectRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "viewer" => Ok(ProjectRole::Viewer),
            "developer" => Ok(ProjectRole::Developer),
            "admin" => Ok(ProjectRole::Admin),
            "owner" => Ok(ProjectRole::Owner),
            _ => Err(format!("Invalid project role: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_role_ordering() {
        assert!(ProjectRole::Viewer < ProjectRole::Developer);
        assert!(ProjectRole::Developer < ProjectRole::Admin);
        assert!(ProjectRole::Admin < ProjectRole::Owner);
    }

    #[test]
    fn test_project_role_from_str() {
        assert_eq!("viewer".parse::<ProjectRole>().unwrap(), ProjectRole::Viewer);
        assert_eq!("developer".parse::<ProjectRole>().unwrap(), ProjectRole::Developer);
        assert_eq!("admin".parse::<ProjectRole>().unwrap(), ProjectRole::Admin);
        assert_eq!("owner".parse::<ProjectRole>().unwrap(), ProjectRole::Owner);
        assert!("invalid".parse::<ProjectRole>().is_err());
    }

    #[test]
    fn test_project_role_display_and_as_str() {
        assert_eq!(ProjectRole::Viewer.as_str(), "viewer");
        assert_eq!(format!("{}", ProjectRole::Owner), "owner");
    }

    #[test]
    fn test_project_role_serde() {
        let json = serde_json::to_string(&ProjectRole::Developer).unwrap();
        assert_eq!(json, "\"developer\"");

        let deserialized: ProjectRole = serde_json::from_str("\"admin\"").unwrap();
        assert_eq!(deserialized, ProjectRole::Admin);
    }
}
