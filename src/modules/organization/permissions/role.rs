use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrgRole {
    Viewer = 1,
    Editor = 2,
    Admin = 3,
    Owner = 4,
}

impl OrgRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrgRole::Viewer => "viewer",
            OrgRole::Editor => "editor",
            OrgRole::Admin => "admin",
            OrgRole::Owner => "owner",
        }
    }
}

impl fmt::Display for OrgRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for OrgRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "viewer" => Ok(OrgRole::Viewer),
            "editor" | "developer" => Ok(OrgRole::Editor),
            "admin" => Ok(OrgRole::Admin),
            "owner" => Ok(OrgRole::Owner),
            _ => Err(format!("Invalid organization role: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_org_role_ordering() {
        assert!(OrgRole::Viewer < OrgRole::Editor);
        assert!(OrgRole::Editor < OrgRole::Admin);
        assert!(OrgRole::Admin < OrgRole::Owner);
    }

    #[test]
    fn test_org_role_from_str() {
        assert_eq!("viewer".parse::<OrgRole>().unwrap(), OrgRole::Viewer);
        assert_eq!("editor".parse::<OrgRole>().unwrap(), OrgRole::Editor);
        assert_eq!("developer".parse::<OrgRole>().unwrap(), OrgRole::Editor);
        assert_eq!("admin".parse::<OrgRole>().unwrap(), OrgRole::Admin);
        assert_eq!("owner".parse::<OrgRole>().unwrap(), OrgRole::Owner);
        assert!("invalid".parse::<OrgRole>().is_err());
    }

    #[test]
    fn test_org_role_display_and_as_str() {
        assert_eq!(OrgRole::Viewer.as_str(), "viewer");
        assert_eq!(format!("{}", OrgRole::Owner), "owner");
    }

    #[test]
    fn test_org_role_serde() {
        let json = serde_json::to_string(&OrgRole::Admin).unwrap();
        assert_eq!(json, "\"admin\"");

        let deserialized: OrgRole = serde_json::from_str("\"editor\"").unwrap();
        assert_eq!(deserialized, OrgRole::Editor);
    }
}
