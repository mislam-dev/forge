define_policy!(
    ProjectsReadPolicy,
    "projects:read",
    "Policy for viewing project details"
);
define_policy!(
    ProjectsCreatePolicy,
    "projects:create",
    "Policy for creating new projects"
);
define_policy!(
    ProjectsUpdatePolicy,
    "projects:update",
    "Policy for updating project configuration"
);
define_policy!(
    ProjectsDeletePolicy,
    "projects:delete",
    "Policy for deleting projects"
);
