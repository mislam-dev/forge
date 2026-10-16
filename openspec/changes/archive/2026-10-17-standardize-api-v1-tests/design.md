# Design

## Context

The Forge Axum backend mounts all modular service routers under `/api/v1` in `src/app/app.rs`. While several integration test suites (such as `access_control_tests.rs`, `projects_tests.rs`, `organization_tests.rs`, `deployments_tests.rs`, and `repositories_tests.rs`) already test against `/api/v1/...`, others were written against unversioned paths like `/api/auth/...`, `/api/teams/...`, `/api/notifications/...`, `/api/dashboard/...`, `/api/health/...`, and `/api/users/...`.

In `docs/03-TASKS.md`, Task T-003 originally suggested re-introducing route aliases (`.nest("/api/auth", auth_router())`, etc.) in `src/app/app.rs` for backward compatibility. However, preserving or re-introducing unversioned aliases adds unnecessary routing bloat and violates clean API versioning. Standardizing test URIs directly to `/api/v1` and establishing a common base path constant aligns all test suites with the production routing contract while remaining adaptable for future API version bumps.

## Goals / Non-Goals

**Goals:**
- Centralize the API version prefix into a single exported constant (`pub const API_V1_PATH: &str = "/api/v1";`) in `src/app/app.rs`.
- Update all integration test suites that currently invoke unversioned `/api/` paths to use the canonical `/api/v1/` endpoints.
- Ensure all targeted integration test targets (`auth_tests`, `teams_tests`, `notifications_tests`, `dashboard_tests`, `health_tests`, and `user_profile_tests`) execute cleanly and assert against `/api/v1` routes.
- Update `docs/03-TASKS.md` (task T-003) to reflect the standardized `/api/v1` approach and common base path rather than server route aliasing.

**Non-Goals:**
- Adding backward-compatibility route aliases or URL rewrites in `src/app/app.rs`.
- Changing OpenAPI 3.0 server definitions (which already specify `/api/v1` as primary).
- Modifying Postman collections (which already leverage `{{baseUrl}}`).

## Decisions

### Decision 1: Direct Test URI Standardization vs. Production Router Aliasing
- **Choice**: Directly update test assertions and request URIs in the integration test files to `/api/v1/...`.
- **Rationale**: The backend canonical prefix is `/api/v1`. Route aliasing in `src/app/app.rs` would introduce duplicate routing tables, increased binary size, and technical debt. Since backward compatibility is explicitly not required, standardizing the tests is the architecturally sound approach.
- **Alternatives Considered**:
  - *Adding route aliases in `src/app/app.rs`*: Rejected because it introduces unnecessary legacy baggage and confuses endpoint contracts.
  - *Parameterizing tests to test both `/api` and `/api/v1`*: Rejected because unversioned endpoints should not exist in production.

### Decision 2: Specific Test File URI Refactoring
- **`tests/auth_tests.rs`**: Refactor 12 endpoint URIs from `/api/auth/...` to `/api/v1/auth/...` (e.g., `/api/v1/auth/me`, `/api/v1/auth/register`, `/api/v1/auth/login`, `/api/v1/auth/logout`, `/api/v1/auth/refresh`, `/api/v1/auth/forgot-password`, `/api/v1/auth/reset-password`, `/api/v1/auth/verify-email`).
- **`tests/teams_tests.rs`**: Refactor 10 endpoint URIs from `/api/teams/...` to `/api/v1/teams/...` (e.g., `/api/v1/teams`, `/api/v1/teams/{id}`, `/api/v1/teams/{id}/members`, `/api/v1/teams/{id}/members/{user_id}`).
- **`tests/notifications_tests.rs`**: Refactor 4 endpoint URIs from `/api/notifications/...` to `/api/v1/notifications/...` (e.g., `/api/v1/notifications`, `/api/v1/notifications/unread-count`, `/api/v1/notifications/{id}/read`, `/api/v1/notifications/read-all`).
- **`tests/dashboard_tests.rs`**: Refactor 3 endpoint URIs from `/api/dashboard/...` to `/api/v1/dashboard/...` (e.g., `/api/v1/dashboard`, `/api/v1/dashboard/user`, `/api/v1/dashboard/org/{id}`).
- **`tests/health_tests.rs`**: Refactor URIs from `/api/health/...` to `/api/v1/health/...`.
- **`tests/user_profile_tests.rs`**: Update line 157 from `/api/users/{}/profile` to `/api/v1/users/{}/profile`.

### Decision 3: Update `docs/03-TASKS.md` Task T-003
- **Choice**: Revise the task definition for T-003 in `docs/03-TASKS.md` from "Restore API Route Aliasing and Fix Failing Integration Tests" to "Standardize Integration Tests to `/api/v1` and Fix Routing Assertions".
- **Rationale**: Maintains accurate documentation and architectural alignment across all engineering task trackers and audit documents.

### Decision 4: Centralized Common API Base Path Constant
- **Choice**: Export `pub const API_V1_PATH: &str = "/api/v1";` in `src/app/app.rs` and mount routes using `Router::new().nest(API_V1_PATH, root_app_router)`.
- **Rationale**: Having a single defined constant for the API prefix makes it trivial to bump the API version or change the mounting prefix in the future without modifying every route registration. Tests can also reference this canonical prefix.
- **Alternatives Considered**:
  - *Hardcoding `"/api/v1"` in every file*: Rejected as inflexible.
  - *Dynamic runtime configuration of base path via environment variables*: Unnecessary complexity for compile-time route nesting in Axum.

## Risks / Trade-offs

- **[Risk] Existing external scripts or clients relying on `/api/*` without `/v1` fail**  
  → *Mitigation*: Backward compatibility is explicitly deprecated and unnecessary per project requirements; the official OpenAPI spec documents `/api/v1` as the primary API target.

- **[Risk] Health check probes in container orchestrators**  
  → *Mitigation*: Root health check endpoints (`/health`, `/health/live`, `/health/ready`) are managed by Task T-002 for Docker daemon probes; API-specific health checks correctly reside under `/api/v1/health`.
