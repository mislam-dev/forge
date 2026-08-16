# Infrastructure Plan: Database & Migrations

> **Plan Type:** Infrastructure
> **Priority:** P0 — Blocker
> **Status:** In Progress (~75%)
> **Last Updated:** 2026-08-17

---

## 1. Overview

PostgreSQL is the **sole authoritative data store** for all persistent business data in the Forge Platform (ADR-001). SeaORM is the exclusive database access layer (ADR-002).

This plan covers:

- Cargo dependencies for database access
- Database connection pool setup
- `sea-orm-migration` directory and runner
- All 20 database migrations (one per table)
- SeaORM entity generation
- Connection pool configuration

---

## 2. Current State

| Item                                 | Status                                                                    |
| ------------------------------------ | ------------------------------------------------------------------------- |
| `src/database/connection.rs`         | Implemented — SeaORM connection pool with backoff retries                 |
| `src/infrastructure/database/mod.rs` | Implemented                                                               |
| Cargo.toml dependencies              | Implemented (`sea-orm`, `sea-orm-migration`, `uuid`, `chrono`, etc.)      |
| Migrations directory                 | Implemented (`src/database/migrations/src/` containing all 20 migration files) |
| SeaORM entities                      | Pending generation (`just entity`)                                        |
| Connection pool                      | Implemented (`connect_db`)                                                |

> **Finding:** All 20 database migrations have been fully written and verified to compile (`cargo check`). Connection pool logic is implemented. SeaORM entity generation remains pending database migration execution.

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

| Migration | Table                           | Module Owner                        |
| --------- | ------------------------------- | ----------------------------------- |
| m001      | `users`                         | Users                               |
| m002      | `roles`                         | Access Control — Roles              |
| m003      | `permissions`                   | Access Control — Permissions        |
| m004      | `role_permissions`              | Access Control — Role-Permissions   |
| m005      | `user_roles`                    | Access Control — User-Roles         |
| m006      | `user_permissions`              | Access Control — User-Permissions   |
| m007      | `refresh_tokens`                | Auth                                |
| m008      | `password_resets`               | Auth                                |
| m009      | `organizations`                 | Organizations                       |
| m010      | `organization_members`          | Org Members                         |
| m011      | `teams`                         | Teams                               |
| m012      | `team_members`                  | Teams                               |
| m013      | `projects`                      | Projects                            |
| m014      | `project_repositories`          | Repository                          |
| m015      | `project_environment_variables` | Environment Variables               |
| m016      | `project_members`               | Project Assignments                 |
| m017      | `project_teams`                 | Project Assignments                 |
| m018      | `deployments`                   | Deployments                         |
| m019      | `build_logs`                    | Build Worker (legacy — see ADR-005) |
| m020      | `notifications`                 | Notifications                       |

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

- [x] Add `sea-orm`, `sea-orm-migration`, `uuid`, `chrono`, `tokio`, `dotenvy` to `Cargo.toml`
- [x] Verify `Cargo.lock` updates

### Database Directory

- [x] Create `src/database/migrations/` directory
- [x] Create `src/database/migrations/src/lib.rs` migration runner
- [x] Consolidate database module into `src/database/` and `src/infrastructure/database/`

### Migrations (in order)

- [x] m001 — `users` table with UUID PK, email UK, timestamps (`m20260816_101158_users.rs`)
- [x] m002 — `roles` table with value UK (`m20260816_110942_create_roles_table.rs`)
- [x] m003 — `permissions` table with value UK (`m20260816_111007_create_permissions_table.rs`)
- [x] m004 — `role_permissions` junction with composite PK + FK CASCADE (`m20260816_111021_create_role_permissions_table.rs`)
- [x] m005 — `user_roles` junction with FK (`m20260816_111037_create_user_roles_table.rs`)
- [x] m006 — `user_permissions` junction with FK (`m20260816_111046_create_user_permissions_table.rs`)
- [x] m007 — `refresh_tokens` with user FK (`m20260816_111110_create_refresh_tokens_table.rs`)
- [x] m008 — `password_resets` with user FK and expires_at (`m20260816_111122_create_password_resets_table.rs`)
- [x] m009 — `organizations` with slug UK (`m20260816_111136_create_organizations_table.rs`)
- [x] m010 — `organization_members` with composite unique (org, user), role CHECK (`m20260816_111142_create_organization_members_table.rs`)
- [x] m011 — `teams` with org FK (`m20260816_111150_create_teams_table.rs`)
- [x] m012 — `team_members` with team+user composite unique (`m20260816_111154_create_team_member_table.rs`)
- [x] m013 — `projects` with composite unique (org, name), runtime CHECK, status CHECK (`m20260816_111201_create_projects_table.rs`)
- [x] m014 — `project_repositories` with auth_type, access_token_encrypted, status (`m20260816_111213_create_project_repositories_table.rs`)
- [x] m015 — `project_environment_variables` with POSIX key CHECK, composite unique (`m20260816_111228_create_project_environment_variables_table.rs`)
- [x] m016 — `project_members` with composite unique (`m20260816_111242_create_project_members_table.rs`)
- [x] m017 — `project_teams` with composite unique (`m20260816_111248_create_project_teams_table.rs`)
- [x] m018 — `deployments` with status CHECK (`m20260816_111300_create_deployment_table.rs`)
- [x] m020 — `notifications` with user FK (`m20260816_111317_create_notifications_table.rs`)

### Connection Pool

- [x] Implement `connect_db()` connection pool in `src/database/connection.rs`
- [x] Expose `DatabaseConnection` via `AppState`
- [x] Configure pool size from environment variables (`DB_MAX_CONNECTIONS`, `DB_MIN_CONNECTIONS`, etc.)

### Entity Generation

- [ ] Run `just entity` for all module entities after migrations pass against live database
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
- [x] Connection pool is accessible from `AppState`
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
