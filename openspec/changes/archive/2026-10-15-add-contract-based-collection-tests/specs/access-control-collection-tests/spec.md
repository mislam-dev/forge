# Spec Delta: Access Control Collection Tests

## Purpose

Provides automated contract test assertion suites for all RBAC System Roles, System Permissions, and Assignment endpoints in the Forge Platform API Postman collection, validating status codes, schemas against OpenAPI contracts, and dynamically persisting created role and permission identifiers into environment variables.

## ADDED Requirements

### Requirement: System Role Creation Response Validation and Identifier Persistence
The system SHALL attach automated test assertion scripts to the `POST /access-control/roles` collection request to validate both successful role creation (HTTP 201 Created conforming to `RoleResponseDto`) and error responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 409 Conflict conforming to `ErrorResponse`), and SHALL update the `role_id` environment variable upon successful creation.

#### Scenario: Successful system role creation
- **WHEN** `POST /access-control/roles` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, `data` contains `id` (UUID format), `name`, `description`, and the `role_id` environment variable is updated with the returned role ID.

#### Scenario: System role creation failure
- **WHEN** `POST /access-control/roles` returns HTTP status code 400, 401, 403, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a non-empty string, and `message` describes the failure.

### Requirement: System Roles Listing Response Validation
The system SHALL attach automated test assertion scripts to the `GET /access-control/roles` collection request to validate role listing responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized or 403 Forbidden conforming to `ErrorResponse`).

#### Scenario: Successful roles listing
- **WHEN** `GET /access-control/roles` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` array of role objects conforming to `RoleResponseDto`, and `message` is present.

#### Scenario: Roles listing unauthorized or forbidden
- **WHEN** `GET /access-control/roles` returns HTTP status code 401 or 403
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates authentication or authorization rejection.

### Requirement: System Role Detail, Update, and Deletion Contract Validation
The system SHALL attach automated test assertion scripts to `GET /access-control/roles/{id}`, `PUT /access-control/roles/{id}`, and `DELETE /access-control/roles/{id}` collection requests to validate single role operations (HTTP 200 OK) and failure responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful single role retrieval or update
- **WHEN** `GET /access-control/roles/{id}` or `PUT /access-control/roles/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` object with `id` matching `role_id`, `name`, and valid role attributes.

#### Scenario: Successful system role deletion
- **WHEN** `DELETE /access-control/roles/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Role operation not found or unauthorized
- **WHEN** role detail, update, or delete request returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: System Permission Creation and Identifier Persistence
The system SHALL attach automated test assertion scripts to the `POST /access-control/permissions` collection request to validate permission creation (HTTP 201 Created conforming to `PermissionResponseDto`) and error responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 409 Conflict), and SHALL update the `permission_id` environment variable upon successful creation.

#### Scenario: Successful system permission creation
- **WHEN** `POST /access-control/permissions` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `data` with `id`, `name`, and `action`, and the `permission_id` environment variable is updated with the returned permission ID.

#### Scenario: System permission creation failure
- **WHEN** `POST /access-control/permissions` returns HTTP status code 400, 401, 403, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a non-empty string, and `message` describes the failure.

### Requirement: System Permissions Listing, Detail, Update, and Deletion Contract Validation
The system SHALL attach automated test assertion scripts to `GET /access-control/permissions`, `GET /access-control/permissions/{id}`, `PUT /access-control/permissions/{id}`, and `DELETE /access-control/permissions/{id}` collection requests to validate listing and lifecycle operations against OpenAPI schemas.

#### Scenario: Successful permissions listing
- **WHEN** `GET /access-control/permissions` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` array of permission objects, pagination details if applicable, and response `message`.

#### Scenario: Successful single permission retrieval, update, or delete
- **WHEN** permission detail or update returns HTTP status code 200, or delete returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response payload conforms to permission contract schema or contains confirmation message.

#### Scenario: Permission operation failure or forbidden
- **WHEN** permission request returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.

### Requirement: Role Permission Assignment Contract Validation
The system SHALL attach automated test assertion scripts to `POST /access-control/roles/permissions/assign`, `POST /access-control/roles/permissions/remove`, and `GET /access-control/roles/permissions/{id}` requests to validate permission assignment to roles, permission revocation from roles, and listing role permissions.

#### Scenario: Successful role permission assignment or removal
- **WHEN** `POST /access-control/roles/permissions/assign` or `POST /access-control/roles/permissions/remove` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Successful role permissions retrieval
- **WHEN** `GET /access-control/roles/permissions/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, and response contains `data` array of assigned permissions.

#### Scenario: Role permission assignment failure
- **WHEN** assignment or revocation returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: User Role and Direct Permission Assignment Contract Validation
The system SHALL attach automated test assertion scripts to `POST /access-control/role/assign`, `POST /access-control/role/remove`, `GET /access-control/role/user/{id}`, `POST /access-control/users/permission/assign`, `POST /access-control/users/permission/remove`, and `GET /access-control/users/permissions/{id}` requests to validate user role bindings and direct permission grants.

#### Scenario: Successful user role or direct permission assignment/revocation
- **WHEN** assigning or removing user roles or direct permissions returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Successful user roles or direct permissions retrieval
- **WHEN** `GET /access-control/role/user/{id}` or `GET /access-control/users/permissions/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, and response contains `data` array of assigned roles or permissions.

#### Scenario: User role or permission assignment failure
- **WHEN** user role or permission assignment request returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.
