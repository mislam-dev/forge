define_policy!(
    TeamsReadPolicy,
    "teams:read",
    "Policy for viewing team details"
);
define_policy!(
    TeamsCreatePolicy,
    "teams:create",
    "Policy for creating new teams"
);
define_policy!(
    TeamsUpdatePolicy,
    "teams:update",
    "Policy for updating team details"
);
define_policy!(
    TeamsDeletePolicy,
    "teams:delete",
    "Policy for deleting teams"
);
define_policy!(
    TeamsMembersReadPolicy,
    "teams:members:read",
    "Policy for viewing team members"
);
define_policy!(
    TeamsMembersCreatePolicy,
    "teams:members:create",
    "Policy for adding members to team"
);
define_policy!(
    TeamsMembersDeletePolicy,
    "teams:members:delete",
    "Policy for removing members from team"
);
