# Infrastructure Plan: Database & Migrations

> **Plan Type:** Infrastructure
> **Priority:** P0 — Blocker
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19

---

## 1. Overview

PostgreSQL is the **sole authoritative data store** for all persistent business data in the Forge Platform (ADR-001). SeaORM is the exclusive database access layer (ADR-002).

This plan covers:

- Cargo dependencies for database access
- Database connection pool setup
- `sea-orm-migration` directory and runner
- All 25 database migrations (one per table/feature extension)
- SeaORM entity models across modules
- Connection pool configuration

---

## 2. Current State

| Item                                 | Status                                                                         |
| ------------------------------------ | ------------------------------------------------------------------------------ |
| `src/database/connection.rs`         | Implemented — SeaORM connection pool with backoff retries                      |
| `src/infrastructure/database/mod.rs` | Implemented                                                                    |
| Cargo.toml dependencies              | Implemented (`sea-orm`, `sea-orm-migration`, `uuid`, `chrono`, etc.)           |
| Migrations directory                 | Implemented (`src/database/migrations/src/` containing all 25 migration files) |
| SeaORM entities                      | Implemented in module entity directories                                       |
| Connection pool                      | Implemented (`connect_db`)                                                     |

> **Finding:** All 25 database migrations have been fully written, verified, and integrated into module tests. Connection pool logic and SeaORM entities are fully functioning.

---

## 3. Dependencies

### Depends On

- Foundation (Cargo.toml set up)
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
sea-orm = { version = "2.0.0", features = ["macros", "mock", "runtime-tokio-rustls", "sqlx-postgres"] }
sea-orm-cli = "2.0.2"

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

## 5. Database Tables Migrated

| Migration | Table / Extension                                                      | Module Owner                      |
| --------- | ---------------------------------------------------------------------- | --------------------------------- |
| m001      | `users`                                                                | Users                             |
| m002      | `roles`                                                                | Access Control — Roles            |
| m003      | `permissions`                                                          | Access Control — Permissions      |
| m004      | `role_permissions`                                                     | Access Control — Role-Permissions |
| m005      | `user_roles`                                                           | Access Control — User-Roles       |
| m006      | `user_permissions`                                                     | Access Control — User-Permissions |
| m007      | `refresh_tokens`                                                       | Auth                              |
| m008      | `password_resets`                                                      | Auth                              |
| m009      | `organizations`                                                        | Organizations                     |
| m010      | `organization_members`                                                 | Org Members                       |
| m011      | `teams`                                                                | Teams                             |
| m012      | `team_members`                                                         | Teams                             |
| m013      | `projects` (supports nullable `organization_id` for Personal projects) | Projects                          |
| m014      | `project_repositories`                                                 | Repository                        |
| m015      | `project_environment_variables`                                        | Environment Variables             |
| m016      | `project_members`                                                      | Project Assignments               |
| m017      | `project_teams`                                                        | Project Assignments               |
| m018      | `deployments`                                                          | Deployments                       |
| m019      | `notifications`                                                        | Notifications                     |
| m020      | `organization_invitations`                                             | Organizations                     |
| m021      | `organizations` (description & logo)                                   | Organizations                     |
| m022      | `team_members` (role)                                                  | Teams                             |
| m023      | `users_profile`                                                        | Users                             |

---

## 6. Implementation Status

- [x] All 25 migrations written and compiling
- [x] Connection pool configured in `AppState`
- [x] All module SeaORM entity models generated and integrated
- [x] FK, UNIQUE, and CHECK constraints active and tested
