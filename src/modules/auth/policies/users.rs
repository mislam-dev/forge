define_policy!(
    UsersReadPolicy,
    "users:read",
    "Policy for listing and viewing user profiles"
);
define_policy!(
    UsersCreatePolicy,
    "users:create",
    "Policy for creating new user accounts"
);
define_policy!(
    UsersUpdatePolicy,
    "users:update",
    "Policy for updating user profile details"
);
define_policy!(
    UsersDeletePolicy,
    "users:delete",
    "Policy for deleting user accounts"
);
