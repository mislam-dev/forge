# Module 12 — Project Assignments

> **Module Type:** Sub-Module (Projects)
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Project Assignments Module](../../modules/projects/project-assignments-module.md)

---

## 1. Module Overview

### Purpose

The Project Assignments sub-module manages the **assignment of individual users and teams to projects**. These assignments determine which users can access a project and what they can do within it.

### Responsibilities

- Assign individual users to a project (project_members)
- Remove users from a project
- List users assigned to a project
- Assign teams to a project (project_teams)
- Remove teams from a project
- List teams assigned to a project

### Scope

**Included:**
- `POST /projects/:project_id/members` — assign user to project
- `GET /projects/:project_id/members` — list project members
- `DELETE /projects/:project_id/members/:user_id` — remove user from project
- `POST /projects/:project_id/teams` — assign team to project
- `GET /projects/:project_id/teams` — list assigned teams
- `DELETE /projects/:project_id/teams/:team_id` — remove team from project

**Excluded:**
- Project-level RBAC roles (Project Permissions sub-module)

---

## 2. Dependencies

### Depends On
- **Projects**
- **Teams**
- **Org Permissions** (only org members can be assigned)
- **Authentication**

### Used By
- **Project Permissions** (reads assignment for access checks)
- **Deployments** (triggers are checked against project membership)

---

## 3. Database Tables

### `project_members`

| Column | Type | Constraints |
|--------|------|-------------|
| project_id | UUID | PK (composite), FK -> projects.id CASCADE |
| user_id | UUID | PK (composite), FK -> users.id CASCADE |
| role | VARCHAR | CHECK(Viewer, Developer, Admin), Not Null |
| assigned_at | TIMESTAMP | Not Null |

### `project_teams`

| Column | Type | Constraints |
|--------|------|-------------|
| project_id | UUID | PK (composite), FK -> projects.id CASCADE |
| team_id | UUID | PK (composite), FK -> teams.id CASCADE |
| assigned_at | TIMESTAMP | Not Null |

**Note:** Project owner (`owner_id` on the `projects` table) always has implicit Owner access and is not in `project_members`.

---

## 4. API Implementation

### POST /projects/:project_id/members

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** `{ user_id, role }` — role is Viewer, Developer, or Admin (not Owner — that's projects.owner_id)
- **Service logic:** Validate user is org member, check not already assigned, insert
- **Response:** `201 { message, data: { project_id, user_id, role, assigned_at } }`
- **Errors:** `409` if already assigned, `400` if invalid role, `404` if user not org member

### GET /projects/:project_id/members

- **Auth:** JWT + project member
- **Response:** `200 { message, data: [members with user info] }`

### DELETE /projects/:project_id/members/:user_id

- **Auth:** JWT + project owner OR org Admin/Owner
- **Service logic:** Cannot remove project owner (they are on projects table, not here)
- **Response:** `200 { message: "Member removed." }`

### POST /projects/:project_id/teams

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** `{ team_id }`
- **Service logic:** Validate team belongs to same org as project, check not already assigned
- **Response:** `201 { message, data: { project_id, team_id, assigned_at } }`

### GET /projects/:project_id/teams

- **Auth:** JWT + project member
- **Response:** `200 { message, data: [teams with member count] }`

### DELETE /projects/:project_id/teams/:team_id

- **Auth:** JWT + project owner OR org Admin/Owner
- **Response:** `200 { message: "Team removed from project." }`

---

## 5. Business Rules

| Rule | Implementation |
|------|---------------|
| Only org members can be assigned to projects | Check org membership before insert |
| Only teams from same org can be assigned | Check team.organization_id == project.organization_id |
| No duplicate assignments | Composite PK enforces at DB level |
| Project owner cannot be removed from members | owner_id is on projects table, not project_members |

---

## 6. Testing

### Integration Tests
- [ ] `POST /members` — valid user, org member: success
- [ ] `POST /members` — user not in org: 400 returned
- [ ] `POST /members` — duplicate assignment: 409 returned
- [ ] `GET /members` — list with user info
- [ ] `DELETE /members/:user_id` — success
- [ ] `POST /teams` — valid team from same org: success
- [ ] `POST /teams` — team from different org: 400 returned
- [ ] `POST /teams` — duplicate: 409 returned
- [ ] `GET /teams` — list with team info
- [ ] `DELETE /teams/:team_id` — success

---

## 7. Implementation Tasks

- [ ] Create `project_members` and `project_teams` migrations
- [ ] Generate SeaORM entities for both tables
- [ ] Implement `ProjectAssignmentsService` with all member/team operations
- [ ] Implement org membership check before project member assignment
- [ ] Implement same-org check before team assignment
- [ ] Implement handlers for all 6 endpoints
- [ ] Register routes in router
- [ ] Write all integration tests

---

## 8. Definition of Done

- [ ] All 6 project assignment endpoints functional
- [ ] Org membership check enforced for user assignments
- [ ] Same-org check enforced for team assignments
- [ ] Duplicate assignment rejected by DB constraint
- [ ] All tests pass

---

## 9. Estimated Effort

**Small-Medium (1 day)**

Straightforward junction table management with cross-entity validation checks.

---

## 10. Recommendations

**Required:**
- Validate that the user being assigned is in the same org as the project.
- Validate that the team being assigned belongs to the same org as the project.

**Recommended:**
- `GET /members` response should join users table to include name and email.
- Role values for project_members should be `Viewer`, `Developer`, `Admin` (not `Owner` — owner is tracked via projects.owner_id).
