# Design

## Context

In earlier iterations of the codebase, error codes were referenced in some modules using fully-qualified inline paths (e.g. `crate::shared::error::codes::access::ACCESS_001`) inside service and repository methods. Meanwhile, newer and recently updated modules (such as `teams`, `deployments`, `health`, `dashboard`, and `notifications`) import `codes` directly at the top of each file via `use crate::shared::error::{codes, AppError};` and reference codes as `codes::<domain>::<CONST>`.

See [proposal.md](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/openspec/changes/standardize-error-code-imports/proposal.md) for background and motivation.

## Goals / Non-Goals

**Goals:**
- Enforce the idiomatic top-level import pattern `use crate::shared::error::{codes, AppError};` across all files currently using inline fully-qualified paths.
- Replace all verbose `crate::shared::error::codes::<domain>::<CONST>` references in function bodies with concise `codes::<domain>::<CONST>`.
- Keep error code references consistent across `access_control`, `organization`, `users`, `auth`, and `projects`.
- Ensure zero regression in test suite or compilation errors.

**Non-Goals:**
- Changing existing error code values, HTTP status codes, or error response formats.
- Modifying business logic, database queries, or route signatures.
- Introducing new error code types or renaming existing constants in `src/shared/error/codes.rs`.

## Decisions

### Decision 1: Standard Import Form `use crate::shared::error::{codes, AppError};`
- **Choice**: Use `use crate::shared::error::{codes, AppError};` at the top of files that construct `AppError` instances with error codes.
- **Rationale**: Keeps imports grouped logically, avoids redundant lines, and makes both `codes` and `AppError` available throughout the file.
- **Alternative Considered**: Glob import `use crate::shared::error::codes::<domain>::*;`. Rejected because importing raw constants like `BAD_REQUEST`, `NOT_FOUND`, or numeric constants could conflict with local variable names or constants from other crates, and loses domain context (`codes::access::...` is immediately self-documenting).

### Decision 2: Referencing Error Codes via `codes::<domain>::<CODE>`
- **Choice**: In function bodies, use `codes::<domain>::<CODE>` (e.g., `codes::access::ACCESS_002`, `codes::auth::AUTH_001`, `codes::organization::ORG_001`, `codes::users::USER_001`, `codes::projects::PRJ_001`).
- **Rationale**: Provides clear namespace provenance without polluting file-level scope, while eliminating 27 redundant characters per error code reference (`crate::shared::error::`).

## Risks / Trade-offs

- **[Risk] Unused import warnings if files already import `AppError` without `codes` or vice versa**
  → *Mitigation*: Consolidate `use crate::shared::error::AppError;` into `use crate::shared::error::{codes, AppError};` only in files that actively use error codes. Run `cargo check` to catch any unused imports.
- **[Risk] Accidental typo in error code identifier during refactoring**
  → *Mitigation*: Only strip the `crate::shared::error::` prefix, retaining exact domain and identifier names; verify with `cargo check` and `cargo test --lib`.
