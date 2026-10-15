# Design

## Context

The Forge Platform API Postman collections are maintained in the Postman v12 Local Workspace format located under `postman/postman/collections/Forge Platform API/`. The canonical API contract is defined in `postman/postman/specs/Forge Platform API/openapi.yaml` (mirrored at `docs/system/05-api/openapi.yaml`).

While Auth, Health, Organizations, Projects, and Teams collections have automated contract assertion test suites attached, the remaining 4 collections—encompassing 32 request files—currently define empty or no `scripts:` blocks:

1. **Users & Profiles (`users/`)** — 7 requests:
   - `Create a new platform user.request.yaml`
   - `List all platform users (paginated).request.yaml`
   - `{id}/Get user account details.request.yaml`
   - `{id}/Update user account.request.yaml`
   - `{id}/Delete user account.request.yaml`
   - `{id}/profile/Get user profile detail.request.yaml`
   - `{id}/profile/Update user profile.request.yaml`

2. **Access Control (`access-control/`)** — 18 requests:
   - System Roles: `roles/List all system roles.request.yaml`, `roles/Create a system role.request.yaml`, `roles/{id}/Get a system role by ID.request.yaml`, `roles/{id}/Update a system role.request.yaml`, `roles/{id}/Delete a system role.request.yaml`
   - Role Permissions: `roles/permissions/assign/Assign permissions to a role.request.yaml`, `roles/permissions/remove/Remove permissions from a role.request.yaml`, `roles/permissions/{id}/Get permissions assigned to a role (paginated).request.yaml`
   - System Permissions: `permissions/List system permissions (paginated).request.yaml`, `permissions/Create a system permission.request.yaml`, `permissions/{id}/Get a system permission by ID.request.yaml`, `permissions/{id}/Update system permission.request.yaml`, `permissions/{id}/Delete system permission.request.yaml`
   - User Role Assignment: `role/assign/Assign system roles to a user.request.yaml`, `role/remove/Remove system roles from a user.request.yaml`, `role/user/{id}/Get system roles assigned to a user.request.yaml`
   - Direct User Permissions: `users/permission/assign/Assign direct permissions to a user.request.yaml`, `users/permission/remove/Remove direct permissions from a user.request.yaml`, `users/permissions/{id}/Get direct permissions assigned to a user.request.yaml`

3. **Notifications (`notifications/`)** — 6 requests:
   - `List user in-app notifications.request.yaml`
   - `unread-count/Get unread notification count.request.yaml`
   - `{id}/read/Mark notification as read.request.yaml`
   - `read-all/Mark all notifications as read.request.yaml`
   - `{id}/Dismiss (soft-delete) a notification (Deprecated).request.yaml`
   - `stream/Open real-time SSE notification stream.request.yaml`

4. **Dashboard (`dashboard/`)** — 1 request:
   - `Get aggregated dashboard metrics.request.yaml`

See `proposal.md` for motivation and capability delta specs for requirement contracts.

## Goals / Non-Goals

**Goals:**
- Use `openapi.yaml` as the canonical contract to anchor all response schema expectations, HTTP status codes, headers, and parameter contracts.
- Provide robust, contract-driven test assertion scripts (`pm.test`) for all 32 requests in `users/`, `access-control/`, `notifications/`, and `dashboard/`.
- Validate HTTP status codes, Content-Type headers (`application/json` or `text/event-stream`), and JSON payload structures matching OpenAPI response definitions (`ApiResponsePaginatedUsers`, `UserResponse`, `RoleResponseDto`, `PermissionResponseDto`, `NotificationResponse`, `NotificationListResponse`, `ErrorResponse`).
- Capture and dynamically persist created identifiers (`user_id`, `role_id`, `permission_id`) into Postman environment variables upon successful creation (HTTP 201 Created) to enable dynamic collection chaining and sequential test execution.
- Maintain dual-branch assertion logic to handle both positive runs (2xx) and negative/RBAC permission runs (4xx/5xx) cleanly without unhandled test runner crashes.

**Non-Goals:**
- Modifying backend Rust application logic, route handlers, or database migrations.
- Modifying the authoritative `openapi.yaml` specification.
- Re-architecting existing collection structure or request paths.

## Decisions

### Decision 1: openapi.yaml as the Single Source of Truth Contract for Validation
- **Choice**: All assertion scripts are directly constructed from the schemas and response definitions in `openapi.yaml`. For each endpoint, the test verifies:
  1. The status code matches `openapi.yaml`'s defined responses for that operation (e.g. 200/201 vs 400/401/403/404/409).
  2. The Content-Type header conforms to the OpenAPI specification (`application/json` or `text/event-stream`).
  3. The response payload structure matches the corresponding schema in `#/components/schemas/` (e.g. validating required fields, nested objects, and arrays).
  4. Error payloads are verified against `#/components/schemas/ErrorResponse` (`is_error: true`, `code` string, `message` string).
- **Rationale**: Eliminates drift between API specifications and automated tests. Assures developers and CI runners that tests validate true contract compliance.
- **Alternatives Considered**: Ad-hoc checks without reference to `openapi.yaml`. Rejected due to risk of divergence.

### Decision 2: Dual-Branch Assertion Logic in `afterResponse` Scripts
- **Choice**: Structure each test script with conditional branching on `pm.response.code`:
  - **Success Path (2xx)**: Assert expected status code (200 OK or 201 Created), verify expected `Content-Type` header (`application/json`), and validate domain-specific payload properties against `openapi.yaml`.
  - **Error Path (4xx/5xx)**: Assert client/server error status code (400, 401, 403, 404, 409), and validate standard `ErrorResponse` contract (`is_error === true`, `code` string, `message` string).
- **Rationale**: Dynamic test runners (like Newman running against multi-role matrices) run collections under different user permissions and seeded states. Dual-branch assertions ensure that when an unauthorized role returns 403 or an invalid ID returns 404, the runner validates the error contract rather than failing with an unhandled schema exception.

### Decision 3: Dynamic Environment Variable Chaining
- **Choice**: In creation endpoints, dynamically persist generated identifiers into the active Postman environment:
  - `POST /users`: Upon 201 Created, set `pm.environment.set("user_id", json_data.data.id)`.
  - `POST /access-control/roles`: Upon 201 Created, set `pm.environment.set("role_id", json_data.data.id)`.
  - `POST /access-control/permissions`: Upon 201 Created, set `pm.environment.set("permission_id", json_data.data.id)`.
- **Rationale**: Dynamic collection execution requires requests downstream to receive valid entity IDs generated earlier in the sequence. Without this, sequential runs require pre-existing static UUIDs.

### Decision 4: Streaming Header Verification for SSE Endpoints
- **Choice**: For `GET /notifications/stream`, inspect streaming headers rather than attempting JSON parsing:
  ```javascript
  pm.test("Response status code is 200 OK or 401 Unauthorized", function () {
      pm.expect(pm.response.code).to.be.oneOf([200, 401]);
  });
  if (pm.response.code === 200) {
      pm.test("Content-Type is text/event-stream", function () {
          pm.expect(pm.response.headers.get("Content-Type")).to.include("text/event-stream");
      });
  } else {
      // Validate ErrorResponse schema
  }
  ```
- **Rationale**: Server-Sent Events do not return a standard JSON response body; attempting `pm.response.json()` would raise a syntax error.

## Risks / Trade-offs

- **[Risk]** Uncaught JSON parse errors if an endpoint returns empty or non-JSON content.
  → **Mitigation**: Wrap `pm.response.json()` in a `try...catch` block with clear assertion failure (`pm.expect.fail("Response body is not valid JSON")`).
- **[Risk]** Missing parent variables (e.g., `user_id` not set before calling `{id}/Get user account details`).
  → **Mitigation**: If `{{user_id}}` is unresolvable or missing, the API returns 400 or 404, which the dual-branch error validator safely verifies without script execution failure.
