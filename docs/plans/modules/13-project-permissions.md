# Module 13 — Project Permissions

> **Module Type:** Sub-Module (Projects)
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Project Permissions Module](../../modules/projects/project-permissions-module.md)

---

## 1. Module Overview

### Purpose

The Project Permissions sub-module implements **Tier 3 of the three-tier RBAC hierarchy** — project-level access control. Like Org Permissions, it is primarily a middleware/service component, not an API-facing module. It resolves a user's effective access level on a specific project.

### Responsibilities

- Define project-level role hierarchy: `Viewer < Developer < Admin < Owner`
- Resolve a user's effective project role (from project_members or project owner status or team membership)
- Provide `require_project_role(minimum_role)` Axum extractor
- Prevent deletion of projects by non-owners (the `owner_id` guard)

### Scope

**Included:**
- `resolve_project_role(project_id, user_id)` service function
- `RequireProjectRole` Axum extractor
- Applied to all project-scoped write endpoints

**Excluded:**
- No dedicated API endpoints
- System-level RBAC (Access Control)
- Org-level RBAC (Org Permissions)

---

## 2. Dependencies

### Depends On
- **Project Assignments** (`project_members`, `project_teams` tables)
- **Teams** (`team_members` table — resolves team membership)
- **Org Permissions** (org Admin/Owner bypass project permission checks)
- **Authentication**

### Used By
- **All project-scoped write endpoints** (update, delete, trigger deployment, manage env vars)
- **Deployments** (trigger requires Developer+ access)

---

## 3. Role Resolution Algorithm

For a given `(project_id, user_id)`:

1. Check if `user_id == projects.owner_id` → `Owner` (highest)
2. Check if user has org role `Admin` or `Owner` → implicit `Admin` or `Owner` on all org projects
3. Check `project_members (project_id, user_id)` → role from junction table
4. Check `project_teams` for project_id → get team_ids → check `team_members` for user_id → if in team, use team's implicit project access (Developer for now — see recommendation)
5. If no match: user has no project access → `403 Forbidden`
6. Return highest resolved role

---

## 4. Project Role Hierarchy

```
Owner > Admin > Developer > Viewer
```

| Role | Source | Permissions |
|------|--------|-------------|
| Owner | projects.owner_id | All: delete, update, manage members, trigger deployments |
| Admin | org role inheritance or project_members | Manage project, update settings, trigger deployments |
| Developer | project_members or team member | Trigger deployments, update non-sensitive settings |
| Viewer | project_members | Read-only access to project resources |

---

## 5. Implementation

### ProjectRole Enum

```rust
#[derive(PartialOrd, PartialEq)]
pub enum ProjectRole {
    Viewer,
    Developer,
    Admin,
    Owner,
}
```

### Resolution Function

```rust
pub async fn resolve_project_role(
    db: &DatabaseConnection,
    project_id: Uuid,
    user_id: Uuid,
    org_id: Uuid,
) -> Result<Option<ProjectRole>, AppError>
```

### Axum Extractor

Applied to route handlers requiring project-level access control.

---

## 6. Owner Guard

The `owner_id` deletion guard:
- `DELETE /projects/:id` requires `projects.owner_id == jwt_user_id` OR org `Owner` role
- This guard is stricter than Admin — even project Admins cannot delete a project

---

## 7. Implementation Tasks

- [ ] Define `ProjectRole` enum with `PartialOrd`
- [ ] Implement `resolve_project_role()` with full resolution algorithm (owner > org admin > project member > team member)
- [ ] Implement Redis caching for project role lookups (key: `forge:proj_role:{project_id}:{user_id}`, TTL: 60s)
- [ ] Implement cache invalidation on project_members / project_teams changes
- [ ] Implement `RequireProjectRole` Axum extractor
- [ ] Implement `RequireProjectOwner` stricter guard for deletion
- [ ] Integrate with Deployments, Env Vars, Repository endpoints

---

## 8. Definition of Done

- [ ] `resolve_project_role()` correctly evaluates owner > org admin > project member > team member
- [ ] System Admin and org Owner bypass project role checks
- [ ] `RequireProjectRole` returns 403 for insufficient access
- [ ] Redis caching working
- [ ] Owner guard enforced for delete operations
- [ ] Unit tests: role resolution for each source (owner, org admin, member, team)

---

## 9. Estimated Effort

**Small (< 1 day)**

The resolution algorithm is moderately complex, but implementation is primarily utility functions.

---

## 10. Recommendations

**Required:**
- System Admin must bypass all project-level role checks.
- Org Owner and org Admin must have at least Admin access to all projects in their org.
- Team members' effective project role should default to `Developer` when their team is assigned to a project (until per-team role is implemented).

**Recommended:**
- Cache project role lookups per user to avoid N+1 DB queries on every authenticated request.
- Invalidate role caches when project_members, project_teams, or team_members change.

**Future Enhancement:**
- Per-team project role assignment (assign a team with Viewer, Developer, or Admin role).
