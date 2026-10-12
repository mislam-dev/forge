# Design

## Context

See `proposal.md` for motivation. Currently, `src/shared/error/app_error_data.rs` and `src/shared/error/error.rs` have been updated so `AppError` variants carry structured error codes (`code`, `message`). However, because `AppError` variants changed from single-tuple to two-tuple variants, existing call sites across all modules in `src/modules/` fail compilation. Furthermore, `docs/` defines comprehensive error code registries for each module (e.g., `AUTH_`, `ACCESS_`, `ORG_`, `PRJ_`, `ENV_`, `DEPLOY_`, `WORKER_`, etc.) that need to be consistently referenced.

## Goals / Non-Goals

**Goals:**
- Define a centralized error code catalog (`src/shared/error/codes.rs`) containing string constants for all documented error codes organized by domain module.
- Add ergonomic constructors/helpers on `AppError` to streamline error creation across the codebase without redundant string conversions.
- Update all call sites across `src/modules/` to pass the correct error codes and messages documented in `docs/`.
- Fix unit tests in `src/shared/error/error.rs` to reflect the updated error data structure.
- Ensure the entire codebase compiles cleanly (`cargo check`) and all unit tests pass (`cargo test`).

**Non-Goals:**
- Modifying business workflows, database schemas, or API routing.
- Changing third-party crate dependencies or logging infrastructure.

## Decisions

### Decision 1: Centralized Error Code Registry (`src/shared/error/codes.rs`)
- **Choice**: Create `src/shared/error/codes.rs` exporting namespaces containing `&'static str` constants (e.g., `codes::auth::AUTH_001`, `codes::org::ORG_001`, `codes::projects::PRJ_001`, `codes::validation::VALIDATION_ERROR`).
- **Rationale**: Using centralized constants guarantees strict alignment with `docs/`, prevents typographical errors, and facilitates IDE auto-completion.
- **Alternatives considered**:
  - *Inline string literals at every call site*: Error-prone, hard to audit against documentation, and likely to cause discrepancies.
  - *A mega-enum with 100+ variants*: Over-complicates `thiserror` mapping and makes Axum status code mapping cumbersome.

### Decision 2: Ergonomic Helper Methods on `AppError`
- **Choice**: Provide helper associated functions on `AppError`, accepting `impl Into<String>`:
  - `AppError::not_found(code, message)`
  - `AppError::bad_request(code, message)`
  - `AppError::unauthorized(code, message)`
  - `AppError::forbidden(code, message)`
  - `AppError::conflict(code, message)`
  - `AppError::validation(code, errors)`
- **Rationale**: Simplifies converting legacy call sites, avoids writing `.to_string()` repeatedly, and keeps service/repository code clean.

### Decision 3: Error Mapping by Module
- Systematically map module error call sites to their documented error codes:
  - **Auth (`docs/modules/auth/`)**: `AUTH_000` through `AUTH_009`.
  - **Access Control (`docs/modules/auth/access-control/`)**: `ACCESS_001` - `ACCESS_004` (Roles), `ACCESS_101` - `ACCESS_104` (Permissions), `ACCESS_201` - `ACCESS_203` (RolePermissions), `ACCESS_301` - `ACCESS_304` (UserRoles), `ACCESS_401` - `ACCESS_403` (UserPermissions).
  - **Organizations (`docs/modules/organization/`)**: `ORG_001` - `ORG_007`, `ORG_PERM_001` - `ORG_PERM_002`.
  - **Projects (`docs/modules/projects/`)**: `PRJ_001` - `PRJ_006`, `PRJ_PERM_001` - `PRJ_PERM_003`, `PRJ_FILE_001` - `PRJ_FILE_004`, `REPO_001` - `REPO_005`, `ENV_001` - `ENV_005`.
  - **Deployments & Workers (`docs/modules/deployments/`)**: `DEPLOY_001` - `DEPLOY_005`, `WORKER_001` - `WORKER_007`, `LOG_001` - `LOG_004`, `HIST_001` - `HIST_005`.
  - **Users & Profiles (`docs/modules/users/`)**: `USER_001` - `USER_006`, `PROFILE_001` - `PROFILE_004`.
  - **Teams (`docs/modules/teams/`)**: `TEAM_001` - `TEAM_005`.
  - **Notifications (`docs/modules/notifications/`)**: `NOTIF_001` - `NOTIF_005`.
  - **Health & Dashboard (`docs/modules/health/`, `docs/modules/dashboard/`)**: `HLTH_001` - `HLTH_003`, `DASH_001` - `DASH_003`.

## Risks / Trade-offs

- **[Risk] High number of affected files across all modules**
  → *Mitigation*: Group updates logically by module domain and run iterative compilation checks with `cargo check` after each module is updated.
- **[Risk] Breaking API response contract for existing frontend/clients**
  → *Mitigation*: The structured response matches the standard documented response envelope (`is_error`, `code`, `message`, `errors`), ensuring client predictability.
