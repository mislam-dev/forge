define_policy!(
    EnvironmentVariablesReadPolicy,
    "environment_variables:read",
    "Policy for viewing masked environment variables"
);
define_policy!(
    EnvironmentVariablesCreatePolicy,
    "environment_variables:create",
    "Policy for creating environment variables"
);
define_policy!(
    EnvironmentVariablesUpdatePolicy,
    "environment_variables:update",
    "Policy for updating environment variables"
);
define_policy!(
    EnvironmentVariablesDeletePolicy,
    "environment_variables:delete",
    "Policy for deleting environment variables"
);
define_policy!(
    EnvironmentVariablesDecryptPolicy,
    "environment_variables:decrypt",
    "Policy for decrypting secret environment variable values"
);
