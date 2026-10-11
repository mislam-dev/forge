# Spec Delta

## Purpose

Provides automated test assertion suites for all authentication collection endpoints in the Forge Platform API Postman collection, verifying success and error response contracts against the OpenAPI specification.

## ADDED Requirements

### Requirement: Registration Endpoint Response Validation
The system SHALL attach automated test assertion scripts to the `POST /auth/register` collection request to validate both successful user registration (HTTP 201 Created conforming to `UserResponse`) and client failure responses (HTTP 400 Bad Request or HTTP 409 Conflict conforming to `ErrorResponse`).

#### Scenario: Successful registration response
- **WHEN** `POST /auth/register` returns HTTP status code 201
- **THEN** tests assert Content-Type is `application/json`, response contains `message` string, and `data` contains `id` (UUID format), `name`, `email`, and `created_at`.

#### Scenario: Registration failure or validation error response
- **WHEN** `POST /auth/register` returns HTTP status code 400 or 409
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a valid string, and `message` is present.

### Requirement: Login Endpoint Response Validation and Token Persistence
The system SHALL attach automated test assertion scripts to the `POST /auth/login` collection request to validate authentication responses (HTTP 200 OK conforming to `AuthTokenResponse` and HTTP 401 Unauthorized conforming to `ErrorResponse`), and SHALL persist `bearerToken` and `refreshToken` into the Postman environment upon successful authentication.

#### Scenario: Successful authentication response
- **WHEN** `POST /auth/login` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message`, `data.access_token`, `data.refresh_token`, and `data.expires_in`, and the environment variables `bearerToken` and `refreshToken` are populated with the returned tokens.

#### Scenario: Authentication failure response
- **WHEN** `POST /auth/login` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, `code` is a string, and `message` describes the authentication failure.

### Requirement: Session Invalidation (Logout) Response Validation
The system SHALL attach automated test assertion scripts to the `POST /auth/logout` collection request to validate session invalidation responses (HTTP 200 OK and HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful logout response
- **WHEN** `POST /auth/logout` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `message` confirming logout, and `data` is an object.

#### Scenario: Unauthorized logout request
- **WHEN** `POST /auth/logout` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` is a string indicating missing or invalid authentication.

### Requirement: Token Refresh Response Validation and Token Update
The system SHALL attach automated test assertion scripts to the `POST /auth/refresh` collection request to validate token refresh responses (HTTP 200 OK conforming to `AuthTokenResponse` and HTTP 401 Unauthorized conforming to `ErrorResponse`), and SHALL update `bearerToken` and `refreshToken` in the environment upon successful refresh.

#### Scenario: Successful token refresh response
- **WHEN** `POST /auth/refresh` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data.access_token` and `data.refresh_token`, and the environment variables `bearerToken` and `refreshToken` are updated with the newly issued tokens.

#### Scenario: Invalid or expired refresh token error
- **WHEN** `POST /auth/refresh` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` represents an expired or invalid token.

### Requirement: Password Reset Request and Confirmation Response Validation
The system SHALL attach automated test assertion scripts to `POST /auth/forgot-password` and `POST /auth/reset-password` requests to validate success responses (HTTP 200 OK) and error conditions (HTTP 400 Bad Request or HTTP 401 Unauthorized).

#### Scenario: Password reset email request response
- **WHEN** `POST /auth/forgot-password` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Successful password reset confirmation
- **WHEN** `POST /auth/reset-password` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains `message` indicating password was reset.

#### Scenario: Invalid or expired reset token failure
- **WHEN** `POST /auth/reset-password` returns HTTP status code 400 or 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates token expiration or invalid payload.

### Requirement: Email Verification Response Validation
The system SHALL attach automated test assertion scripts to the `GET /auth/verify-email` request to validate activation responses (HTTP 200 OK) and token failure responses (HTTP 400 Bad Request conforming to `ErrorResponse`).

#### Scenario: Successful email verification
- **WHEN** `GET /auth/verify-email` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains activation `message`.

#### Scenario: Expired or invalid verification token
- **WHEN** `GET /auth/verify-email` returns HTTP status code 400
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` is `"AUTH_006"` or `"AUTH_007"`.

### Requirement: Authenticated User Profile Response Validation
The system SHALL attach automated test assertion scripts to the `GET /auth/me` request to validate profile retrieval responses (HTTP 200 OK conforming to `UserResponse`) and unauthorized access (HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful profile retrieval
- **WHEN** `GET /auth/me` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, and response contains `data` with `id`, `email`, and `name`.

#### Scenario: Missing or invalid token on profile retrieval
- **WHEN** `GET /auth/me` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` is `"AUTH_000"`.

### Requirement: Standardized Error Schema Validation Across Auth Endpoints
When an authentication collection endpoint returns an HTTP 4xx or 5xx status code, the test assertion script SHALL verify that the response body conforms to the OpenAPI `ErrorResponse` schema.

#### Scenario: Error response structure verification
- **WHEN** an authentication request receives an HTTP error status code (4xx or 5xx)
- **THEN** tests assert `is_error` is boolean `true`, `code` is a non-empty string, `message` is a non-empty string, and `errors` is an object if present.
