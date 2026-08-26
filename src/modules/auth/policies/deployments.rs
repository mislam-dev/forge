define_policy!(
    DeploymentsReadPolicy,
    "deployments:read",
    "Policy for viewing deployment details and history"
);
define_policy!(
    DeploymentsCreatePolicy,
    "deployments:create",
    "Policy for triggering a new deployment"
);
define_policy!(
    DeploymentsRedeployPolicy,
    "deployments:redeploy",
    "Policy for redeploying at specific commit"
);
define_policy!(
    DeploymentsRollbackPolicy,
    "deployments:rollback",
    "Policy for rolling back to last successful deployment"
);
define_policy!(
    DeploymentsStatusUpdatePolicy,
    "deployments:status:update",
    "Policy for updating deployment status (Internal build worker)"
);
