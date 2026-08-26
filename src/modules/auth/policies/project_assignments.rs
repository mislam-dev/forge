define_policy!(
    ProjectAssignmentsReadPolicy,
    "project_assignments:read",
    "Policy for viewing project member and team assignments"
);
define_policy!(
    ProjectAssignmentsCreatePolicy,
    "project_assignments:create",
    "Policy for assigning members or teams to project"
);
define_policy!(
    ProjectAssignmentsDeletePolicy,
    "project_assignments:delete",
    "Policy for removing members or teams from project"
);
