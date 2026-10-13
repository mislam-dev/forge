# Tasks

## 1. Users and Profiles Collection Contract Tests

- [x] 1.1 Add test assertion script to `Create a new platform user.request.yaml` validating 201 Created (`UserResponse`), 400/401/403/409 (`ErrorResponse`), and environment variable persistence (`user_id`), and verify YAML syntax
- [x] 1.2 Add test assertion script to `List all platform users (paginated).request.yaml` validating 200 OK (`ApiResponsePaginatedUsers`) and 401/403 (`ErrorResponse`), and verify YAML syntax
- [x] 1.3 Add test assertion script to `{id}/Get user account details.request.yaml` validating 200 OK (`UserResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.4 Add test assertion script to `{id}/Update user account.request.yaml` validating 200 OK (`UserResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.5 Add test assertion script to `{id}/Delete user account.request.yaml` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.6 Add test assertion script to `{id}/profile/Get user profile detail.request.yaml` validating 200 OK profile data and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.7 Add test assertion script to `{id}/profile/Update user profile.request.yaml` validating 200 OK updated profile data and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax

## 2. Access Control System Roles and Role Permissions Contract Tests

- [x] 2.1 Add test assertion script to `roles/Create a system role.request.yaml` validating 201 Created (`RoleResponseDto`), 400/401/403/409 (`ErrorResponse`), and environment variable persistence (`role_id`), and verify YAML syntax
- [x] 2.2 Add test assertion script to `roles/List all system roles.request.yaml` validating 200 OK (`data` array of `RoleResponseDto`) and 401/403 (`ErrorResponse`), and verify YAML syntax
- [x] 2.3 Add test assertion script to `roles/{id}/Get a system role by ID.request.yaml` validating 200 OK (`RoleResponseDto`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.4 Add test assertion script to `roles/{id}/Update a system role.request.yaml` validating 200 OK (`RoleResponseDto`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.5 Add test assertion script to `roles/{id}/Delete a system role.request.yaml` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.6 Add test assertion script to `roles/permissions/assign/Assign permissions to a role.request.yaml` validating 200 OK confirmation message and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.7 Add test assertion script to `roles/permissions/remove/Remove permissions from a role.request.yaml` validating 200 OK confirmation message and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.8 Add test assertion script to `roles/permissions/{id}/Get permissions assigned to a role (paginated).request.yaml` validating 200 OK assigned permissions array and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 3. Access Control System Permissions and User Assignments Contract Tests

- [x] 3.1 Add test assertion script to `permissions/Create a system permission.request.yaml` validating 201 Created (`PermissionResponseDto`), 400/401/403/409 (`ErrorResponse`), and environment variable persistence (`permission_id`), and verify YAML syntax
- [x] 3.2 Add test assertion script to `permissions/List system permissions (paginated).request.yaml` validating 200 OK permissions array and 401/403 (`ErrorResponse`), and verify YAML syntax
- [x] 3.3 Add test assertion script to `permissions/{id}/Get a system permission by ID.request.yaml` validating 200 OK (`PermissionResponseDto`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.4 Add test assertion script to `permissions/{id}/Update system permission.request.yaml` validating 200 OK (`PermissionResponseDto`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.5 Add test assertion script to `permissions/{id}/Delete system permission.request.yaml` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.6 Add test assertion script to `role/assign/Assign system roles to a user.request.yaml` validating 200 OK confirmation message and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.7 Add test assertion script to `role/remove/Remove system roles from a user.request.yaml` validating 200 OK confirmation message and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.8 Add test assertion script to `role/user/{id}/Get system roles assigned to a user.request.yaml` validating 200 OK assigned roles array and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.9 Add test assertion script to `users/permission/assign/Assign direct permissions to a user.request.yaml` validating 200 OK confirmation message and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.10 Add test assertion scripts to `users/permission/remove/Remove direct permissions from a user.request.yaml` and `users/permissions/{id}/Get direct permissions assigned to a user.request.yaml` validating 200 OK responses and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax

## 4. Notifications and Dashboard Collection Contract Tests

- [x] 4.1 Add test assertion script to `List user in-app notifications.request.yaml` validating 200 OK (`NotificationListResponse`) and 401 (`ErrorResponse`), and verify YAML syntax
- [x] 4.2 Add test assertion script to `unread-count/Get unread notification count.request.yaml` validating 200 OK (`unread_count` integer) and 401 (`ErrorResponse`), and verify YAML syntax
- [x] 4.3 Add test assertion script to `{id}/read/Mark notification as read.request.yaml` validating 200 OK (`NotificationResponse`, `is_read: true`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 4.4 Add test assertion script to `read-all/Mark all notifications as read.request.yaml` validating 200 OK confirmation message and 401 (`ErrorResponse`), and verify YAML syntax
- [x] 4.5 Add test assertion script to `{id}/Dismiss (soft-delete) a notification (Deprecated).request.yaml` validating 200 OK confirmation message and 401/404 (`ErrorResponse`), and verify YAML syntax
- [x] 4.6 Add test assertion script to `stream/Open real-time SSE notification stream.request.yaml` validating 200 OK `Content-Type: text/event-stream` and 401 (`ErrorResponse`), and verify YAML syntax
- [x] 4.7 Add test assertion script to `dashboard/Get aggregated dashboard metrics.request.yaml` validating 200 OK metrics payload structure and 401 (`ErrorResponse`), and verify YAML syntax

## 5. Collection Validation and Execution Verification

- [x] 5.1 Validate YAML formatting and schema compliance across all 32 updated request files against `openapi.yaml`
- [x] 5.2 Verify complete Postman workspace collection execution and dynamic environment chaining
