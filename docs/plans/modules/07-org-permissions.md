# Module 07 — Organization Permissions

> **Module Type:** Sub-Module (Organizations)
> **Priority:** P1 — Core
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19
> **Source Docs:** [Org Permissions Module](../../modules/organization/organization-permissions-module.md)

---

## 1. Module Overview

### Purpose

The Organization Permissions sub-module implements **Tier 2 of the three-tier RBAC hierarchy** — org-level role enforcement for all operations within an organization. It does not own its own database table; it reads `organization_members.role` to determine authorization.

This is primarily a **middleware/service component**, not an API-facing module. It provides:
- `require_org_role(minimum_role)` middleware/extractor used by Org Members, Teams, Projects, etc.
- The org role resolution function

### Responsibilities

- Define org-level role hierarchy: `Viewer < Developer < Admin < Owner`
- Provide `resolve_org_role(org_id, user_id) -> Option<OrgRole>` service function
- Provide `require_org_role(minimum_role)` Axum middleware extractor
- Return `403 Forbidden` when the user's org role is below the minimum required

### Scope

**Included:**
- Org role resolution service function
- `require_org_role()` Axum extractor/layer
- Applied to all `organizations/*`, `teams/*`, and `projects/*` endpoints

**Excluded:**
- No dedicated API endpoints
- System-level RBAC (Access Control module)
- Project-level RBAC (Project Permissions sub-module)

---

## 2. Dependencies

### Depends On
- **Org Members** (`organization_members` table must exist and be populated)
- **Authentication** (user_id from JWT)

### Used By
- **All endpoints** under `/organizations/:org_id/*`
- **Teams** endpoints
- **Projects** endpoints
- **Project Assignments** endpoints
- **Project Permissions** logic

---

## 3. Org Role Hierarchy

```
Owner > Admin > Developer > Viewer
```

| Role | Can Do |
|------|--------|
| Viewer | Read-only access to org resources |
| Developer | Create/update projects, trigger deployments |
| Admin | Manage members, update org settings |
| Owner | Delete org, transfer ownership |

---

## 4. Implementation

### OrgRole Enum

```rust
#[derive(PartialOrd, PartialEq)]
pub enum OrgRole {
    Viewer,
    Developer,
    Admin,
    Owner,
}
```

### Resolution Function

```rust
pub async fn resolve_org_role(
    db: &DatabaseConnection,
    org_id: Uuid,
    user_id: Uuid,
) -> Result<Option<OrgRole>, AppError>
```

### Axum Extractor

```rust
// Usage in handlers:
// async fn create_team(
//     State(state): State<AppState>,
//     OrgAuth(user, org_role): OrgAuth<{ MinRole::Admin }>,
//     Path(org_id): Path<Uuid>,
//     ...
// )
```

---

## 5. Authorization Decision Logic

For any org-scoped request:
1. Extract `user_id` from JWT claims
2. Extract `org_id` from path parameter
3. Query `organization_members` for `(org_id, user_id)` → get `role`
4. If no row: `403 Forbidden` (not a member)
5. If role < minimum_required: `403 Forbidden`
6. If role >= minimum_required: allow request

---

## 6. Caching

Org role lookups are common. Cache results in Redis:
- Key: `forge:org_role:{org_id}:{user_id}`
- TTL: 60 seconds
- Invalidate: when org membership is updated or deleted

---

## 7. Implementation Tasks

- [ ] Define `OrgRole` enum with `PartialOrd` (for role comparison)
- [ ] Implement `resolve_org_role()` service function
- [ ] Implement Redis caching for role lookups
- [ ] Implement `RequireOrgRole` Axum extractor (or middleware layer)
- [ ] Integrate with Org Members, Teams, Projects handlers

---

## 8. Definition of Done

- [ ] `resolve_org_role()` returns correct role from DB
- [ ] `RequireOrgRole` returns 403 for insufficient role
- [ ] `RequireOrgRole` returns 403 for non-members
- [ ] Redis caching working with cache invalidation
- [ ] Unit tests: role comparison logic correct

---

## 9. Estimated Effort

**Small (< 1 day)**

This is primarily a utility function and middleware component, not a full CRUD module.

---

## 10. Recommendations

**Required:**
- Use `PartialOrd` on `OrgRole` enum — do not use string comparison for role hierarchy.
- System Admin role must bypass org role checks (System Admin can access any org).

**Recommended:**
- Cache role lookups to avoid per-request database queries.
- Log all 403 responses with the required role and actual role for audit.

**Future Enhancement:**
- Custom org permissions per user (override on top of role-based).
