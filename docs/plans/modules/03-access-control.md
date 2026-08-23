# Module 03 — Access Control (RBAC)

> **Module Type:** Core Sub-Module (Auth)
> **Priority:** P0 — Blocker
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19
> **Source Docs:** [Roles](../../modules/auth/access-control/00.Roles.md) | [Permissions](../../modules/auth/access-control/01.Permissions.md) | [Role-Permissions](../../modules/auth/access-control/02.RolePermissions.md) | [User-Roles](../../modules/auth/access-control/03.UserRoles.md) | [User-Permissions](../../modules/auth/access-control/04.UserPermissions.md)

---

## 1. Module Overview

### Purpose

The Access Control module manages the **system-wide RBAC layer** (Tier 1 of the three-tier RBAC hierarchy). It defines global roles (e.g., `admin`, `developer`, `viewer`) and atomic permissions, maps roles to permissions, and assigns roles/permissions to users.

### Responsibilities

- CRUD for `roles` (system-wide role definitions)
- CRUD for `permissions` (atomic permission definitions)
- Assign/remove permissions to/from roles (`role_permissions`)
- Assign/remove roles to/from users (`user_roles`)
- Assign/remove direct permissions to/from users (`user_permissions`)
- Provide a permission resolution service for the JWT middleware and all RBAC guards

### Scope

**Included:**
- All endpoints under `/access-control/*`
- System RBAC tables: `roles`, `permissions`, `role_permissions`, `user_roles`, `user_permissions`
- Permission resolution function: `resolve_user_permissions(user_id) -> Set<String>`

**Excluded:**
- Org-level permissions (Org Permissions sub-module)
- Project-level permissions (Project Permissions sub-module)
- All non-admin user operations

> **Important:** All `/access-control/*` endpoints are **System Admin only**. Regular users have no access to this module's API.

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/access_control/mod.rs` | Exists — empty stub |
| Handlers | Not implemented |
| Service | Not implemented |
| SeaORM entities | Not generated |
| Tests | None |

---

## 3. Dependencies

### Depends On
- **Foundation** (AppState, AppError)
- **Database** (roles, permissions, role_permissions, user_roles, user_permissions tables)
- **Authentication** (JWT middleware — all endpoints require System Admin JWT)

### Used By
- **All modules** (permission resolution consulted by RBAC guards)
- **Auth module** (JWT claims may embed roles/permissions)

---

## 4. Database Tables

### `roles`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| key | VARCHAR(100) | Not Null (human label, e.g., "System Administrator") |
| value | VARCHAR(100) | Unique, Not Null (code identifier, e.g., "admin") |
| descriptions | VARCHAR(255) | Nullable |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

### `permissions`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| key | VARCHAR(100) | Not Null (human label) |
| value | VARCHAR(100) | Unique, Not Null (code identifier) |
| descriptions | VARCHAR(255) | Nullable |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

### `role_permissions` (junction)

| Column | Type | Constraints |
|--------|------|-------------|
| role_id | UUID | PK (composite), FK -> roles.id CASCADE |
| permission_id | UUID | PK (composite), FK -> permissions.id CASCADE |

### `user_roles` (junction)

| Column | Type | Constraints |
|--------|------|-------------|
| user_id | UUID | PK (composite), FK -> users.id CASCADE |
| role_id | UUID | PK (composite), FK -> roles.id CASCADE |

### `user_permissions` (junction — direct override)

| Column | Type | Constraints |
|--------|------|-------------|
| user_id | UUID | PK (composite), FK -> users.id CASCADE |
| permission_id | UUID | PK (composite), FK -> permissions.id CASCADE |

---

## 5. API Implementation

All endpoints require `System Admin` role.

### Roles API

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/access-control/roles` | List all roles | 200 paginated list |
| POST | `/access-control/roles` | Create a role | 201 role |
| PUT | `/access-control/roles/:id` | Update a role | 200 role |
| DELETE | `/access-control/roles/:id` | Delete a role | 200 success |

### Permissions API

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/access-control/permissions` | List all permissions | 200 paginated list |
| POST | `/access-control/permissions` | Create a permission | 201 permission |
| PUT | `/access-control/permissions/:id` | Update a permission | 200 permission |
| DELETE | `/access-control/permissions/:id` | Delete a permission | 200 success |

### Role-Permission Mapping

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| POST | `/access-control/roles/permissions/assign` | Assign permissions to role | 200 success |
| POST | `/access-control/roles/permissions/remove` | Remove permissions from role | 200 success |
| GET | `/access-control/roles/permissions/:role_id` | Get permissions for a role | 200 list |

### User-Role Assignment

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| POST | `/access-control/role/assign` | Assign roles to a user | 200 success |
| POST | `/access-control/role/remove` | Remove roles from a user | 200 success |

### User-Permission Assignment (Direct Override)

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| POST | `/access-control/users/permission/assign` | Assign direct permissions to user | 200 success |
| POST | `/access-control/users/permission/remove` | Remove direct permissions from user | 200 success |
| GET | `/access-control/users/permissions/:user_id` | Get user's direct permissions | 200 list |

---

## 6. Permission Resolution Service

This is the core function used by all RBAC guards across the platform:

```rust
// Resolves all permissions for a user:
// = permissions from all assigned roles + direct user permissions
pub async fn resolve_user_permissions(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<HashSet<String>, AppError>
```

**Algorithm:**
1. Load all roles assigned to `user_id` via `user_roles`
2. For each role, load all permissions via `role_permissions`
3. Load all direct permissions via `user_permissions`
4. Union all permission `value` strings
5. Return as `HashSet<String>`

---

## 7. Authorization

All endpoints in this module require the `admin` system role (or equivalent system-admin permission). Regular authenticated users receive `403 Forbidden`.

---

## 8. Logging

| Event | Level | Fields |
|-------|-------|--------|
| Role created | INFO | role_id, value, admin_user_id |
| Role updated | INFO | role_id, admin_user_id |
| Role deleted | WARN | role_id, admin_user_id |
| Permission created | INFO | permission_id, value, admin_user_id |
| Permission deleted | WARN | permission_id, admin_user_id |
| Role assigned to user | INFO | user_id, role_id, admin_user_id |
| Role removed from user | WARN | user_id, role_id, admin_user_id |
| Direct permission granted | INFO | user_id, permission_id, admin_user_id |
| Unauthorized access attempt | WARN | endpoint, request_id, user_id |

---

## 9. Testing

### Unit Tests
- [ ] Permission resolution: user with role → correct permissions returned
- [ ] Permission resolution: user with direct permission override → included
- [ ] Permission resolution: no roles → empty set returned

### Integration Tests
- [ ] `GET /access-control/roles` — admin: 200 returned
- [ ] `GET /access-control/roles` — non-admin: 403 returned
- [ ] `POST /access-control/roles` — create role: 201 returned
- [ ] `POST /access-control/roles` — duplicate value: 409 returned
- [ ] `PUT /access-control/roles/:id` — update success
- [ ] `DELETE /access-control/roles/:id` — delete success
- [ ] `POST /access-control/roles/permissions/assign` — permission assigned to role
- [ ] `POST /access-control/role/assign` — role assigned to user
- [ ] Permission resolution end-to-end: user -> role -> permission

> **Status:** Completed (100%)
> **Last Updated:** 2026-08-18

---

## 10. Implementation Tasks

### Database
- [x] Verify roles, permissions, role_permissions, user_roles, user_permissions migrations created (in Database plan)
- [x] Generate SeaORM entities for all 5 tables

### Service
- [x] Implement `AccessControlService` in `src/modules/access_control/service.rs`
- [x] Implement role CRUD operations
- [x] Implement permission CRUD operations
- [x] Implement role-permission assignment/removal
- [x] Implement user-role assignment/removal
- [x] Implement user-permission assignment/removal
- [x] Implement `resolve_user_permissions(user_id)` function
- [ ] Seed default system roles on startup (`admin`, `developer`, `viewer`)

### Handlers
- [x] Implement handlers for all access-control endpoints
- [x] Register routes in router — all under `/access-control/*`
- [x] Apply System Admin guard to all routes

### Testing
- [x] Write unit tests for permission resolution
- [x] Write integration tests for all endpoints

---

## 11. Definition of Done

- [ ] All 16 access-control endpoints functional
- [ ] All endpoints return 403 for non-admin users
- [ ] `resolve_user_permissions()` correctly unions role + direct permissions
- [ ] Default system roles seeded
- [ ] All listed tests pass

---

## 12. Estimated Effort

**Medium (1–2 days)**

The CRUD operations are straightforward. The permission resolution function is the most important piece and must be correct.

---

## 13. Recommendations

**Required:**
- The `resolve_user_permissions()` function must be available before other modules implement RBAC guards.
- Default system roles (`admin`, `developer`, `viewer`) must be seeded via migration or startup code.

**Recommended:**
- Cache permission resolution results per user in Redis (TTL: 60s) to avoid N+1 database queries on every request. Cache key: `forge:permissions:{user_id}`.
- Invalidate cache when user roles/permissions change.

**Future Enhancement:**
- Hierarchical permission inheritance (e.g., `admin` inherits all `developer` permissions automatically).
