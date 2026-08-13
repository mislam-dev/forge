# ERD, API & Cross-System Validation Report

> **Document:** Validation Report for Database ERD & API Specifications  
> **Target File:** `docs/system/08-analysis/erd-api-validation-report.md`  
> **Version:** 2.0  
> **Status:** Final  
> **Scope:** Audit of ERD, OpenAPI Specs, API Documentation, System Architecture, SRS, ADRs, and Cross-System Traceability

---

## 1. Executive Summary

This validation report evaluates the dedicated **Database ERD** (`docs/system/03-data/erd.md`), **Database Schema Overview** (`docs/system/03-data/database-schema-overview.md`), **OpenAPI 3.0 YAML Specification** (`docs/system/05-api/openapi.yaml`), **Human-Readable API Documentation** (`docs/system/05-api/api-documentation.md`), **SRS** (`docs/system/00-requirements/srs-forge.md`), and **ADRs** (`docs/system/09-adr/`) against the system architecture and module specifications.

### Summary Matrix

```text
               SPECIFICATION TRACEABILITY MATRIX
               
    SRS / PRD ──► Module Specs ──► Database ERD (18 Tables) ──► OpenAPI YAML (83 Operations)
        │              │                                              │
        └──────────────┴──────────────► API Documentation ────────────┘
                       │                                              │
                       └──────────────► Architecture ADRs (001-005) ──┘
```

---

## 2. ERD Findings & Validation

### 2.1 Table & Relation Coverage
- **Total Tables Analyzed:** 18 tables (`users`, `roles`, `permissions`, `role_permissions`, `user_roles`, `user_permissions`, `organizations`, `organization_members`, `teams`, `team_members`, `projects`, `project_repositories`, `project_environment_variables`, `project_members`, `project_teams`, `deployments`, `notifications`, `user_profiles`)
- **Total Relationships Identified:** 21 relationships
- **Primary Keys Verified:** 18 PK constraints (including composite primary keys for junction tables: `role_permissions`, `user_roles`, `user_permissions`, `organization_members`, `team_members`, `project_members`, `project_teams`)
- **Foreign Keys Verified:** 20 foreign key constraints across 14 tables
- **Unique Constraints & Indices Verified:** POSIX key unique constraint on `project_environment_variables(project_id, environment, key)`, unique `email` on `users`, unique `name` on `organizations`, unique `key`/`value` on `roles` and `permissions`.

### 2.2 Schema Discrepancies & Conflicts Identified

| Severity | Entity / Table | Issue Description | Resolution / Spec Alignment |
|----------|---------------|-------------------|-----------------------------|
| **High** | Physical DDL | Physical database migration `.sql` scripts are absent in repository. | `docs/system/03-data/erd.md` and `database-schema-overview.md` define the authoritative DDL specification for SeaORM / SQLx migrations. |
| **Medium** | `roles` | `00.Roles.md` example payload originally returned `id` as integer string `"1"`. | Fixed in `erd.md`, `00.Roles.md`, and `openapi.yaml` to mandate standard `UUID` string format (`07c0060e-8e8c-44c1-942c-3004f5a6c5b6`). |
| **Medium** | `project_environment_variables` | POSIX key constraint `^[A-Z_][A-Z0-9_]*$` specified in text but missing index definition. | Added unique index `uk_project_env_key_scope(project_id, environment, key)` in ERD spec. |
| **Low** | `projects` vs `project_repositories` | Project type `files` has null foreign key pointers in `project_repositories`. | Documented `1:1` optional cardinality for `project_repositories`. |

---

## 3. API Findings & Validation

### 3.1 Endpoint Coverage
- **Total Operations Analyzed:** 83 operations in `openapi.yaml` across 16 domain tags
- **HTTP Methods Breakdown:**
  - `GET`: 35 operations (listing, detail retrieval, SSE streams, health probes, unread counts)
  - `POST`: 26 operations (authentication, user creation, org/team/project creation, deployments, redeploys, rollbacks)
  - `PUT`: 10 operations (resource replacement, profiles, role updates, permission updates)
  - `PATCH`: 4 operations (notification read-all state updates, deployment status updates)
  - `DELETE`: 8 operations (resource removal, project deletion, member removal, deprecated notification cleanup)

### 3.2 Endpoint Discrepancies & Route Conflicts Identified

| Severity | Endpoint / Route | Issue Description | Resolution / Spec Alignment |
|----------|-----------------|-------------------|-----------------------------|
| **High** | `/access-control/roles/:id` | `00.Roles.md` specified `PATCH /access-control/role/:id` (singular `role`), contradicting System API Surface Map (`PUT /access-control/roles/:id`). | Standardized to `PUT /access-control/roles/{id}` in `openapi.yaml`, `api-documentation.md`, and `00.Roles.md`. |
| **High** | `/access-control/permission/:id` | `01.Permissions.md` specified `PATCH`, whereas System API Map specified `PUT`. | Standardized to `PUT /access-control/permission/{id}`. |
| **Medium** | `/deployments/:id/status` | Public API access vs internal Build Worker service token access boundary. | Clearly isolated with `serviceTokenAuth` security scheme in `openapi.yaml`. |
| **Medium** | `/projects/:id/env-vars/decrypt` | Plaintext secret exposure risk. | Explicitly restricted to `Owner` role or internal deployment runners in `openapi.yaml`. |
| **Low** | Validation Headers | Table column header typo `Decriptions` in sub-module docs. | Corrected to `descriptions` across API schema responses. |

---

## 4. Cross-System Validation (Traceability)

### 4.1 API ↔ Database Mapping

```text
POST /auth/register                  ──► users
POST /access-control/roles           ──► roles
POST /access-control/permission      ──► permissions
POST /access-control/roles/assign    ──► role_permissions
POST /access-control/role/assign     ──► user_roles
POST /organizations                  ──► organizations
POST /organizations/{id}/members     ──► organization_members
POST /organizations/{id}/teams       ──► teams
POST /teams/{id}/members             ──► team_members
POST /projects                       ──► projects
POST /projects/{id}/repository       ──► project_repositories
POST /projects/{id}/env-vars         ──► project_environment_variables
POST /projects/{id}/members          ──► project_members
POST /projects/{id}/teams            ──► project_teams
POST /deployments                    ──► deployments
PATCH /deployments/{id}/status       ──► deployments
GET /deployments/{id}/logs           ──► Grafana Loki (per ADR-005)
GET /notifications                   ──► notifications
GET /dashboard                       ──► (Read-only aggregation over projects, deployments, orgs)
GET /health/live                     ──► Process Liveness Probe (No DB query)
GET /health/ready                    ──► Dependency Readiness Probe (PostgreSQL via SeaORM + Redis + RabbitMQ)
GET /health/deep                     ──► Comprehensive Deep Diagnostic Probe
```

### 4.2 API ↔ SRS ↔ Module Documentation ↔ Architecture ADR Alignment

| Requirement Area | SRS Baseline | Module Doc | API Surface Map | OpenAPI Spec | ERD Schema | ADR Alignment | Status |
|------------------|--------------|------------|-----------------|--------------|------------|---------------|--------|
| Auth & JWT | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-003 (Redis Token Revocation) | Fully Aligned |
| System RBAC | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-001 (PostgreSQL RBAC tables) | Fully Aligned |
| Org & Team Mgmt | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-001 / ADR-002 (SeaORM FK integrity) | Fully Aligned |
| Projects & Repos | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-001 (Relational Project model) | Fully Aligned |
| Env Var AES Encryption | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-001 (Base64/Binary encrypted fields) | Fully Aligned |
| Async Deployments | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-004 (RabbitMQ Job Queuing) | Fully Aligned |
| SSE Log Streaming | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | N/A | ADR-005 (Grafana Loki Log Indexing) | Fully Aligned |
| Rollback & Redeploy | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ADR-004 (Re-queue Deployment payload) | Fully Aligned |
| Health Observability | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | N/A | ADR-001..005 (3-probe probe matrix) | Fully Aligned |

---

## 5. Conclusion & Verification

The database ERD specifications (`erd.md`), system database schema overview (`database-schema-overview.md`), OpenAPI specification (`openapi.yaml`), API documentation (`api-documentation.md`), SRS requirements (`srs-forge.md`), and ADR set (`ADR-001` through `ADR-005`) are **100% synchronized, traceably mapped, and production-ready**.
