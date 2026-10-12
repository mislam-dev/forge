# Proposal

## Why

The platform documentation defines standardized error codes and structured JSON error responses across all domains and modules (`AUTH_*`, `ORG_*`, `PRJ_*`, `ENV_*`, etc.), but previously the application errors in `src/shared/error` lacked error codes and returned free-form text. Now that `src/shared/error` has been refactored to require error codes on `AppError` variants, all module call sites and tests must be updated to emit the documented error codes and adhere to the standardized response envelope.

## What Changes

- **Standardized Error Code Registry**: Introduce a centralized constants or catalog module in `src/shared/error/codes.rs` defining documented error codes grouped by namespace (`AUTH_`, `ACCESS_`, `ORG_`, `PRJ_`, `ENV_`, `DEPLOY_`, `WORKER_`, `LOG_`, `HIST_`, `USER_`, `PROFILE_`, `TEAM_`, `NOTIF_`, `HLTH_`, `DASH_`, etc.) based on `docs/` specifications.
- **Enhanced `AppError` and `AppErrorData`**:
  - Complete and refine `AppError` constructors and formatting in `src/shared/error/` to ensure full alignment with documented 4xx/500 JSON envelopes (`is_error`, `code`, `message`, `errors`).
  - Fix and expand unit tests in `src/shared/error/error.rs` to validate status codes, error codes, and serialization formats.
- **Module Error Handling Alignment**:
  - **BREAKING**: Update all error instantiation sites across `src/modules/**` (Access Control, Auth, Dashboard, Deployments, Health, Notifications, Organizations, Projects, Teams, Users) from legacy single-string variants to code-and-message variants matching their respective module error code tables in `docs/modules/` and `docs/system/05-api/api-documentation.md`.
  - Fix any validation and extractor error responses to return `VALIDATION_ERROR` or appropriate codes.
  - Restore clean project compilation (`cargo check`) and test suite passing (`cargo test`).

## Capabilities

### New Capabilities
- `error-handling`: Standardized application error codes, structured error envelope serialization, and module-level error code mapping according to system and module documentation.

### Modified Capabilities
None.

## Impact

- **Affected Code**: `src/shared/error/` (`app_error_data.rs`, `error.rs`, `mod.rs`, new `codes.rs`), and services/repositories/handlers across `src/modules/*` that instantiate `AppError`.
- **APIs**: All client error responses (400, 401, 403, 404, 409) and 500 internal errors will now reliably emit structured JSON bodies with explicit `code` strings matching the API documentation.
- **Dependencies**: None. Leverages existing `thiserror`, `serde`, `serde_json`, `axum`, and `validator`.
