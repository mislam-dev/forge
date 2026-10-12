# Proposal

## Why

The Forge Platform API project endpoints (`/projects/*`) in the Postman collection—encompassing core project lifecycle, Git repository integrations, member and team assignments, environment variable configurations, and deployment operations—currently lack automated test assertions. Without `afterResponse` contract tests validating HTTP status codes, JSON response schemas, and failure envelopes against the OpenAPI specifications, automated test runners (Newman, Postman CLI, CI regression suites) cannot verify that project-related endpoints behave correctly or maintain contract compatibility.

## What Changes

- Add comprehensive Postman test assertion scripts (`scripts` with `type: afterResponse`) to all 30 project collection requests under `postman/postman/collections/Forge Platform API/projects/`:
  - **Core Projects (5 requests)**:
    - `POST /projects`: Assert 201 Created (`ProjectResponse` schema) or 400/401/403/409 (`ErrorResponse` schema), and persist `project_id` into Postman environment variables upon success.
    - `GET /projects`: Assert 200 OK (`data` array of `ProjectResponse` items with pagination metadata) or 400/401/403 (`ErrorResponse` schema).
    - `GET /projects/{id}`: Assert 200 OK (`ProjectResponse` schema) or 401/403/404 (`ErrorResponse` schema).
    - `PUT /projects/{id}`: Assert 200 OK (`ProjectResponse` schema) or 400/401/403/404 (`ErrorResponse` schema).
    - `DELETE /projects/{id}`: Assert 200 OK deletion confirmation message or 401/403/404 (`ErrorResponse` schema).
  - **Repository Connections (4 requests)**:
    - `POST /projects/{id}/repository`: Assert 201 Created (`ProjectRepositoryResponse`) or 400/401/403/404/409 (`ErrorResponse`).
    - `GET /projects/{id}/repository`: Assert 200 OK (`ProjectRepositoryResponse`) or 401/403/404 (`ErrorResponse`).
    - `PATCH /projects/{id}/repository`: Assert 200 OK (`ProjectRepositoryResponse`) or 400/401/403/404 (`ErrorResponse`).
    - `DELETE /projects/{id}/repository`: Assert 200 OK disconnection message or 401/403/404 (`ErrorResponse`).
  - **Member Assignments (3 requests)**:
    - `GET /projects/{id}/members`: Assert 200 OK (`data` array of assigned user objects) or 401/403/404 (`ErrorResponse`).
    - `POST /projects/{id}/members`: Assert 201 Created or 200 OK assignment confirmation or 400/401/403/404/409 (`ErrorResponse`).
    - `DELETE /projects/{id}/members/{user_id}`: Assert 200 OK removal confirmation message or 401/403/404 (`ErrorResponse`).
  - **Team Assignments (3 requests)**:
    - `GET /projects/{id}/teams`: Assert 200 OK (`data` array of assigned team objects) or 401/403/404 (`ErrorResponse`).
    - `POST /projects/{id}/teams`: Assert 201 Created or 200 OK assignment confirmation or 400/401/403/404/409 (`ErrorResponse`).
    - `DELETE /projects/{id}/teams/{team_id}`: Assert 200 OK removal confirmation message or 401/403/404 (`ErrorResponse`).
  - **Environment Variables (5 requests)**:
    - `POST /projects/{id}/env-vars`: Assert 201 Created (`ProjectEnvironmentVariableResponse`) or 400/401/403/404/409 (`ErrorResponse`), and persist `env_id` into Postman environment variables upon success.
    - `GET /projects/{id}/env-vars`: Assert 200 OK (`data` array of environment variable objects) or 401/403/404 (`ErrorResponse`).
    - `POST /projects/{id}/env-vars/bulk`: Assert 200 OK or 201 Created (`data` array of upserted env vars) or 400/401/403/404 (`ErrorResponse`).
    - `PATCH /projects/{id}/env-vars/{env_id}`: Assert 200 OK (`ProjectEnvironmentVariableResponse`) or 400/401/403/404 (`ErrorResponse`).
    - `DELETE /projects/{id}/env-vars/{env_id}`: Assert 200 OK deletion confirmation message or 401/403/404 (`ErrorResponse`).
  - **Deployments & Logs (10 requests)**:
    - `POST /projects/{id}/deployments`: Assert 201 Created (`DeploymentResponse`) or 400/401/403/404/409 (`ErrorResponse`), and persist `deployment_id` into Postman environment variables upon success.
    - `GET /projects/{id}/deployments`: Assert 200 OK (`data` array of `DeploymentResponse` items with pagination) or 400/401/403/404 (`ErrorResponse`).
    - `GET /projects/{id}/deployments/{deployment_id}`: Assert 200 OK (`DeploymentResponse`) or 401/403/404 (`ErrorResponse`).
    - `POST /projects/{id}/deployments/{deployment_id}/redeploy`: Assert 201 Created or 202 Accepted (`DeploymentResponse`) or 400/401/403/404 (`ErrorResponse`), and update `deployment_id` upon success.
    - `POST /projects/{id}/deployments/rollback`: Assert 201 Created or 202 Accepted (`DeploymentResponse`) or 400/401/403/404 (`ErrorResponse`), and update `deployment_id` upon success.
    - `GET /projects/{id}/deployments/{deployment_id}/logs`: Assert 200 OK (stored logs structure) or 401/403/404 (`ErrorResponse`).
    - `GET /projects/{id}/deployments/{deployment_id}/logs/download`: Assert 200 OK (text/plain log content) or 401/403/404 (`ErrorResponse`).
    - `GET /projects/{id}/deployments/{deployment_id}/logs/search`: Assert 200 OK (search results) or 400/401/403/404 (`ErrorResponse`).
    - `GET /projects/{id}/deployments/{deployment_id}/logs/stream`: Assert 200 OK with `text/event-stream` Content-Type or 401/403/404 (`ErrorResponse`).
    - `PUT /projects/internal/deployments/{deployment_id}/status`: Assert 200 OK status update confirmation or 400/401/403/404 (`ErrorResponse`).
- Implement dual-branch assertions in every test script to validate both expected success payloads and standardized error payloads (`is_error: true`, `code`, `message`).
- Safe JSON parse handling via `try...catch` blocks to fail cleanly on invalid response bodies.

## Capabilities

### New Capabilities
- `projects-collection-tests`: Defines contract and automated response assertions across all project and related collection endpoints in Postman/Newman, validating both success (2xx) and client/server error responses according to OpenAPI specifications.

### Modified Capabilities
<!-- None -->

## Impact

- Updates 30 Postman request YAML files located under `postman/postman/collections/Forge Platform API/projects/`.
- No modifications to backend Rust services, database models, or OpenAPI specifications.
- Enables CI Newman / Postman runner suites to execute end-to-end regression testing across the entire project module and downstream deployment pipelines.
