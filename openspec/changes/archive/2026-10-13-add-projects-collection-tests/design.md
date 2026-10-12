# Design

## Context

The Forge Platform API Postman collections are structured using the Postman v12 Local Workspace format under `postman/postman/collections/Forge Platform API/`. All project lifecycle and auxiliary services reside under the `projects/` directory across 30 `*.request.yaml` files organized into functional subdirectories:
1. Core Projects (`Create a project`, `List projects for organization`, `{id}/Get project by ID`, `{id}/Update project configuration`, `{id}/Delete project`)
2. Repository Connection (`{id}/repository/*`)
3. Member Management (`{id}/members/*`)
4. Team Management (`{id}/teams/*`)
5. Environment Variables (`{id}/env-vars/*`)
6. Deployments and Logs (`{id}/deployments/*` and `internal/deployments/{deployment_id}/status`)

Currently, none of these 30 requests define `scripts:` blocks.

See `proposal.md` for motivation and `specs/projects-collection-tests/spec.md` for requirement contracts.

## Goals / Non-Goals

**Goals:**
- Provide robust, contract-driven test assertion scripts (`pm.test` in `scripts` with `type: afterResponse`) for all 30 requests under `postman/postman/collections/Forge Platform API/projects/`.
- Validate HTTP status codes, Content-Type headers (`application/json`, or `text/plain` / `text/event-stream` for log download and SSE stream), and JSON body structures for both successful (2xx) and error (4xx) responses conforming to OpenAPI specifications.
- Capture and persist dynamic entity identifiers (`project_id`, `env_id`, `deployment_id`) into Postman environment variables upon creation or redeployment to support smooth chained execution in collection runners.
- Maintain full compatibility with Newman and Postman CLI automation runners.

**Non-Goals:**
- Modifying backend Rust codebase, routing handlers, database schemas, or business logic.
- Modifying OpenAPI contract specifications or generating mock servers.
- Adding tests to non-project modules (such as auth, access-control, users, or notifications).

## Decisions

### Decision 1: Dual-Branch Assertion Logic in `afterResponse` Scripts
- **Choice**: Structure each script to check `pm.response.code`:
  - When `pm.response.code >= 200 && pm.response.code < 300`:
    - Assert expected success status (e.g., 200 OK or 201 Created).
    - Assert appropriate `Content-Type` header (`application/json`, `text/plain` for log downloads, or `text/event-stream` for SSE).
    - For JSON endpoints, assert endpoint-specific schema contract (`ProjectResponse`, `DeploymentResponse`, `ProjectEnvironmentVariableResponse`, or confirmation message).
  - When `pm.response.code >= 400`:
    - Assert client/server error status code (400, 401, 403, 404, 409).
    - Assert standardized `ErrorResponse` schema (`is_error === true`, `code` string, `message` string).
- **Rationale**: Dual-branch assertions ensure that positive integration tests validate response contracts while negative tests or auth failures are caught cleanly without script crashes.
- **Alternatives Considered**: Asserting only 2xx success. Rejected because running collection requests against fresh environments or negative suites would throw unhandled exceptions and break CI reporting.

### Decision 2: Chained Environment Variable Capture
- **Choice**: Persist key identifiers created during runner execution:
  - `POST /projects` -> sets `project_id` from `data.id`.
  - `POST /projects/{id}/env-vars` -> sets `env_id` from `data.id`.
  - `POST /projects/{id}/deployments`, redeploy, and rollback -> sets `deployment_id` from `data.id`.
- **Rationale**: Allows Newman and collection runner suites to seamlessly flow through project creation, variable addition, and deployment workflows without requiring hardcoded IDs.
- **Alternatives Considered**: Requiring pre-populated environment variables. Rejected because automated CI suites require dynamic creation and testing from scratch.

### Decision 3: Safe Parsing and Content-Type Handling
- **Choice**: In JSON endpoints, wrap `pm.response.json()` in a `try...catch` block. For non-JSON endpoints (e.g., log download and SSE streaming), skip JSON parsing and assert `pm.response.headers.get("Content-Type")`.
- **Rationale**: Prevents unhandled JavaScript syntax errors when endpoints return non-JSON data or unexpected plain-text responses.

## Risks / Trade-offs

- **[Risk]** Uncaught JSON parse errors if an endpoint returns non-JSON content.
  → **Mitigation**: Wrap `pm.response.json()` in `try...catch` and fail cleanly via `pm.expect.fail("Response body is not valid JSON")`.
- **[Risk]** Missing or invalid environment variables (`organization_id`, `project_id`, `user_id`, `team_id`, `env_id`, `deployment_id`) during runner execution.
  → **Mitigation**: Dual-branch assertions ensure that if an invalid ID causes a 400/404, the error response is verified against the `ErrorResponse` schema without crashing the runner.
- **[Risk]** Streaming or file download endpoints do not return JSON responses on success.
  → **Mitigation**: Specifically inspect Content-Type for log download (`text/plain`) and SSE stream (`text/event-stream`), only enforcing JSON schema checks on error responses (4xx).
