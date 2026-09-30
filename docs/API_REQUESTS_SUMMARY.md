# Forge Platform — Complete API Requests Reference & Summary

> **Companion Document for OpenSpec & Antigravity**  
> **Source OpenAPI Spec:** [`docs/system/05-api/openapi.yaml`](./system/05-api/openapi.yaml)  
> **Version:** 1.0.0  
> **Status:** Active  

This document provides a comprehensive, structured reference of all HTTP API endpoints across the Forge platform. Each entry contains the HTTP method, endpoint path, authentication requirements, a concise functional description, request parameters/payload, and expected responses.

---

## 1. Global Conventions & Architecture

### Base URLs & Route Prefixes
All endpoints are available under the versioned prefix `/api/v1`, the legacy `/api` prefix, or the direct root prefix:
- **Primary / Recommended:** `http://localhost:9000/api/v1` (Production: `https://api.forge.dev/api/v1`)
- **Compatibility Aliases:** `http://localhost:9000/api` and `http://localhost:9000`

### Common Headers
| Header | Requirement | Value / Format |
|---|---|---|
| `Content-Type` | Required for POST/PUT/PATCH | `application/json` |
| `Authorization` | Required for authenticated routes | `Bearer <access_token>` |
| `Organization-ID` | Optional / Contextual | `<organization_uuid>` (Multi-tenant context) |
| `x-request-id` | Optional (Request & Response) | Correlation UUID (allowed and exposed via CORS) |
| `Accept` | Optional | `application/json` (or `text/event-stream` for SSE) |

### Standard Response Envelope
- **Success (200 OK / 201 Created):**
  ```json
  {
    "message": "Operation completed successfully.",
    "data": { ... }
  }
  ```
- **Paginated List (200 OK):**
  ```json
  {
    "message": "Resource list retrieved successfully.",
    "data": [ ... ],
    "pagination": { "page": 1, "limit": 20, "total": 100 }
  }
  ```
- **Error Response (4xx / 5xx):**
  ```json
  {
    "is_error": true,
    "code": "ERROR_CODE",
    "message": "Human-readable description.",
    "errors": { "field": ["validation error detail"] }
  }
  ```

---

## 2. Authentication APIs (`/auth`)

### 1. `POST /auth/register`
- **Description:** Registers a new user account and sets up the default user role.
- **Auth:** 🌐 Public (No token)
- **Request Body (JSON):**
  - `name` *(string, required)*: Full name of the user.
  - `email` *(string, required, format: email)*: Unique email address.
  - `password` *(string, required, min: 8)*: User password.
- **Success (201 Created):** Returns user ID, name, email, and timestamp.
- **Errors:** `400 Bad Request` (validation), `409 Conflict` (`AUTH_001` email already registered).

### 2. `POST /auth/login`
- **Description:** Authenticates credentials and returns a short-lived JWT access token and a refresh token.
- **Auth:** 🌐 Public
- **Request Body (JSON):**
  - `email` *(string, required)*
  - `password` *(string, required)*
- **Success (200 OK):** Returns `access_token`, `refresh_token`, `token_type: "Bearer"`, `expires_in: 900`, and user summary.
- **Errors:** `401 Unauthorized` (`AUTH_002` invalid credentials), `403 Forbidden` (`AUTH_003` email unverified or inactive).

### 3. `POST /auth/logout`
- **Description:** Invalidates current session and revokes the active refresh token.
- **Auth:** ✅ Authenticated (Bearer Token)
- **Request Body (JSON, Optional):**
  - `refresh_token` *(string, optional)*: Specific refresh token to revoke.
- **Success (200 OK):** Session invalidated message.

### 4. `POST /auth/refresh`
- **Description:** Issues a new JWT access token using a valid, non-expired refresh token.
- **Auth:** 🌐 Public / Refresh Token Header
- **Request Body (JSON):**
  - `refresh_token` *(string, required)*: Valid refresh token.
- **Success (200 OK):** New `access_token` and updated expiration timestamp.
- **Errors:** `401 Unauthorized` (`AUTH_004` expired or revoked refresh token).

### 5. `GET /auth/me`
- **Description:** Retrieves the identity, role, permissions, and profile of the currently logged-in user.
- **Auth:** ✅ Authenticated (Bearer Token)
- **Success (200 OK):** User details, assigned roles, system permissions, and organization memberships.

### 6. `POST /auth/forgot-password`
- **Description:** Initiates password reset flow by sending a secure reset token via email.
- **Auth:** 🌐 Public
- **Request Body (JSON):**
  - `email` *(string, required)*
- **Success (200 OK):** Generic success confirmation (prevents email enumeration).

### 7. `POST /auth/reset-password`
- **Description:** Resets user password using the verification token received via email.
- **Auth:** 🌐 Public
- **Request Body (JSON):**
  - `token` *(string, required)*: Verification token.
  - `new_password` *(string, required, min: 8)*: New password.
- **Success (200 OK):** Password reset confirmation.
- **Errors:** `400 Bad Request` (`AUTH_005` invalid or expired token).

### 8. `POST /auth/verify-email` (or `GET /auth/verify-email`)
- **Description:** Verifies user email address using the token link from registration.
- **Auth:** 🌐 Public
- **Request (Query or JSON):**
  - `token` *(string, required)*: Verification token.
- **Success (200 OK):** Email confirmed successfully.

---

## 3. Access Control APIs (`/access-control`)

*All Access Control endpoints require **System Admin** role.*

### Roles Management
- **`GET /access-control/roles`**
  - **Description:** List all defined system roles (paginated).
  - **Query:** `page`, `limit`, `search`.
  - **Success (200 OK):** Array of role objects (`id`, `name`, `description`, `is_system`, `created_at`).
- **`POST /access-control/roles`**
  - **Description:** Create a new custom role.
  - **Body (JSON):** `name` *(string)*, `description` *(string, optional)*.
  - **Success (201 Created):** Created role object.
- **`GET /access-control/roles/{id}`**
  - **Description:** Fetch detailed role information by ID.
  - **Success (200 OK):** Role object with attached permissions.
- **`PATCH /access-control/roles/{id}` (or `PUT`)**
  - **Description:** Update role name and description.
  - **Body (JSON):** `name` *(optional)*, `description` *(optional)*.
  - **Success (200 OK):** Updated role object.
- **`DELETE /access-control/roles/{id}`**
  - **Description:** Delete a custom role (built-in system roles cannot be deleted).
  - **Success (200 OK):** Deletion confirmation.

### Permissions Management
- **`GET /access-control/permissions`**
  - **Description:** List all available granular system permissions.
  - **Success (200 OK):** Array of permission objects (`id`, `name`, `code`, `module`, `description`).
- **`POST /access-control/permissions`**
  - **Description:** Register a new atomic system permission.
  - **Body (JSON):** `name`, `code`, `module`, `description`.
  - **Success (201 Created):** Created permission object.
- **`GET /access-control/permissions/{id}`**
  - **Description:** Fetch permission details by ID.
  - **Success (200 OK):** Permission object.
- **`PATCH /access-control/permissions/{id}`**
  - **Description:** Update an existing permission definition.
  - **Body (JSON):** `name`, `description`.
  - **Success (200 OK):** Updated permission object.
- **`DELETE /access-control/permissions/{id}`**
  - **Description:** Delete an atomic permission.
  - **Success (200 OK):** Deletion confirmation.

### Role-Permission Assignments
- **`POST /access-control/roles/permissions/assign`**
  - **Description:** Map one or more permissions to a specific role.
  - **Body (JSON):** `role_id` *(UUID)*, `permission_ids` *(array of UUIDs)*.
  - **Success (200 OK):** Assignment confirmation.
- **`POST /access-control/roles/permissions/remove`**
  - **Description:** Unlink permissions from a role.
  - **Body (JSON):** `role_id` *(UUID)*, `permission_ids` *(array of UUIDs)*.
  - **Success (200 OK):** Removal confirmation.
- **`GET /access-control/roles/permissions/{id}`**
  - **Description:** List all permissions assigned to a given role ID.
  - **Success (200 OK):** Array of assigned permissions.

### User-Role Assignments
- **`POST /access-control/role/assign`**
  - **Description:** Assign one or more roles to a target user.
  - **Body (JSON):** `user_id` *(UUID)*, `role_ids` *(array of UUIDs)*.
  - **Success (200 OK):** Assignment confirmation.
- **`POST /access-control/role/remove`**
  - **Description:** Revoke roles from a user.
  - **Body (JSON):** `user_id` *(UUID)*, `role_ids` *(array of UUIDs)*.
  - **Success (200 OK):** Revocation confirmation.
- **`GET /access-control/role/user/{id}`**
  - **Description:** Get all roles currently assigned to a user ID.
  - **Success (200 OK):** List of assigned roles.

### Direct User-Permission Overrides
- **`POST /access-control/users/permission/assign`**
  - **Description:** Grant direct atomic permissions to a specific user (overriding roles).
  - **Body (JSON):** `user_id` *(UUID)*, `permission_ids` *(array of UUIDs)*.
  - **Success (200 OK):** Grant confirmation.
- **`POST /access-control/users/permission/remove`**
  - **Description:** Remove direct permission overrides from a user.
  - **Body (JSON):** `user_id` *(UUID)*, `permission_ids` *(array of UUIDs)*.
  - **Success (200 OK):** Removal confirmation.
- **`GET /access-control/users/permissions/{id}`**
  - **Description:** List all direct permissions granted to a user ID.
  - **Success (200 OK):** List of direct permissions.

---

## 4. Users Module APIs (`/users`)

### 1. `GET /users`
- **Description:** Retrieve paginated list of registered users.
- **Auth:** 🔒 System Admin
- **Query:** `page`, `limit`, `search`, `status`.
- **Success (200 OK):** Paginated array of user accounts.

### 2. `POST /users`
- **Description:** Manually create an administrative or standard user account.
- **Auth:** 🔒 System Admin
- **Body (JSON):** `name`, `email`, `password`, `role_ids` *(array)*.
- **Success (201 Created):** Created user details.

### 3. `GET /users/{id}`
- **Description:** Retrieve basic user account details by ID.
- **Auth:** ✅ Self or System Admin
- **Success (200 OK):** User record (`id`, `name`, `email`, `created_at`, `status`).

### 4. `PATCH /users/{id}` (or `PUT`)
- **Description:** Update account details (name, email, status).
- **Auth:** ✅ Self or System Admin
- **Body (JSON):** `name` *(optional)*, `email` *(optional)*, `status` *(optional, Admin only)*.
- **Success (200 OK):** Updated user record.

### 5. `DELETE /users/{id}`
- **Description:** Soft-delete or purge user account.
- **Auth:** 🔒 Self or System Admin
- **Success (200 OK):** Account deleted confirmation.

### 6. `GET /users/{id}/profile`
- **Description:** Retrieve extended user profile (bio, avatar URL, preferences).
- **Auth:** ✅ Authenticated
- **Success (200 OK):** Profile object.

### 7. `PUT /users/{id}/profile`
- **Description:** Update user profile attributes.
- **Auth:** ✅ Self
- **Body (JSON):** `bio` *(optional)*, `avatar_url` *(optional)*, `github_handle` *(optional)*.
- **Success (200 OK):** Updated profile object.

---

## 5. Notifications Module APIs (`/notifications`)

### 1. `GET /notifications`
- **Description:** List paginated in-app notifications for the authenticated user.
- **Auth:** ✅ Authenticated
- **Query:** `page`, `limit`, `unread_only` *(boolean)*.
- **Success (200 OK):** Paginated notifications (`id`, `type`, `title`, `message`, `read_at`, `created_at`).

### 2. `GET /notifications/unread-count`
- **Description:** Get the total count of unread notifications for the active user.
- **Auth:** ✅ Authenticated
- **Success (200 OK):** `{ "unread_count": 5 }`.

### 3. `PATCH /notifications/{id}/read`
- **Description:** Mark a specific notification as read.
- **Auth:** ✅ Authenticated (Owner of notification)
- **Success (200 OK):** Updated notification with `read_at` timestamp.

### 4. `PATCH /notifications/read-all`
- **Description:** Mark all unread notifications for the user as read.
- **Auth:** ✅ Authenticated
- **Success (200 OK):** Count of notifications marked as read.

### 5. `DELETE /notifications/{id}`
- **Description:** Dismiss or remove a single notification.
- **Auth:** ✅ Authenticated (Owner)
- **Success (200 OK):** Notification dismissed.

### 6. `GET /notifications/stream`
- **Description:** Server-Sent Events (SSE) connection for real-time live notification pushes.
- **Auth:** ✅ Authenticated
- **Headers:** `Accept: text/event-stream`
- **Success (200 OK):** Continuous SSE stream (`event: notification\ndata: {...}\n\n`).

### 7. `POST /notifications/internal`
- **Description:** Internal system endpoint for backend services (deployments, build worker) to trigger user alerts.
- **Auth:** ⚙️ Internal Service Token
- **Body (JSON):** `user_id`, `type`, `title`, `message`, `metadata`.
- **Success (201 Created):** Created notification.

---

## 6. Organization Module APIs (`/organizations`)

### Organization Lifecycle
- **`POST /organizations`**
  - **Description:** Create a new tenant organization; creator automatically becomes Org Owner.
  - **Auth:** ✅ Authenticated
  - **Body (JSON):** `name` *(string, required)*, `slug` *(string, optional)*, `description` *(string, optional)*.
  - **Success (201 Created):** Organization object.
- **`GET /organizations`**
  - **Description:** List all organizations where the active user is a member.
  - **Auth:** ✅ Authenticated
  - **Success (200 OK):** Array of organizations with the user's role in each.
- **`GET /organizations/{id}`**
  - **Description:** Fetch detailed metadata for a specific organization.
  - **Auth:** ✅ Org Member (Viewer+)
  - **Success (200 OK):** Organization details and metrics.
- **`PATCH /organizations/{id}` (or `PUT`)**
  - **Description:** Update organization settings (name, avatar, description).
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Body (JSON):** `name`, `description`, `slug`.
  - **Success (200 OK):** Updated organization.
- **`DELETE /organizations/{id}`**
  - **Description:** Soft-delete or archive organization and its child resources.
  - **Auth:** 🔒 Org Owner only
  - **Success (200 OK):** Organization deletion confirmation.

### Organization Invitations & Members
- **`POST /organizations/{id}/invitations`**
  - **Description:** Invite a user by email to join the organization with a specified role.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Body (JSON):** `email` *(string)*, `role` *(string: "admin" | "developer" | "viewer")*.
  - **Success (201 Created):** Invitation record with invite token and expiration.
- **`GET /organizations/{id}/invitations`**
  - **Description:** List pending invitations for the organization.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Success (200 OK):** Array of pending invitations.
- **`POST /organizations/invitations/{token}/accept`**
  - **Description:** Accept an invitation using the secure invite token.
  - **Auth:** ✅ Authenticated
  - **Success (200 OK):** Membership confirmation.
- **`GET /organizations/{id}/members`**
  - **Description:** List all members of the organization with their roles.
  - **Auth:** ✅ Org Member (Viewer+)
  - **Query:** `page`, `limit`, `role`.
  - **Success (200 OK):** Paginated array of members.
- **`PATCH /organizations/{id}/members/{user_id}`**
  - **Description:** Change a member's organizational role.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Body (JSON):** `role` *(string: "admin" | "developer" | "viewer")*.
  - **Success (200 OK):** Updated membership record.
- **`DELETE /organizations/{id}/members/{user_id}`**
  - **Description:** Remove a member from the organization.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Success (200 OK):** Removal confirmation.

---

## 7. Teams Module APIs (`/teams`)

### Team Management
- **`POST /teams` (or `POST /organizations/{id}/teams`)**
  - **Description:** Create a new team within an organization.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Body (JSON):** `organization_id` *(UUID)*, `name` *(string)*, `description` *(string, optional)*.
  - **Success (201 Created):** Team object.
- **`GET /teams` (or `GET /organizations/{id}/teams`)**
  - **Description:** List all teams within an organization.
  - **Auth:** ✅ Org Member (Viewer+)
  - **Success (200 OK):** Array of teams with member counts.
- **`GET /teams/{id}`**
  - **Description:** Get team metadata and member list by ID.
  - **Auth:** ✅ Org Member (Viewer+)
  - **Success (200 OK):** Team details.
- **`PATCH /teams/{id}` (or `PUT`)**
  - **Description:** Update team name and description.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Body (JSON):** `name`, `description`.
  - **Success (200 OK):** Updated team object.
- **`DELETE /teams/{id}`**
  - **Description:** Delete a team.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Success (200 OK):** Deletion confirmation.

### Team Members
- **`POST /teams/{id}/members`**
  - **Description:** Add a user to a team.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Body (JSON):** `user_id` *(UUID)*, `team_role` *(string, optional)*.
  - **Success (201 Created):** Team membership record.
- **`GET /teams/{id}/members`**
  - **Description:** List all members assigned to the team.
  - **Auth:** ✅ Org Member (Viewer+)
  - **Success (200 OK):** Array of team members.
- **`DELETE /teams/{id}/members/{user_id}`**
  - **Description:** Remove a user from a team.
  - **Auth:** 🔒 Org Admin / Org Owner
  - **Success (200 OK):** Removal confirmation.

---

## 8. Projects Module APIs (`/projects`)

### Project Lifecycle
- **`GET /projects`**
  - **Description:** List all projects accessible to the authenticated user within their organization.
  - **Auth:** ✅ Viewer+
  - **Query:** `organization_id`, `page`, `limit`, `search`.
  - **Success (200 OK):** Paginated project list with repository details and last deployment status.
- **`POST /projects`**
  - **Description:** Create a new project under an organization.
  - **Auth:** 🔒 Developer+
  - **Body (JSON):**
    - `organization_id` *(UUID, required)*
    - `name` *(string, required)*
    - `description` *(string, optional)*
    - `framework` *(string, optional, e.g. "rust", "nodejs", "docker")*
    - `build_command` *(string, optional)*
    - `run_command` *(string, optional)*
  - **Success (201 Created):** Project record.
- **`GET /projects/{id}`**
  - **Description:** Fetch comprehensive project information by ID.
  - **Auth:** ✅ Viewer+
  - **Success (200 OK):** Project details, repository status, deployment counters, and active config.
- **`PATCH /projects/{id}` (or `PUT`)**
  - **Description:** Update project settings (build commands, framework, name).
  - **Auth:** 🔒 Developer+
  - **Body (JSON):** `name`, `description`, `build_command`, `run_command`, `framework`.
  - **Success (200 OK):** Updated project object.
- **`DELETE /projects/{id}`**
  - **Description:** Delete a project and tear down linked resources.
  - **Auth:** 🔒 Project Owner / Org Admin
  - **Success (200 OK):** Project deletion confirmation.

---

## 9. Repository Sub-Module APIs (`/projects/{id}/repository`)

- **`POST /projects/{id}/repository`**
  - **Description:** Connect a Git repository to the project. If a repository is already connected, it idempotently updates and reconnects the repository configuration. Authentication type is automatically derived (`pat` when `access_token` is provided, otherwise `none`).
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Body (JSON):**
    - `repository_url` *(string, required, min: 5)*: Valid Git repository URL.
    - `access_token` *(string, optional)*: Personal access token (PAT) for private repositories (stored encrypted, returned masked).
    - `default_branch` *(string, optional, default: "main")*: Target working branch.
  - **Success (200 OK):** Connected repository configuration with masked token (`••••••••`) and `status: "connected"`.
  - **Errors:** `400 Bad Request` (validation error), `401 Unauthorized`, `404 Not Found`.

- **`GET /projects/{id}/repository`**
  - **Description:** Retrieve currently linked Git repository details and connection status. Sensitive tokens are always masked in responses.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Success (200 OK):** Repository record (`id`, `project_id`, `repository_url`, `access_token: "••••••••"`, `default_branch`, `status: "connected" | "disconnected"`).
  - **Errors:** `401 Unauthorized`, `404 Not Found`.

- **`PATCH /projects/{id}/repository`**
  - **Description:** Update repository configuration parameters such as repository URL, access token, or default branch. If a new access token is provided, it is encrypted and authentication is set to PAT.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Body (JSON):**
    - `repository_url` *(string, optional, min: 5)*
    - `access_token` *(string, optional)*
    - `default_branch` *(string, optional)*
  - **Success (200 OK):** Updated repository configuration.
  - **Errors:** `400 Bad Request`, `401 Unauthorized`, `404 Not Found`.

- **`DELETE /projects/{id}/repository`**
  - **Description:** Disconnect Git repository from project, setting `status` to `disconnected` and erasing the stored access token.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Success (200 OK):** Disconnected repository response with cleared token and `status: "disconnected"`.
  - **Errors:** `401 Unauthorized`, `404 Not Found`.

---

## 10. Environment Variables APIs (`/projects/{id}/env-vars`)

- **`POST /projects/{id}/env-vars`**
  - **Description:** Create an environment variable for a project within a specific deployment environment (`development`, `staging`, `production`).
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Body (JSON):**
    - `key` *(string, required, max: 255)*: Environment variable key.
    - `value` *(string, required)*: Environment variable value.
    - `environment` *(string, required, enum: `"development"` | `"staging"` | `"production"`)*
    - `is_secret` *(boolean, optional, default: false)*: Secret flag.
  - **Success (201 Created):** Created variable metadata including `environment`, `key`, `value`, `is_secret`, timestamps.
  - **Errors:** `400 Bad Request`, `401 Unauthorized`, `409 Conflict` (if key already exists in specified environment).

- **`POST /projects/{id}/env-vars/bulk`**
  - **Description:** Batch create or upsert multiple environment variables simultaneously across environments. If a variable with the same key and environment exists, its value and settings are updated.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Body (JSON):**
    - `vars` *(array of objects, required)*:
      - `key` *(string, required)*
      - `value` *(string, required)*
      - `environment` *(string, required, enum: `"development"` | `"staging"` | `"production"`)*
      - `is_secret` *(boolean, optional, default: false)*
  - **Success (201 Created):** Array of created or updated environment variable records.
  - **Errors:** `400 Bad Request`, `401 Unauthorized`.

- **`GET /projects/{id}/env-vars`**
  - **Description:** List environment variables configured for a project. Supports optional filtering by environment.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Query Parameters:**
    - `environment` *(string, optional, enum: `"development"` | `"staging"` | `"production"`)*
  - **Success (200 OK):** Array of environment variable records.
  - **Errors:** `401 Unauthorized`, `404 Not Found`.

- **`PATCH /projects/{id}/env-vars/{env_id}`**
  - **Description:** Partially update an existing environment variable's value or secrecy status.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Body (JSON):**
    - `value` *(string, optional)*: New variable value.
    - `is_secret` *(boolean, optional)*: Updated secrecy status.
  - **Success (200 OK):** Updated environment variable record.
  - **Errors:** `400 Bad Request`, `401 Unauthorized`, `404 Not Found`.

- **`DELETE /projects/{id}/env-vars/{env_id}`**
  - **Description:** Permanently remove an environment variable.
  - **Auth:** 🔒 Authenticated (Bearer Token)
  - **Success (200 OK):** Variable deleted confirmation.
  - **Errors:** `401 Unauthorized`, `404 Not Found`.

---

## 11. Project Assignments APIs (`/projects/{id}/members` & `/projects/{id}/teams`)

- **`POST /projects/{id}/members`**
  - **Description:** Explicitly assign a user directly to a project with a specific project role.
  - **Auth:** 🔒 Project Owner / Org Admin
  - **Body (JSON):** `user_id` *(UUID)*, `role` *(string: "admin" | "developer" | "viewer")*.
  - **Success (201 Created):** Assignment record.
- **`GET /projects/{id}/members`**
  - **Description:** List all users directly assigned to the project.
  - **Auth:** ✅ Viewer+
  - **Success (200 OK):** Array of assigned members with their project roles.
- **`DELETE /projects/{id}/members/{user_id}`**
  - **Description:** Unassign a user from the project.
  - **Auth:** 🔒 Project Owner / Org Admin
  - **Success (200 OK):** Removal confirmation.
- **`POST /projects/{id}/teams`**
  - **Description:** Assign an entire team to the project, granting team members inherited access.
  - **Auth:** 🔒 Project Owner / Org Admin
  - **Body (JSON):** `team_id` *(UUID)*, `role` *(string: "developer" | "viewer")*.
  - **Success (201 Created):** Team assignment record.
- **`GET /projects/{id}/teams`**
  - **Description:** List all teams assigned to the project.
  - **Auth:** ✅ Viewer+
  - **Success (200 OK):** Array of assigned teams.
- **`DELETE /projects/{id}/teams/{team_id}`**
  - **Description:** Remove team assignment from the project.
  - **Auth:** 🔒 Project Owner / Org Admin
  - **Success (200 OK):** Removal confirmation.

---

## 12. Deployments & Build Worker APIs (`/projects/{id}/deployments`)

- **`POST /projects/{id}/deployments`**
  - **Description:** Triggers a new asynchronous build and deployment pipeline for the specified project.
  - **Auth:** 🔒 Authenticated (Bearer Token, Developer+)
  - **Path Parameter:** `id` *(UUID, project ID)*
  - **Body (JSON):**
    - `branch` *(string, optional, defaults to project branch)*
    - `commit_hash` *(string, optional)*
  - **Success (201 Created):** `{"message": "Deployment triggered successfully.", "data": DeploymentResponse}` with `status: "Queued"`.
- **`GET /projects/{id}/deployments`**
  - **Description:** List paginated historical deployments for a given project.
  - **Auth:** ✅ Project Viewer+ (Bearer Token)
  - **Path Parameter:** `id` *(UUID, project ID)*
  - **Query:** `page`, `limit`, `status`, `branch`.
  - **Success (200 OK):** `{"message": "Deployments retrieved successfully.", "data": PaginatedResponse<DeploymentResponse>}`.
- **`GET /projects/{id}/deployments/{deployment_id}`**
  - **Description:** Fetch detailed status, timing, commit info, and execution metrics for a specific deployment.
  - **Auth:** ✅ Project Viewer+ (Bearer Token)
  - **Path Parameters:** `id` *(UUID, project ID)*, `deployment_id` *(UUID, deployment ID)*
  - **Success (200 OK):** `{"message": "Deployment details retrieved successfully.", "data": DeploymentResponse}`.
- **`POST /projects/{id}/deployments/{deployment_id}/redeploy`**
  - **Description:** Re-triggers a deployment using the identical commit and configuration of a past deployment.
  - **Auth:** 🔒 Authenticated (Bearer Token, Developer+)
  - **Path Parameters:** `id` *(UUID, project ID)*, `deployment_id` *(UUID, deployment ID)*
  - **Success (201 Created):** `{"message": "Redeploy triggered successfully.", "data": DeploymentResponse}`.
- **`POST /projects/{id}/deployments/rollback`**
  - **Description:** Rollback project to the most recent known successful deployment.
  - **Auth:** 🔒 Authenticated (Bearer Token, Project Owner / Org Admin)
  - **Path Parameter:** `id` *(UUID, project ID)*
  - **Success (201 Created):** `{"message": "Rollback deployment triggered successfully.", "data": DeploymentResponse}`.
- **`PUT /projects/internal/deployments/{deployment_id}/status`**
  - **Description:** Internal endpoint used by Build Worker to update execution stages and transitions (`Queued` -> `Building` -> `Deploying` -> `Running` -> `Success` | `Failed`).
  - **Auth:** ⚙️ Build Worker Service Token (`x-service-token` header)
  - **Path Parameter:** `deployment_id` *(UUID, deployment ID)*
  - **Body (JSON):**
    - `status` *(string, required: `"Queued"` | `"Building"` | `"Deploying"` | `"Running"` | `"Failed"` | `"Success"`)*
    - `build_duration` *(integer, optional, in seconds)*
    - `deploy_duration` *(integer, optional, in seconds)*
    - `error_message` *(string, optional)*
  - **Success (200 OK):** `{"message": "Deployment status updated successfully.", "data": DeploymentResponse}`.

---

## 13. Live Build Logs APIs (`/deployments/{id}/logs`)

- **`GET /deployments/{id}/logs/stream`**
  - **Description:** Real-time Server-Sent Events (SSE) stream of live container build and run logs.
  - **Auth:** ✅ Project Viewer+
  - **Headers:** `Accept: text/event-stream`
  - **Success (200 OK):** Continuous SSE stream (`data: {"line": 102, "text": "Compiling forge v0.1.0..."}\n\n`).
- **`GET /deployments/{id}/logs`**
  - **Description:** Query stored historical build logs with pagination and stage filtering.
  - **Auth:** ✅ Project Viewer+
  - **Query:** `stage` *(e.g. "build", "run")*, `limit`, `offset`.
  - **Success (200 OK):** Array of log lines with timestamps and levels.
- **`POST /deployments/{id}/logs`**
  - **Description:** Internal ingestion endpoint for Build Worker / Docker runner to write log chunks.
  - **Auth:** ⚙️ Build Worker Service Token
  - **Body (JSON):** `lines` *(array of log line objects)*.
  - **Success (200 OK):** Log lines ingested.
- **`GET /deployments/{id}/logs/search`**
  - **Description:** Full-text keyword search across deployment log history.
  - **Auth:** ✅ Project Viewer+
  - **Query:** `q` *(search term, required)*.
  - **Success (200 OK):** Matched log lines with line numbers and contexts.
- **`GET /deployments/{id}/logs/download`**
  - **Description:** Download the complete raw build and deployment log as a single plain text file (`.log`).
  - **Auth:** ✅ Project Viewer+
  - **Success (200 OK):** Streamed `text/plain` file download (`Content-Disposition: attachment; filename="deployment-<id>.log"`).

---

## 14. Dashboard Module APIs (`/dashboard`)

- **`GET /dashboard`**
  - **Description:** System-wide aggregated metrics overview (total users, active deployments, system health).
  - **Auth:** 🔒 System Admin
  - **Success (200 OK):** Aggregated system metrics.
- **`GET /dashboard/user`**
  - **Description:** Personalized user dashboard containing recent projects, personal deployment activities, and pending invites.
  - **Auth:** ✅ Authenticated User
  - **Success (200 OK):** User-centric dashboard data.
- **`GET /dashboard/org/{org_id}`**
  - **Description:** Aggregated organizational metrics (team counts, active projects, deployment failure rates, resource consumption).
  - **Auth:** ✅ Org Member (Viewer+)
  - **Success (200 OK):** Organization dashboard metrics.

---

## 15. Health & Observability APIs (`/health`)

- **`GET /health/live` (or `GET /health`)**
  - **Description:** Kubernetes liveness probe — verifies the HTTP server process is running.
  - **Auth:** 🌐 Public
  - **Success (200 OK):** `{ "status": "ok", "uptime_seconds": 12450 }`.
- **`GET /health/ready`**
  - **Description:** Kubernetes readiness probe — verifies critical dependencies (PostgreSQL database pool & RabbitMQ broker) are reachable before receiving traffic.
  - **Auth:** 🌐 Public
  - **Success (200 OK):** `{ "status": "ready", "database": "up", "rabbitmq": "up" }`.
  - **Errors:** `503 Service Unavailable` if PostgreSQL or RabbitMQ is unreachable.
- **`GET /health/deep` (or `GET /health/details`)**
  - **Description:** Deep diagnostic probe checking DB pool latency, Redis cache, RabbitMQ queues, Loki logging pipeline, and Docker daemon connectivity.
  - **Auth:** 🔒 System Admin
  - **Success (200 OK):** Complete subsystem diagnostics object with latencies and memory usage.

---

## 16. Documentation & Swagger UI (`/docs`)

- **`GET /docs`**
  - **Description:** Interactive Swagger UI web interface for testing and exploring the API.
  - **Auth:** 🌐 Public
  - **Success (200 OK):** HTML/JS Swagger UI application.
- **`GET /docs/openapi.yaml`**
  - **Description:** Serves the raw machine-readable OpenAPI 3.0 specification file.
  - **Auth:** 🌐 Public
  - **Success (200 OK):** `application/x-yaml` payload containing the complete OpenAPI contract.

---

## Quick Reference Summary Table

| Category | Method | Path | Short Description | Min Auth |
|---|---|---|---|---|
| **Auth** | `POST` | `/auth/register` | Register new user account | Public |
| **Auth** | `POST` | `/auth/login` | Authenticate & issue JWT tokens | Public |
| **Auth** | `POST` | `/auth/logout` | Revoke session & refresh token | Authenticated |
| **Auth** | `POST` | `/auth/refresh` | Issue new access token | Public |
| **Auth** | `GET` | `/auth/me` | Fetch active user identity & roles | Authenticated |
| **Auth** | `POST` | `/auth/forgot-password` | Request password reset token | Public |
| **Auth** | `POST` | `/auth/reset-password` | Set new password with token | Public |
| **Auth** | `POST` | `/auth/verify-email` | Confirm email verification token | Public |
| **Access Control** | `GET` | `/access-control/roles` | List all system roles | System Admin |
| **Access Control** | `POST` | `/access-control/roles` | Create new system role | System Admin |
| **Access Control** | `GET` | `/access-control/roles/{id}` | Get role details | System Admin |
| **Access Control** | `PATCH` | `/access-control/roles/{id}` | Update role details | System Admin |
| **Access Control** | `DELETE` | `/access-control/roles/{id}` | Delete custom role | System Admin |
| **Access Control** | `GET` | `/access-control/permissions` | List all atomic permissions | System Admin |
| **Access Control** | `POST` | `/access-control/permissions` | Create new atomic permission | System Admin |
| **Access Control** | `GET` | `/access-control/permissions/{id}` | Get permission details | System Admin |
| **Access Control** | `PATCH` | `/access-control/permissions/{id}` | Update permission details | System Admin |
| **Access Control** | `DELETE` | `/access-control/permissions/{id}` | Delete permission | System Admin |
| **Access Control** | `POST` | `/access-control/roles/permissions/assign` | Map permissions to role | System Admin |
| **Access Control** | `POST` | `/access-control/roles/permissions/remove` | Unmap permissions from role | System Admin |
| **Access Control** | `GET` | `/access-control/roles/permissions/{id}` | List permissions for role | System Admin |
| **Access Control** | `POST` | `/access-control/role/assign` | Assign roles to user | System Admin |
| **Access Control** | `POST` | `/access-control/role/remove` | Revoke roles from user | System Admin |
| **Access Control** | `GET` | `/access-control/role/user/{id}` | Get roles for user | System Admin |
| **Access Control** | `POST` | `/access-control/users/permission/assign` | Assign direct permission to user | System Admin |
| **Access Control** | `POST` | `/access-control/users/permission/remove` | Remove direct permission from user | System Admin |
| **Access Control** | `GET` | `/access-control/users/permissions/{id}` | Get direct permissions for user | System Admin |
| **Users** | `GET` | `/users` | List all users (paginated) | System Admin |
| **Users** | `POST` | `/users` | Create user account manually | System Admin |
| **Users** | `GET` | `/users/{id}` | Get user by ID | Self / Admin |
| **Users** | `PATCH` | `/users/{id}` | Update user account | Self / Admin |
| **Users** | `DELETE` | `/users/{id}` | Delete user account | Self / Admin |
| **Users** | `GET` | `/users/{id}/profile` | Get user profile | Authenticated |
| **Users** | `PUT` | `/users/{id}/profile` | Update personal profile | Self |
| **Notifications** | `GET` | `/notifications` | List user notifications | Authenticated |
| **Notifications** | `GET` | `/notifications/unread-count` | Get unread notification count | Authenticated |
| **Notifications** | `PATCH` | `/notifications/{id}/read` | Mark notification as read | Authenticated |
| **Notifications** | `PATCH` | `/notifications/read-all` | Mark all notifications read | Authenticated |
| **Notifications** | `DELETE` | `/notifications/{id}` | Dismiss notification | Authenticated |
| **Notifications** | `GET` | `/notifications/stream` | Stream live notifications (SSE) | Authenticated |
| **Notifications** | `POST` | `/notifications/internal` | Post notification from service | Internal Worker |
| **Organizations** | `POST` | `/organizations` | Create tenant organization | Authenticated |
| **Organizations** | `GET` | `/organizations` | List user organizations | Authenticated |
| **Organizations** | `GET` | `/organizations/{id}` | Get organization metadata | Org Viewer |
| **Organizations** | `PATCH` | `/organizations/{id}` | Update organization details | Org Admin |
| **Organizations** | `DELETE` | `/organizations/{id}` | Delete organization | Org Owner |
| **Org Members** | `POST` | `/organizations/{id}/invitations` | Send invitation by email | Org Admin |
| **Org Members** | `GET` | `/organizations/{id}/invitations` | List pending invitations | Org Admin |
| **Org Members** | `POST` | `/organizations/invitations/{token}/accept` | Accept invite by token | Authenticated |
| **Org Members** | `GET` | `/organizations/{id}/members` | List organization members | Org Viewer |
| **Org Members** | `PATCH` | `/organizations/{id}/members/{user_id}` | Update member role | Org Admin |
| **Org Members** | `DELETE` | `/organizations/{id}/members/{user_id}` | Remove member from org | Org Admin |
| **Teams** | `POST` | `/teams` | Create team in org | Org Admin |
| **Teams** | `GET` | `/teams` | List teams in org | Org Viewer |
| **Teams** | `GET` | `/teams/{id}` | Get team details & members | Org Viewer |
| **Teams** | `PATCH` | `/teams/{id}` | Update team details | Org Admin |
| **Teams** | `DELETE` | `/teams/{id}` | Delete team | Org Admin |
| **Teams** | `POST` | `/teams/{id}/members` | Add member to team | Org Admin |
| **Teams** | `GET` | `/teams/{id}/members` | List team members | Org Viewer |
| **Teams** | `DELETE` | `/teams/{id}/members/{user_id}` | Remove member from team | Org Admin |
| **Projects** | `GET` | `/projects` | List projects | Viewer |
| **Projects** | `POST` | `/projects` | Create new project | Developer |
| **Projects** | `GET` | `/projects/{id}` | Get project by ID | Viewer |
| **Projects** | `PATCH` | `/projects/{id}` | Update project config | Developer |
| **Projects** | `DELETE` | `/projects/{id}` | Delete project | Org Admin / Project Owner |
| **Repository** | `POST` | `/projects/{id}/repository/validate` | Test Git credentials & URL | Viewer |
| **Repository** | `POST` | `/projects/{id}/repository` | Save repository configuration | Developer |
| **Repository** | `GET` | `/projects/{id}/repository` | Get repository configuration | Viewer |
| **Repository** | `POST` | `/projects/{id}/repository/clone` | Trigger worker repo clone | Developer |
| **Repository** | `GET` | `/projects/{id}/repository/commit` | Fetch latest remote commit | Viewer |
| **Repository** | `PUT` | `/projects/{id}/repository/branch` | Change active branch | Developer |
| **Repository** | `GET` | `/projects/{id}/repository/branches` | List all remote branches | Viewer |
| **Env Vars** | `POST` | `/projects/{id}/env-vars` | Add encrypted variable | Developer |
| **Env Vars** | `POST` | `/projects/{id}/env-vars/bulk` | Bulk upsert variables | Developer |
| **Env Vars** | `GET` | `/projects/{id}/env-vars` | List variables (masked) | Viewer |
| **Env Vars** | `PUT` | `/projects/{id}/env-vars/{env_id}` | Update variable | Developer |
| **Env Vars** | `DELETE` | `/projects/{id}/env-vars/{env_id}` | Delete variable | Developer |
| **Env Vars** | `GET` | `/projects/{id}/env-vars/decrypt` | Decrypt secrets for runner | Internal Worker / Owner |
| **Assignments** | `POST` | `/projects/{id}/members` | Assign user to project | Project Owner / Admin |
| **Assignments** | `GET` | `/projects/{id}/members` | List assigned users | Viewer |
| **Assignments** | `DELETE` | `/projects/{id}/members/{user_id}` | Remove user from project | Project Owner / Admin |
| **Assignments** | `POST` | `/projects/{id}/teams` | Assign team to project | Project Owner / Admin |
| **Assignments** | `GET` | `/projects/{id}/teams` | List assigned teams | Viewer |
| **Assignments** | `DELETE` | `/projects/{id}/teams/{team_id}` | Remove team from project | Project Owner / Admin |
| **Deployments** | `POST` | `/projects/{id}/deployments` | Trigger async deployment | Developer |
| **Deployments** | `GET` | `/projects/{id}/deployments` | List project deployments | Viewer |
| **Deployments** | `GET` | `/projects/{id}/deployments/{deployment_id}` | Get deployment status | Viewer |
| **Deployments** | `POST` | `/projects/{id}/deployments/{deployment_id}/redeploy` | Redeploy past deployment | Developer |
| **Deployments** | `POST` | `/projects/{id}/deployments/rollback` | Rollback to healthy state | Project Owner / Admin |
| **Deployments** | `PUT` | `/projects/internal/deployments/{deployment_id}/status` | Update execution stage | Internal Build Worker |
| **Logs** | `GET` | `/deployments/{id}/logs/stream` | Stream live build logs (SSE) | Viewer |
| **Logs** | `GET` | `/deployments/{id}/logs` | Query stored logs | Viewer |
| **Logs** | `POST` | `/deployments/{id}/logs` | Ingest worker log batch | Internal Build Worker |
| **Logs** | `GET` | `/deployments/{id}/logs/search` | Search log text | Viewer |
| **Logs** | `GET` | `/deployments/{id}/logs/download` | Download raw `.log` file | Viewer |
| **Dashboard** | `GET` | `/dashboard` | System-wide platform metrics | System Admin |
| **Dashboard** | `GET` | `/dashboard/user` | User personalized overview | Authenticated |
| **Dashboard** | `GET` | `/dashboard/org/{org_id}` | Org operational metrics | Org Viewer |
| **Health** | `GET` | `/health/live` | Liveness probe | Public |
| **Health** | `GET` | `/health/ready` | Readiness probe (DB + Queue) | Public |
| **Health** | `GET` | `/health/deep` | Deep multi-subsystem probe | System Admin |
| **Documentation** | `GET` | `/docs` | Interactive Swagger UI | Public |
| **Documentation** | `GET` | `/docs/openapi.yaml` | Raw OpenAPI 3.0 specification | Public |
