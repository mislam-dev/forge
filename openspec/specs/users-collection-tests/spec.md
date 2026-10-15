# Spec: Users Collection Tests

## Purpose

Provides automated contract test assertion suites for all Users and User Profile endpoints in the Forge Platform API Postman collection, validating status codes, response schemas against OpenAPI contracts, and dynamically persisting created user identifiers into environment variables.

## Requirements

### Requirement: User Creation Contract Validation and Identifier Persistence
The system SHALL attach automated test assertion scripts to the `POST /users` collection request to validate both successful user creation (HTTP 201 Created conforming to `UserResponse`) and failure conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 409 Conflict conforming to `ErrorResponse`), and SHALL persist `user_id` into the active Postman environment upon successful creation.

#### Scenario: Successful user creation response
- **WHEN** `POST /users` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, `data` contains `id` (UUID format), `name`, `email`, and `created_at`, and the `user_id` environment variable is updated with the returned user ID.

#### Scenario: User creation validation or conflict failure
- **WHEN** `POST /users` returns HTTP status code 400, 401, 403, or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a non-empty string, and `message` describes the failure reason.

### Requirement: Paginated Users List Response Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /users` collection request to validate paginated user list responses (HTTP 200 OK conforming to `ApiResponsePaginatedUsers`) and error conditions (HTTP 401 Unauthorized or 403 Forbidden conforming to `ErrorResponse`).

#### Scenario: Successful paginated users list retrieval
- **WHEN** `GET /users` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` array of user objects, `pagination` object with `page`, `per_page`, `total`, and `total_pages`, and response `message` is present.

#### Scenario: Unauthorized or forbidden users listing
- **WHEN** `GET /users` returns HTTP status code 401 or 403
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates authentication or authorization error.

### Requirement: User Detail Retrieval Response Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /users/{id}` collection request to validate single user retrieval responses (HTTP 200 OK conforming to `UserResponse`) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful user detail retrieval
- **WHEN** `GET /users/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` object with `id` matching the requested user, `name`, `email`, and `is_active`.

#### Scenario: User not found or forbidden
- **WHEN** `GET /users/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` represents the error reason.

### Requirement: User Update Response Contract Validation
The system SHALL attach automated test assertion scripts to the `PUT /users/{id}` collection request to validate user account modification responses (HTTP 200 OK conforming to `UserResponse`) and error responses (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful user account update
- **WHEN** `PUT /users/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and `data` contains updated user attributes.

#### Scenario: User update failure or not found
- **WHEN** `PUT /users/{id}` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates update error.

### Requirement: User Deletion Response Contract Validation
The system SHALL attach automated test assertion scripts to the `DELETE /users/{id}` collection request to validate user deletion responses (HTTP 200 OK or 204 No Content) and failure conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful user deletion
- **WHEN** `DELETE /users/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: User deletion unauthorized or not found
- **WHEN** `DELETE /users/{id}` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error.

### Requirement: User Profile Detail Retrieval Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /users/{id}/profile` collection request to validate profile retrieval responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful profile retrieval
- **WHEN** `GET /users/{id}/profile` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` object with `user_id`, profile attributes, and non-empty status.

#### Scenario: Profile not found or unauthorized
- **WHEN** `GET /users/{id}/profile` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error.

### Requirement: User Profile Update Contract Validation
The system SHALL attach automated test assertion scripts to the `PUT /users/{id}/profile` collection request to validate profile update responses (HTTP 200 OK) and error conditions (HTTP 400 Bad Request, 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful profile update
- **WHEN** `PUT /users/{id}/profile` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, and updated profile data in `data`.

#### Scenario: Profile update validation failure or forbidden
- **WHEN** `PUT /users/{id}/profile` returns HTTP status code 400, 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` describes the validation or access error.
