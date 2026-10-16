# Spec Delta: Health Probes

## ADDED Requirements

### Requirement: Root and Versioned Health Check Routing
The system SHALL expose health check routes under both the unversioned root `/health` namespace and the canonical versioned `/api/v1/health` namespace. The unauthenticated probe endpoints `GET /health`, `GET /health/live`, and `GET /health/ready` SHALL be directly accessible without JWT authentication tokens under both route hierarchies.

#### Scenario: Unauthenticated root health check
- **WHEN** an unauthenticated client sends a `GET` request to `/health`
- **THEN** the system returns HTTP 200 (or HTTP 503 if critical dependencies fail) with the current health status response payload without requiring an `Authorization` header

#### Scenario: Root liveness probe access
- **WHEN** an orchestrator, container runtime, or client sends a `GET` request to `/health/live`
- **THEN** the system returns HTTP 200 with JSON indicating `status` is `"healthy"`, `service` is `"forge-platform"`, and current `timestamp`

#### Scenario: Root readiness probe access
- **WHEN** an unauthenticated client sends a `GET` request to `/health/ready`
- **THEN** the system returns the dependency readiness check status without requiring authentication
