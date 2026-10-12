# Spec Delta

## Purpose

Defines the system-wide structured error responses and standardized error codes across all API endpoints and modules in accordance with the platform API documentation.

## ADDED Requirements

### Requirement: Structured Error Response Envelope
The system SHALL format all HTTP 4xx client error and HTTP 500 server error responses as a structured JSON object containing `is_error: true`, `code: string`, `message: string`, and `errors: object`.

#### Scenario: Client error response envelope
- **WHEN** any API endpoint encounters a client error (HTTP 400, 401, 403, 404, 409)
- **THEN** the response body is JSON with `is_error` set to true, `code` set to the specific documented error code string, `message` containing a human-readable explanation, and `errors` containing an object (or null/empty object when no field-level errors exist).

#### Scenario: Internal server error response envelope
- **WHEN** an unexpected internal error or database failure occurs
- **THEN** the system returns HTTP 500 Internal Server Error with `code: "INTERNAL_SERVER_ERROR"`, a sanitized generic message, and `is_error: true`.

### Requirement: Standardized Domain Error Code Namespaces
The system SHALL assign error codes prefixed with domain-specific namespaces adhering to the platform documentation across all modules.

#### Scenario: Authentication and Access Control error codes
- **WHEN** an authentication failure or access control check fails
- **THEN** the system returns an error code using the `AUTH_` prefix (e.g., `AUTH_000` to `AUTH_009`) or `ACCESS_` prefix (e.g., `ACCESS_001` to `ACCESS_403`).

#### Scenario: Organization and Project error codes
- **WHEN** an organization, project, repository, or environment variable operation fails with a client error
- **THEN** the system returns the corresponding domain code using `ORG_`, `ORG_PERM_`, `PRJ_`, `PRJ_PERM_`, `REPO_`, or `ENV_` prefixes as defined in the module specifications.

#### Scenario: Deployments, Logs, and Worker error codes
- **WHEN** deployment creation, log streaming, history inspection, or worker execution fails
- **THEN** the system returns error codes prefixed with `DEPLOY_`, `LOG_`, `HIST_`, or `WORKER_`.

#### Scenario: User, Profile, Team, Health, and Notification error codes
- **WHEN** an operation fails within users, profiles, teams, health checks, dashboard, or notifications
- **THEN** the system returns error codes prefixed with `USER_`, `PROFILE_`, `TEAM_`, `HLTH_`, `DASH_`, or `NOTIF_`.

### Requirement: Validation Error Responses
The system SHALL return HTTP 400 Bad Request with `code: "VALIDATION_ERROR"` and detailed field error mappings when payload validation fails.

#### Scenario: Payload validation failure
- **WHEN** an incoming request payload fails schema or field-level validation rules
- **THEN** the system returns HTTP 400 with `code: "VALIDATION_ERROR"` and an `errors` object mapping each invalid field name to a list of error messages explaining the validation constraints.

### Requirement: Permission and Guard Error Responses
The system SHALL return HTTP 403 Forbidden with specific permission error codes when an authenticated user lacks requisite privileges.

#### Scenario: Viewer attempting modification
- **WHEN** a viewer attempts to modify project resources
- **THEN** the system returns HTTP 403 Forbidden with `code: "PRJ_PERM_001"`.

#### Scenario: Non-owner deleting project or organization
- **WHEN** a user without owner privileges attempts to delete or rename restricted resources
- **THEN** the system returns HTTP 403 Forbidden with the corresponding guard code (such as `PRJ_PERM_002` or `ORG_PERM_002`).

### Requirement: Resource Conflict Error Responses
The system SHALL return HTTP 409 Conflict with documented collision error codes when creating duplicate unique resources.

#### Scenario: Duplicate resource creation
- **WHEN** a request attempts to register an already existing email or duplicate unique name
- **THEN** the system returns HTTP 409 Conflict with the specific conflict error code (e.g., `AUTH_003` for email, `ORG_004` for org member, `ENV_002` for duplicate env var key).
