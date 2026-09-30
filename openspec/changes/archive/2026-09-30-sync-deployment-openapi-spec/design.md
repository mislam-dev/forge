# Design

## Context

The Forge backend implementation organizes deployment endpoints under `src/modules/projects/deployments/router.rs`, which is mounted through `projects_router()` under `/projects` (and `/api/v1/projects`).

The actual backend endpoints are:
- `POST /projects/{id}/deployments`: Triggers a new deployment for project `{id}`.
- `GET /projects/{id}/deployments`: Lists deployments for project `{id}` with pagination and optional filters (`status`, `branch`).
- `GET /projects/{id}/deployments/{deployment_id}`: Retrieves details for a specific deployment under project `{id}`.
- `POST /projects/{id}/deployments/{deployment_id}/redeploy`: Redeploys a specific previous deployment under project `{id}`.
- `POST /projects/{id}/deployments/rollback`: Rolls back project `{id}` to the most recent successful deployment.
- `PUT /projects/internal/deployments/{deployment_id}/status`: Internal endpoint for build workers to transition deployment status using the `x-service-token` header.

However, `docs/system/05-api/openapi.yaml` and `docs/API_REQUESTS_SUMMARY.md` contain legacy routes (such as root `/deployments`, `PATCH /deployments/{id}/status`, `POST /projects/{id}/rollback`, and redundant `project_id` in request payloads).

## Goals / Non-Goals

**Goals:**
- Update `docs/system/05-api/openapi.yaml` paths, HTTP methods, headers, path/query parameters, request bodies, and response schemas to 100% match the Axum router and Rust DTOs.
- Retain existing pagination parameters (`page`, `limit`) on deployment listing.
- Correct `TriggerDeploymentRequest`, `UpdateDeploymentStatusRequest`, and `DeploymentResponse` schemas in OpenAPI components.
- Update `docs/API_REQUESTS_SUMMARY.md` Section 12 (Deployments & Build Worker APIs) and Section 16 (Endpoint Summary Table) to reflect the synchronized routes and parameters.

**Non-Goals:**
- Modifying Rust backend code or database schemas.
- Modifying build log endpoints (`/logs`), which belong to the separate live log streaming module.
- Modifying pagination query parameters (`page`, `limit`).

## Decisions

### Decision 1: Structure all public deployment routes under `/projects/{id}/deployments`
- **Choice**: Replace root `/deployments` and `/deployments/{id}` in `openapi.yaml` with `/projects/{id}/deployments` and `/projects/{id}/deployments/{deployment_id}`.
- **Rationale**: The Axum router nests deployment routes inside `projects_router()`. All public deployment operations are scoped to a specific project.
- **Alternative considered**: Keeping aliases for root `/deployments`. Rejected because the Axum router does not mount deployment routes at `/deployments`, only at `/projects` (and `/api/v1/projects`).

### Decision 2: Align internal status update to `PUT /projects/internal/deployments/{deployment_id}/status`
- **Choice**: Document the status update endpoint at `/projects/internal/deployments/{deployment_id}/status` using HTTP `PUT` and requiring the `x-service-token` header.
- **Rationale**: `src/modules/projects/deployments/router.rs` mounts `"/internal/deployments/{deployment_id}/status"` under `projects_router()` using `put(...)`. The handler validates the `x-service-token` header against `AppConfig::secrets.master_encryption_key`.
- **Alternative considered**: Documenting as `PATCH`. Rejected because Axum router specifically binds `put(handlers::update_status_internal)` and rejects other methods with 405 Method Not Allowed.

### Decision 3: Preserve existing pagination parameters
- **Choice**: Retain `page` and `limit` in `GET /projects/{id}/deployments` query parameters. Add `branch` filter alongside `status`.
- **Rationale**: User explicitly instructed to keep pagination-related parameters unchanged.

### Decision 4: Align deployment status enum values
- **Choice**: Document `DeploymentStatus` enum values in OpenAPI as `["Queued", "Building", "Deploying", "Running", "Failed", "Success"]`.
- **Rationale**: SeaORM active enums and the `as_str()` implementation in `status.rs` serialize and parse these exact title-cased values.

## Risks / Trade-offs

- **[Risk]** API clients expecting the legacy `/deployments` or `PATCH` paths fail.
  - **Mitigation**: Document the exact canonical paths matching the Axum backend and highlight the path changes in API documentation and commit notes.
- **[Risk]** Formatting or YAML syntax errors in `openapi.yaml`.
  - **Mitigation**: Validate the YAML file syntax with automated checks or parsers after updating.
