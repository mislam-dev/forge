# Design

## Context

The Forge Platform API Postman collections are maintained in the Postman v12 Local Workspace format located at `postman/postman/collections/Forge Platform API/`. Team and team membership endpoints reside under `teams/` across 9 `*.request.yaml` files:
1. `Create a team.request.yaml`
2. `List teams in organization.request.yaml`
3. `{id}/Get team details.request.yaml`
4. `{id}/Update team.request.yaml`
5. `{id}/Delete team.request.yaml`
6. `{id}/members/List team members.request.yaml`
7. `{id}/members/Add user to team.request.yaml`
8. `{id}/members/{user_id}/Update team member role.request.yaml`
9. `{id}/members/{user_id}/Remove user from team.request.yaml`

Currently, none of the 9 team request files define `scripts:` with `pm.test` assertions.

See `proposal.md` for motivation and `specs/teams-collection-tests/spec.md` for requirement contracts.

## Goals / Non-Goals

**Goals:**
- Provide robust, contract-driven test assertion scripts (`pm.test`) for all 9 requests under `postman/postman/collections/Forge Platform API/teams/`.
- Validate HTTP status codes, Content-Type headers (`application/json`), and JSON body structures for both successful (2xx) and error (4xx) responses matching OpenAPI schemas (`TeamResponse`, `TeamMemberResponse`, `ErrorResponse`).
- Capture and persist `team_id` in Postman environment variables upon successful team creation (`POST /teams`) to facilitate downstream request chaining.
- Ensure compatibility with Newman and Postman CLI automation runners.

**Non-Goals:**
- Modifying backend Rust code, router, or database entities.
- Modifying OpenAPI specifications or generating mock servers.
- Adding tests to non-team collection modules.

## Decisions

### Decision 1: Dual-Branch Assertion Logic in `afterResponse` Scripts
- **Choice**: Structure each test script to inspect `pm.response.code`:
  - When `pm.response.code >= 200 && pm.response.code < 300`:
    - Assert expected success status (200 OK or 201 Created).
    - Assert `Content-Type` header includes `application/json`.
    - Assert endpoint-specific schema contract (`TeamResponse`, `TeamMemberResponse`, or confirmation message).
  - When `pm.response.code >= 400`:
    - Assert client/server error status code (400, 401, 403, 404, 409).
    - Assert standard `ErrorResponse` schema (`is_error === true`, `code` string, `message` string).
- **Rationale**: Collection requests are run in diverse testing contexts (e.g. positive integration runs or negative validation/authorization runs). Dual-branch assertion prevents script crashes and guarantees valid contract verification regardless of the scenario executed.
- **Alternatives Considered**: Strict success-only assertions. Rejected because executing negative test cases or encountering expired auth credentials would trigger unhandled JavaScript exceptions instead of validating the API error structure.

### Decision 2: Domain Schema Validation Alignment
- **Choice**: Map assertion expectations directly to OpenAPI components:
  - `TeamResponse`: Assert properties `id`, `name`, `member_count`, and `created_at` (along with optional `organization_id` and `descriptions`).
  - `TeamMemberResponse`: Assert properties `team_id`, `user_id`, `role`, and `joined_at` (or `user` object if populated).
  - `ErrorResponse`: Standardized structure across all endpoints with `is_error: true`, `code`, and `message`.
- **Rationale**: Keeps automated test suites aligned with API contract schemas and OpenAPI definitions.

### Decision 3: Environment Variable Synchronization for Created Teams
- **Choice**: In `POST /teams`, when response code is 201 Created and `data.id` is present, execute:
  ```javascript
  if (json_data.data && json_data.data.id) {
      pm.environment.set("team_id", json_data.data.id);
  }
  ```
- **Rationale**: Enables subsequent team-scoped collection requests (`GET /teams/{{team_id}}`, member assignments, etc.) to run sequentially in runner suites without manual environment configuration.

## Risks / Trade-offs

- **[Risk]** Uncaught JSON parse errors if an endpoint returns non-JSON content.
  → **Mitigation**: Wrap `pm.response.json()` in a `try...catch` block and fail cleanly with `pm.expect.fail("Response body is not valid JSON")`.
- **[Risk]** Missing or invalid environment variables (`organization_id`, `team_id`, `user_id`) during runner execution.
  → **Mitigation**: Dual-branch assertions ensure that even if an invalid ID is provided and a 400/404 is returned, the error response is verified against the `ErrorResponse` schema without crashing the test runner.
