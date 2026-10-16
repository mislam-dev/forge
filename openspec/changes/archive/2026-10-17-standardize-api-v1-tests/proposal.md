# Proposal

## Why

The Forge Axum application mounts all domain service routers under the `/api/v1` namespace in `src/app/app.rs`. However, several Rust integration test suites (`auth_tests.rs`, `teams_tests.rs`, `notifications_tests.rs`, `dashboard_tests.rs`, `health_tests.rs`, and `user_profile_tests.rs`) were written against legacy unversioned `/api/` paths, causing integration tests to fail with 404 Not Found.

Rather than cluttering the production router with deprecated backward-compatibility route aliases (as initially outlined in task T-003), this change centralizes the API version prefix into a single source of truth (`pub const API_V1_PATH: &str = "/api/v1";`) in `src/app/app.rs`, standardizes all integration tests to target this canonical common base path directly, and updates task T-003 in `docs/03-TASKS.md` to reflect this clean architecture.

## What Changes

- **Centralize Common API Base Path**: Define and export `pub const API_V1_PATH: &str = "/api/v1";` in `src/app/app.rs` and nest the router via this constant.
- **Update Authentication Tests**: Refactor all endpoint URIs in `tests/auth_tests.rs` from `/api/auth/...` to use the common base path `/api/v1/auth/...` across register, login, logout, refresh, me, password reset, and email verification endpoints.
- **Update Teams Tests**: Refactor all endpoint URIs in `tests/teams_tests.rs` from `/api/teams/...` to use the common base path `/api/v1/teams/...` across team CRUD and team membership endpoints.
- **Update Notifications Tests**: Refactor all endpoint URIs in `tests/notifications_tests.rs` from `/api/notifications/...` to use the common base path `/api/v1/notifications/...` across list, unread count, read single, and read all endpoints.
- **Update Dashboard Tests**: Refactor all endpoint URIs in `tests/dashboard_tests.rs` from `/api/dashboard/...` to use the common base path `/api/v1/dashboard/...` across summary, user metrics, and organization metrics endpoints.
- **Update Health Tests**: Refactor API probe URIs in `tests/health_tests.rs` from `/api/health/...` to use the common base path `/api/v1/health/...`.
- **Fix User Profile Test Route**: Correct the legacy unversioned route `/api/users/{}/profile` at line 157 in `tests/user_profile_tests.rs` to `/api/v1/users/{}/profile`.
- **Align Task T-003 Documentation**: Update task T-003 in `docs/03-TASKS.md` to replace the route aliasing approach with canonical `/api/v1` test standardization using a common base path, removing unnecessary backward-compatibility shims.

## Capabilities

### New Capabilities
- `api-v1-integration-tests`: Defines requirements and verification criteria for Rust Axum integration test suites targeting the canonical common base path `/api/v1` route hierarchy directly without backward-compatibility route aliases.

### Modified Capabilities
<!-- None: No existing specs change requirements. -->

## Impact

- **Production Code**: `src/app/app.rs` exports `API_V1_PATH` and nests routes via this constant.
- **Test Suites**: `tests/auth_tests.rs`, `tests/teams_tests.rs`, `tests/notifications_tests.rs`, `tests/dashboard_tests.rs`, `tests/health_tests.rs`, `tests/user_profile_tests.rs`.
- **Documentation**: `docs/03-TASKS.md` (task T-003 specification).
