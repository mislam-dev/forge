pub use super::members::service::OrganizationMembersService;
pub use super::orgs::service::OrganizationService;
pub use super::permissions::service::OrgPermissionsService;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_service_reexports() {
        let _ = OrganizationService;
        let _ = OrganizationMembersService;
        let _ = OrgPermissionsService;
    }
}
