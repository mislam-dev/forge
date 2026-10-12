# Tasks

## 1. Access Control Module

- [x] 1.1 Update `src/modules/access_control/user_roles/` (`service.rs`, `repository.rs`) to import `codes` at top and use `codes::access::*`, verifying via `cargo check`
- [x] 1.2 Update `src/modules/access_control/roles/` (`service.rs`, `repository.rs`) to import `codes` at top and use `codes::access::*`, verifying via `cargo check`
- [x] 1.3 Update `src/modules/access_control/permissions/` (`service.rs`, `repository.rs`) to import `codes` at top and use `codes::access::*`, verifying via `cargo check`
- [x] 1.4 Update `src/modules/access_control/role_permissions/` (`service.rs`, `repository.rs`) to import `codes` at top and use `codes::access::*`, verifying via `cargo check`
- [x] 1.5 Update `src/modules/access_control/user_permissions/` (`service.rs`, `repository.rs`) to import `codes` at top and use `codes::access::*`, verifying via `cargo check`

## 2. Organization Module

- [x] 2.1 Update `src/modules/organization/members/service.rs` to import `codes` at top and use `codes::organization::*` and `codes::system::*`, verifying via `cargo check`

## 3. Users Module

- [x] 3.1 Update `src/modules/users/service.rs`, `repository.rs`, and `password.rs` to import `codes` at top and replace inline paths with `codes::users::*` / `codes::auth::*`, verifying via `cargo check`
- [x] 3.2 Update `src/modules/users/profile/handlers.rs` and `src/modules/users/profile/service.rs` to import `codes` at top and use `codes::access::*` and `codes::users::*`, verifying via `cargo check`

## 4. Auth Module

- [x] 4.1 Update `src/modules/auth/token/password_reset.rs`, `src/modules/auth/token/auth/refresh_token.rs`, and `src/modules/auth/token/auth/access_token.rs` to import `codes` at top and use `codes::auth::*`, verifying via `cargo check`
- [x] 4.2 Update `src/modules/auth/repository/refresh_token_repository.rs` and `src/modules/auth/policies/base_policy.rs` to import `codes` at top and use `codes::auth::*` / `codes::access::*`, verifying via `cargo check`

## 5. Projects Module

- [x] 5.1 Update `src/modules/projects/projects/service.rs`, `src/modules/projects/permissions/service.rs`, and `src/modules/projects/extractors/organization_validation.rs` to import `codes` at top and use `codes::projects::*` / `codes::organization::*`, verifying via `cargo check`

## 6. Verification and Clean Build

- [x] 6.1 Confirm no remaining inline `crate::shared::error::codes` references remain across `src/` via `grep -rn "crate::shared::error::codes" src/`
- [x] 6.2 Run full test suite with `cargo test --lib` to ensure all 273+ unit tests pass without warnings or errors
