# Spec: Projects Collection Tests

## Purpose

Provides comprehensive automated test assertion suites for all 30 project and related collection endpoints in the Forge Platform API Postman collection, verifying status codes, headers, response schemas, and variable persistence against OpenAPI contracts.

## Requirements

### Requirement: Project Creation Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects` collection request to validate both successful project creation (HTTP 201 Created conforming to `ProjectResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 409 Conflict conforming to `ErrorResponse`), and SHALL update the `project_id` environment variable upon successful creation.

#### Scenario: Successful project creation
- **WHEN** `POST /projects` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, `data` contains `id` (UUID format), `name`, `type`, `runtime`, and `created_at`, and the `project_id` environment variable is updated with the created project ID.

#### Scenario: Project creation validation or authorization failure
- **WHEN** `POST /projects` returns HTTP status code 400, 401, 403, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a string, and `message` describes the failure.

### Requirement: Projects List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects` collection request to validate projects list responses (HTTP 200 OK) and error requests (HTTP 400 Bad Request, 401 Unauthorized, or 403 Forbidden conforming to `ErrorResponse`).

#### Scenario: Successful projects list retrieval
- **WHEN** `GET /projects` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, and `data` is an array of project objects conforming to `ProjectResponse`.

#### Scenario: Projects listing validation or authorization failure
- **WHEN** `GET /projects` returns HTTP status code 400, 401, or 403
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.

### Requirement: Project Detail Retrieval Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}` collection request to validate single project retrieval responses (HTTP 200 OK conforming to `ProjectResponse`) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful project detail retrieval
- **WHEN** `GET /projects/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains `id`, `name`, `type`, `runtime`, and project configuration properties.

#### Scenario: Project not found or unauthorized
- **WHEN** `GET /projects/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.

### Requirement: Project Update Response Validation
The system SHALL attach automated test assertion scripts to the `PUT /projects/{id}` collection request to validate project update responses (HTTP 200 OK conforming to `ProjectResponse`) and failure responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful project update
- **WHEN** `PUT /projects/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains updated project properties conforming to `ProjectResponse`.

#### Scenario: Project update validation failure, unauthorized, or not found
- **WHEN** `PUT /projects/{id}` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Project Deletion Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /projects/{id}` collection request to validate project deletion responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful project deletion
- **WHEN** `DELETE /projects/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Project deletion unauthorized or not found
- **WHEN** `DELETE /projects/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error condition.

### Requirement: Connect Git Repository Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/repository` collection request to validate Git repository connection responses (HTTP 201 Created conforming to `ProjectRepositoryResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, or 409 Conflict conforming to `ErrorResponse`).

#### Scenario: Successful repository connection
- **WHEN** `POST /projects/{id}/repository` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains repository connection details (`repository_url`, `default_branch`).

#### Scenario: Repository connection failure or unauthorized
- **WHEN** `POST /projects/{id}/repository` returns HTTP status code 400, 401, 403, 404, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Get Connected Repository Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/repository` collection request to validate connected repository retrieval responses (HTTP 200 OK conforming to `ProjectRepositoryResponse`) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful repository configuration retrieval
- **WHEN** `GET /projects/{id}/repository` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains repository configuration details.

#### Scenario: Repository configuration not found or unauthorized
- **WHEN** `GET /projects/{id}/repository` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Update Connected Repository Response Validation
The system SHALL attach automated test assertion scripts to the `PATCH /projects/{id}/repository` collection request to validate repository update responses (HTTP 200 OK conforming to `ProjectRepositoryResponse`) and failure responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful repository configuration update
- **WHEN** `PATCH /projects/{id}/repository` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains updated repository settings.

#### Scenario: Repository update validation failure or unauthorized
- **WHEN** `PATCH /projects/{id}/repository` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Disconnect Repository Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /projects/{id}/repository` collection request to validate repository disconnection responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful repository disconnection
- **WHEN** `DELETE /projects/{id}/repository` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Repository disconnection unauthorized or not found
- **WHEN** `DELETE /projects/{id}/repository` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Project Members List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/members` collection request to validate project members list responses (HTTP 200 OK) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful project members list retrieval
- **WHEN** `GET /projects/{id}/members` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` is an array of assigned member objects.

#### Scenario: Project members listing unauthorized or not found
- **WHEN** `GET /projects/{id}/members` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Assign User to Project Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/members` collection request to validate user assignment responses (HTTP 200 OK or 201 Created) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, or 409 Conflict conforming to `ErrorResponse`).

#### Scenario: Successful user assignment
- **WHEN** `POST /projects/{id}/members` returns HTTP status code 200 or 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and member assignment is confirmed.

#### Scenario: User assignment validation failure or unauthorized
- **WHEN** `POST /projects/{id}/members` returns HTTP status code 400, 401, 403, 404, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Remove User from Project Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /projects/{id}/members/{user_id}` collection request to validate user removal responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful user removal from project
- **WHEN** `DELETE /projects/{id}/members/{user_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: User removal unauthorized or not found
- **WHEN** `DELETE /projects/{id}/members/{user_id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Project Teams List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/teams` collection request to validate project teams list responses (HTTP 200 OK) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful project teams list retrieval
- **WHEN** `GET /projects/{id}/teams` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` is an array of assigned team objects.

#### Scenario: Project teams listing unauthorized or not found
- **WHEN** `GET /projects/{id}/teams` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Assign Team to Project Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/teams` collection request to validate team assignment responses (HTTP 200 OK or 201 Created) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, or 409 Conflict conforming to `ErrorResponse`).

#### Scenario: Successful team assignment
- **WHEN** `POST /projects/{id}/teams` returns HTTP status code 200 or 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and team assignment is confirmed.

#### Scenario: Team assignment validation failure or unauthorized
- **WHEN** `POST /projects/{id}/teams` returns HTTP status code 400, 401, 403, 404, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Remove Team from Project Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /projects/{id}/teams/{team_id}` collection request to validate team removal responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful team removal from project
- **WHEN** `DELETE /projects/{id}/teams/{team_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Team removal unauthorized or not found
- **WHEN** `DELETE /projects/{id}/teams/{team_id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Create Environment Variable Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/env-vars` collection request to validate environment variable creation responses (HTTP 201 Created conforming to `ProjectEnvironmentVariableResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, or 409 Conflict conforming to `ErrorResponse`), and SHALL update the `env_id` environment variable upon successful creation.

#### Scenario: Successful environment variable creation
- **WHEN** `POST /projects/{id}/env-vars` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, `data` contains `id`, `project_id`, `key`, `environment`, and `created_at`, and `env_id` environment variable is updated with the created variable ID.

#### Scenario: Environment variable creation validation failure or conflict
- **WHEN** `POST /projects/{id}/env-vars` returns HTTP status code 400, 401, 403, 404, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: List Environment Variables Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/env-vars` collection request to validate environment variables listing responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful environment variables listing
- **WHEN** `GET /projects/{id}/env-vars` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` is an array of environment variable objects.

#### Scenario: Environment variables listing unauthorized or not found
- **WHEN** `GET /projects/{id}/env-vars` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Bulk Environment Variables Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/env-vars/bulk` collection request to validate bulk upsert responses (HTTP 200 OK or 201 Created) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful bulk environment variables upsert
- **WHEN** `POST /projects/{id}/env-vars/bulk` returns HTTP status code 200 or 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains an array of processed environment variables.

#### Scenario: Bulk environment variables validation failure or unauthorized
- **WHEN** `POST /projects/{id}/env-vars/bulk` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Update Environment Variable Response Validation
The system SHALL attach automated test assertion scripts to the `PATCH /projects/{id}/env-vars/{env_id}` collection request to validate environment variable update responses (HTTP 200 OK conforming to `ProjectEnvironmentVariableResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful environment variable update
- **WHEN** `PATCH /projects/{id}/env-vars/{env_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains updated environment variable properties.

#### Scenario: Environment variable update validation failure or not found
- **WHEN** `PATCH /projects/{id}/env-vars/{env_id}` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Delete Environment Variable Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /projects/{id}/env-vars/{env_id}` collection request to validate environment variable deletion responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful environment variable deletion
- **WHEN** `DELETE /projects/{id}/env-vars/{env_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Environment variable deletion unauthorized or not found
- **WHEN** `DELETE /projects/{id}/env-vars/{env_id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error reason.

### Requirement: Trigger Deployment Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/deployments` collection request to validate deployment trigger responses (HTTP 201 Created conforming to `DeploymentResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, or 409 Conflict conforming to `ErrorResponse`), and SHALL update the `deployment_id` environment variable upon successful creation.

#### Scenario: Successful deployment trigger
- **WHEN** `POST /projects/{id}/deployments` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, `data` contains `id` (UUID format), `project_id`, `status`, and `created_at`, and the `deployment_id` environment variable is updated.

#### Scenario: Deployment trigger validation failure or unauthorized
- **WHEN** `POST /projects/{id}/deployments` returns HTTP status code 400, 401, 403, 404, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Deployment History List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/deployments` collection request to validate deployment history listing responses (HTTP 200 OK) and error conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful deployment history listing
- **WHEN** `GET /projects/{id}/deployments` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains an array of deployment objects conforming to `DeploymentResponse`.

#### Scenario: Deployment history listing error or unauthorized
- **WHEN** `GET /projects/{id}/deployments` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Deployment Detail Retrieval Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/deployments/{deployment_id}` collection request to validate deployment detail retrieval responses (HTTP 200 OK conforming to `DeploymentResponse`) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful deployment detail retrieval
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains `id`, `project_id`, `status`, `commit_hash`, `branch`, and timestamps.

#### Scenario: Deployment not found or unauthorized
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Redeploy Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/deployments/{deployment_id}/redeploy` collection request to validate redeployment responses (HTTP 201 Created or 202 Accepted conforming to `DeploymentResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`), and SHALL update the `deployment_id` environment variable upon successful creation.

#### Scenario: Successful redeployment
- **WHEN** `POST /projects/{id}/deployments/{deployment_id}/redeploy` returns HTTP status code 201 or 202
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, `data` contains new deployment details, and `deployment_id` environment variable is updated.

#### Scenario: Redeployment error or unauthorized
- **WHEN** `POST /projects/{id}/deployments/{deployment_id}/redeploy` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Rollback Deployment Response Validation
The system SHALL attach automated test assertion scripts to the `POST /projects/{id}/deployments/rollback` collection request to validate rollback deployment responses (HTTP 201 Created or 202 Accepted conforming to `DeploymentResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`), and SHALL update the `deployment_id` environment variable upon successful creation.

#### Scenario: Successful rollback deployment
- **WHEN** `POST /projects/{id}/deployments/rollback` returns HTTP status code 201 or 202
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, `data` contains rollback deployment details, and `deployment_id` environment variable is updated.

#### Scenario: Rollback error or unauthorized
- **WHEN** `POST /projects/{id}/deployments/rollback` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Get Stored Build Logs Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/deployments/{deployment_id}/logs` collection request to validate build logs retrieval responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful build logs retrieval
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains stored log entries.

#### Scenario: Build logs not found or unauthorized
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Download Deployment Log File Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/deployments/{deployment_id}/logs/download` collection request to validate plain text log file download responses (HTTP 200 OK with `text/plain` Content-Type) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful log file download
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs/download` returns HTTP status code 200
- **THEN** tests assert response status is 200 and Content-Type contains `text/plain` or `application/octet-stream`.

#### Scenario: Log file download unauthorized or not found
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs/download` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Search Build Logs Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/deployments/{deployment_id}/logs/search` collection request to validate keyword search responses (HTTP 200 OK) and error conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful log search
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs/search` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains filtered log matches.

#### Scenario: Log search validation error or unauthorized
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs/search` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Stream Build Logs SSE Response Validation
The system SHALL attach automated test assertion scripts to the `GET /projects/{id}/deployments/{deployment_id}/logs/stream` collection request to validate Server-Sent Events stream initialization (HTTP 200 OK with `text/event-stream` Content-Type) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful SSE log stream opening
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs/stream` returns HTTP status code 200
- **THEN** tests assert Content-Type includes `text/event-stream`.

#### Scenario: SSE stream unauthorized or not found
- **WHEN** `GET /projects/{id}/deployments/{deployment_id}/logs/stream` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Internal Worker Deployment Status Update Validation
The system SHALL attach automated test assertion scripts to the `PUT /projects/internal/deployments/{deployment_id}/status` collection request to validate internal status update responses (HTTP 200 OK) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful internal status update
- **WHEN** `PUT /projects/internal/deployments/{deployment_id}/status` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and status update is confirmed.

#### Scenario: Internal status update unauthorized or not found
- **WHEN** `PUT /projects/internal/deployments/{deployment_id}/status` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.
