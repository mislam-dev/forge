use crate::modules::organization::permissions::{
    extractors::permissions::{OrgRoleRequirements, RequireOrgRole},
    role::OrgRole,
};

#[derive(Debug, Clone, Copy)]
pub struct AdminRole;

impl OrgRoleRequirements for AdminRole {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Admin]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OwnerRole;

impl OrgRoleRequirements for OwnerRole {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Owner]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EditorRole;

impl OrgRoleRequirements for EditorRole {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Editor]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ViewerRole;

impl OrgRoleRequirements for ViewerRole {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Viewer]
    }
}

pub type RequireAdmin = RequireOrgRole<AdminRole>;
pub type RequireOwner = RequireOrgRole<OwnerRole>;
pub type RequireEditor = RequireOrgRole<EditorRole>;
pub type RequireViewer = RequireOrgRole<ViewerRole>;
