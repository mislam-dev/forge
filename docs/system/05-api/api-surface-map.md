# API Surface Map

> **Document:** API Surface Map  
> **Section:** 05 — API  
> **Version:** 1.0  
> **Status:** Draft

This document provides a **complete inventory of all API endpoints** across every Forge module. Use this as a quick reference for routing, minimum role requirements, and which module owns each endpoint.

---

## Legend

| Symbol | Meaning                                      |
| ------ | -------------------------------------------- |
| ✅     | All authenticated roles can access           |
| 🔒     | Requires minimum role as specified           |
| ⚙️     | Internal endpoint — service token only       |
| 🌐     | Public endpoint — no authentication required |

---

## 1. Auth Module

| Method | Endpoint                    | Description                                  | Auth             |
| ------ | --------------------------- | -------------------------------------------- | ---------------- |
| `POST` | `/auth/register`            | Register a new user                          | 🌐 Public        |
| `POST` | `/auth/login`               | Authenticate and receive JWT + refresh token | 🌐 Public        |
| `POST` | `/auth/logout`              | Invalidate current session                   | ✅ Authenticated |
| `POST` | `/auth/refresh`             | Exchange refresh token for new access token  | 🔒 Refresh token |
| `GET`  | `/auth/me`                  | Get current authenticated user info          | ✅ Authenticated |
| `POST` | `/auth/forgot-password`     | Request a password reset email               | 🌐 Public        |
| `POST` | `/auth/reset-password`      | Reset password using a valid reset token     | 🌐 Public        |
| `GET`  | `/auth/verify-email`        | Verify user email address via token link     | 🌐 Public        |

---

## 2. Access Control Module

> All endpoints restricted to **System Admin** only.

| Method   | Endpoint                                     | Description                         | Auth            |
| -------- | -------------------------------------------- | ----------------------------------- | --------------- |
| `GET`    | `/access-control/roles`                      | List all roles                      | 🔒 System Admin |
| `POST`   | `/access-control/roles`                      | Create a new role                   | 🔒 System Admin |
| `PUT`    | `/access-control/roles/:id`                  | Update a role                       | 🔒 System Admin |
| `DELETE` | `/access-control/roles/:id`                  | Delete a role                       | 🔒 System Admin |
| `GET`    | `/access-control/permission`                 | List all permissions                | 🔒 System Admin |
| `POST`   | `/access-control/permission`                 | Create a new permission             | 🔒 System Admin |
| `PUT`    | `/access-control/permission/:id`             | Update a permission                 | 🔒 System Admin |
| `DELETE` | `/access-control/permission/:id`             | Delete a permission                 | 🔒 System Admin |
| `POST`   | `/access-control/roles/permissions/assign`   | Assign permissions to a role        | 🔒 System Admin |
| `POST`   | `/access-control/roles/permissions/remove`   | Remove permissions from a role      | 🔒 System Admin |
| `GET`    | `/access-control/roles/permissions/:role_id` | Get permissions for a role          | 🔒 System Admin |
| `POST`   | `/access-control/role/assign`                | Assign roles to a user              | 🔒 System Admin |
| `POST`   | `/access-control/role/remove`                | Remove roles from a user            | 🔒 System Admin |
| `POST`   | `/access-control/users/permission/assign`    | Assign permissions directly to user | 🔒 System Admin |
| `POST`   | `/access-control/users/permission/remove`    | Remove direct permissions from user | 🔒 System Admin |
| `GET`    | `/access-control/users/permissions/:user_id` | Get user's direct permissions       | 🔒 System Admin |

---

## 3. Users Module

| Method   | Endpoint             | Description              | Auth             |
| -------- | -------------------- | ------------------------ | ---------------- |
| `GET`    | `/users`             | List all users           | 🔒 System Admin  |
| `GET`    | `/users/:id`         | Get user by ID           | ✅ Self or Admin |
| `PUT`    | `/users/:id`         | Update user profile      | ✅ Self or Admin |
| `DELETE` | `/users/:id`         | Delete user account      | 🔒 Self or Admin |
| `GET`    | `/users/:id/profile` | Get user profile details | ✅ Authenticated |
| `PUT`    | `/users/:id/profile` | Update user profile      | ✅ Self          |

---

## 4. Notifications Module

| Method   | Endpoint                         | Description                           | Auth             |
| -------- | -------------------------------- | ------------------------------------- | ---------------- |
| `GET`    | `/notifications`                 | List user's notifications (paginated) | ✅ Authenticated |
| `PATCH`  | `/notifications/:id/read`        | Mark single notification as read      | ✅ Authenticated |
| `PATCH`  | `/notifications/read-all`        | Mark all notifications as read        | ✅ Authenticated |
| `GET`    | `/notifications/unread-count`    | Get unread notification count         | ✅ Authenticated |
| `DELETE` | `/notifications/:id`             | Dismiss (soft-delete) a notification  | ✅ Authenticated |
| `GET`    | `/notifications/stream`          | Open real-time SSE notification stream| ✅ Authenticated |

> **Note:** `DELETE /notifications/:id` is marked for future removal — prefer marking notifications as dismissed via state rather than deleting.

> **Note:** `GET /notifications/stream` uses Server-Sent Events (SSE). Clients must set `Accept: text/event-stream`. Standard OpenAPI tooling does not render SSE streams — see API documentation for usage details.

---

## 5. Organization Module

| Method   | Endpoint             | Description               | Auth             | Min Role    |
| -------- | -------------------- | ------------------------- | ---------------- | ----------- |
| `POST`   | `/organizations`     | Create an organization    | ✅ Authenticated | Any         |
| `GET`    | `/organizations`     | List user's organizations | ✅ Authenticated | Any         |
| `GET`    | `/organizations/:id` | Get organization details  | ✅ Member        | Viewer      |
| `PUT`    | `/organizations/:id` | Update organization       | 🔒               | Admin/Owner |
| `DELETE` | `/organizations/:id` | Delete organization       | 🔒               | Owner       |

---

## 6. Org Members Sub-Module

| Method   | Endpoint                              | Description            | Auth | Min Role    |
| -------- | ------------------------------------- | ---------------------- | ---- | ----------- |
| `POST`   | `/organizations/:id/members`          | Add member to org      | 🔒   | Admin/Owner |
| `GET`    | `/organizations/:id/members`          | List org members       | ✅   | Viewer      |
| `PUT`    | `/organizations/:id/members/:user_id` | Update member role     | 🔒   | Admin/Owner |
| `DELETE` | `/organizations/:id/members/:user_id` | Remove member from org | 🔒   | Admin/Owner |

---

## 7. Teams Module

| Method   | Endpoint                      | Description           | Auth | Min Role    |
| -------- | ----------------------------- | --------------------- | ---- | ----------- |
| `POST`   | `/organizations/:id/teams`    | Create a team         | 🔒   | Admin/Owner |
| `GET`    | `/organizations/:id/teams`    | List org teams        | ✅   | Viewer      |
| `GET`    | `/teams/:id`                  | Get team details      | ✅   | Viewer      |
| `PUT`    | `/teams/:id`                  | Update team           | 🔒   | Admin/Owner |
| `DELETE` | `/teams/:id`                  | Delete team           | 🔒   | Admin/Owner |
| `POST`   | `/teams/:id/members`          | Add user to team      | 🔒   | Admin/Owner |
| `GET`    | `/teams/:id/members`          | List team members     | ✅   | Viewer      |
| `DELETE` | `/teams/:id/members/:user_id` | Remove user from team | 🔒   | Admin/Owner |

---

## 8. Projects Module

| Method   | Endpoint        | Description           | Auth | Min Role                        |
| -------- | --------------- | --------------------- | ---- | ------------------------------- |
| `GET`    | `/projects`     | List all org projects | ✅   | Viewer                          |
| `POST`   | `/projects`     | Create a project      | 🔒   | Developer                       |
| `GET`    | `/projects/:id` | Get project by ID     | ✅   | Viewer                          |
| `PUT`    | `/projects/:id` | Update project        | 🔒   | Developer                       |
| `DELETE` | `/projects/:id` | Delete project        | 🔒   | Developer (owner) / Admin/Owner |

---

## 9. Repository Sub-Module

| Method | Endpoint                            | Description              | Auth | Min Role  |
| ------ | ----------------------------------- | ------------------------ | ---- | --------- |
| `POST` | `/projects/:id/repository/validate` | Validate repo connection | ✅   | Viewer    |
| `POST` | `/projects/:id/repository`          | Connect/save repository  | 🔒   | Developer |
| `GET`  | `/projects/:id/repository`          | Get repository config    | ✅   | Viewer    |
| `POST` | `/projects/:id/repository/clone`    | Trigger clone            | 🔒   | Developer |
| `GET`  | `/projects/:id/repository/commit`   | Fetch latest commit      | ✅   | Viewer    |
| `PUT`  | `/projects/:id/repository/branch`   | Change active branch     | 🔒   | Developer |
| `GET`  | `/projects/:id/repository/branches` | List remote branches     | ✅   | Viewer    |

---

## 10. Environment Variables Sub-Module

| Method   | Endpoint                         | Description                 | Auth | Min Role                |
| -------- | -------------------------------- | --------------------------- | ---- | ----------------------- |
| `POST`   | `/projects/:id/env-vars`         | Create env variable         | 🔒   | Developer               |
| `GET`    | `/projects/:id/env-vars`         | List env variables (masked) | ✅   | Viewer                  |
| `PUT`    | `/projects/:id/env-vars/:env_id` | Update env variable         | 🔒   | Developer               |
| `DELETE` | `/projects/:id/env-vars/:env_id` | Delete env variable         | 🔒   | Developer               |
| `GET`    | `/projects/:id/env-vars/decrypt` | Decrypt secret values       | ⚙️   | Owner / Internal Runner |

---

## 11. Project Assignments Sub-Module

| Method   | Endpoint                         | Description              | Auth | Min Role                |
| -------- | -------------------------------- | ------------------------ | ---- | ----------------------- |
| `POST`   | `/projects/:id/members`          | Assign user to project   | 🔒   | Owner (project) / Admin |
| `GET`    | `/projects/:id/members`          | List project members     | ✅   | Viewer                  |
| `DELETE` | `/projects/:id/members/:user_id` | Remove user from project | 🔒   | Owner (project) / Admin |
| `POST`   | `/projects/:id/teams`            | Assign team to project   | 🔒   | Owner (project) / Admin |
| `GET`    | `/projects/:id/teams`            | List project teams       | ✅   | Viewer                  |
| `DELETE` | `/projects/:id/teams/:team_id`   | Remove team from project | 🔒   | Owner (project) / Admin |

---

## 12. Deployments Module

| Method  | Endpoint                    | Description                  | Auth | Min Role                |
| ------- | --------------------------- | ---------------------------- | ---- | ----------------------- |
| `POST`  | `/deployments`              | Trigger a new deployment     | 🔒   | Developer               |
| `GET`   | `/deployments/:id`          | Get deployment by ID         | ✅   | Viewer                  |
| `GET`   | `/projects/:id/deployments` | List deployments for project | ✅   | Viewer                  |
| `PATCH` | `/deployments/:id/status`   | Update deployment status     | ⚙️   | Build Worker (Internal) |

---

## 13. Live Build Logs Sub-Module

| Method | Endpoint                         | Description                    | Auth                       |
| ------ | -------------------------------- | ------------------------------ | -------------------------- |
| `GET`  | `/deployments/:id/logs/stream`   | Stream live logs (SSE)         | ✅ Project access required |
| `GET`  | `/deployments/:id/logs`          | Get stored logs (with filters) | ✅ Project access required |
| `GET`  | `/deployments/:id/logs/search`   | Search logs by keyword         | ✅ Project access required |
| `GET`  | `/deployments/:id/logs/download` | Download logs as `.log` file   | ✅ Project access required |

---

## 14. Deployment History Sub-Module

| Method | Endpoint                    | Description                            | Auth | Min Role    |
| ------ | --------------------------- | -------------------------------------- | ---- | ----------- |
| `GET`  | `/projects/:id/deployments` | List deployment history (paginated)    | ✅   | Viewer      |
| `GET`  | `/deployments/:id`          | Get deployment detail                  | ✅   | Viewer      |
| `POST` | `/deployments/:id/redeploy` | Redeploy at same commit                | 🔒   | Developer   |
| `POST` | `/projects/:id/rollback`    | Rollback to last successful deployment | 🔒   | Admin/Owner |

---

## 15. Dashboard Module

| Method | Endpoint     | Description                  | Auth             |
| ------ | ------------ | ---------------------------- | ---------------- |
| `GET`  | `/dashboard` | Aggregated platform overview | ✅ Authenticated |

---

## 16. Health Module

> Health endpoints follow the **Kubernetes 3-probe pattern** for compatibility with orchestrators and load balancers.

| Method | Endpoint          | Description                                                      | Auth            |
| ------ | ----------------- | ---------------------------------------------------------------- | --------------- |
| `GET`  | `/health/live`    | Liveness probe — is the process alive?                           | 🌐 Public       |
| `GET`  | `/health/ready`   | Readiness probe — are critical dependencies available?           | 🌐 Public       |
| `GET`  | `/health/deep`    | Full deep check — all services and external dependencies         | 🔒 System Admin |

---

## 17. Internal-Only Endpoints Summary

These endpoints are **not** accessible from external clients. They require internal service credentials (Build Worker service token):

| Method  | Endpoint                         | Description                            |
| ------- | -------------------------------- | -------------------------------------- |
| `PATCH` | `/deployments/:id/status`        | Build Worker status transitions        |
| `POST`  | `/deployments/:id/logs`          | Build Worker log writes                |
| `GET`   | `/projects/:id/env-vars/decrypt` | Secret injection for deployment runner |

---

## 18. Error Response Conventions

All endpoints return errors in a consistent format:

```json
{
  "is_error": true,
  "code": "ERROR_CODE",
  "message": "Human-readable error description",
  "errors": {
    "field_name": ["Validation error detail"]
  }
}
```

### System-Wide Error Code Namespaces

| Prefix      | Module                |
| ----------- | --------------------- |
| `AUTH_`     | Auth Module           |
| `ACCESS_`   | Access Control        |
| `ORG_`      | Organization Module   |
| `PRJ_`      | Projects Module       |
| `PRJ_PERM_` | Project Permissions   |
| `REPO_`     | Repository Sub-Module |
| `ENV_`      | Environment Variables |
| `DEPLOY_`   | Deployments Module    |
| `WORKER_`   | Build Worker          |
| `LOG_`      | Live Build Logs       |
| `HIST_`     | Deployment History    |

---

**Document Version:** 1.0  
**Last Updated:** 2026-08-12  
**Author:** Backend Architecture Team
