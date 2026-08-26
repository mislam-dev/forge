define_policy!(
    AccessControlRolesReadPolicy,
    "access_control:roles:read",
    "Policy for listing and viewing system roles"
);
define_policy!(
    AccessControlRolesCreatePolicy,
    "access_control:roles:create",
    "Policy for creating new system roles"
);
define_policy!(
    AccessControlRolesUpdatePolicy,
    "access_control:roles:update",
    "Policy for updating system roles"
);
define_policy!(
    AccessControlRolesDeletePolicy,
    "access_control:roles:delete",
    "Policy for deleting system roles"
);
define_policy!(
    AccessControlPermissionsReadPolicy,
    "access_control:permissions:read",
    "Policy for listing and viewing system permissions"
);
define_policy!(
    AccessControlPermissionsCreatePolicy,
    "access_control:permissions:create",
    "Policy for creating new system permissions"
);
define_policy!(
    AccessControlPermissionsUpdatePolicy,
    "access_control:permissions:update",
    "Policy for updating system permissions"
);
define_policy!(
    AccessControlPermissionsDeletePolicy,
    "access_control:permissions:delete",
    "Policy for deleting system permissions"
);
define_policy!(
    AccessControlRolePermissionsAssignPolicy,
    "access_control:role_permissions:assign",
    "Policy for assigning permissions to system roles"
);
define_policy!(
    AccessControlRolePermissionsRemovePolicy,
    "access_control:role_permissions:remove",
    "Policy for removing permissions from system roles"
);
define_policy!(
    AccessControlUserRolesAssignPolicy,
    "access_control:user_roles:assign",
    "Policy for assigning system roles to users"
);
define_policy!(
    AccessControlUserRolesRemovePolicy,
    "access_control:user_roles:remove",
    "Policy for removing system roles from users"
);
define_policy!(
    AccessControlUserPermissionsAssignPolicy,
    "access_control:user_permissions:assign",
    "Policy for assigning direct permissions to users"
);
define_policy!(
    AccessControlUserPermissionsRemovePolicy,
    "access_control:user_permissions:remove",
    "Policy for removing direct permissions from users"
);
