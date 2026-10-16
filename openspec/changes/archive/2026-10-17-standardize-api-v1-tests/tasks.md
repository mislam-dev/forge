# Tasks

## 1. Centralized Base Path & Authentication Tests

- [x] 1.1 Export `pub const API_V1_PATH: &str = "/api/v1";` in `src/app/app.rs`, refactor router to `.nest(API_V1_PATH, root_app_router)`, and verify project builds with `cargo check`
- [x] 1.2 Refactor all test URIs in `tests/auth_tests.rs` from `/api/auth/` to `/api/v1/auth/` across all 12 endpoints (me, register, login, logout, refresh, forgot-password, reset-password, verify-email) and verify execution with `cargo test --test auth_tests`

## 2. Teams Integration Tests

- [x] 2.1 Refactor all test URIs in `tests/teams_tests.rs` from `/api/teams` to `/api/v1/teams` across team CRUD and membership operations, and verify execution with `cargo test --test teams_tests`

## 3. Notifications & Dashboard Integration Tests

- [x] 3.1 Refactor all test URIs in `tests/notifications_tests.rs` from `/api/notifications` to `/api/v1/notifications` across list, unread count, read single, and read all endpoints, and verify execution with `cargo test --test notifications_tests`
- [x] 3.2 Refactor all test URIs in `tests/dashboard_tests.rs` from `/api/dashboard` to `/api/v1/dashboard` across summary, user metrics, and org metrics endpoints, and verify execution with `cargo test --test dashboard_tests`

## 4. Health & User Profile Integration Tests

- [x] 4.1 Refactor API health probe URIs in `tests/health_tests.rs` from `/api/health/` to `/api/v1/health/` and verify execution with `cargo test --test health_tests`
- [x] 4.2 Update legacy URI at line 157 in `tests/user_profile_tests.rs` from `/api/users/{}/profile` to `/api/v1/users/{}/profile` and verify execution with `cargo test --test user_profile_tests`

## 5. Documentation & Regression Verification

- [x] 5.1 Update task T-003 in `docs/03-TASKS.md` to document direct `/api/v1` test standardization with common base path rather than production route aliasing in `src/app/app.rs`, and verify markdown consistency
- [x] 5.2 Execute complete test suite regression check across all updated targets using `cargo test --test auth_tests --test teams_tests --test notifications_tests --test dashboard_tests --test user_profile_tests`
