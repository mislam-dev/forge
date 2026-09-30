# Proposal

## Why

The OpenAPI specification (`docs/system/05-api/openapi.yaml`) and companion documentation (`docs/API_REQUESTS_SUMMARY.md`) have drifted from the actual deployment router implementation in `src/modules/projects/deployments/router.rs`. Currently, deployment endpoints in `openapi.yaml` use legacy or mismatched paths (e.g. `/deployments` instead of project-scoped `/projects/{id}/deployments`, `/projects/{id}/rollback` instead of `/projects/{id}/deployments/rollback`, and `/deployments/{id}/status` PATCH instead of `/projects/internal/deployments/{deployment_id}/status` PUT), and mismatched DTO request bodies. This change aligns the OpenAPI specification and API documentation with the actual Axum router and DTOs while preserving existing pagination parameters (`page`, `limit`).

## What Changes

- **Synchronize Deployment Endpoints**:
  - Update trigger deployment from `POST /deployments` to `POST /projects/{id}/deployments`, taking `branch` and `commit_hash` in the request body while obtaining `project_id` from the path parameter `{id}`.
  - Update list deployments to `GET /projects/{id}/deployments`, keeping existing pagination query parameters (`page`, `limit`) and adding the `branch` query filter.
  - Update get deployment details from `GET /deployments/{id}` to `GET /projects/{id}/deployments/{deployment_id}` with dual path parameters `{id}` (project ID) and `{deployment_id}`.
  - Update redeploy from `POST /deployments/{id}/redeploy` to `POST /projects/{id}/deployments/{deployment_id}/redeploy` with dual path parameters.
  - Update rollback from `POST /projects/{id}/rollback` to `POST /projects/{id}/deployments/rollback` with path parameter `{id}`.
  - Update internal status update from `PATCH /deployments/{id}/status` to `PUT /projects/internal/deployments/{deployment_id}/status`, requiring the `x-service-token` header and matching `UpdateDeploymentStatusRequest`.
- **Align Schemas & Response Wrappers**:
  - Update `TriggerDeploymentRequest` schema: remove `project_id` from body, keep optional `branch` and `commit_hash`.
  - Update `DeploymentResponse` schema: include `updated_at`, `error_message`, and accurate enum values for `status` (`Queued`, `Building`, `Deploying`, `Running`, `Failed`, `Success`).
  - Wrap responses in standard `ApiResponse` structure (`message`, `data`).
- **Update Documentation**:
  - Update `docs/system/05-api/openapi.yaml` with the corrected paths, parameters, schemas, and tags.
  - Update `docs/API_REQUESTS_SUMMARY.md` deployment sections and endpoint summary table.

## Capabilities

### New Capabilities
- `project-deployments`: Project deployment lifecycle management including triggering async builds, fetching paginated deployment history, querying deployment details, redeploying previous commits, rolling back to previous successful deployments, and internal build worker status updates.

### Modified Capabilities
<!-- None: No pre-existing spec exists for project-deployments. -->

## Impact

- **Affected Files**:
  - `docs/system/05-api/openapi.yaml`
  - `docs/API_REQUESTS_SUMMARY.md`
  - `openspec/specs/project-deployments/spec.md` (new delta spec)
- **APIs**:
  - `POST /projects/{id}/deployments`
  - `GET /projects/{id}/deployments`
  - `GET /projects/{id}/deployments/{deployment_id}`
  - `POST /projects/{id}/deployments/{deployment_id}/redeploy`
  - `POST /projects/{id}/deployments/rollback`
  - `PUT /projects/internal/deployments/{deployment_id}/status`
- **Dependencies & Architecture**:
  - No Rust codebase or external dependency changes required.
  - Fixes documentation drift so OpenAPI tools, client generators, and documentation consumers match the actual backend API.
