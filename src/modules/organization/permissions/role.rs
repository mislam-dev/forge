use std::fmt;
use std::str::FromStr;

pub use crate::modules::organization::members::entities::sea_orm_active_enums::OrganizationMemberRole;
pub type OrgRole = OrganizationMemberRole;

impl OrganizationMemberRole {
    pub fn level(&self) -> u8 {
        match self {
            OrganizationMemberRole::Viewer => 1,
            OrganizationMemberRole::Editor => 2,
            OrganizationMemberRole::Admin => 3,
            OrganizationMemberRole::Owner => 4,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            OrganizationMemberRole::Viewer => "viewer",
            OrganizationMemberRole::Editor => "editor",
            OrganizationMemberRole::Admin => "admin",
            OrganizationMemberRole::Owner => "owner",
        }
    }
}

impl PartialOrd for OrganizationMemberRole {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrganizationMemberRole {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level().cmp(&other.level())
    }
}

impl fmt::Display for OrganizationMemberRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for OrganizationMemberRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "viewer" => Ok(OrganizationMemberRole::Viewer),
            "editor" | "developer" => Ok(OrganizationMemberRole::Editor),
            "admin" => Ok(OrganizationMemberRole::Admin),
            "owner" => Ok(OrganizationMemberRole::Owner),
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
        assert_eq!(OrgRole::Editor.as_str(), "editor");
        assert_eq!(OrgRole::Admin.as_str(), "admin");
        assert_eq!(OrgRole::Owner.as_str(), "owner");
        assert_eq!(format!("{}", OrgRole::Owner), "owner");
    }

    #[test]
    fn test_org_role_serde() {
        let json = serde_json::to_string(&OrgRole::Admin).unwrap();
        assert_eq!(json, "\"Admin\"");

        let deserialized: OrgRole = serde_json::from_str("\"Admin\"").unwrap();
        assert_eq!(deserialized, OrgRole::Admin);
    }
}
