# Spec: Teams Collection Tests

## Purpose

Provides automated test assertion suites for all team and team membership collection endpoints in the Forge Platform API Postman collection, verifying status codes, headers, and payload schemas against OpenAPI contracts.

## Requirements

### Requirement: Team Creation Response Validation
The system SHALL attach automated test assertion scripts to the `POST /teams` collection request to validate both successful team creation (HTTP 201 Created conforming to `TeamResponse`) and failure responses (HTTP 400 Bad Request, HTTP 401 Unauthorized, or HTTP 403 Forbidden conforming to `ErrorResponse`), and SHALL update the `team_id` environment variable upon successful creation.

#### Scenario: Successful team creation
- **WHEN** `POST /teams` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, `data` contains `id` (UUID format), `organization_id`, `name`, `descriptions`, `member_count`, and `created_at`, and `team_id` environment variable is updated with the newly created team ID.

#### Scenario: Team creation validation or authorization failure
- **WHEN** `POST /teams` returns HTTP status code 400, 401, or 403
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a string, and `message` describes the failure.

### Requirement: Teams List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /teams` collection request to validate teams list responses (HTTP 200 OK) and error requests (HTTP 400 Bad Request, HTTP 401 Unauthorized, or HTTP 403 Forbidden conforming to `ErrorResponse`).

#### Scenario: Successful teams list retrieval
- **WHEN** `GET /teams` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, and `data` is an array of team objects conforming to `TeamResponse`.

#### Scenario: Teams listing validation or authorization failure
- **WHEN** `GET /teams` returns HTTP status code 400, 401, or 403
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.

### Requirement: Team Detail Retrieval Response Validation
The system SHALL attach automated test assertion scripts to the `GET /teams/{id}` collection request to validate single team retrieval responses (HTTP 200 OK conforming to `TeamResponse`) and error responses (HTTP 401 Unauthorized, HTTP 403 Forbidden, or HTTP 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful team detail retrieval
- **WHEN** `GET /teams/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains `id`, `organization_id`, `name`, `descriptions`, `member_count`, and `created_at`.

#### Scenario: Team not found or unauthorized
- **WHEN** `GET /teams/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.

### Requirement: Team Update Response Validation
The system SHALL attach automated test assertion scripts to the `PATCH /teams/{id}` collection request to validate team update responses (HTTP 200 OK conforming to `TeamResponse`) and failure responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful team update
- **WHEN** `PATCH /teams/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains updated team properties conforming to `TeamResponse`.

#### Scenario: Team update validation failure, unauthorized, or not found
- **WHEN** `PATCH /teams/{id}` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates validation, authorization, or missing resource error.

### Requirement: Team Deletion Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /teams/{id}` collection request to validate team deletion responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful team deletion
- **WHEN** `DELETE /teams/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Team deletion unauthorized or not found
- **WHEN** `DELETE /teams/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error reason.

### Requirement: Team Members List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /teams/{id}/members` collection request to validate team member list responses (HTTP 200 OK) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful team members list retrieval
- **WHEN** `GET /teams/{id}/members` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` is an array of team member objects conforming to `TeamMemberResponse` with `team_id`, `user_id`, and `role`.

#### Scenario: Team members list unauthorized or not found
- **WHEN** `GET /teams/{id}/members` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error reason.

### Requirement: Add User to Team Response Validation
The system SHALL attach automated test assertion scripts to the `POST /teams/{id}/members` collection request to validate member addition responses (HTTP 201 Created conforming to `TeamMemberResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, or 409 Conflict conforming to `ErrorResponse`).

#### Scenario: Successful member addition
- **WHEN** `POST /teams/{id}/members` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` conforms to `TeamMemberResponse` with `team_id`, `user_id`, and `role`, and `joined_at`.

#### Scenario: Member addition validation failure, conflict, or unauthorized
- **WHEN** `POST /teams/{id}/members` returns HTTP status code 400, 401, 403, 404, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Update Team Member Role Response Validation
The system SHALL attach automated test assertion scripts to the `PATCH /teams/{id}/members/{user_id}` collection request to validate member role update responses (HTTP 200 OK conforming to `TeamMemberResponse`) and error conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful member role update
- **WHEN** `PATCH /teams/{id}/members/{user_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` conforms to `TeamMemberResponse` with updated `role`.

#### Scenario: Member role update validation failure, unauthorized, or not found
- **WHEN** `PATCH /teams/{id}/members/{user_id}` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error reason.

### Requirement: Remove User from Team Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /teams/{id}/members/{user_id}` collection request to validate member removal responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful member removal
- **WHEN** `DELETE /teams/{id}/members/{user_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Member removal unauthorized or not found
- **WHEN** `DELETE /teams/{id}/members/{user_id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error reason.
