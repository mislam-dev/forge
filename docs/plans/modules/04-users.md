# Module 04 — Users & User Profile

> **Module Type:** Core Module
> **Priority:** P0 — Blocker
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Users Module](../../modules/users/Users-Module-Documentation.md) | [User Profile](../../modules/users/user-profile-module.md)

---

## 1. Module Overview

### Purpose

The Users module manages **user accounts and profile information** as persistent platform resources. It provides user lifecycle management (list, get, update, delete) and the user profile sub-module that stores extended profile data.

Note: User registration and authentication are handled by the Auth module. This module manages user resources after they exist.

### Responsibilities

- List all users (System Admin only)
- Get user by ID (self or admin)
- Update user account details (self or admin)
- Delete user account (self or admin with confirmation)
- User profile CRUD (first_name, last_name, phone, date_of_birth, gender, image)

### Scope

**Included:**

- `GET /users` — list all users (admin)
- `GET /users/:id` — get user
- `PUT /users/:id` — update user
- `DELETE /users/:id` — delete user
- `GET /users/:id/profile` — get profile
- `PUT /users/:id/profile` — update profile

**Excluded:**

- Registration and login (Auth module)
- Role/permission assignment (Access Control module)
- Org membership (Org Members module)

---

## 2. Current State

| Item                       | Status              |
| -------------------------- | ------------------- |
| `src/modules/users/mod.rs` | Exists — empty stub |
| Handlers                   | Not implemented     |
| Service                    | Not implemented     |
| SeaORM entities            | Not generated       |
| Tests                      | None                |

---

## 3. Dependencies

### Depends On

- **Foundation**
- **Database** (users table)
- **Authentication** (JWT middleware)

### Used By

- **Organizations** (org membership references users)
- **Teams** (team membership references users)
- **Projects** (owner_id references users)
- **Deployments** (triggered_by references users)
- **Notifications** (user_id references users)
- **All modules** (user identity lookups)

---

## 4. Database Tables

### `users`

| Column         | Type         | Constraints                                              |
| -------------- | ------------ | -------------------------------------------------------- |
| id             | UUID         | PK                                                       |
| name           | VARCHAR(255) | Not Null                                                 |
| email          | VARCHAR(255) | Unique, Not Null                                         |
| password_hash  | VARCHAR(255) | Not Null (never returned in API)                         |
| email_verified | BOOLEAN      | Default false                                            |
| status         | VARCHAR      | CHECK(Active, Unverified, Disabled, Suspended, Inactive) |
| created_at     | TIMESTAMP    | Not Null                                                 |
| updated_at     | TIMESTAMP    | Not Null                                                 |

> **Note:** `email_verified` and `status` are documented in the auth module but must be added to the users migration. The Users module owns the `users` table.

> **User profile fields** are not in the ERD's simplified users table. Per the module documentation, user profile data (first_name, last_name, phone, date_of_birth, gender, image_url) either extends the users table or lives in a separate `user_profiles` table. **Decision: Extend the `users` table with nullable profile columns** (simpler, no join required).

---

## 5. API Implementation

### GET /users

- **Auth:** JWT + System Admin role
- **Service logic:** Paginated list of all users (exclude password_hash)
- **Query params:** `page`, `per_page`, `search` (optional email/name filter)
- **Response:** `200 { message, data: [users], meta: { page, per_page, total } }`

### GET /users/:id

- **Auth:** JWT + (Self OR System Admin)
- **Service logic:** Load user by ID, exclude password_hash
- **Response:** `200 { message, data: user }`
- **Errors:** `404 Not Found`, `403 Forbidden` (not self, not admin)

### PUT /users/:id

- **Auth:** JWT + (Self OR System Admin)
- **Request:** `{ name: optional, email: optional }`
- **Service logic:** Update name and/or email; email uniqueness check if changing
- **Response:** `200 { message, data: updated_user }`

### DELETE /users/:id

- **Auth:** JWT + (Self OR System Admin)
- **Service logic:** Soft-delete or hard-delete user (check org ownership before deletion)
- **Response:** `200 { message: "User deleted." }`
- **Errors:** `409 Conflict` if user is sole org owner

### GET /users/:id/profile

- **Auth:** JWT (any authenticated user — public profile)
- **Response:** `200 { message, data: { first_name, last_name, phone, date_of_birth, gender, image_url } }`

### PUT /users/:id/profile

- **Auth:** JWT + Self only
- **Request:** `{ first_name, last_name, phone, date_of_birth, gender, image_url }`
- **Response:** `200 { message, data: updated_profile }`

---

## 6. Authorization Matrix

| Action         | Self                          | Org Admin/Owner | System Admin |
| -------------- | ----------------------------- | --------------- | ------------ |
| List all users | No                            | No              | Yes          |
| Get user by ID | Yes                           | No              | Yes          |
| Update user    | Yes                           | No              | Yes          |
| Delete user    | Yes                           | No              | Yes          |
| Get profile    | (own only for private fields) | No              | Yes          |
| Update profile | Yes                           | No              | No           |

---

## 7. Logging

| Event                               | Level | Fields                                   |
| ----------------------------------- | ----- | ---------------------------------------- |
| User profile updated                | INFO  | user_id, request_id                      |
| User account deleted                | WARN  | user_id, deleted_by, request_id          |
| User list accessed by admin         | INFO  | admin_user_id, request_id                |
| Unauthorized profile update attempt | WARN  | target_user_id, requester_id, request_id |

---

## 8. Testing

### Integration Tests

- [ ] `GET /users` — admin: list returned
- [ ] `GET /users` — non-admin: 403 returned
- [ ] `GET /users/:id` — self: 200 returned
- [ ] `GET /users/:id` — different user, not admin: 403 returned
- [ ] `GET /users/:id` — not found: 404 returned
- [ ] `PUT /users/:id` — self: update success
- [ ] `PUT /users/:id` — duplicate email: 409 returned
- [ ] `DELETE /users/:id` — self: success
- [ ] `GET /users/:id/profile` — profile returned
- [ ] `PUT /users/:id/profile` — self: update success
- [ ] `PUT /users/:id/profile` — other user: 403 returned

---

## 9. Implementation Tasks

### Database

- [ ] Ensure `users` migration includes all required columns
- [ ] Generate SeaORM entity for `users`

### Service

- [ ] Implement `UserService` in `src/modules/users/service.rs`
- [ ] Implement `list_users()` with pagination and optional search
- [ ] Implement `get_user_by_id()` — never return password_hash
- [ ] Implement `update_user()` — name, email (uniqueness check)
- [ ] Implement `delete_user()` — check for sole org ownership before deletion
- [ ] Implement `get_user_profile()` — profile fields subset
- [ ] Implement `update_user_profile()` — profile fields update

### Handlers

- [ ] Implement handlers for all 6 user endpoints
- [ ] Register routes in router

### Authorization

- [ ] Implement `require_self_or_admin()` Axum extractor/middleware
- [ ] Apply to PUT, DELETE, GET /:id

### Testing

- [ ] Write all integration tests listed above

---

## 10. Definition of Done

- [ ] All 6 user endpoints functional
- [ ] Password hash never returned in any response
- [ ] Self-access restriction enforced
- [ ] Pagination working on user list
- [ ] All listed tests pass

---

## 11. Estimated Effort

**Medium (1–2 days)**

User CRUD is straightforward. The profile sub-module is minimal. The main complexity is the authorization guard (self vs. admin).

---

## 12. Recommendations

**Required:**

- `password_hash` must be excluded from all API responses — use a separate DTO struct.
- Self-access check: `requested_user_id == jwt_user_id` OR user has `admin` system role.

**Recommended:**

- Use a separate `UserResponse` DTO (without `password_hash`) to make it impossible to accidentally leak the hash.
- Profile image: store as URL (not binary) in MVP — let the client handle image upload to object storage.

**Future Enhancement:**

- User profile picture upload endpoint (requires S3-compatible object storage).
- Account deactivation (soft-delete) instead of hard delete.
