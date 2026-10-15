# Proposal

## Why

The Forge platform API collection in Postman has comprehensive endpoint definitions and contract assertion suites for Auth, Health, Organizations, Projects, and Teams. However, the remaining endpoint collections—Users & Profiles, Access Control (RBAC roles, permissions, assignments), Notifications, and Dashboard—currently lack automated contract test assertion scripts and dynamic environment variable chaining based directly on the authoritative API contract.

The canonical contract for the Forge platform is defined in `openapi.yaml` (`postman/postman/specs/Forge Platform API/openapi.yaml` / `docs/system/05-api/openapi.yaml`). Without contract-based tests validating directly against this specification, automated collection runners (such as Newman and CI matrix jobs) cannot dynamically validate response schemas, status codes, data types, formats, or error structures against the OpenAPI specification. Furthermore, sequential dynamic workflows (such as creating a user/role/permission and dynamically passing the resulting identifiers to downstream update, assignment, or deletion requests) fail or rely on manually seeded data. Establishing `openapi.yaml` as the contract reference across all collections enables robust, end-to-end dynamic API testing.

## What Changes

- Use `openapi.yaml` as the authoritative contract definition for all response validation, schema assertions, status codes, and headers.
- Add automated contract test assertion scripts to all **Users & User Profiles** endpoints (`GET /users`, `POST /users`, `GET /users/{id}`, `PUT /users/{id}`, `DELETE /users/{id}`, `GET /users/{id}/profile`, `PUT /users/{id}/profile`), verifying HTTP status codes, headers, and response schemas (`ApiResponsePaginatedUsers`, `UserResponse`, `UserProfileDetailResponse`) directly against `openapi.yaml`, while dynamically persisting `user_id`.
- Add automated contract test assertion scripts to all **Access Control** endpoints (`GET/POST /access-control/roles`, `GET/PUT/DELETE /access-control/roles/{id}`, `GET/POST /access-control/permissions`, `GET/PUT/DELETE /access-control/permissions/{id}`, role permission assignments, user role assignments, and direct user permissions), verifying status codes and component schemas (`RoleResponseDto`, `PermissionResponseDto`) against `openapi.yaml`, while dynamically capturing `role_id` and `permission_id`.
- Add automated contract test assertion scripts to all **Notifications** endpoints (`GET /notifications`, `GET /notifications/unread-count`, `PATCH /notifications/{id}/read`, `POST /notifications/read-all`, `DELETE /notifications/{id}`, `GET /notifications/stream`), verifying status codes, response schemas (`NotificationListResponse`, `NotificationResponse`, count), and SSE streaming headers defined in `openapi.yaml`.
- Add automated contract test assertion scripts to the **Dashboard** metrics endpoint (`GET /dashboard`), verifying HTTP status code 200, Content-Type, and aggregated payload structure against `openapi.yaml`.
- Enforce standard `ErrorResponse` validation (`is_error: true`, string error `code`, message string) against `#/components/schemas/ErrorResponse` in `openapi.yaml` for all 4xx/5xx responses to facilitate negative and RBAC permission matrix testing.

## Capabilities

### New Capabilities
- `users-collection-tests`: Contract test assertion suites for all Users and User Profile endpoints in the Postman collection, validating response payloads against `openapi.yaml` schemas and dynamically persisting `user_id`.
- `access-control-collection-tests`: Contract test assertion suites for all RBAC system role, system permission, and assignment endpoints, validating contracts against `openapi.yaml` and dynamically capturing `role_id` and `permission_id`.
- `notifications-collection-tests`: Contract test assertion suites for notification management endpoints, verifying response schemas against `openapi.yaml` and state changes.
- `dashboard-collection-tests`: Contract test assertion suite for aggregated dashboard metrics endpoint, verifying payload structure against `openapi.yaml` and unauthorized handling.

### Modified Capabilities
<!-- No existing spec requirements are modified. Existing collection test specs remain authoritative contracts. -->

## Impact

- **Postman Collection**: Modifies 32 request definitions under `postman/postman/collections/Forge Platform API/` across `users/`, `access-control/`, `notifications/`, and `dashboard/` to include `afterResponse` contract assertion test scripts verified against `openapi.yaml`.
- **Dynamic Chaining**: Enables dynamic collection runs where created IDs (`user_id`, `role_id`, `permission_id`) are captured into the active Postman environment and used by parameterized downstream endpoints.
- **CI / Newman Automation**: Facilitates dynamic headless execution and multi-role regression testing across the entire API surface without manual intervention.
