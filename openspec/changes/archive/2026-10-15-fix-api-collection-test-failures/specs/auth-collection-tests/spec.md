# Spec Delta: Auth Collection Tests

## MODIFIED Requirements

### Requirement: Email Verification Response Validation
The system SHALL route and handle HTTP GET requests at `/auth/verify-email?token=<token>` and attach automated test assertion scripts to validate activation responses (HTTP 200 OK) and token failure responses (HTTP 400 Bad Request conforming to `ErrorResponse`).

#### Scenario: Successful email verification
- **WHEN** `GET /auth/verify-email` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains activation `message`.

#### Scenario: Expired or invalid verification token
- **WHEN** `GET /auth/verify-email` returns HTTP status code 400
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` is `"AUTH_006"` or `"AUTH_007"`.
