# Forge Platform API Documentation

> **Document:** Human-Readable API Specifications  
> **Target File:** `./api-documentation.md`  
> **Version:** 1.0.0  
> **Machine-Readable Contract:** [`openapi.yaml`](./openapi.yaml)

---

## 1. Overview

The Forge API is organized around REST principles. All API requests must be made over HTTPS. Responses are formatted in JSON wrappers.

The platform provides endpoints for managing users, authentication, multi-tier RBAC access control, organizations, teams, projects, Git repositories, encrypted environment variables, asynchronous deployments, live build log streaming, dashboard metrics, and system health observability.

---

## 2. Base URL

| Environment       | Base URL                |
| ----------------- | ----------------------- |
| Local Development | `http://localhost:3000` |
| Production        | `https://api.forge.dev` |

---

## 3. Authentication

Most API requests require authentication via JSON Web Tokens (JWT).

Pass the access token in the HTTP `Authorization` header:

```http
Authorization: Bearer <access-token>
```

### Internal Service Tokens (Build Worker)

Internal endpoints reserved for the Build Worker service use an internal service token:

```http
Authorization: Bearer <internal-service-token>
```

---

## 4. Common Headers

| Header          | Required                   | Value                                                     |
| --------------- | -------------------------- | --------------------------------------------------------- |
| `Content-Type`  | Yes (for POST/PUT/PATCH)   | `application/json`                                        |
| `Authorization` | Yes (except public routes) | `Bearer <access-token>`                                   |
| `Accept`        | Optional                   | `application/json` (or `text/event-stream` for live logs) |

---

## 5. Standard Response Formats

JSON-based endpoints generally use the following response conventions. Endpoints using other protocols or content types, such as Server-Sent Events or binary log file downloads, are documented separately in their respective endpoint sections.

### 5.1 Success Responses (2xx: 200 OK, 201 Created)

#### Single Resource Response (200 OK / 201 Created)

```json
{
  "message": "Resource retrieved or operation completed successfully.",
  "data": {}
}
```

#### Paginated List Response (200 OK)

```json
{
  "message": "List of resources retrieved successfully.",
  "data": [],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 42
  }
}
```

---

### 5.2 Client Error Responses (4xx: 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 409 Conflict)

#### Standard Client Error Response

```json
{
  "is_error": true,
  "code": "ERROR_CODE_NAMESPACE",
  "message": "Human-readable error description explaining why the request failed.",
  "errors": {}
}
```

#### Validation Error Response (400 Bad Request)

```json
{
  "is_error": true,
  "code": "VALIDATION_ERROR",
  "message": "Validation failed for one or more request fields.",
  "errors": {
    "field_name": ["Field error message detailing validation constraints."]
  }
}
```

---

### 5.3 Server Error Responses (500 Internal Server Error)

```json
{
  "is_error": true,
  "code": "INTERNAL_SERVER_ERROR",
  "message": "An unexpected server error occurred while processing the request.",
  "errors": {}
}
```

---

## 6. Authentication APIs

### POST /auth/register

#### Description

Registers a new user account and assigns default system role.

#### Authentication

None (Public).

#### Request

```http
POST /auth/register
Content-Type: application/json

{
  "name": "Monirul Islam",
  "email": "monirul@example.com",
  "password": "StrongPassword123!"
}
```

#### Success Response (201 Created)

```json
{
  "message": "User registered successfully.",
  "data": {
    "id": "456e7890-e89b-12d3-a456-426614174000",
    "name": "Monirul Islam",
    "email": "monirul@example.com",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

#### Error Response (409 Conflict)

```json
{
  "is_error": true,
  "code": "AUTH_001",
  "message": "User with this email already exists.",
  "errors": {}
}
```

---

### POST /auth/login

#### Description

Authenticates user credentials and returns JWT access token and refresh token session.

#### Authentication

None (Public).

#### Request

```http
POST /auth/login
Content-Type: application/json

{
  "email": "monirul@example.com",
  "password": "StrongPassword123!"
}
```

#### Success Response (200 OK)

```json
{
  "message": "Login successful.",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "rfr_890a7b6c5d4e3f2a1b0c...",
    "expires_in": 900
  }
}
```

#### Error Response (401 Unauthorized)

```json
{
  "is_error": true,
  "code": "AUTH_002",
  "message": "Invalid email or password.",
  "errors": {}
}
```

---

### POST /auth/refresh

#### Description

Exchanges a valid refresh token for a new short-lived access token.

#### Authentication

None (Requires refresh_token in request body).

#### Request

```http
POST /auth/refresh
Content-Type: application/json

{
  "refresh_token": "rfr_890a7b6c5d4e3f2a1b0c..."
}
```

#### Success Response (200 OK)

```json
{
  "message": "Token refreshed successfully.",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "rfr_new_token_string...",
    "expires_in": 900
  }
}
```

#### Error Response (401 Unauthorized)

```json
{
  "is_error": true,
  "code": "AUTH_003",
  "message": "Invalid or expired refresh token.",
  "errors": {}
}
```

---

### POST /auth/logout

#### Description

Invalidates the current session refresh token in database.

#### Authentication

Required (Bearer JWT).

#### Request

```http
POST /auth/logout
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Logged out successfully.",
  "data": {}
}
```

---

### GET /auth/me

#### Description

Retrieves profile data for current authenticated user.

#### Authentication

Required (Bearer JWT).

#### Request

```http
GET /auth/me
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Authenticated user profile retrieved.",
  "data": {
    "id": "456e7890-e89b-12d3-a456-426614174000",
    "name": "Monirul Islam",
    "email": "monirul@example.com",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

---

## 7. Access Control & Role APIs

### GET /access-control/roles

#### Description

Lists all system roles.

#### Authentication & Authorization

Required (System Admin only).

#### Request

```http
GET /access-control/roles
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Roles retrieved successfully.",
  "data": [
    {
      "id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
      "key": "Administrator",
      "value": "admin",
      "descriptions": "System administrator"
    }
  ]
}
```

#### Error Response (403 Forbidden)

```json
{
  "is_error": true,
  "code": "ACCESS_005",
  "message": "Unauthorized access. System Admin role required.",
  "errors": {}
}
```

---

### POST /access-control/roles

#### Description

Creates a new system role.

#### Authentication & Authorization

Required (System Admin only).

#### Request

```http
POST /access-control/roles
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "key": "Developer",
  "value": "developer",
  "descriptions": "Standard developer role"
}
```

#### Success Response (201 Created)

```json
{
  "message": "Role created successfully.",
  "data": {
    "id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b7",
    "key": "Developer",
    "value": "developer",
    "descriptions": "Standard developer role"
  }
}
```

---

### PUT /access-control/roles/{id}

#### Description

Updates an existing system role.

#### Path Parameters

| Parameter | Type | Required | Description            |
| --------- | ---- | -------- | ---------------------- |
| id        | UUID | Yes      | Role unique identifier |

#### Request

```http
PUT /access-control/roles/07c0060e-8e8c-44c1-942c-3004f5a6c5b6
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "key": "Updated Administrator",
  "value": "admin",
  "descriptions": "Updated system administrator description"
}
```

#### Success Response (200 OK)

```json
{
  "message": "Role updated successfully.",
  "data": {
    "id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
    "key": "Updated Administrator",
    "value": "admin",
    "descriptions": "Updated system administrator description"
  }
}
```

---

### DELETE /access-control/roles/{id}

#### Description

Deletes a system role and removes it from all assigned users.

#### Path Parameters

| Parameter | Type | Required | Description            |
| --------- | ---- | -------- | ---------------------- |
| id        | UUID | Yes      | Role unique identifier |

#### Request

```http
DELETE /access-control/roles/07c0060e-8e8c-44c1-942c-3004f5a6c5b6
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Role deleted successfully.",
  "data": {}
}
```

---

### POST /access-control/roles/permissions/assign

#### Description

Assigns permissions to a role.

#### Request

```http
POST /access-control/roles/permissions/assign
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "role_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
  "permission_ids": ["18d1071f-9f9d-55d2-a53d-4115g6b7d6c7"]
}
```

#### Success Response (200 OK)

```json
{
  "message": "Permission assigned to role successfully.",
  "data": {}
}
```

---

### POST /access-control/role/assign

#### Description

Assigns system roles to a user.

#### Request

```http
POST /access-control/role/assign
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "user_id": "456e7890-e89b-12d3-a456-426614174000",
  "role_ids": ["07c0060e-8e8c-44c1-942c-3004f5a6c5b6"]
}
```

#### Success Response (200 OK)

```json
{
  "message": "Roles assigned to user successfully.",
  "data": {}
}
```

---

## 8. User & Profile APIs

### GET /users/{id}

#### Description

Retrieves single user account details.

#### Path Parameters

| Parameter | Type | Required | Description            |
| --------- | ---- | -------- | ---------------------- |
| id        | UUID | Yes      | User unique identifier |

#### Request

```http
GET /users/456e7890-e89b-12d3-a456-426614174000
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "User details retrieved successfully.",
  "data": {
    "id": "456e7890-e89b-12d3-a456-426614174000",
    "name": "Monirul Islam",
    "email": "monirul@example.com",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

---

### PUT /users/{id}/profile

#### Description

Updates profile information for a user.

#### Path Parameters

| Parameter | Type | Required | Description            |
| --------- | ---- | -------- | ---------------------- |
| id        | UUID | Yes      | User unique identifier |

#### Request

```http
PUT /users/456e7890-e89b-12d3-a456-426614174000/profile
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "name": "Monirul Islam (Updated)"
}
```

#### Success Response (200 OK)

```json
{
  "message": "Profile updated successfully.",
  "data": {
    "id": "456e7890-e89b-12d3-a456-426614174000",
    "name": "Monirul Islam (Updated)",
    "email": "monirul@example.com",
    "updated_at": "2026-08-12T18:00:00Z"
  }
}
```

---

## 9. Organization & Member APIs

### POST /organizations

#### Description

Creates a new organization tenant.

#### Request

```http
POST /organizations
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "name": "Acme Corporation",
  "slug": "acme-corp"
}
```

#### Success Response (201 Created)

```json
{
  "message": "Organization created successfully.",
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Acme Corporation",
    "slug": "acme-corp",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

---

### GET /organizations

#### Description

Lists organizations the authenticated user is a member of.

#### Request

```http
GET /organizations
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Organizations retrieved successfully.",
  "data": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Acme Corporation",
      "slug": "acme-corp",
      "created_at": "2026-08-12T17:00:00Z"
    }
  ]
}
```

---

### POST /organizations/{id}/members

#### Description

Adds a user to an organization with a specific organizational role (`Viewer`, `Developer`, `Admin`, `Owner`).

#### Path Parameters

| Parameter | Type | Required | Description                    |
| --------- | ---- | -------- | ------------------------------ |
| id        | UUID | Yes      | Organization unique identifier |

#### Request

```http
POST /organizations/123e4567-e89b-12d3-a456-426614174000/members
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "user_id": "456e7890-e89b-12d3-a456-426614174000",
  "role": "Developer"
}
```

#### Success Response (201 Created)

```json
{
  "message": "Organization member added successfully.",
  "data": {
    "id": "mem-12345678-e89b-12d3-a456-426614174000",
    "organization_id": "123e4567-e89b-12d3-a456-426614174000",
    "user_id": "456e7890-e89b-12d3-a456-426614174000",
    "role": "Developer",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

---

## 10. Teams APIs

### POST /organizations/{id}/teams

#### Description

Creates a team within an organization. (Requires Admin or Owner role).

#### Path Parameters

| Parameter | Type | Required | Description                    |
| --------- | ---- | -------- | ------------------------------ |
| id        | UUID | Yes      | Organization unique identifier |

#### Request

```http
POST /organizations/123e4567-e89b-12d3-a456-426614174000/teams
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "name": "Backend Engineering"
}
```

#### Success Response (201 Created)

```json
{
  "message": "Team created successfully.",
  "data": {
    "id": "987f6543-e21b-32d1-b654-987654321000",
    "organization_id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Backend Engineering",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

---

## 11. Projects & Repository APIs

### POST /projects

#### Description

Creates a new project in an organization. Requires `Developer`, `Admin`, or `Owner` role. (Auto-assigns creator as `owner_id`).

#### Request

```http
POST /projects
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "organization_id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Forge Backend",
  "type": "repo",
  "repository_url": "https://github.com/mislam-dev/forge.git",
  "default_branch": "main",
  "runtime": "Rust",
  "framework": "Actix Web",
  "descriptions": "Main Rust API backend service"
}
```

#### Success Response (201 Created)

```json
{
  "message": "Project created successfully.",
  "data": {
    "id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
    "organization_id": "123e4567-e89b-12d3-a456-426614174000",
    "owner_id": "456e7890-e89b-12d3-a456-426614174000",
    "name": "Forge Backend",
    "type": "repo",
    "repository_url": "https://github.com/mislam-dev/forge.git",
    "default_branch": "main",
    "runtime": "Rust",
    "framework": "Actix Web",
    "status": "active",
    "descriptions": "Main Rust API backend service",
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

#### Error Response (403 Forbidden)

```json
{
  "is_error": true,
  "code": "PRJ_PERM_001",
  "message": "Access denied. Viewers cannot create projects.",
  "errors": {}
}
```

---

### DELETE /projects/{id}

#### Description

Deletes a project. Developers can delete **only self-created projects** (`owner_id == self.id`).

#### Path Parameters

| Parameter | Type | Required | Description               |
| --------- | ---- | -------- | ------------------------- |
| id        | UUID | Yes      | Project unique identifier |

#### Request

```http
DELETE /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Project deleted successfully.",
  "data": {}
}
```

#### Error Response (403 Forbidden)

```json
{
  "is_error": true,
  "code": "PRJ_PERM_002",
  "message": "Access denied. Developers can only delete projects created by themselves.",
  "errors": {}
}
```

---

### POST /projects/{id}/repository/validate

#### Description

Tests remote Git repository URL and credential access (`public` or `pat`).

#### Path Parameters

| Parameter | Type | Required | Description               |
| --------- | ---- | -------- | ------------------------- |
| id        | UUID | Yes      | Project unique identifier |

#### Request

```http
POST /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/repository/validate
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "repository_url": "https://github.com/mislam-dev/forge.git",
  "auth_type": "pat",
  "access_token": "github_pat_11ABCXYZ123456789"
}
```

#### Success Response (200 OK)

```json
{
  "message": "Repository validation successful.",
  "data": {
    "is_valid": true,
    "default_branch": "main",
    "branches": ["main", "dev", "feature/auth"]
  }
}
```

---

## 12. Environment Variables APIs

### POST /projects/{id}/env-vars

#### Description

Creates an environment variable scoped to target environment (`Development`, `Preview`, `Production`). Secret values are automatically encrypted with AES-256-GCM.

#### Path Parameters

| Parameter | Type | Required | Description               |
| --------- | ---- | -------- | ------------------------- |
| id        | UUID | Yes      | Project unique identifier |

#### Request

```http
POST /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/env-vars
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "key": "DATABASE_URL",
  "value": "postgres://username:<password>@db.example.com:5432/database",
  "environment": "Production",
  "is_secret": true
}
```

#### Success Response (201 Created)

```json
{
  "message": "Environment variable created successfully.",
  "data": {
    "id": "env-12345678-8e8c-44c1-942c-3004f5a6c5b6",
    "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
    "key": "DATABASE_URL",
    "value": "••••••••",
    "environment": "Production",
    "is_secret": true,
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

#### Error Response (400 Bad Request)

```json
{
  "is_error": true,
  "code": "ENV_001",
  "message": "Validation failed for the request payload.",
  "errors": {
    "key": [
      "Invalid key format. Must match POSIX uppercase regex ^[A-Z_][A-Z0-9_]*$"
    ]
  }
}
```

---

## 13. Deployment, Log & History APIs

### POST /deployments

#### Description

Triggers an async deployment job for a project. Returns immediately with `status: Queued`.

#### Request

```http
POST /deployments
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
  "branch": "main",
  "commit_hash": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
}
```

#### Success Response (201 Created)

```json
{
  "message": "Deployment queued successfully.",
  "data": {
    "id": "deploy-abc123-8e8c-44c1-942c-3004f5a6c5b6",
    "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
    "triggered_by": "456e7890-e89b-12d3-a456-426614174000",
    "branch": "main",
    "commit_hash": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    "status": "Queued",
    "build_duration": null,
    "deploy_duration": null,
    "created_at": "2026-08-12T17:00:00Z"
  }
}
```

---

### GET /deployments/{id}/logs/stream

#### Description

Opens a real-time Server-Sent Events (SSE) stream pushing build worker logs line-by-line.

#### Path Parameters

| Parameter | Type | Required | Description                  |
| --------- | ---- | -------- | ---------------------------- |
| id        | UUID | Yes      | Deployment unique identifier |

#### Request

```http
GET /deployments/deploy-abc123-8e8c-44c1-942c-3004f5a6c5b6/logs/stream
Authorization: Bearer <access-token>
Accept: text/event-stream
```

#### Stream Response

```text
data: {"timestamp":"2026-08-12T17:00:01Z","level":"INFO","step":"clone","message":"Cloning repository..."}

data: {"timestamp":"2026-08-12T17:00:04Z","level":"INFO","step":"build","message":"Step 1/8 : FROM rust:1.79-slim"}

data: {"timestamp":"2026-08-12T17:01:23Z","level":"INFO","step":"health_check","message":"Health check passed. Status: Success."}

event: close
data: {"status":"Success"}
```

---

### POST /projects/{id}/rollback

#### Description

Triggers a new deployment targeting the commit hash of the most recent `Success` deployment on the branch. (Requires Admin or Owner role).

#### Path Parameters

| Parameter | Type | Required | Description               |
| --------- | ---- | -------- | ------------------------- |
| id        | UUID | Yes      | Project unique identifier |

#### Request

```http
POST /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/rollback
Authorization: Bearer <access-token>
```

#### Success Response (201 Created)

```json
{
  "message": "Rollback deployment queued. Targeting last successful commit.",
  "data": {
    "id": "deploy-roll01-8e8c-44c1-942c-3004f5a6c5b6",
    "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
    "branch": "main",
    "commit_hash": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    "status": "Queued",
    "created_at": "2026-08-12T18:05:00Z"
  }
}
```

---

## 14. Dashboard APIs

### GET /dashboard

#### Description

Retrieves aggregated cross-module metrics (projects count by status, deployment counts, recent activities). Dashboard module owns no tables and performs read-only aggregation.

#### Request

```http
GET /dashboard
Authorization: Bearer <access-token>
```

#### Success Response (200 OK)

```json
{
  "message": "Dashboard data retrieved successfully.",
  "data": {
    "projects_summary": {
      "active": 12,
      "archived": 2,
      "draft": 1
    },
    "deployments_summary": {
      "total": 145,
      "success": 138,
      "failed": 7
    }
  }
}
```

---

## 15. Health Observability APIs

### GET /health

#### Description

Public platform health status probe for load balancers and uptime monitors.

#### Authentication

None (Public).

#### Request

```http
GET /health
```

#### Success Response (200 OK)

```json
{
  "message": "Platform health check complete.",
  "data": {
    "status": "ok",
    "timestamp": "2026-08-12T17:00:00Z"
  }
}
```

---

## 16. Error Code Namespaces

| Code Prefix | Domain / Module                        |
| ----------- | -------------------------------------- |
| `AUTH_`     | Authentication & JWT tokens            |
| `ACCESS_`   | System Access Control RBAC             |
| `ORG_`      | Organizations & Membership             |
| `PRJ_`      | Projects                               |
| `PRJ_PERM_` | Project Permissions & Ownership Guards |
| `REPO_`     | Repository & PAT credentials           |
| `ENV_`      | Environment Variables & Encryption     |
| `DEPLOY_`   | Deployments                            |
| `WORKER_`   | Build Worker execution                 |
| `LOG_`      | Live Build Logs                        |
| `HIST_`     | Deployment History & Rollbacks         |
