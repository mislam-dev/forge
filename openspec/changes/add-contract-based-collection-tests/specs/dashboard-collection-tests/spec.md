# Spec Delta: Dashboard Collection Tests

## Purpose

Provides automated contract test assertion suites for the Dashboard metrics endpoint in the Forge Platform API Postman collection, validating response status codes, header expectations, and aggregated metric schemas.

## ADDED Requirements

### Requirement: Aggregated Dashboard Metrics Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /dashboard` collection request to validate aggregated metrics responses (HTTP 200 OK) for authenticated users and unauthorized responses (HTTP 401 Unauthorized conforming to `ErrorResponse`) for unauthenticated callers.

#### Scenario: Successful dashboard metrics retrieval
- **WHEN** `GET /dashboard` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` object containing metric sections (such as platform, project, or deployment activity summaries), and `message` string is present.

#### Scenario: Unauthenticated dashboard metrics access
- **WHEN** `GET /dashboard` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, string `code` indicates unauthorized access, and `message` describes the authentication failure.

### Requirement: Standardized Error Schema Validation Across Dashboard Requests
When the dashboard endpoint returns an HTTP 4xx or 5xx status code (such as unauthorized access or internal service error), the test assertion scripts SHALL verify that the response body conforms to the standardized `ErrorResponse` schema.

#### Scenario: Error response structure verification
- **WHEN** `GET /dashboard` returns an HTTP 4xx or 5xx status code
- **THEN** tests assert Content-Type is `application/json`, `is_error` is boolean `true`, `code` is a non-empty string, and `message` is a non-empty string.
