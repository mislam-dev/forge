# Module 09 — Projects

> **Module Type:** Core Module
> **Priority:** P1 — Core
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19
> **Source Docs:** [Projects Module](../../modules/projects/projects-module.md)

---

## 1. Module Overview

### Purpose

The Projects module manages **deployable units within an organization**. A project represents a single application, service, or static site that can be built and deployed through the Forge platform. It is the central resource that connects repositories, environment variables, team assignments, and deployments.

### Responsibilities

- Create projects within an organization
- List projects in an organization
- Get a specific project by ID
- Update project configuration
- Delete a project (owner/admin guard)
- Validate project type (`repo` vs `files`)
- Store runtime configuration (runtime, port, health check URL)

### Scope

**Included:**
- `POST /organizations/:org_id/projects` — create project
- `GET /organizations/:org_id/projects` — list projects
- `GET /organizations/:org_id/projects/:project_id` — get project
- `PUT /organizations/:org_id/projects/:project_id` — update project
- `DELETE /organizations/:org_id/projects/:project_id` — delete project

**Excluded:**
- Git repository connection (Repository sub-module)
- Environment variables (Environment Variables sub-module)
- Project member/team assignments (Project Assignments sub-module)
- Deployment triggering (Deployments module)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/projects/mod.rs` | Exists — empty stub |
| Handlers | Not implemented |
| Service | Not implemented |
| Tests | None |

---

## 3. Dependencies

### Depends On
- **Organizations**
- **Org Permissions** (Admin/Owner to create/delete projects)
- **Authentication**
- **RabbitMQ** (project deployment triggers require queue — declared later)

### Used By
- **Repository** (repository belongs to project)
- **Environment Variables** (env vars belong to project)
- **Project Assignments** (members/teams assigned to project)
- **Project Permissions** (reads project.owner_id)
- **Deployments** (project is the deployable unit)
- **Build Worker** (reads project config)
- **Dashboard** (aggregates project metrics)

---

## 4. Database Table

### `projects`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| organization_id | UUID | FK -> organizations.id CASCADE, Not Null |
| owner_id | UUID | FK -> users.id, Not Null |
| name | VARCHAR(255) | Not Null |
| description | TEXT | Nullable |
| project_type | VARCHAR | CHECK(repo, files), Not Null |
| runtime | VARCHAR | CHECK(Node.js, Rust, Python, Go, Static Site), Not Null |
| port | INTEGER | Not Null (default 3000) |
| health_check_url | VARCHAR | Nullable (default /health) |
| status | VARCHAR | CHECK(Active, Inactive, Archived), Default Active |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

**Constraint:** `(organization_id, name)` composite unique index.

---

## 5. API Implementation

### POST /organizations/:org_id/projects

- **Auth:** JWT + org role: Developer, Admin, or Owner
- **Request:**
  ```json
  {
    "name": "string",
    "description": "string (optional)",
    "project_type": "repo | files",
    "runtime": "Node.js | Rust | Python | Go | Static Site",
    "port": 3000,
    "health_check_url": "/health (optional)"
  }
  ```
- **Service logic:**
  1. Check org membership and role
  2. Validate project_type and runtime combination
  3. Check name uniqueness within org
  4. Set owner_id to authenticated user
  5. Insert project
- **Response:** `201 { message, data: project }`
- **Errors:** `409` duplicate name, `400` invalid runtime/type combo

### GET /organizations/:org_id/projects

- **Auth:** JWT + org member (any role, filtered by project membership for Viewer/Developer)
- **Response:** `200 { message, data: [projects], meta: pagination }`

### GET /organizations/:org_id/projects/:project_id

- **Auth:** JWT + project member OR org Admin/Owner
- **Response:** `200 { message, data: project }`

### PUT /organizations/:org_id/projects/:project_id

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** All fields optional
- **Service logic:** Update non-null fields; name uniqueness check if name changes
- **Response:** `200 { message, data: updated_project }`

### DELETE /organizations/:org_id/projects/:project_id

- **Auth:** JWT + project owner OR org Owner
- **Service logic:** Check no Running deployments exist before deletion
- **Response:** `200 { message: "Project deleted." }`
- **Errors:** `409 Conflict` if active deployment running

---

## 6. Authorization Matrix

| Action | Viewer | Developer | Admin | Owner | Project Owner |
|--------|--------|-----------|-------|-------|---------------|
| Create project | No | Yes | Yes | Yes | N/A |
| List projects | Own/assigned | Yes | Yes | Yes | Yes |
| Get project | If assigned | Yes | Yes | Yes | Yes |
| Update project | No | No | Yes | Yes | Yes |
| Delete project | No | No | No | Yes | Yes |

---

## 7. Logging

| Event | Level | Fields |
|-------|-------|--------|
| Project created | INFO | project_id, org_id, owner_id, runtime |
| Project updated | INFO | project_id, user_id |
| Project deleted | WARN | project_id, org_id, user_id |
| Delete blocked by running deployment | WARN | project_id, deployment_id |

---

## 8. Testing

### Integration Tests
- [ ] `POST /projects` — success: project created with correct owner
- [ ] `POST /projects` — duplicate name in org: 409 returned
- [ ] `POST /projects` — invalid runtime: 400 returned
- [ ] `POST /projects` — Developer role: success
- [ ] `POST /projects` — Viewer role: 403 returned
- [ ] `GET /projects` — org admin: all projects listed
- [ ] `GET /projects/:id` — project member: success
- [ ] `GET /projects/:id` — non-member: 403 returned
- [ ] `PUT /projects/:id` — owner: update success
- [ ] `DELETE /projects/:id` — owner: success
- [ ] `DELETE /projects/:id` — active deployment exists: 409 returned

---

## 9. Implementation Tasks

- [ ] Create `projects` migration with all columns and constraints
- [ ] Generate SeaORM entity for `projects`
- [ ] Implement `ProjectsService` with all CRUD operations
- [ ] Implement name uniqueness check within org
- [ ] Implement active deployment check before deletion
- [ ] Implement handlers for all 5 project endpoints
- [ ] Register routes in router
- [ ] Write all integration tests

---

## 10. Definition of Done

- [ ] All 5 project endpoints functional
- [ ] project_type and runtime validations enforced
- [ ] Name uniqueness within org enforced
- [ ] owner_id set on creation
- [ ] Delete blocked by running deployment
- [ ] All tests pass

---

## 11. Estimated Effort

**Medium (1–2 days)**

---

## 12. Recommendations

**Required:**
- `project_type` must be `repo` or `files` — validated at service layer and enforced by DB CHECK constraint.
- `runtime` must be one of the documented values — same pattern.
- Project deletion must check for active (Running) deployments.

**Recommended:**
- The `files` project type (project-files sub-module) is marked as Medium priority in docs — implement the database column but stub the API for MVP.
- Return project with related repository status in GET project response (join).

**Future Enhancement:**
- Project templates (pre-configured runtimes).
- Project archiving (Archived status) instead of deletion.
