//! Standardized error codes matching platform API documentation across all modules.

pub mod system {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const INTERNAL_SERVER_ERROR: &str = "INTERNAL_SERVER_ERROR";
    pub const BAD_REQUEST: &str = "BAD_REQUEST";
    pub const NOT_FOUND: &str = "NOT_FOUND";
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const FORBIDDEN: &str = "FORBIDDEN";
    pub const CONFLICT: &str = "CONFLICT";
}

pub mod auth {
    pub const AUTH_000: &str = "AUTH_000"; // Token Missing or Invalid
    pub const AUTH_001: &str = "AUTH_001"; // Invalid Email
    pub const AUTH_002: &str = "AUTH_002"; // Incorrect Password
    pub const AUTH_003: &str = "AUTH_003"; // Email Already Exists
    pub const AUTH_004: &str = "AUTH_004"; // JWT Expired
    pub const AUTH_005: &str = "AUTH_005"; // Invalid Refresh Token
    pub const AUTH_006: &str = "AUTH_006"; // Reset Token Expired
    pub const AUTH_007: &str = "AUTH_007"; // Email Not Verified
    pub const AUTH_008: &str = "AUTH_008"; // Account Suspended
    pub const AUTH_009: &str = "AUTH_009"; // Account Disabled
}

pub mod access {
    // Roles
    pub const ACCESS_001: &str = "ACCESS_001"; // Role Already Exists
    pub const ACCESS_002: &str = "ACCESS_002"; // Role Not Found
    pub const ACCESS_003: &str = "ACCESS_003"; // Invalid Role ID
    pub const ACCESS_004: &str = "ACCESS_004"; // Missing Required Field
    pub const ACCESS_005: &str = "ACCESS_005"; // Insufficient permissions

    // Permissions
    pub const ACCESS_101: &str = "ACCESS_101"; // Permission Already Exists
    pub const ACCESS_102: &str = "ACCESS_102"; // Permission Not Found
    pub const ACCESS_103: &str = "ACCESS_103"; // Invalid Permission ID
    pub const ACCESS_104: &str = "ACCESS_104"; // Missing Required Field

    // Role Permissions
    pub const AUTH_201: &str = "AUTH_201"; // Role Not Found
    pub const AUTH_202: &str = "AUTH_202"; // Permission Not Found
    pub const AUTH_203: &str = "AUTH_203"; // Role Permission record Not Found
    pub const ACCESS_201: &str = "ACCESS_201";
    pub const ACCESS_202: &str = "ACCESS_202";
    pub const ACCESS_203: &str = "ACCESS_203";

    // User Roles
    pub const ACCESS_301: &str = "ACCESS_301"; // User Not Found
    pub const ACCESS_302: &str = "ACCESS_302"; // Role Already Assigned
    pub const ACCESS_303: &str = "ACCESS_303"; // Role Not Found
    pub const ACCESS_304: &str = "ACCESS_304"; // Invalid Request

    // User Permissions
    pub const ACCESS_401: &str = "ACCESS_401"; // User Not Found
    pub const ACCESS_402: &str = "ACCESS_402"; // Permission Not Found
    pub const ACCESS_403: &str = "ACCESS_403"; // Permission Assigned Failed
}

pub mod organization {
    pub const ORG_001: &str = "ORG_001"; // Organization Not Found
    pub const ORG_002: &str = "ORG_002"; // Invalid Organization ID
    pub const ORG_003: &str = "ORG_003"; // Missing Required Field
    pub const ORG_004: &str = "ORG_004"; // Member Already Exists
    pub const ORG_005: &str = "ORG_005"; // Invalid or Expired Invitation Token
    pub const ORG_006: &str = "ORG_006"; // Cannot Modify Organization Owner
    pub const ORG_007: &str = "ORG_007"; // Member Not Found

    // Org Permissions
    pub const ORG_PERM_001: &str = "ORG_PERM_001"; // Insufficient organization role for this operation
    pub const ORG_PERM_002: &str = "ORG_PERM_002"; // Only the Organization Owner can rename or delete this organization
}

pub mod projects {
    pub const PRJ_001: &str = "PRJ_001"; // Project Not Found
    pub const PRJ_002: &str = "PRJ_002"; // Invalid Project ID
    pub const PRJ_003: &str = "PRJ_003"; // Missing Required Field
    pub const PRJ_004: &str = "PRJ_004"; // Invalid Runtime Environment / Assignment Already Exists
    pub const PRJ_005: &str = "PRJ_005"; // Invalid Project Type / Assignment Not Found
    pub const PRJ_006: &str = "PRJ_006"; // Unauthorized Project Operation

    // Project Files
    pub const PRJ_FILE_001: &str = "PRJ_FILE_001"; // File or Directory Not Found
    pub const PRJ_FILE_002: &str = "PRJ_FILE_002"; // Invalid Path or Path Traversal Attempt
    pub const PRJ_FILE_003: &str = "PRJ_FILE_003"; // File Already Exists
    pub const PRJ_FILE_004: &str = "PRJ_FILE_004"; // File Operation Failed

    // Project Permissions
    pub const PRJ_PERM_001: &str = "PRJ_PERM_001"; // Read-only access. Viewers cannot modify project resources
    pub const PRJ_PERM_002: &str = "PRJ_PERM_002"; // Access denied. Developers can only delete projects created by themselves
    pub const PRJ_PERM_003: &str = "PRJ_PERM_003"; // Access denied. Only the project owner or Org Admin can manage assignments

    // Repository
    pub const REPO_001: &str = "REPO_001"; // Repository Inaccessible or Not Found
    pub const REPO_002: &str = "REPO_002"; // Authentication Failed (Invalid Personal Access Token)
    pub const REPO_003: &str = "REPO_003"; // Repository Clone Failed
    pub const REPO_004: &str = "REPO_004"; // Target Branch Not Found on Remote
    pub const REPO_005: &str = "REPO_005"; // Encryption Error During Secret Storage

    // Environment Variables
    pub const ENV_001: &str = "ENV_001"; // Invalid Key Format
    pub const ENV_002: &str = "ENV_002"; // Duplicate Variable Key for Environment
    pub const ENV_003: &str = "ENV_003"; // Invalid Target Environment
    pub const ENV_004: &str = "ENV_004"; // Environment Variable Not Found
    pub const ENV_005: &str = "ENV_005"; // Encryption / Decryption Error
}

pub mod deployments {
    // Deployment
    pub const DEPLOY_001: &str = "DEPLOY_001"; // Deployment Not Found
    pub const DEPLOY_002: &str = "DEPLOY_002"; // Project Not Found
    pub const DEPLOY_003: &str = "DEPLOY_003"; // Invalid State Transition
    pub const DEPLOY_004: &str = "DEPLOY_004"; // Project Not Active
    pub const DEPLOY_005: &str = "DEPLOY_005"; // Missing Required Fields

    // Build Worker
    pub const WORKER_001: &str = "WORKER_001"; // Repository Clone Failed
    pub const WORKER_002: &str = "WORKER_002"; // Dockerfile Not Found
    pub const WORKER_003: &str = "WORKER_003"; // Docker Build Failed
    pub const WORKER_004: &str = "WORKER_004"; // Container Start Failed
    pub const WORKER_005: &str = "WORKER_005"; // Health Check Timeout / Failed
    pub const WORKER_006: &str = "WORKER_006"; // Environment Variable Decryption Error
    pub const WORKER_007: &str = "WORKER_007"; // Log Store Unavailable

    // Live Build Logs
    pub const LOG_001: &str = "LOG_001"; // Deployment Not Found
    pub const LOG_002: &str = "LOG_002"; // Unauthorized Access to Deployment Logs
    pub const LOG_003: &str = "LOG_003"; // No Logs Available Yet
    pub const LOG_004: &str = "LOG_004"; // Stream Connection Failed

    // Deployment History
    pub const HIST_001: &str = "HIST_001"; // Project Not Found
    pub const HIST_002: &str = "HIST_002"; // Deployment Not Found
    pub const HIST_003: &str = "HIST_003"; // Project Not Active
    pub const HIST_004: &str = "HIST_004"; // No Successful Deployment Found for Rollback
    pub const HIST_005: &str = "HIST_005"; // Unauthorized Access to Deployment History
}

pub mod users {
    // Users
    pub const USER_001: &str = "USER_001"; // User Not Found
    pub const USER_002: &str = "USER_002"; // Email Already Exists
    pub const USER_003: &str = "USER_003"; // Invalid Input Data
    pub const USER_004: &str = "USER_004"; // Invalid Verification Token
    pub const USER_005: &str = "USER_005"; // Token Expired
    pub const USER_006: &str = "USER_006"; // Missing Required Fields

    // Profiles
    pub const PROFILE_001: &str = "PROFILE_001"; // Profile Not Found
    pub const PROFILE_002: &str = "PROFILE_002"; // Invalid Image Format
    pub const PROFILE_003: &str = "PROFILE_003"; // Missing Required Fields
    pub const PROFILE_004: &str = "PROFILE_004"; // Profile Already Exists
}

pub mod teams {
    pub const TEAM_001: &str = "TEAM_001"; // Team Not Found
    pub const TEAM_002: &str = "TEAM_002"; // Member Already Exists in Team
    pub const TEAM_003: &str = "TEAM_003"; // Member Not Found in Team
    pub const TEAM_005: &str = "TEAM_005"; // Invalid Team Member Role
}

pub mod notifications {
    pub const NOTIF_001: &str = "NOTIF_001"; // Notification Not Found
    pub const NOTIF_002: &str = "NOTIF_002"; // Unauthorized — Notification Belongs to Another User
    pub const NOTIF_003: &str = "NOTIF_003"; // Invalid Notification Type
    pub const NOTIF_004: &str = "NOTIF_004"; // Missing Required Fields
    pub const NOTIF_005: &str = "NOTIF_005"; // SSE Stream Connection Failed
}

pub mod health {
    pub const HLTH_001: &str = "HLTH_001"; // Service Probe Timeout
    pub const HLTH_002: &str = "HLTH_002"; // Unauthorized — Deep Check Requires Admin Access
    pub const HLTH_003: &str = "HLTH_003"; // Invalid timeout_ms Parameter
}

pub mod dashboard {
    pub const DASH_001: &str = "DASH_001"; // Organization Not Found
    pub const DASH_002: &str = "DASH_002"; // Unauthorized — User Not a Member of This Organization
    pub const DASH_003: &str = "DASH_003"; // Aggregation Timeout
}
