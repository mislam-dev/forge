# ERD, API & Cross-System Validation Report

> **Document:** Validation Report for Database ERD & API Specifications  
> **Target File:** `docs/12-analysis/erd-api-validation-report.md`  
> **Version:** 1.0  
> **Status:** Final  
> **Scope:** Audit of ERD, OpenAPI Specs, API Documentation, and Cross-System Traceability

---

## 1. Executive Summary

This validation report evaluates the dedicated **Database ERD** (`docs/04-database/erd.md`), **OpenAPI 3.0 YAML Specification** (`docs/05-api/openapi.yaml`), and **Human-Readable API Documentation** (`docs/05-api/api-documentation.md`) against the system architecture and module specifications.

### Summary Matrix

```text
               SPECIFICATION TRACEABILITY MATRIX
               
    Module Specs ──► Database ERD (18 Tables) ──► OpenAPI YAML (58 Endpoints)
         │                                              │
         └──────────────────► API Documentation ────────┘
```

---

## 2. ERD Findings & Validation

### 2.1 Table & Relation Coverage
- **Total Tables Analyzed:** 18 tables
- **Total Relationships Identified:** 21 relationships
- **Primary Keys Verified:** 18 PK constraints (including 3 composite primary keys for junction tables: `role_permissions`, `user_roles`, `user_permissions`)
- **Foreign Keys Verified:** 20 foreign key constraints across 14 tables

### 2.2 Schema Discrepancies & Conflicts Identified

| Severity | Entity / Table | Issue Description | Resolution / Spec Alignment |
|----------|---------------|-------------------|-----------------------------|
| **High** | Source Code / DDL | Physical database migration `.sql` scripts are absent in `src/`. | The ERD defines the authoritative DDL specification for implementation. |
| **Medium** | `roles` | `00.Roles.md` example payload returns `id` as integer string `"1"`. | Fixed in `erd.md` and `openapi.yaml` to mandate standard `UUID` string format. |
| **Medium** | `project_environment_variables` | POSIX key constraint `^[A-Z_][A-Z0-9_]*$` specified in text but missing index definition. | Added unique index `uk_project_env_key_scope(project_id, environment, key)` in ERD. |
| **Low** | `projects` vs `project_repositories` | Project type `files` has null foreign key pointers in `project_repositories`. | Documented `1:1` optional cardinality for `project_repositories`. |

---

## 3. API Findings & Validation

### 3.1 Endpoint Coverage
- **Total Endpoints Analyzed:** 58 endpoints
- **HTTP Methods Breakdown:**
  - `GET`: 24 endpoints
  - `POST`: 18 endpoints
  - `PUT`: 8 endpoints
  - `PATCH`: 2 endpoints
  - `DELETE`: 6 endpoints

### 3.2 Endpoint Discrepancies & Route Conflicts Identified

| Severity | Endpoint / Route | Issue Description | Resolution / Spec Alignment |
|----------|-----------------|-------------------|-----------------------------|
| **High** | `/access-control/roles/:id` | `00.Roles.md` specified `PATCH /access-control/role/:id` (singular `role`), contradicting System API Surface Map (`PUT /access-control/roles/:id`). | Standardized to `PUT /access-control/roles/{id}` in `openapi.yaml` and `api-documentation.md`. |
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
GET /health                          ──► (No table — live service & DB connection probe)
```

### 4.2 API ↔ SRS / Module Documentation Alignment

| Requirement Area | Module Doc | API Surface | OpenAPI Spec | ERD Schema | Status |
|------------------|------------|-------------|--------------|------------|--------|
| Auth & JWT | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| System RBAC | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned (Routes Standardized) |
| Org Membership | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| Teams | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| Projects & Repos | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| Env Var AES Encryption | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| Async Deployments | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| SSE Log Streaming | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| Rollback / Redeploy | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | Fully Aligned |
| Health Observability | ✅ Yes | ✅ Yes | ✅ Yes | N/A | Fully Aligned |
