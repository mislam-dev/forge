# Spec Delta

## Purpose

Provides automated test assertion suites for all organization and membership collection endpoints in the Forge Platform API Postman collection, verifying status codes, headers, and payload schemas against OpenAPI contracts.

## ADDED Requirements

### Requirement: Organization Creation Response Validation
The system SHALL attach automated test assertion scripts to the `POST /organizations` collection request to validate both successful organization creation (HTTP 201 Created conforming to `OrganizationResponse`) and failure responses (HTTP 400 Bad Request or HTTP 401 Unauthorized conforming to `ErrorResponse`), and SHALL update the `organization_id` environment variable upon successful creation.

#### Scenario: Successful organization creation
- **WHEN** `POST /organizations` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, `data` contains `id` (UUID format), `name`, `type`, and `created_at`, and `organization_id` environment variable is updated with the newly created organization ID.

#### Scenario: Organization creation validation failure
- **WHEN** `POST /organizations` returns HTTP status code 400 or 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a string, and `message` describes the failure.

### Requirement: Organization List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /organizations` collection request to validate organization list responses (HTTP 200 OK) and unauthorized requests (HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful organization list retrieval
- **WHEN** `GET /organizations` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, and `data` is an array of organization objects.

#### Scenario: Unauthorized organization listing request
- **WHEN** `GET /organizations` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates unauthorized access.

### Requirement: Organization Detail Retrieval Response Validation
The system SHALL attach automated test assertion scripts to the `GET /organizations/{id}` collection request to validate single organization retrieval responses (HTTP 200 OK conforming to `OrganizationResponse`) and error responses (HTTP 401 Unauthorized or HTTP 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful organization detail retrieval
- **WHEN** `GET /organizations/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains `id`, `name`, `type`, and `created_at`.

#### Scenario: Organization not found or unauthorized
- **WHEN** `GET /organizations/{id}` returns HTTP status code 401 or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error condition.

### Requirement: Organization Update Response Validation
The system SHALL attach automated test assertion scripts to the `PATCH /organizations/{id}` collection request to validate organization update responses (HTTP 200 OK conforming to `OrganizationResponse`) and failure responses (HTTP 400 Bad Request, 401 Unauthorized, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful organization update
- **WHEN** `PATCH /organizations/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains updated organization properties.

#### Scenario: Organization update validation failure or not found
- **WHEN** `PATCH /organizations/{id}` returns HTTP status code 400, 401, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates validation, authorization, or missing resource error.

### Requirement: Organization Deletion Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /organizations/{id}` collection request to validate organization deletion responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful organization deletion
- **WHEN** `DELETE /organizations/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Organization deletion unauthorized or not found
- **WHEN** `DELETE /organizations/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error reason.

### Requirement: Member Invitation Response Validation
The system SHALL attach automated test assertion scripts to the `POST /organizations/{id}/invitations` collection request to validate member invitation responses (HTTP 201 Created conforming to `OrgInvitationResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 409 Conflict conforming to `ErrorResponse`).

#### Scenario: Successful invitation dispatch
- **WHEN** `POST /organizations/{id}/invitations` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` conforms to `OrgInvitationResponse` with `id`, `email`, `role`, and `token`.

#### Scenario: Invalid invitation payload or conflict
- **WHEN** `POST /organizations/{id}/invitations` returns HTTP status code 400, 401, 403, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Pending Invitations List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /organizations/{id}/invitations` collection request to validate pending invitation retrieval responses (HTTP 200 OK) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful invitation list retrieval
- **WHEN** `GET /organizations/{id}/invitations` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` is an array of invitation objects conforming to `OrgInvitationResponse`.

#### Scenario: Unauthorized or forbidden invitation listing
- **WHEN** `GET /organizations/{id}/invitations` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error.

### Requirement: Invitation Acceptance Response Validation
The system SHALL attach automated test assertion scripts to the `POST /organizations/invitations/{token}/accept` collection request to validate invitation acceptance responses (HTTP 200 OK conforming to `OrgMemberResponse`) and token rejection responses (HTTP 400 Bad Request, 401 Unauthorized, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful invitation acceptance
- **WHEN** `POST /organizations/invitations/{token}/accept` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` conforms to `OrgMemberResponse` with `user_id`, `role`, and membership details.

#### Scenario: Invalid or expired invitation token
- **WHEN** `POST /organizations/invitations/{token}/accept` returns HTTP status code 400, 401, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates token expiration or invalidity.

### Requirement: Organization Members List Response Validation
The system SHALL attach automated test assertion scripts to the `GET /organizations/{id}/members` collection request to validate member listing responses (HTTP 200 OK) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful members list retrieval
- **WHEN** `GET /organizations/{id}/members` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` is an array of objects conforming to `OrgMemberResponse`.

#### Scenario: Unauthorized member listing
- **WHEN** `GET /organizations/{id}/members` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error.

### Requirement: Member Role Update Response Validation
The system SHALL attach automated test assertion scripts to the `PATCH /organizations/{id}/members/{user_id}` collection request to validate member role modification responses (HTTP 200 OK conforming to `OrgMemberResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful member role update
- **WHEN** `PATCH /organizations/{id}/members/{user_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` reflects the updated role.

#### Scenario: Member role update validation failure or forbidden
- **WHEN** `PATCH /organizations/{id}/members/{user_id}` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error.

### Requirement: Member Removal Response Validation
The system SHALL attach automated test assertion scripts to the `DELETE /organizations/{id}/members/{user_id}` collection request to validate member removal responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful member removal
- **WHEN** `DELETE /organizations/{id}/members/{user_id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Member removal unauthorized, forbidden, or not found
- **WHEN** `DELETE /organizations/{id}/members/{user_id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates the error.

### Requirement: Standardized Error Schema Validation Across Organization Endpoints
When an organization collection endpoint returns an HTTP 4xx or 5xx status code, the test assertion script SHALL verify that the response body conforms to the OpenAPI `ErrorResponse` schema.

#### Scenario: Error response structure verification
- **WHEN** an organization request receives an HTTP error status code (4xx or 5xx)
- **THEN** tests assert `is_error` is boolean `true`, `code` is a non-empty string, `message` is a non-empty string, and `errors` is an object if present.
