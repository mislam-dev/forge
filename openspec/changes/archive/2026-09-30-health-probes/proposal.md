# Proposal

## Why

The platform requires operational health endpoints matching the OpenAPI specification (`/health/live`, `/health/ready`, and `/health/deep`). These endpoints allow load balancers, container orchestrators (such as Kubernetes), and system administrators to monitor application liveness, check dependency readiness before routing traffic, and perform authenticated deep operational diagnostic checks.

## What Changes

- Implement `GET /health/live` (Liveness probe): Lightweight, unauthenticated probe returning process status, service name, version, environment, and ISO 8601 timestamp.
- Implement `GET /health/ready` (Readiness probe): Dependency readiness probe checking critical components (`database`, `job_queue`, `container_runtime`) returning status 200 when ready or 503 when critical dependencies fail. Initially populated with static data.
- Implement `GET /health/deep` (Deep check): Authenticated operational check restricted to System Admin via bearer JWT token, supporting an optional `timeout_ms` query parameter (default 3000ms), returning full operational and uptime health details.
- Define request/response DTOs conforming to the OpenAPI specification.
- Integrate the routes into the health module router mounted under `/health`, `/api/health`, and `/api/v1/health`.

## Capabilities

### New Capabilities
- `health-probes`: Health probe and diagnostic endpoints (`/health/live`, `/health/ready`, `/health/deep`) for process liveness, dependency readiness, and authenticated administrative deep health inspection.

### Modified Capabilities
<!-- None -->

## Impact

- **Affected code**: `src/modules/health/` (`router.rs`, `handlers.rs`, `dto/`, `service.rs`).
- **Dependencies**: No external crate changes required; utilizes existing Axum, Serde, and JWT authentication extractors.
- **API**: Exposes new endpoints under existing health router mount paths (`/health/live`, `/health/ready`, `/health/deep`).
