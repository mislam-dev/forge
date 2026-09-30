# Design

## Context

The Forge backend is built with Rust and Axum (`axum 0.8`). The routing architecture mounts modular sub-routers inside `src/app/app.rs`. The health module (`src/modules/health/`) already exports a `health_router()` mounted at `/health`, `/api/health`, and `/api/v1/health`.

The OpenAPI specification (`docs/system/05-api/openapi.yaml`) defines three specific probe endpoints:
- `GET /health/live` (Liveness probe: public, status 200)
- `GET /health/ready` (Readiness probe: public, status 200 or 503 depending on dependency health)
- `GET /health/deep` (Deep check: bearerAuth JWT required, System Admin role, optional query parameter `timeout_ms`)

The user requested workable implementations using static data for now, while ensuring the endpoints are properly structured and adhere to the API contract.

## Goals / Non-Goals

**Goals:**
- Implement route definitions for `/live`, `/ready`, and `/deep` within `src/modules/health/router.rs`.
- Define Serde serializable and deserializable response models matching the OpenAPI schemas and examples.
- Return HTTP 200 with static healthy data for `/live` and `/ready`.
- Protect `/deep` using existing JWT authentication (`JwtClaims`) and ensure only users with `admin` or `system_admin` roles can retrieve the deep health report.
- Support optional `timeout_ms` (default 3000) query extraction on `GET /health/deep`.
- Preserve existing health endpoints (`/` and `/details`) to prevent regressions.

**Non-Goals:**
- Dynamic live pinging or querying of PostgreSQL, RabbitMQ, or Docker runtime engines (these are planned for future iterations once dependency health checkers are unified).
- Changing OpenAPI specification contracts or modifying OpenAPI YAML.

## Decisions

### 1. Route Mounting within `health_router()`
- **Decision**: Mount `/live`, `/ready`, and `/deep` in `src/modules/health/router.rs`.
- **Rationale**: Since `src/app/app.rs` nests `health_router()` at `/health`, `/api/health`, and `/api/v1/health`, routes registered as `/live`, `/ready`, and `/deep` automatically become accessible across all base paths as defined in OpenAPI.
- **Alternatives Considered**: Mounting directly in `src/app/app.rs`. Rejected because modular domain boundaries in Forge place route handlers inside their respective module folders.

### 2. Response Serialization Format
- **Decision**: Return direct `axum::Json<T>` responses for the probe endpoints rather than wrapping in `ApiResponse<T>`.
- **Rationale**: Standard infrastructure probes (Kubernetes, AWS ALB, Envoy) and the OpenAPI specification require root-level fields (`status`, `service`, `version`, `checks`, etc.) without custom envelope wrappers (`message`, `data`).
- **Alternatives Considered**: Using `ApiResponse<T>`. Rejected because it would introduce extraneous `{ message, data }` envelopes that violate the published OpenAPI spec.

### 3. Static Payload Modeling
- **Decision**: Define strongly-typed DTOs for each probe:
  - `LivenessProbeResponse`: `{ status, service, version, environment, timestamp }`
  - `ReadinessProbeResponse`: `{ status, service, timestamp, checks: { database, job_queue, container_runtime } }`
  - `DependencyCheck`: `{ status, latency_ms, message }`
  - `DeepHealthResponse`: `{ status, service, version, environment, uptime_seconds, timestamp }`
  - `DeepHealthQuery`: `{ timeout_ms: Option<u64> }`
- **Rationale**: Strongly typed models allow easy transition from static responses to real diagnostic probes in the future without altering route handlers or response serialization.

### 4. Authentication and Authorization on `/health/deep`
- **Decision**: Extract `claims: JwtClaims` on the handler and verify the caller possesses `admin` or `system_admin` in `claims.roles`.
- **Rationale**: Keeps consistency with `check_health_details` in `src/modules/health/handlers.rs` while returning standard `AppError::Forbidden` or `AppError::Unauthorized`.

## Risks / Trade-offs

- **[Risk] Static checks may mask real downtime in staging/production until real dependency checks are hooked up.**  
  → *Mitigation*: Clearly document that static data is a temporary milestone satisfying the API contract; real service pings will replace static checks in a subsequent change.
- **[Risk] Path collision if `/` or `/details` overlap.**  
  → *Mitigation*: `/live`, `/ready`, and `/deep` are distinct path segments; existing `/` and `/details` routes remain intact.
