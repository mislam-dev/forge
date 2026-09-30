# Spec Delta

## Purpose

Provides standardized operational health endpoints including lightweight liveness probes, dependency readiness probes, and authenticated deep diagnostic checks.

## ADDED Requirements

### Requirement: Liveness Probe Endpoint
The system SHALL provide an unauthenticated endpoint `GET /health/live` that checks whether the application process is alive. The endpoint SHALL return HTTP status 200 with a JSON payload containing `status` (`"healthy"`), `service` (`"forge-platform"`), `version`, `environment`, and ISO 8601 UTC `timestamp`.

#### Scenario: Successful liveness probe check
- **WHEN** an unauthenticated client or load balancer sends a `GET` request to `/health/live`
- **THEN** the system returns HTTP 200 with JSON indicating `status` is `"healthy"`, `service` is `"forge-platform"`, and current `timestamp`

### Requirement: Readiness Probe Endpoint
The system SHALL provide an unauthenticated endpoint `GET /health/ready` that checks the availability of critical dependencies before routing traffic. The endpoint SHALL report dependency status for `database`, `job_queue`, and `container_runtime`. When all critical dependencies are ready, the system SHALL return HTTP status 200 with `status: "ready"` and per-dependency check details (including `status` and `latency_ms`). When one or more critical dependencies are unhealthy, the system SHALL return HTTP status 503 with `status: "not_ready"`.

#### Scenario: All dependencies ready returns HTTP 200
- **WHEN** an unauthenticated client or orchestrator sends a `GET` request to `/health/ready` and all dependencies are operational
- **THEN** the system returns HTTP 200 with JSON containing `status: "ready"`, `service: "forge-platform"`, `timestamp`, and individual dependency checks for `database`, `job_queue`, and `container_runtime`

#### Scenario: Unhealthy dependency returns HTTP 503
- **WHEN** an unauthenticated client or orchestrator sends a `GET` request to `/health/ready` and a critical dependency is unavailable
- **THEN** the system returns HTTP 503 with JSON containing `status: "not_ready"`, `service: "forge-platform"`, and details of the failed check

### Requirement: Deep Operational Health Check Endpoint
The system SHALL provide an authenticated endpoint `GET /health/deep` restricted to System Admin users via Bearer token authentication. The endpoint SHALL support an optional integer query parameter `timeout_ms` (defaulting to 3000). The endpoint SHALL return HTTP status 200 with an aggregated diagnostic report containing `status` (`"healthy"`), `service` (`"forge-platform"`), `version`, `environment`, `uptime_seconds`, and ISO 8601 UTC `timestamp`. If the request is unauthenticated or the authenticated user lacks administrative roles (`admin` or `system_admin`), the system SHALL reject the request with HTTP status 401 Unauthorized or HTTP status 403 Forbidden.

#### Scenario: System admin retrieves deep health check
- **WHEN** a user authenticated with admin or system_admin role sends a `GET` request to `/health/deep` with optional `timeout_ms`
- **THEN** the system returns HTTP 200 with JSON containing deep diagnostic report details including `uptime_seconds` and `timestamp`

#### Scenario: Unauthenticated request to deep health check is rejected
- **WHEN** an unauthenticated client sends a `GET` request to `/health/deep`
- **THEN** the system returns HTTP 401 Unauthorized

#### Scenario: Non-admin authenticated user request is rejected
- **WHEN** a user authenticated without administrative privileges sends a `GET` request to `/health/deep`
- **THEN** the system returns HTTP 403 Forbidden
