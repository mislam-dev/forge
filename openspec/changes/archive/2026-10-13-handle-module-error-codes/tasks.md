# Tasks

## 1. Shared Error Infrastructure

- [x] 1.1 Create `src/shared/error/codes.rs` defining error code constants grouped by domain namespace (`AUTH_`, `ACCESS_`, `ORG_`, `ORG_PERM_`, `PRJ_`, `PRJ_PERM_`, `REPO_`, `ENV_`, `DEPLOY_`, `WORKER_`, `LOG_`, `HIST_`, `USER_`, `PROFILE_`, `TEAM_`, `NOTIF_`, `HLTH_`, `DASH_`, `VALIDATION_ERROR`) and re-export in `src/shared/error/mod.rs`. Verify by compiling module exports.
- [x] 1.2 Implement ergonomic constructor helper methods on `AppError` (`not_found`, `bad_request`, `unauthorized`, `forbidden`, `conflict`, `validation`, `bad_request_with_errors`) and update unit tests in `src/shared/error/error.rs`. Verify with `cargo test --lib shared::error`.

## 2. Auth, Access Control, Users, and Organizations

- [x] 2.1 Update error handling and call sites in `src/modules/auth/` to use documented `AUTH_000` through `AUTH_009` error codes. Verify with `cargo check --lib`.
- [x] 2.2 Update error handling and call sites in `src/modules/access_control/` (roles, permissions, role_permissions, user_roles, user_permissions) to use documented `ACCESS_` error codes. Verify with `cargo check --lib`.
- [x] 2.3 Update error handling and call sites in `src/modules/users/` (users, user profile) to use documented `USER_` and `PROFILE_` error codes. Verify with `cargo check --lib`.
- [x] 2.4 Update error handling and call sites in `src/modules/organization/` (organizations, members, permissions) to use documented `ORG_` and `ORG_PERM_` error codes. Verify with `cargo check --lib`.

## 3. Projects, Deployments, and Peripheral Modules

- [x] 3.1 Update error handling and call sites in `src/modules/projects/` (project CRUD, environment variables, repository, files, assignments, project permissions) to use documented `PRJ_`, `PRJ_FILE_`, `PRJ_PERM_`, `REPO_`, and `ENV_` error codes. Verify with `cargo check --lib`.
- [x] 3.2 Update error handling and call sites in `src/modules/deployments/` (deployments, history, live_build_logs, build_worker) to use documented `DEPLOY_`, `HIST_`, `LOG_`, and `WORKER_` error codes. Verify with `cargo check --lib`.
- [x] 3.3 Update error handling and call sites in `src/modules/teams/`, `src/modules/notifications/`, `src/modules/health/`, and `src/modules/dashboard/` to use documented `TEAM_`, `NOTIF_`, `HLTH_`, and `DASH_` error codes. Verify with `cargo check --lib`.

## 4. Full Verification and Integration

- [x] 4.1 Run full project compilation `cargo check` and resolve any remaining compile-time errors or warnings. Verify zero compilation errors.
- [x] 4.2 Run entire test suite `cargo test` and verify that all error-related unit and integration tests pass cleanly.
