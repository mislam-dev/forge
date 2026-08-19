# Module 05 — Organizations

> **Module Type:** Core Module
> **Priority:** P1 — Core
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19
> **Source Docs:** [Organization Module](../../modules/organization/organization-module.md)

---

## 1. Module Overview

### Purpose

The Organizations module manages **organization lifecycle** — the top-level multi-tenant container in the Forge Platform. Every project, team, and deployment belongs to an organization.

### Responsibilities

- Create organizations (creator automatically becomes Owner)
- List organizations the authenticated user belongs to
- Get a specific organization by ID or slug
- Update organization details (name, slug)
- Delete organization (Owner or System Admin only)
- Enforce one-organization-per-slug uniqueness

### Scope

**Included:**
- `POST /organizations` — create org
- `GET /organizations` — list user's orgs
- `GET /organizations/:id` — get org
- `PUT /organizations/:id` — update org
- `DELETE /organizations/:id` — delete org

**Excluded:**
- Org membership management (Org Members sub-module)
- Org-level RBAC (Org Permissions sub-module)
- Teams within org (Teams module)
- Projects within org (Projects module)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/organization/mod.rs` | Exists — empty stub |
| Handlers | Not implemented |
| Service | Not implemented |
| SeaORM entities | Not generated |
| Tests | None |

---

## 3. Dependencies

### Depends On
- **Foundation**
- **Database** (organizations table)
- **Authentication** (JWT middleware)
- **Users** (org creator must be an authenticated user)

### Used By
- **Org Members** (references organizations)
- **Org Permissions** (references organizations)
- **Teams** (references organizations)
- **Projects** (references organizations)
- **Dashboard** (aggregates org metrics)

---

## 4. Database Table

### `organizations`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| name | VARCHAR(255) | Not Null |
| slug | VARCHAR(100) | Unique, Not Null |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

> **Note:** The module documentation mentions logo and description fields. These are not in the ERD. **Decision: Add nullable `logo_url VARCHAR` and `description TEXT` columns** per module documentation requirements.

**Key constraint:** `organizations(slug)` must be globally unique.

---

## 5. API Implementation

### POST /organizations

- **Auth:** JWT required (any authenticated user)
- **Request:** `{ name, slug, description?, logo_url? }`
- **Service logic (atomic transaction):**
  1. Validate slug format (lowercase, alphanumeric + hyphens)
  2. Check slug uniqueness
  3. Insert `organizations` record
  4. Insert `organization_members` record for creator with role `Owner`
- **Response:** `201 { message, data: organization }`
- **Errors:** `409 Conflict` on duplicate slug, `400` on validation failure

### GET /organizations

- **Auth:** JWT required
- **Service logic:** Return all organizations where `user_id` has an `organization_members` record
- **Response:** `200 { message, data: [organizations] }`

### GET /organizations/:id

- **Auth:** JWT + must be org member
- **Service logic:** Load org by ID, verify membership
- **Response:** `200 { message, data: organization }`

### PUT /organizations/:id

- **Auth:** JWT + org role: Admin or Owner
- **Request:** `{ name?, slug?, description?, logo_url? }`
- **Service logic:** Update non-null fields; slug uniqueness check if slug changes
- **Response:** `200 { message, data: updated_org }`

### DELETE /organizations/:id

- **Auth:** JWT + org role: Owner OR System Admin
- **Service logic:**
  1. Verify no active deployments are running
  2. Delete org (cascades to members, projects if on DELETE CASCADE)
- **Response:** `200 { message: "Organization deleted." }`

---

## 6. Authorization Matrix

| Action | Viewer | Developer | Admin | Owner | System Admin |
|--------|--------|-----------|-------|-------|--------------|
| Create org | N/A | Any authenticated | N/A | N/A | Yes |
| List my orgs | Yes | Yes | Yes | Yes | Yes |
| Get org | Yes | Yes | Yes | Yes | Yes |
| Update org | No | No | Yes | Yes | Yes |
| Delete org | No | No | No | Yes | Yes |

---

## 7. Logging

| Event | Level | Fields |
|-------|-------|--------|
| Organization created | INFO | org_id, slug, user_id, request_id |
| Organization updated | INFO | org_id, user_id, request_id |
| Organization deleted | WARN | org_id, user_id, request_id |
| Unauthorized access attempt | WARN | org_id, user_id, required_role, request_id |

---

## 8. Testing

### Integration Tests
- [ ] `POST /organizations` — success: org created, creator is Owner
- [ ] `POST /organizations` — duplicate slug: 409 returned
- [ ] `POST /organizations` — invalid slug format: 400 returned
- [ ] `GET /organizations` — returns only user's orgs
- [ ] `GET /organizations/:id` — member: 200 returned
- [ ] `GET /organizations/:id` — non-member: 403 returned
- [ ] `PUT /organizations/:id` — Admin: update success
- [ ] `PUT /organizations/:id` — Developer: 403 returned
- [ ] `DELETE /organizations/:id` — Owner: success
- [ ] `DELETE /organizations/:id` — Admin (not Owner): 403 returned

---

## 9. Implementation Tasks

### Database
- [ ] Create organizations migration with slug UK, nullable logo_url, description
- [ ] Generate SeaORM entity for `organizations`

### Service
- [ ] Implement `OrganizationService` in `src/modules/organization/service.rs`
- [ ] Implement `create_organization()` as atomic transaction (org + owner member)
- [ ] Implement `list_user_organizations()` — join with organization_members
- [ ] Implement `get_organization_by_id()` — check membership
- [ ] Implement `update_organization()` — slug uniqueness check
- [ ] Implement `delete_organization()` — cascade check

### Handlers
- [ ] Implement handlers for all 5 org endpoints
- [ ] Register routes in router

### Authorization
- [ ] Implement org membership check middleware/extractor
- [ ] Implement `require_org_role(minimum_role)` guard

### Testing
- [ ] Write all integration tests listed above

---

## 10. Definition of Done

- [ ] All 5 org endpoints functional
- [ ] Creator automatically becomes Owner
- [ ] Slug uniqueness enforced
- [ ] Org deletion restricted to Owner
- [ ] Non-members get 403 on org access
- [ ] All listed tests pass

---

## 11. Estimated Effort

**Medium (1–2 days)**

Org CRUD is straightforward. The main complexity is the atomic transaction for org creation + owner assignment, and the org membership check middleware.

---

## 12. Recommendations

**Required:**
- Org creation must be a single atomic transaction — if org_member insert fails, org record must be rolled back.
- Non-members must receive 403, not 404, to prevent org ID enumeration.

**Recommended:**
- Validate slug format: `^[a-z0-9][a-z0-9-]*[a-z0-9]$` (lowercase, no leading/trailing hyphens).
- Return both `id` and `slug` in all org responses for flexibility.

**Future Enhancement:**
- Org logo image upload (requires object storage).
- Org billing/subscription settings.
