# Design

## Context

The Forge Platform API Postman collections are maintained in the Postman v12 Local Workspace format located at `postman/postman/collections/Forge Platform API/`. Organization and membership endpoints reside under `organizations/` across 11 `*.request.yaml` files.

Currently, none of the 11 organization request files define `scripts:` with `pm.test` assertions.

See `proposal.md` for motivation and `specs/organization-collection-tests/spec.md` for requirement contracts.

## Goals / Non-Goals

**Goals:**
- Provide robust, contract-driven test assertion scripts (`pm.test`) for all 11 requests under `postman/postman/collections/Forge Platform API/organizations/`.
- Validate HTTP status codes, Content-Type headers (`application/json`), and JSON body structures for both successful (2xx) and error (4xx) responses matching OpenAPI schemas (`OrganizationResponse`, `OrgInvitationResponse`, `OrgMemberResponse`, `ErrorResponse`).
- Capture and persist `organization_id` in Postman environment variables upon successful organization creation (`POST /organizations`) to facilitate downstream request chaining.
- Ensure compatibility with Newman and Postman CLI automation runners.

**Non-Goals:**
- Modifying backend Rust code, router, or database entities.
- Modifying OpenAPI specifications or generating mock servers.
- Adding tests to non-organization collection modules.

## Decisions

### Decision 1: Dual-Branch Assertion Logic in `afterResponse` Scripts
- **Choice**: Structure each test script to inspect `pm.response.code`:
  - When `pm.response.code >= 200 && pm.response.code < 300`:
    - Assert expected success status (200 OK or 201 Created).
    - Assert `Content-Type` header includes `application/json`.
    - Assert endpoint-specific schema contract (`OrganizationResponse`, `OrgInvitationResponse`, `OrgMemberResponse`, or confirmation message).
  - When `pm.response.code >= 400`:
    - Assert client/server error status code (400, 401, 403, 404, 409).
    - Assert standard `ErrorResponse` schema (`is_error === true`, `code` string, `message` string).
- **Rationale**: Collection requests are run in diverse testing contexts (e.g. positive integration runs or negative validation/authorization runs). Dual-branch assertion prevents script crashes and guarantees valid contract verification regardless of the scenario executed.
- **Alternatives Considered**: Strict success-only assertions. Rejected because executing negative test cases or encountering expired auth credentials would trigger unhandled JavaScript exceptions instead of validating the API error structure.

### Decision 2: Domain Schema Validation Alignment
- **Choice**: Map assertion expectations directly to OpenAPI components:
  - `OrganizationResponse`: Assert properties `id`, `name`, `type`, and `created_at`.
  - `OrgInvitationResponse`: Assert properties `id`, `email`, `role`, and `token`.
  - `OrgMemberResponse`: Assert properties `id`, `user_id`, `role`, and `created_at` (or `joined_at`).
  - `ErrorResponse`: Standardized structure across all 11 endpoints with `is_error: true`, `code`, and `message`.
- **Rationale**: Keeps automated test suites aligned with API contract schemas and OpenAPI definitions.

### Decision 3: Environment Variable Synchronization for Created Organizations
- **Choice**: In `POST /organizations`, when response code is 201 Created and `data.id` is present, execute:
  ```javascript
  if (json_data.data && json_data.data.id) {
      pm.environment.set("organization_id", json_data.data.id);
  }
  ```
- **Rationale**: Enables subsequent organization-scoped collection requests (`GET /organizations/{{organization_id}}`, member invites, etc.) to run sequentially in runner suites without manual environment configuration.

## Risks / Trade-offs

- **[Risk]** Uncaught JSON parse errors if an endpoint returns non-JSON content.
  → **Mitigation**: Wrap `pm.response.json()` in a `try...catch` block and fail cleanly with `pm.expect.fail("Response body is not valid JSON")`.
- **[Risk]** Missing environment variables (`organization_id`, `user_id`, `token`) during runner execution.
  → **Mitigation**: Dual-branch assertions ensure that even if an invalid ID is provided and a 400/404 is returned, the error response is verified against the `ErrorResponse` schema without crashing the test runner.
