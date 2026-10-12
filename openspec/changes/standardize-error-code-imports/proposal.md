# Proposal

## Why

Currently, several services and repositories across modules such as `access_control`, `organization`, `users`, `auth`, and parts of `projects` reference error codes using verbose inline paths like `crate::shared::error::codes::<domain>::<CODE>` deep inside function bodies. In contrast, modern modules (such as `projects`, `teams`, `health`, `dashboard`, `deployments`, and `notifications`) import error codes at the top of the file (e.g. `use crate::shared::error::codes;` or `use crate::shared::error::codes::<domain>::*;`), improving readability, consistency, and maintainability across the codebase.

## What Changes

- Standardize top-level error code imports across all files currently using inline fully-qualified paths:
  - Add `use crate::shared::error::codes;` (or domain-scoped error codes alongside `AppError`) to module header imports.
  - Replace inline `crate::shared::error::codes::<domain>::<CODE>` references in function bodies with concise `codes::<domain>::<CODE>` (or direct `<CODE>` where domain is uniquely imported).
- Audit and clean up all modules:
  - `src/modules/access_control/` (`user_roles`, `roles`, `permissions`, `role_permissions`, `user_permissions` services & repositories)
  - `src/modules/organization/` (`members/service.rs`)
  - `src/modules/users/` (`service.rs`, `repository.rs`, `password.rs`, `profile/handlers.rs`, `profile/service.rs`)
  - `src/modules/auth/` (`token/password_reset.rs`, `token/auth/refresh_token.rs`, `token/auth/access_token.rs`, `repository/refresh_token_repository.rs`, `policies/base_policy.rs`)
  - `src/modules/projects/` (`projects/service.rs`, `permissions/service.rs`, `extractors/organization_validation.rs`)
- Preserve exact existing error codes, HTTP statuses, and error message semantics without altering external API behaviors.

## Capabilities

### New Capabilities
None. This is an internal architectural refactoring to enforce import consistency and clean code style across modules.

### Modified Capabilities
None. No externally visible behavior, validation rules, or API error contracts are modified.

## Impact

- **Affected Code**: Rust files in `src/modules/access_control/`, `src/modules/organization/`, `src/modules/users/`, `src/modules/auth/`, and `src/modules/projects/`.
- **APIs & Dependencies**: No breaking API changes, no database schema changes, and no dependency additions.
- **Verification**: Verified using `cargo check` and `cargo test --lib`.
