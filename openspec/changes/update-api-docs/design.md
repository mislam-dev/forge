# Design

## Context

Recent changes in the Forge repository updated the data transfer objects (DTOs), active enums, handler semantics, and route definitions for project repositories and environment variables:
- `src/modules/projects/repositories`: `auth_type` removed from request/response; connect handles reconnect/update; status is an enum (`connected` | `disconnected`); Axum router mounts `GET`, `POST`, `PATCH`, `DELETE` at `/{id}/repository`.
- `src/modules/projects/environment_variables`: `environment` strictly typed to `development`, `staging`, `production`; `bulk` endpoint added (`POST /{id}/env-vars/bulk`) with `environment` placed per item; updates use `PATCH` instead of `PUT`; response values return the stored value.
- `src/config/env.rs` & `src/app/middleware.rs`: Default port set to 9000; CORS headers include `x-request-id` and `organization-id`.

The OpenAPI spec (`docs/system/05-api/openapi.yaml`) and companion guide (`docs/API_REQUESTS_SUMMARY.md`) currently have conflicting paths, HTTP methods, and parameter schemas that need alignment.

## Goals / Non-Goals

**Goals:**
- Update `docs/system/05-api/openapi.yaml` to precisely reflect the implemented Axum routes, HTTP methods, and DTO schemas for project repositories and environment variables.
- Eliminate phantom endpoints in OpenAPI that are not implemented in the Rust backend (e.g. `/repository/validate`, `/clone`, `/commit`, `/branch`, `/branches`, `/env-vars/decrypt`).
- Update component schemas (`RepoConnectRequest`, `RepoUpdateRequest`, `RepoResponse`, `EnvVarCreateRequest`, `EnvVarBulkCreateRequest`, `EnvVarResponse`, `ProjectEnvEnum`).
- Update `docs/API_REQUESTS_SUMMARY.md` so endpoint listings and request/response examples match the code.

**Non-Goals:**
- Modifying Rust backend code or database schemas.
- Re-architecting other platform modules (auth, users, teams, deployments).
- Introducing code-generation frameworks (such as Utoipa) in this change.

## Decisions

### Decision 1: Align OpenAPI schemas directly to Axum DTOs
- **Choice**: Hand-edit `docs/system/05-api/openapi.yaml` to ensure every path, operation, parameter, and component schema mirrors the Axum router and Rust DTO definitions.
- **Rationale**: Forge maintains `openapi.yaml` as the canonical API documentation. Direct synchronization guarantees 100% accuracy with existing codebase patterns without introducing heavy macro dependencies.
- **Alternative considered**: Introducing `utoipa` crate to auto-generate OpenAPI from Rust types. Rejected because it would require rewriting entity and DTO annotations across the entire repository.

### Decision 2: Use lowercase enum values for environment variable environments
- **Choice**: Specify enum values as `development`, `staging`, and `production` in OpenAPI components and documentation.
- **Rationale**: `ProjectEnvironmentVariablesEnvironment` in SeaORM and its `Display`/`FromStr` implementations parse only lowercase values. Previous OpenAPI specs had Titlecase ("Development", "Production"), which causes deserialization failures on the server.

### Decision 3: Remove unimplemented Git repository sub-routes from OpenAPI
- **Choice**: Remove `/projects/{id}/repository/validate`, `/projects/{id}/repository/clone`, `/projects/{id}/repository/commit`, `/projects/{id}/repository/branch`, and `/projects/{id}/repository/branches`.
- **Rationale**: The Axum router (`src/modules/projects/repositories/router.rs`) only exposes `/{id}/repository` for `GET`, `POST`, `PATCH`, and `DELETE`. Documenting unimplemented routes leads to client confusion and 404s.

### Decision 4: Reflect bulk upsert and per-item environment scoping
- **Choice**: Add `POST /projects/{id}/env-vars/bulk` with request body `BulkCreateProjectEnvVarDTO` containing `vars: Vec<ProjectEnvVarItemDTO>` where each item defines its own `environment`.
- **Rationale**: Reflects the updated batch configuration workflow where developers can submit variables for multiple environments in one atomic call.

## Risks / Trade-offs

- **[Risk]** Existing API consumers sending `auth_type` or Titlecase `Environment` values.
  - **Mitigation**: Highlight the breaking schema change in the API documentation and OpenSpec change summary.
- **[Risk]** Missing endpoints in summary markdown vs openapi.yaml.
  - **Mitigation**: Cross-check `docs/API_REQUESTS_SUMMARY.md` sections with `docs/system/05-api/openapi.yaml` after editing.
