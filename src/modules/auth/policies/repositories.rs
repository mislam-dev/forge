define_policy!(
    RepositoriesReadPolicy,
    "repositories:read",
    "Policy for viewing repository configuration and branches"
);
define_policy!(
    RepositoriesCreatePolicy,
    "repositories:create",
    "Policy for connecting repository to project"
);
define_policy!(
    RepositoriesUpdatePolicy,
    "repositories:update",
    "Policy for updating repository settings or active branch"
);
define_policy!(
    RepositoriesValidatePolicy,
    "repositories:validate",
    "Policy for validating repository access credentials"
);
define_policy!(
    RepositoriesClonePolicy,
    "repositories:clone",
    "Policy for triggering repository clone operation"
);
