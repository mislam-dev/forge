# Spec Delta

## Purpose

Provides automated test assertion suites for all health probe endpoints in the Forge Platform API Postman collection, verifying status codes, headers, and payload schemas against OpenAPI contracts.

## ADDED Requirements

### Requirement: Liveness Probe Response Validation
The system SHALL attach automated test assertion scripts to the `GET /health/live` collection request to validate process liveness responses against OpenAPI contracts.

#### Scenario: Successful liveness probe response
- **WHEN** `GET /health/live` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `status` equal to `"healthy"`, `service` equal to `"forge-platform"`, non-empty string `version`, non-empty string `environment`, and ISO 8601 UTC `timestamp`.

### Requirement: Readiness Probe Response Validation
The system SHALL attach automated test assertion scripts to the `GET /health/ready` collection request to validate dependency readiness responses for both operational (HTTP 200 OK) and degraded (HTTP 503 Service Unavailable) states.

#### Scenario: All dependencies ready response
- **WHEN** `GET /health/ready` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `status` equal to `"ready"`, `service` equal to `"forge-platform"`, ISO 8601 UTC `timestamp`, and `checks` object containing valid status records for `database`, `job_queue`, and `container_runtime`.

#### Scenario: Dependency failure or degraded readiness response
- **WHEN** `GET /health/ready` returns HTTP status code 503
- **THEN** tests assert Content-Type is `application/json`, response contains `status` equal to `"not_ready"`, `service` equal to `"forge-platform"`, ISO 8601 UTC `timestamp`, and `checks` detailing the unavailable dependency.

### Requirement: Deep Health Diagnostic Response Validation
The system SHALL attach automated test assertion scripts to the `GET /health/deep` collection request to validate deep diagnostic responses for authorized admin callers (HTTP 200 OK) and access rejection for unauthenticated (HTTP 401 Unauthorized) or forbidden (HTTP 403 Forbidden) callers.

#### Scenario: Authorized administrator deep diagnostic retrieval
- **WHEN** `GET /health/deep` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `status` equal to `"healthy"`, `service` equal to `"forge-platform"`, non-empty string `version`, non-empty string `environment`, non-negative integer `uptime_seconds`, and ISO 8601 UTC `timestamp`.

#### Scenario: Unauthenticated deep diagnostic access rejection
- **WHEN** `GET /health/deep` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is boolean `true`, string `code` indicates authentication failure, and `message` describes the failure.

#### Scenario: Forbidden non-admin deep diagnostic access rejection
- **WHEN** `GET /health/deep` returns HTTP status code 403
- **THEN** tests assert Content-Type is `application/json`, `is_error` is boolean `true`, string `code` indicates authorization denial, and `message` describes forbidden access.

### Requirement: Standardized Error Schema Validation Across Health Endpoints
When a health endpoint returns an HTTP 4xx or 5xx status code (e.g., unauthorized access or service unavailability), the test assertion scripts SHALL verify that the response body conforms to the standardized `ErrorResponse` schema unless specified otherwise by the endpoint's contract.

#### Scenario: Standard error response schema verification
- **WHEN** a health endpoint request returns an HTTP 4xx or 5xx status code containing an error payload
- **THEN** tests assert Content-Type is `application/json`, `is_error` is boolean `true`, `code` is a non-empty string, and `message` is a non-empty string.
