# Module 08 — Teams

> **Module Type:** Core Module
> **Priority:** P1 — Core
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19
> **Source Docs:** [Teams Module](../../modules/teams/teams-module.md)

---

## 1. Module Overview

### Purpose

The Teams module manages **groups of users within an organization** that can be collectively assigned to projects. Teams provide a convenient way to manage project access for groups rather than individual users.

### Responsibilities

- Create teams within an organization
- List teams in an organization
- Get a specific team
- Update team name/description
- Delete a team
- Add members to a team
- Remove members from a team
- List team members

### Scope

**Included:**
- `POST /organizations/:org_id/teams` — create team
- `GET /organizations/:org_id/teams` — list teams in org
- `GET /organizations/:org_id/teams/:team_id` — get team
- `PUT /organizations/:org_id/teams/:team_id` — update team
- `DELETE /organizations/:org_id/teams/:team_id` — delete team
- `POST /organizations/:org_id/teams/:team_id/members` — add member
- `DELETE /organizations/:org_id/teams/:team_id/members/:user_id` — remove member
- `GET /organizations/:org_id/teams/:team_id/members` — list members

**Excluded:**
- Assigning teams to projects (Project Assignments module)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/teams/mod.rs` | Exists — empty stub |
| Handlers | Not implemented |
| Service | Not implemented |
| Tests | None |

---

## 3. Dependencies

### Depends On
- **Organizations** (team belongs to an org)
- **Org Members** (only org members can be added to teams)
- **Org Permissions** (Admin/Owner role required for team management)
- **Authentication**

### Used By
- **Project Assignments** (`project_teams` junction references teams)

---

## 4. Database Tables

### `teams`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| organization_id | UUID | FK -> organizations.id CASCADE, Not Null |
| name | VARCHAR(255) | Not Null |
| description | TEXT | Nullable |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

### `team_members`

| Column | Type | Constraints |
|--------|------|-------------|
| team_id | UUID | PK (composite), FK -> teams.id CASCADE |
| user_id | UUID | PK (composite), FK -> users.id CASCADE |
| joined_at | TIMESTAMP | Not Null |

**Constraint:** team name must be unique within an organization.

---

## 5. API Implementation

### POST /organizations/:org_id/teams

- **Auth:** JWT + org role: Admin or Owner
- **Request:** `{ name, description? }`
- **Service logic:** Validate org membership, check name uniqueness within org, insert team
- **Response:** `201 { message, data: team }`

### GET /organizations/:org_id/teams

- **Auth:** JWT + org member (any role)
- **Response:** `200 { message, data: [teams] }`

### GET /organizations/:org_id/teams/:team_id

- **Auth:** JWT + org member (any role)
- **Response:** `200 { message, data: team }`

### PUT /organizations/:org_id/teams/:team_id

- **Auth:** JWT + org role: Admin or Owner
- **Request:** `{ name?, description? }`
- **Response:** `200 { message, data: updated_team }`

### DELETE /organizations/:org_id/teams/:team_id

- **Auth:** JWT + org role: Admin or Owner
- **Service logic:** Delete team, cascade removes team_members and project_teams assignments
- **Response:** `200 { message: "Team deleted." }`

### POST /organizations/:org_id/teams/:team_id/members

- **Auth:** JWT + org role: Admin or Owner
- **Request:** `{ user_id }`
- **Service logic:** Verify user is an org member, verify not already in team, insert
- **Response:** `201 { message, data: team_member }`

### DELETE /organizations/:org_id/teams/:team_id/members/:user_id

- **Auth:** JWT + org role: Admin or Owner
- **Response:** `200 { message: "Member removed from team." }`

### GET /organizations/:org_id/teams/:team_id/members

- **Auth:** JWT + org member (any role)
- **Response:** `200 { message, data: [members with user info] }`

---

## 6. Authorization Matrix

| Action | Viewer | Developer | Admin | Owner |
|--------|--------|-----------|-------|-------|
| List teams | Yes | Yes | Yes | Yes |
| Get team | Yes | Yes | Yes | Yes |
| Create team | No | No | Yes | Yes |
| Update team | No | No | Yes | Yes |
| Delete team | No | No | Yes | Yes |
| Add member to team | No | No | Yes | Yes |
| Remove member from team | No | No | Yes | Yes |
| List team members | Yes | Yes | Yes | Yes |

---

## 7. Testing

### Integration Tests
- [ ] `POST /teams` — success: team created
- [ ] `POST /teams` — duplicate name in org: 409 returned
- [ ] `GET /teams` — list returned
- [ ] `GET /teams/:id` — success
- [ ] `GET /teams/:id` — wrong org: 404 returned
- [ ] `PUT /teams/:id` — Admin: update success
- [ ] `PUT /teams/:id` — Developer: 403 returned
- [ ] `DELETE /teams/:id` — success
- [ ] `POST /teams/:id/members` — user is org member: success
- [ ] `POST /teams/:id/members` — user not in org: 400 returned
- [ ] `POST /teams/:id/members` — duplicate: 409 returned
- [ ] `DELETE /teams/:id/members/:user_id` — success

---

## 8. Implementation Tasks

- [ ] Create `teams` and `team_members` migrations
- [ ] Generate SeaORM entities for `teams` and `team_members`
- [ ] Implement `TeamsService` with all CRUD + member operations
- [ ] Verify user is org member before adding to team
- [ ] Implement handlers for all 8 endpoints
- [ ] Register routes in router
- [ ] Write all integration tests

---

## 9. Definition of Done

- [ ] All 8 team endpoints functional
- [ ] Team name unique within org enforced
- [ ] Only org members can be added to teams
- [ ] Admin/Owner required for team management
- [ ] All tests pass

---

## 10. Estimated Effort

**Small-Medium (1 day)**

---

## 11. Recommendations

**Required:**
- Verify that a user being added to a team is an `organization_members` member of the same org.
- Team name uniqueness should be enforced at the database level: unique index on `(organization_id, name)`.

**Recommended:**
- Return member count in team list response (useful for UI).

**Future Enhancement:**
- Team lead designation.
- Team-level notification settings.
