define_policy!(
    OrganizationsReadPolicy,
    "organizations:read",
    "Policy for viewing organization details"
);
define_policy!(
    OrganizationsCreatePolicy,
    "organizations:create",
    "Policy for creating new organizations"
);
define_policy!(
    OrganizationsUpdatePolicy,
    "organizations:update",
    "Policy for updating organization settings"
);
define_policy!(
    OrganizationsDeletePolicy,
    "organizations:delete",
    "Policy for deleting organizations"
);
define_policy!(
    OrganizationsMembersReadPolicy,
    "organizations:members:read",
    "Policy for viewing organization members"
);
define_policy!(
    OrganizationsMembersCreatePolicy,
    "organizations:members:create",
    "Policy for adding members to organization"
);
define_policy!(
    OrganizationsMembersUpdatePolicy,
    "organizations:members:update",
    "Policy for updating member roles in organization"
);
define_policy!(
    OrganizationsMembersDeletePolicy,
    "organizations:members:delete",
    "Policy for removing members from organization"
);
