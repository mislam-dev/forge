# Proposal

## Why

Recent codebase updates modified project repository connection workflows, environment variable handling (introducing typed environment enums, bulk upsert logic, and moving the environment parameter per-item), CORS headers, and default port settings. The OpenAPI specification (`docs/system/05-api/openapi.yaml`) and related documentation currently have discrepancies against the actual Axum router and DTO definitions. This change documents and specifies these updated endpoints and reconciles the OpenAPI and Markdown documentation.

## What Changes

- **Repository Management API (`/projects/{id}/repository`)**:
  - **BREAKING**: Removed `auth_type` from `ConnectProjectRepositoryDTO` and `UpdateProjectRepositoryDTO`; authentication type is now automatically inferred on the backend (e.g. `pat` when access token is provided).
  - Connect repository endpoint (`POST /projects/{id}/repository`) now idempotently updates/reconnects an existing repository instead of returning `409 Conflict`.
  - Added support for `PATCH /projects/{id}/repository` (update repository) and `DELETE /projects/{id}/repository` (disconnect repository) in the OpenAPI specification.
  - Updated `ProjectRepositoryStatus` enum to `connected` and `disconnected`.
  - Removed outdated / unimplemented Git routes from OpenAPI documentation (`/validate`, `/clone`, `/commit`, `/branch`, `/branches`).
- **Environment Variables API (`/projects/{id}/env-vars`)**:
  - **BREAKING**: `environment` is now strictly typed to an enum (`development`, `staging`, `production`) across request/response DTOs.
  - Added `POST /projects/{id}/env-vars/bulk` endpoint specification to OpenAPI.
  - **BREAKING**: In `BulkCreateProjectEnvVarDTO`, the `environment` parameter was moved from the root object into each item within `vars`.
  - Bulk create now operates with upsert behavior (updates existing variable if key exists within the target environment).
  - Corrected `PUT /projects/{id}/env-vars/{env_id}` to `PATCH /projects/{id}/env-vars/{env_id}` in OpenAPI documentation.
  - Synchronized response serialization where secret masking defaults and plaintext encryption behaviors align with implementation.
- **Server Configuration & Middleware**:
  - Updated default `SERVER_PORT` documentation to `9000`.
  - Documented allowed CORS headers (`x-request-id`, `organization-id`) and exposed header (`x-request-id`).
- **OpenAPI and Companion Docs Synchronization**:
  - Update `docs/system/05-api/openapi.yaml` to accurately define request bodies, parameters, schemas, and responses.
  - Update `docs/API_REQUESTS_SUMMARY.md` to reflect these endpoint and schema adjustments.

## Capabilities

### New Capabilities
- `project-repositories`: Manage Git repository configuration, connection, update, and disconnect for projects.
- `project-environment-variables`: Manage environment variables for projects including single creation, bulk upsert, patch update, deletion, and listing with environment scoping.

### Modified Capabilities
<!-- None: No pre-existing specs exist in openspec/specs/. -->

## Impact

- **Affected Files**:
  - `docs/system/05-api/openapi.yaml`
  - `docs/API_REQUESTS_SUMMARY.md`
  - `openspec/specs/project-repositories/spec.md`
  - `openspec/specs/project-environment-variables/spec.md`
- **APIs**:
  - `/projects/{id}/repository` (POST, GET, PATCH, DELETE)
  - `/projects/{id}/env-vars` (GET, POST)
  - `/projects/{id}/env-vars/bulk` (POST)
  - `/projects/{id}/env-vars/{env_id}` (PATCH, DELETE)
- **Dependencies & Architecture**:
  - No new external crate dependencies.
  - OpenAPI spec consumers and frontend clients will align with the backend Axum routes and DTO structures.
