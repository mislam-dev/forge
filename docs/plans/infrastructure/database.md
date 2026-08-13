# Infrastructure Plan: Database & Migrations

> **Plan Type:** Infrastructure
> **Priority:** P0 — Blocker
> **Status:** Not Started
> **Last Updated:** 2026-08-13

---

## 1. Overview

PostgreSQL is the **sole authoritative data store** for all persistent business data in the Forge Platform (ADR-001). SeaORM is the exclusive database access layer (ADR-002).

This plan covers:
- Cargo dependencies for database access
- Database connection pool setup
- `sea-orm-migration` directory and runner
- All 18 database migrations (one per table)
- SeaORM entity generation
- Connection pool configuration

The database module lives in `src/infrastructure/database/` and `src/databse/` (note: existing directory has a typo — `src/databse/` — which should be corrected during foundation work).

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/databse/mod.rs` | Exists — empty stub |
| `src/databse/connection.rs` | Exists — empty stub |
| `src/infrastructure/database/mod.rs` | Exists — empty stub |
| Cargo.toml dependencies | Not added |
| Migrations directory | Missing (`src/database/migrations/`) |
| SeaORM entities | Not generated |
| Connection pool | Not implemented |

> **Note:** The justfile already references `src/database/migrations` as the migration directory. This path does not yet exist.

---

## 3. Dependencies

### Depends On
- Foundation (Cargo.toml must be set up first)
- PostgreSQL server (Docker Compose service)

### Used By
- Every module that reads or writes data (all modules)

### External Dependencies
- PostgreSQL 15+
- `sea-orm` crate with `sqlx-postgres` feature
- `sea-orm-migration` crate
- `sqlx` crate (used by SeaORM internally)

---

## 4. Required Cargo Dependencies

```toml
[dependencies]
# Database / ORM
sea-orm = { version = "1", features = ["sqlx-postgres", "runtime-tokio-rustls", "macros", "with-chrono", "with-uuid"] }
sea-orm-migration = { version = "1", features = ["sqlx-postgres", "runtime-tokio-rustls"] }

# UUID support
uuid = { version = "1", features = ["v4", "serde"] }

# DateTime support
chrono = { version = "0.4", features = ["serde"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Environment variables
dotenvy = "0.15"
```

---

## 5. Database Tables to Migrate (in dependency order)

> All tables derived from `docs/system/03-data/erd.md` and `docs/system/03-data/database-schema-overview.md`.

| Migration | Table | Module Owner |
|-----------|-------|--------------|
| m001 | `users` | Users |
| m002 | `roles` | Access Control — Roles |
| m003 | `permissions` | Access Control — Permissions |
| m004 | `role_permissions` | Access Control — Role-Permissions |
| m005 | `user_roles` | Access Control — User-Roles |
| m006 | `user_permissions` | Access Control — User-Permissions |
| m007 | `refresh_tokens` | Auth |
| m008 | `password_resets` | Auth |
| m009 | `organizations` | Organizations |
| m010 | `organization_members` | Org Members |
| m011 | `teams` | Teams |
| m012 | `team_members` | Teams |
| m013 | `projects` | Projects |
| m014 | `project_repositories` | Repository |
| m015 | `project_environment_variables` | Environment Variables |
| m016 | `project_members` | Project Assignments |
| m017 | `project_teams` | Project Assignments |
| m018 | `deployments` | Deployments |
| m019 | `build_logs` | Build Worker (legacy — see ADR-005) |
| m020 | `notifications` | Notifications |

---

## 6. Key Constraints to Enforce in Migrations

Per ADR-001:

- **Primary Keys:** All use UUID v4
- **Unique constraints:**
  - `users.email`
  - `organizations.slug`
  - `roles.value`
  - `permissions.value`
  - `organization_members(organization_id, user_id)` — composite
  - `projects(organization_id, name)` — composite
  - `project_environment_variables(project_id, environment, key)` — composite
  - `team_members(team_id, user_id)` — composite
- **Check constraints:**
  - `project_environment_variables.key ~ '^[A-Z_][A-Z0-9_]*$'`
  - `deployments.status IN ('Queued','Building','Deploying','Running','Failed','Success')`
  - `projects.runtime IN ('Node.js','Rust','Python','Go','Static Site')`
  - `organization_members.role IN ('Viewer','Developer','Admin','Owner')`
- **Partial index for single Running deployment per project:**
  - `CREATE UNIQUE INDEX idx_single_running_deployment ON deployments (project_id) WHERE status = 'Running'`
- **Performance indexes:**
  - `deployments(project_id, created_at DESC)`
  - `notifications(user_id, is_read, created_at DESC)`
  - B-tree on all FK columns

---

## 7. Connection Pool Setup

**Location:** `src/infrastructure/database/mod.rs`

```rust
// Conceptual (not prescriptive — follow SeaORM patterns exactly)
use sea_orm::{Database, DatabaseConnection};

pub async fn create_connection_pool(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(database_url.to_string());
    opts.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));
    Database::connect(opts).await
}
```

---

## 8. SeaORM Entity Generation

After migrations run, entities are generated per module using `just entity`:

```bash
just entity users/entities users
just entity auth/entities refresh_tokens,password_resets
just entity access_control/entities roles,permissions,role_permissions,user_roles,user_permissions
just entity organization/entities organizations,organization_members
just entity teams/entities teams,team_members
just entity projects/entities projects
just entity repositories/entities project_repositories
just entity enviroment_variables/entities project_environment_variables
just entity projects/entities project_members,project_teams
just entity deployments/entities deployments,build_logs
just entity logs/entities build_logs
just entity modules/notifications/entities notifications
```

All entities must include `--with-serde both --date-time-crate chrono`.

---

## 9. Implementation Tasks

### Cargo Setup
- [ ] Add `sea-orm`, `sea-orm-migration`, `uuid`, `chrono`, `tokio`, `dotenvy` to `Cargo.toml`
- [ ] Verify `Cargo.lock` updates

### Database Directory
- [ ] Create `src/database/migrations/` directory
- [ ] Create `src/database/migrations/mod.rs` migration runner
- [ ] Resolve typo: consolidate `src/databse/` into `src/infrastructure/database/`

### Migrations (in order)
- [ ] m001 — `users` table with UUID PK, email UK, timestamps
- [ ] m002 — `roles` table with value UK
- [ ] m003 — `permissions` table with value UK
- [ ] m004 — `role_permissions` junction with composite PK + FK CASCADE
- [ ] m005 — `user_roles` junction with FK
- [ ] m006 — `user_permissions` junction with FK
- [ ] m007 — `refresh_tokens` with user FK
- [ ] m008 — `password_resets` with user FK and expires_at
- [ ] m009 — `organizations` with slug UK
- [ ] m010 — `organization_members` with composite unique (org, user), role CHECK
- [ ] m011 — `teams` with org FK
- [ ] m012 — `team_members` with team+user composite unique
- [ ] m013 — `projects` with composite unique (org, name), runtime CHECK, status CHECK
- [ ] m014 — `project_repositories` with auth_type, access_token_encrypted, status
- [ ] m015 — `project_environment_variables` with POSIX key CHECK, composite unique
- [ ] m016 — `project_members` with composite unique
- [ ] m017 — `project_teams` with composite unique
- [ ] m018 — `deployments` with status CHECK, partial unique index for Running
- [ ] m019 — `build_logs` (legacy, see ADR-005 — create but do not use for live logs)
- [ ] m020 — `notifications` with user FK

### Connection Pool
- [ ] Implement `create_connection_pool()` in `src/infrastructure/database/mod.rs`
- [ ] Expose `DatabaseConnection` via `AppState`
- [ ] Configure pool size from environment variables

### Entity Generation
- [ ] Run `just entity` for all module entities after migrations pass
- [ ] Verify all generated entity files compile

### Testing
- [ ] Migration `up` runs cleanly from empty DB
- [ ] Migration `down` rolls back cleanly
- [ ] All FK constraints are enforced (foreign key violation test)
- [ ] Unique constraints are enforced (duplicate insert test)
- [ ] Check constraints are enforced (invalid enum value test)
- [ ] Partial index prevents duplicate Running deployment

---

## 10. Definition of Done

- [ ] All 20 migrations run with `just db-up` from empty database
- [ ] All migrations roll back with `just db-down`
- [ ] All 18 entity files generated and compiling
- [ ] Connection pool is accessible from `AppState`
- [ ] FK constraints tested
- [ ] Unique constraints tested
- [ ] Check constraints tested
- [ ] Partial deployment index tested

---

## 11. Estimated Effort

**Large (3–5 days)**

Migration writing is mechanical but careful. Entity generation is automated but requires validation. The constraint and index setup requires careful SQL knowledge.

---

## 12. Recommendations

**Required:**
- All constraints from ADR-001 must be implemented in migrations — not just at the application layer.
- The `password_resets` and `refresh_tokens` tables are required by the auth module documentation but are not in the simplified ERD diagram — they must be included.

**Recommended:**
- Use `uuid-ossp` PostgreSQL extension for `uuid_generate_v4()` or use Rust-generated UUIDs (both are acceptable per ADR-001).
- Apply `NOT NULL DEFAULT CURRENT_TIMESTAMP` on all `created_at`/`updated_at` columns.
- Add `ON UPDATE CURRENT_TIMESTAMP` trigger or handle `updated_at` in Rust service layer.

**Future Enhancement:**
- Database-level row-level security (RLS) for multi-tenancy enforcement at the PostgreSQL layer.
