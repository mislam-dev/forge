# Module 06 — Organization Members

> **Module Type:** Sub-Module (Organizations)
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Org Members Module](../../modules/organization/organization-members-module.md)

---

## 1. Module Overview

### Purpose

Manages **user membership within organizations**, including inviting members, updating their org-level role, and removing them.

### Responsibilities

- Invite a user to an organization
- List all members of an organization
- Update a member's org role
- Remove a member from an organization
- Prevent an owner from removing themselves (sole owner guard)

### Scope

**Included:**

- `POST /organizations/:org_id/members` — invite member
- `GET /organizations/:org_id/members` — list members
- `PUT /organizations/:org_id/members/:user_id` — update member role
- `DELETE /organizations/:org_id/members/:user_id` — remove member

**Excluded:**

- Org-level RBAC permission resolution (Org Permissions sub-module)
- Team membership (Teams module)

---

## 2. Dependencies

### Depends On

- **Organizations** (org must exist)
- **Users** (user being invited must exist)
- **Authentication**

### Used By

- **Org Permissions** (reads organization_members for role resolution)
- **Project Assignments** (membership checks)

---

## 3. Database Table

### `organization_members`

| Column          | Type      | Constraints                                      |
| --------------- | --------- | ------------------------------------------------ |
| organization_id | UUID      | PK (composite), FK -> organizations.id CASCADE   |
| user_id         | UUID      | PK (composite), FK -> users.id CASCADE           |
| role            | VARCHAR   | CHECK(Viewer, Developer, Admin, Owner), Not Null |
| joined_at       | TIMESTAMP | Not Null                                         |

---

## 4. API Implementation

### POST /organizations/:org_id/members

- **Auth:** JWT + org role: Admin or Owner
- **Request:** `{ user_id, role }` (role must be one of Viewer, Developer, Admin, Owner)
- **Service logic:** Check org exists, check user exists, check not already member, insert
- **Response:** `201 { message, data: member }`
- **Errors:** `404` org/user not found, `409` already a member, `400` invalid role

### GET /organizations/:org_id/members

- **Auth:** JWT + org member (any role)
- **Response:** `200 { message, data: [members with user info] }`

### PUT /organizations/:org_id/members/:user_id

- **Auth:** JWT + org role: Admin or Owner
- **Request:** `{ role }`
- **Service logic:** Cannot demote the sole Owner
- **Response:** `200 { message, data: updated_member }`
- **Errors:** `409 Conflict` if demoting sole owner

### DELETE /organizations/:org_id/members/:user_id

- **Auth:** JWT + org role: Admin or Owner (cannot remove sole Owner)
- **Service logic:** Check member exists, check not sole owner
- **Response:** `200 { message: "Member removed." }`

---

## 5. Business Rules

| Rule                                   | Implementation                                    |
| -------------------------------------- | ------------------------------------------------- |
| Cannot remove the sole Owner of an org | Check count of Owner-role members before deletion |
| Cannot demote the sole Owner           | Same check before role update                     |
| Duplicate membership not allowed       | Composite PK enforces this at DB level            |
| Only Admin or Owner can invite         | Authorization guard                               |

---

## 6. Testing

### Integration Tests

- [ ] `POST /members` — success: member added
- [ ] `POST /members` — duplicate: 409 returned
- [ ] `POST /members` — invalid role: 400 returned
- [ ] `GET /members` — list returned with user info
- [ ] `PUT /members/:user_id` — role updated
- [ ] `PUT /members/:user_id` — sole owner demote: 409 returned
- [ ] `DELETE /members/:user_id` — success
- [ ] `DELETE /members/:user_id` — sole owner: 409 returned
- [ ] `DELETE /members/:user_id` — not a member: 404 returned

---

## 7. Implementation Tasks

- [ ] Create `organization_members` migration (if not in main DB plan)
- [ ] Generate SeaORM entity for `organization_members`
- [ ] Implement `OrgMembersService` with all CRUD operations
- [ ] Implement sole-owner guard
- [ ] Implement handlers for all 4 endpoints
- [ ] Register routes in router
- [ ] Write all integration tests

---

## 8. Definition of Done

- [ ] All 4 member endpoints functional
- [ ] Sole owner protection enforced
- [ ] Duplicate membership rejected by DB constraint
- [ ] All tests pass

---

## 9. Estimated Effort

**Small-Medium (1 day)**

---

## 10. Recommendations

**Required:**

- Sole-owner guard: count owners before any removal or demotion operation.
- Role must be validated as one of: `Viewer`, `Developer`, `Admin`, `Owner`.

**Recommended:**

- For `GET /members`, join with the `users` table to return name and email alongside membership data.

**Future Enhancement:**

- Email invitation flow (invite by email address rather than user_id).
