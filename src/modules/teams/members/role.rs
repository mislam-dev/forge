use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamRole {
    Viewer = 1,
    Developer = 2,
    Admin = 3,
}

impl TeamRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            TeamRole::Viewer => "viewer",
            TeamRole::Developer => "developer",
            TeamRole::Admin => "admin",
        }
    }
}

impl fmt::Display for TeamRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for TeamRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "viewer" => Ok(TeamRole::Viewer),
            "developer" | "editor" => Ok(TeamRole::Developer),
            "admin" => Ok(TeamRole::Admin),
            _ => Err(format!("Invalid team role: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_role_ordering() {
        assert!(TeamRole::Viewer < TeamRole::Developer);
        assert!(TeamRole::Developer < TeamRole::Admin);
    }

    #[test]
    fn test_team_role_from_str() {
        assert_eq!("viewer".parse::<TeamRole>().unwrap(), TeamRole::Viewer);
        assert_eq!("developer".parse::<TeamRole>().unwrap(), TeamRole::Developer);
        assert_eq!("editor".parse::<TeamRole>().unwrap(), TeamRole::Developer);
        assert_eq!("admin".parse::<TeamRole>().unwrap(), TeamRole::Admin);
        assert!("invalid".parse::<TeamRole>().is_err());
    }

    #[test]
    fn test_team_role_display_and_as_str() {
        assert_eq!(TeamRole::Viewer.as_str(), "viewer");
        assert_eq!(format!("{}", TeamRole::Admin), "admin");
    }

    #[test]
    fn test_team_role_serde() {
        let json = serde_json::to_string(&TeamRole::Developer).unwrap();
        assert_eq!(json, "\"developer\"");

        let deserialized: TeamRole = serde_json::from_str("\"admin\"").unwrap();
        assert_eq!(deserialized, TeamRole::Admin);
    }
}
