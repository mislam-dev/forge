# Module 11 — Environment Variables

> **Module Type:** Sub-Module (Projects)
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Environment Variables Module](../../modules/projects/environment-variables-module.md)

---

## 1. Module Overview

### Purpose

The Environment Variables sub-module manages **encrypted key-value configuration** for projects, scoped per deployment environment (Development, Preview, Production). Sensitive values are encrypted at rest using AES-256-GCM.

### Responsibilities

- Create environment variables (key=value pairs, per environment)
- List environment variables (values masked by default)
- Update an environment variable's value
- Delete an environment variable
- Bulk create environment variables
- Decrypt and return environment variable values (privileged only)
- Provide decrypted env var map to Build Worker (internal)

### Scope

**Included:**
- `POST /projects/:project_id/env-vars` — create env var
- `GET /projects/:project_id/env-vars` — list env vars (values masked)
- `PUT /projects/:project_id/env-vars/:id` — update env var
- `DELETE /projects/:project_id/env-vars/:id` — delete env var
- `POST /projects/:project_id/env-vars/bulk` — bulk create
- Internal service: decrypt all env vars for a project + environment (Build Worker use)

**Excluded:**
- Reading plaintext env var values via public API (security requirement)
- System-level configuration (these are project-specific only)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/enviroment_variables/mod.rs` | Exists — empty stub (note: typo in directory name) |
| Handlers | Not implemented |
| Service | Not implemented |
| Tests | None |

> **Note:** Directory name has a typo: `enviroment_variables` (missing 'n'). This typo must be preserved for backward compatibility with existing file paths, OR corrected with a module rename — this is a **decision point** for the implementation engineer.

---

## 3. Dependencies

### Depends On
- **Projects** (env vars belong to a project)
- **Encryption** (AES-256-GCM encryption of values)
- **Authentication**

### Used By
- **Build Worker** (reads decrypted env vars for build-time injection)
- **Deployments** (validates env vars exist before triggering)

---

## 4. Database Table

### `project_environment_variables`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| project_id | UUID | FK -> projects.id CASCADE, Not Null |
| environment | VARCHAR | CHECK(Development, Preview, Production), Not Null |
| key | VARCHAR(255) | Not Null, POSIX regex: `^[A-Z_][A-Z0-9_]*$` |
| value_encrypted | TEXT | Not Null |
| is_secret | BOOLEAN | Default true |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

**Constraints:**
- Composite unique: `(project_id, environment, key)` — no duplicate keys per env
- CHECK constraint: `key ~ '^[A-Z_][A-Z0-9_]*$'` (POSIX regex — uppercase with underscores)

---

## 5. Environment Values

The `environment` field must be one of:
- `Development`
- `Preview`
- `Production`

---

## 6. API Implementation

### POST /projects/:project_id/env-vars

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** `{ environment, key, value, is_secret? }`
- **Validation:**
  - `key` must match `^[A-Z_][A-Z0-9_]*$`
  - `environment` must be Development, Preview, or Production
- **Service logic:**
  1. Check uniqueness: `(project_id, environment, key)` must be unique
  2. Encrypt value: `EncryptionService::encrypt(value, project_id)`
  3. Insert record
- **Response:** `201 { message, data: { id, key, environment, is_secret, value: "••••••••" } }`

### GET /projects/:project_id/env-vars

- **Auth:** JWT + project member
- **Query params:** `environment` (optional filter)
- **Service logic:** Load all env vars; **never decrypt values** — return masked `"••••••••"`
- **Response:** `200 { message, data: [env_vars_with_masked_values] }`

### PUT /projects/:project_id/env-vars/:id

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** `{ value?, is_secret? }` (key and environment cannot change)
- **Service logic:** Encrypt new value, update record
- **Response:** `200 { message, data: updated env var (value masked) }`

### DELETE /projects/:project_id/env-vars/:id

- **Auth:** JWT + project owner OR org Admin/Owner
- **Response:** `200 { message: "Environment variable deleted." }`

### POST /projects/:project_id/env-vars/bulk

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** `{ environment, vars: [{ key, value, is_secret }] }`
- **Service logic:** Atomic transaction — insert all or none. Encrypt each value.
- **Response:** `201 { message, data: [created_env_vars] }`

### Internal: Get Decrypted Env Vars (Build Worker only)

- **Auth:** Service token (`SERVICE_TOKEN` header)
- **Service logic:** Decrypt all env vars for a project+environment
- **Returns:** `HashMap<String, String>` (in-memory, never persisted in logs)

---

## 7. Security Rules

- Values must **never** be returned in plaintext via any public API endpoint
- Values are masked as `"••••••••"` in all API responses (regardless of actual length)
- Decryption only happens in the service layer for Build Worker injection
- Build Worker receives decrypted values in-memory; they must never be logged
- `key` format must be validated against the POSIX pattern at service layer AND enforced by DB CHECK constraint

---

## 8. Testing

### Unit Tests
- [ ] Key validation: `DATABASE_URL` passes, `database-url` fails, `1INVALID` fails
- [ ] Environment validation: `Production` passes, `staging` fails
- [ ] Value encrypt/decrypt round trip

### Integration Tests
- [ ] `POST /env-vars` — valid key: success, value masked in response
- [ ] `POST /env-vars` — invalid key format: 400 returned
- [ ] `POST /env-vars` — duplicate key in same env: 409 returned
- [ ] `GET /env-vars` — all values masked
- [ ] `PUT /env-vars/:id` — new value encrypted on update
- [ ] `DELETE /env-vars/:id` — success
- [ ] `POST /env-vars/bulk` — all created atomically
- [ ] `POST /env-vars/bulk` — partial failure: all rolled back

---

## 9. Implementation Tasks

### Database
- [ ] Create `project_environment_variables` migration with POSIX CHECK constraint and composite unique index
- [ ] Generate SeaORM entity for `project_environment_variables`

### Service
- [ ] Implement `EnvVarsService` in `src/modules/enviroment_variables/service.rs`
- [ ] Implement key format validation (`^[A-Z_][A-Z0-9_]*$`)
- [ ] Implement environment enum validation
- [ ] Implement encryption on write, masking on read
- [ ] Implement `get_decrypted_env_vars(project_id, environment)` for Build Worker (internal)
- [ ] Implement bulk create as atomic transaction

### Handlers
- [ ] Implement handlers for all 5 public endpoints
- [ ] Register routes in router

### Internal Endpoint / Service Function
- [ ] `get_decrypted_env_vars()` service function with SERVICE_TOKEN auth guard

### Testing
- [ ] Write all unit and integration tests

---

## 10. Definition of Done

- [ ] All 5 public env var endpoints functional
- [ ] Values always masked in public API responses
- [ ] POSIX key format enforced
- [ ] Composite unique constraint enforced
- [ ] Bulk create is atomic (all or none)
- [ ] Decrypted values available for Build Worker
- [ ] All tests pass

---

## 11. Estimated Effort

**Medium (1–2 days)**

The POSIX key validation, bulk create atomicity, and encryption integration are the main complexity points.

---

## 12. Recommendations

**Required:**
- POSIX regex validation must happen at the service layer (before DB write) in addition to the DB CHECK constraint.
- The masked value placeholder must always be exactly `"••••••••"` (8 bullet characters) — do not vary by value length.

**Recommended:**
- Provide a separate `is_secret` flag: even for non-secret values, still encrypt (consistency). The `is_secret` flag controls UI display only.

**Future Enhancement:**
- Environment variable groups/categories.
- Integration with external secrets managers (AWS Secrets Manager, HashiCorp Vault).
- Variable interpolation: `${OTHER_VAR}` syntax support.
