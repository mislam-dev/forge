# Tasks

## 1. Health Probe DTOs

- [x] 1.1 Define probe response models (`LivenessProbeResponse`, `ReadinessProbeResponse`, `ReadinessChecks`, `DependencyCheck`, `DeepHealthResponse`, `DeepHealthQuery`) in `src/modules/health/dto/response.rs` and verify serialization via unit tests.
- [x] 1.2 Re-export new DTO models in `src/modules/health/dto/mod.rs` and verify module exports compile cleanly.

## 2. Handler Implementation

- [x] 2.1 Implement `health_liveness_probe` handler in `src/modules/health/handlers.rs` returning static healthy status, service name, version, environment, and timestamp with HTTP 200.
- [x] 2.2 Implement `health_readiness_probe` handler in `src/modules/health/handlers.rs` returning static dependency readiness for database, job_queue, and container_runtime with HTTP 200.
- [x] 2.3 Implement `health_deep_check` handler in `src/modules/health/handlers.rs` with `JwtClaims` validation, system admin / admin authorization, optional `timeout_ms` query extraction, and static deep diagnostic payload.

## 3. Router Registration and Verification

- [x] 3.1 Mount `/live`, `/ready`, and `/deep` routes in `src/modules/health/router.rs` and ensure backward compatibility with existing `/` and `/details` routes.
- [x] 3.2 Add integration and unit tests covering `GET /health/live`, `GET /health/ready`, and `GET /health/deep` (validating unauthenticated probe access, 401 unauthorized on deep check without token, 403 forbidden for non-admin user, and 200 OK for system admin) and verify with `cargo test`.
