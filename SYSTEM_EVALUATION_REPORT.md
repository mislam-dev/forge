# Forge Platform — System & Modules Evaluation Report

> **Document Version:** 4.0  
> **Evaluation Scope:** System SRS (`docs/system/00-requirements/srs-forge.md`), Architecture Specifications (`docs/system/02-architecture/`), OpenAPI Specification (`docs/system/05-api/openapi.yaml`), Architecture Decision Records (ADRs 001–005), Domain Module Documentation (`docs/modules/`), and Rust/Axum Codebase (`src/`, `tests/`).  
> **Target System:** Forge Self-Hosted PaaS (Rust 2024 / Axum 0.8 / SeaORM 2.0 / Tokio / amqprs / Bollard)  
> **Evaluation Date:** October 2026  
> **Author:** Senior Principal Systems Architect & Evaluation Lead

---

## Executive Summary

An exhaustive evaluation of the **Forge Platform** was performed by contrasting the formal specifications in `docs/` against the actual implementation in `src/` and `tests/`.

The platform implements an impressive **Modular Monolith** architecture with clean separation across domain HTTP handlers, stateless service layers, and SeaORM entity repositories. It includes an IEEE-830 compliant SRS, 25 database migration scripts, an OpenAPI 3.0.3 specification with zero lint errors, and 330+ automated tests.

However, recent refactoring commits (`8d7a8e7`, `a8dbc9a`, `47c74f0`, `ce0c39d`) introduced critical regressions that broke test suites and exposed security/routing defects, alongside notable omissions of promised ADR infrastructure (such as Redis and Email Delivery).

---

## 1. Completed Modules & Features (Rated out of 10)

The following modules have complete or production-ready core business logic, database persistence, and architectural decoupling matching the SRS and module specifications.

| Module / Component                                                                                  | Score (0–10) |    Status    | Key Evaluation Rationale & Verification                                                                                                                                                                                                                                                                                      |
| --------------------------------------------------------------------------------------------------- | :----------: | :----------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Access Control (RBAC)** (`src/modules/access_control/`)                                           | **9.6 / 10** | **Complete** | Full 5-tier relational model (`roles`, `permissions`, `role_permissions`, `user_roles`, `user_permissions`). SeaORM migration scripts `m002` through `m006`, permission resolver (`AccessControlService::resolve_user_permissions`), and 6/6 passing integration tests (`tests/access_control_tests.rs`).                    |
| **Organizations Lifecycle** (`src/modules/organization/orgs/`)                                      | **9.5 / 10** | **Complete** | Multi-tenant organization CRUD, automated URL slug generation, logo handling, owner assignment, tenant isolation, and 9/9 passing integration tests (`tests/organization_tests.rs`).                                                                                                                                         |
| **Organization Members & Permissions** (`src/modules/organization/members/`, `permissions/`)        | **9.4 / 10** | **Complete** | Invitation tokens, role assignments (`Owner`, `Admin`, `Developer`, `Viewer`), sole-owner protection guard preventing owner self-demotion or removal.                                                                                                                                                                        |
| **Projects Core** (`src/modules/projects/projects/`)                                                | **9.5 / 10** | **Complete** | Dual-workspace architecture supporting Personal Projects (`org_id` is null) and Organization Projects. Runtimes, frameworks, statuses, and 10/10 passing tests in `tests/projects_tests.rs`.                                                                                                                                 |
| **Git Repository Management** (`src/modules/projects/repositories/`)                                | **9.2 / 10** | **Complete** | Public and private Git repository connection, AES-256-GCM Personal Access Token (PAT) encryption, token masking (`"••••••••"`), and 5/5 passing tests in `tests/repositories_tests.rs`.                                                                                                                                      |
| **Project Assignments & Project Permissions** (`src/modules/projects/assignments/`, `permissions/`) | **9.5 / 10** | **Complete** | Direct member assignments and team assignments to projects, role hierarchy resolution (`Owner > Admin > Developer > Viewer`), and 8/8 passing tests in `tests/assignments_tests.rs`.                                                                                                                                         |
| **Deployments Lifecycle & History** (`src/modules/projects/deployments/`)                           | **9.3 / 10** | **Complete** | State machine (`Queued` $\rightarrow$ `Building` $\rightarrow$ `Deploying` $\rightarrow$ `Running` $\rightarrow$ `Success` / `Failed`). Single in-progress deployment lock, internal `SERVICE_TOKEN` authorization callback guard, redeploy and rollback-to-last-success. 8/8 passing tests in `tests/deployments_tests.rs`. |
| **Database Migrations & Persistence** (`src/database/migrations/`)                                  | **9.7 / 10** | **Complete** | 25 SeaORM migration files covering all 18 domain entities, foreign keys, unique constraints, indices, connection pool management, and exponential backoff retry.                                                                                                                                                             |
| **API Specification & OpenAPI 3.0** (`docs/system/05-api/`, `docs/API_REQUESTS_SUMMARY.md`)         | **9.8 / 10** | **Complete** | Validated OpenAPI 3.0.3 specification (83 endpoints, 0 Redocly lint errors), Swagger UI mounting via `utoipa-swagger-ui`, and exhaustive API companion summary.                                                                                                                                                              |

---

## 2. Incomplete Modules & Features (Rated out of 10)

These modules are functional in isolation or partially implemented, but contain regressions, broken test suites, unfulfilled SRS functional requirements, or missing architectural links.

| Module / Component                                                                              | Score (0–10) |           Status           | Identified Gaps, Regressions & Failure Points                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------------------------------------------------------------------------------------- | :----------: | :------------------------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Application Router & Route Aliasing** (`src/app/app.rs`)                                      | **6.5 / 10** |       **Regressed**        | **Critical Route Deletion:** Commit `8d7a8e7` removed route aliases (`/api/*` and `/*`), mounting only `/api/v1/*`. This broke backward compatibility, the documented API contract, and caused **34 integration tests to fail with HTTP 404** across Auth, Teams, Notifications, Dashboard, Health, and User Profile test suites.                                                                                                                                                                                                                                                                                                                                            |
| **Authentication Module** (`src/modules/auth/`)                                                 | **7.0 / 10** | **Incomplete & Regressed** | 1. **Token Refresh RBAC Stripping:** In `AuthService::refresh()` (`src/modules/auth/service.rs:127-128`), regenerated access tokens hardcode `permissions: vec![]` and `roles: vec![]`, stripping all RBAC privileges on token refresh.<br>2. **Email Verification Stub:** SRS 5.1 requires email verification; currently `service.rs:243` contains `// todo: implement this` and reuses password reset tokens.<br>3. **Password Reset Incomplete:** Generates a token and prints it to stdout (`println!`), with `// todo send email to the user` (line 198).<br>4. **Test Failures:** Tests in `tests/auth_tests.rs` fail with 404 due to `/api/auth` route alias removal. |
| **Environment Variables Module** (`src/modules/projects/environment_variables/`)                | **7.5 / 10** |       **Regressed**        | 1. **Secret Masking Leak:** In `ProjectEnvVarResponse::from_model` (`src/modules/projects/environment_variables/dto/response.rs:26`), `value` is assigned `model.value_encrypted` directly instead of masking sensitive values as `"••••••••"`, leaking secrets and failing unit tests.<br>2. **Missing Validation:** `BulkCreateProjectEnvVarDTO.vars` lacks `#[validate(length(min = 1))]`, allowing empty bulk payloads and failing unit tests.                                                                                                                                                                                                                           |
| **Build Worker & Runtimes** (`src/modules/projects/build_worker/`)                              | **7.8 / 10** |       **Incomplete**       | 1. Builders exist for Dockerfile, Go, Node.js, Python, Rust, and Static Files using Bollard.<br>2. **Path Mismatch Bug:** `DeploymentPath::new` creates directories prefixed with `project_` and `deployment_`, but `clean_project` and `clean_deployment` remove directories without this prefix, causing `NotFound` OS errors and unit test panics (`src/modules/projects/build_worker/deployment_path.rs`).<br>3. Live container health checks and dynamic port binding cleanup need end-to-end container testing.                                                                                                                                                        |
| **Live Build Logs & Loki Client** (`src/modules/projects/logs/`, `src/infrastructure/logging/`) | **7.2 / 10** |       **Incomplete**       | 1. SSE endpoint and RabbitMQ fanout topic consumer are wired.<br>2. In `src/infrastructure/logging/loki_client.rs:62`, `push()` still contains `// TODO: implement push`.<br>3. Hardcoded Loki URL `http://127.0.0.1:3100` in `src/shared/logger.rs:39` instead of reading `AppConfig::get_infra().loki_url`.                                                                                                                                                                                                                                                                                                                                                                |
| **Teams & Team Members** (`src/modules/teams/`)                                                 | **8.0 / 10** |      **Route-Broken**      | All business logic, roles (`admin`, `developer`, `viewer`), and org checks exist, but all 10 tests in `tests/teams_tests.rs` fail with HTTP 404 because `/api/teams` alias was stripped from `app.rs`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| **Notifications Module** (`src/modules/notifications/`)                                         | **7.0 / 10** |       **Incomplete**       | In-app notification entity and CRUD handlers are written, but integration tests fail with 404 due to missing `/api/notifications` alias. Furthermore, event triggers from deployment success/failure and org invitations are not automatically publishing in-app notifications.                                                                                                                                                                                                                                                                                                                                                                                              |
| **Dashboard Aggregator** (`src/modules/dashboard/`)                                             | **8.0 / 10** |      **Route-Broken**      | Aggregation queries across orgs, projects, and deployments are written, but all 3 integration tests fail with 404 due to route alias removal in `app.rs`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| **Health & Observability** (`src/modules/health/`)                                              | **7.5 / 10** |       **Incomplete**       | `/api/v1/health` works, but probes at `/health`, `/health/liveness`, `/health/readiness`, and `/health/deep` fail with 404. RabbitMQ broker health is not aggregated into readiness status.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **RabbitMQ Asynchronous Messaging** (`src/infrastructure/queue/`)                               | **8.2 / 10** |       **Incomplete**       | AMQP topology (`forge.deployments.jobs`, `forge.logs`, DLX) and competing consumer are implemented, but consumer reconnection supervisor on broker disconnect is missing, and unused trait async warnings exist.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

---

## 3. Didn't Implement Yet (Not Started / Missing Requirements)

The following capabilities are explicitly specified in the SRS (`docs/system/00-requirements/srs-forge.md`) and Architectural Decision Records (ADRs), but have **zero implementation (0%)** in the codebase:

1. **Redis Integration (ADR-003):**
   - **Status:** **0% Implemented.**
   - **Details:** `Cargo.toml` lacks a Redis client dependency (e.g., `redis` or `deadpool-redis`). Only a configuration variable `REDIS_URL` exists.
   - **Missing Capabilities:**
     - Distributed rate limiting middleware on API routes (SRS 7, ADR-003).
     - Access token revocation / blacklisting on `/auth/logout` (`src/modules/auth/service.rs:105` is marked `// todo remove access_token from redis`).
     - Session storage and query cache layer.

2. **Email Delivery Service (SMTP / Transactional Email Provider):**
   - **Status:** **0% Implemented.**
   - **Details:** No email sending library (e.g., `lettre` or AWS SES/SendGrid SDK) exists in `Cargo.toml`.
   - **Missing Capabilities:**
     - Sending password reset emails with secure links (SRS 5.1).
     - Sending email verification emails upon registration (SRS 5.1).
     - Organization invitation emails to invitees (SRS 5.2, 5.11).
     - Deployment failure/success email alerts (SRS 5.11).

3. **Rate Limiting Middleware:**
   - **Status:** **0% Implemented.**
   - **Details:** Required by SRS Section 6 (Non-Functional Requirements) and Section 7 (Security Requirements) to prevent brute-force attacks on `/auth/login` and API flooding.

4. **Background Job Retry Engine:**
   - **Status:** **0% Implemented.**
   - **Details:** SRS Section 11 specifies background worker retry of failed jobs with exponential backoff and dead-letter processing. Dead-letter queue is declared in RabbitMQ topology, but no DLQ consumer or retry worker exists.

5. **E2E & Automated Load Testing:**
   - **Status:** **0% Implemented.**
   - **Details:** SRS Section 13 mandates End-to-End Tests and Load Testing (10,000+ concurrent requests target in SRS Section 6). No k6, Locust, or automated E2E test scripts exist in the repository.

6. **CI/CD Pipeline Configurations:**
   - **Status:** **0% Implemented.**
   - **Details:** No GitHub Actions workflow (`.github/workflows/`) exists for automated pull request testing, building, or migration verification.

---

## 4. Final Recommendations

The recommendations are prioritized into **Critical (Must Do)**, **Severe**, and **Normal (Not Required Now)**.

```
+------------------------------------------------------------------------------------+
|                         RECOMMENDED REMEDIATION ROADMAP                             |
+------------------------------------------------------------------------------------+
|                                                                                    |
|  [CRITICAL: Must Do Immediately]                                                   |
|    +-- Restore Route Aliases in app.rs (/api/*, /*)                                |
|    +-- Fix Secret Masking in ProjectEnvVarResponse ("••••••••")                   |
|    +-- Fix Auth Token Refresh (Preserve Roles & Permissions)                       |
|    +-- Fix DeploymentPath Directory Creation / Clean Path Prefix Mismatch          |
|    +-- Fix Config Server Port & Env Var DTO Validation Tests                       |
|                                                                                    |
|  [SEVERE: Core Architectural Deficits]                                             |
|    +-- Implement Redis Layer (ADR-003): Rate Limiting & Token Blacklisting         |
|    +-- Implement Transactional Email Delivery (Lettre / SMTP)                      |
|    +-- Connect Configurable Loki URL in logger.rs & Complete Push Handler          |
|    +-- Wire Automated In-App Notifications on Deployment Events                    |
|                                                                                    |
|  [NORMAL: Future Enhancements / Not Required Now]                                 |
|    +-- GitHub/GitLab App OAuth Integration & Webhooks                              |
|    +-- Automated E2E & Load Testing (k6 / Locust)                                  |
|    +-- Multi-region / Kubernetes Helm Charts                                       |
|                                                                                    |
+------------------------------------------------------------------------------------+
```

### Part 1: Critical (Must Do — Immediate Action Required)

These issues are direct regressions, security vulnerabilities, or test blockers currently present in the codebase:

1. **Restore Route Aliasing in `src/app/app.rs`:**
   - _Issue:_ Commit `8d7a8e7` removed route aliases. As a result, 34 integration tests fail with 404, and clients accessing `/api/auth`, `/api/teams`, `/api/notifications`, `/health`, etc. fail.
   - _Fix:_ Re-nest canonical aliases in `create_app()`:
     ```rust
     .nest("/api/v1/auth", auth_router())
     .nest("/api/auth", auth_router())
     .nest("/auth", auth_router())
     .nest("/api/v1/users", user_router())
     .nest("/api/users", user_router())
     .nest("/users", user_router())
     .nest("/api/v1/access-control", access_control_router())
     .nest("/api/access-control", access_control_router())
     .nest("/access-control", access_control_router())
     .nest("/api/v1/organizations", organization_router())
     .nest("/api/organizations", organization_router())
     .nest("/organizations", organization_router())
     .nest("/api/v1/teams", teams_router())
     .nest("/api/teams", teams_router())
     .nest("/teams", teams_router())
     .nest("/api/v1/projects", projects_router())
     .nest("/api/projects", projects_router())
     .nest("/projects", projects_router())
     .nest("/api/v1/notifications", notifications_router())
     .nest("/api/notifications", notifications_router())
     .nest("/notifications", notifications_router())
     .nest("/api/v1/dashboard", dashboard_router())
     .nest("/api/dashboard", dashboard_router())
     .nest("/dashboard", dashboard_router())
     .nest("/api/v1/health", health_router())
     .nest("/api/health", health_router())
     .nest("/health", health_router())
     ```
2. **Fix Secret Masking in `ProjectEnvVarResponse` (`src/modules/projects/environment_variables/dto/response.rs`):**
   - _Issue:_ Secrets are exposed directly via `value: model.value_encrypted`.
   - _Fix:_ Enforce secret masking:
     ```rust
     value: if model.is_secret.unwrap_or(false) {
         "••••••••".to_string()
     } else {
         model.value_encrypted
     },
     ```
3. **Fix Role & Permission Loss in `AuthService::refresh` (`src/modules/auth/service.rs`):**
   - _Issue:_ Refreshing an access token clears permissions and roles to `vec![]`, locking users out of RBAC-guarded routes after token refresh.
   - _Fix:_ Resolve permissions and roles via `AccessControlService::resolve_user_permissions` and `AccessControlService::get_user_roles_by_user_id` inside `refresh()` before building the new access token payload.
4. **Fix Path Prefix Mismatch in `DeploymentPath` (`src/modules/projects/build_worker/deployment_path.rs`):**
   - _Issue:_ Directory creation adds `project_` and `deployment_` prefixes, but `clean_project` and `clean_deployment` look for unprefixed paths, causing `NotFound` errors and unit test panics.
   - _Fix:_ Standardize prefix formatting in `clean_project` and `clean_deployment` to match `new()`.
5. **Fix Server Config Port Test & `BulkCreateProjectEnvVarDTO` Validation:**
   - _Fix 1:_ Update `src/config/env.rs:190` test assertion to match the updated default port `9000` (or make the default match `3000`).
   - _Fix 2:_ Add `#[validate(length(min = 1))]` to `BulkCreateProjectEnvVarDTO.vars`.

---

### Part 2: Severe (High Priority Architectural Needs)

These items fulfill the primary requirements of the SRS and ADRs to make the platform robust and secure:

1. **Implement Redis Caching & Rate-Limiting Layer (ADR-003):**
   - Add `deadpool-redis` or `redis` crate to `Cargo.toml`.
   - Create an Axum rate-limiting middleware (fixed window or token bucket) for public endpoints (`/auth/login`, `/auth/register`).
   - Implement access token blacklisting on `/auth/logout` so revoked JWTs cannot be used before expiration.
2. **Implement Transactional Email Dispatcher Service:**
   - Integrate an email crate (such as `lettre`) and SMTP configuration in `AppConfig`.
   - Implement real delivery for:
     - Password reset token links.
     - Email verification confirmation.
     - Organization invitation tokens.
3. **Connect Dynamic Loki URL & Complete Log Push:**
   - Replace hardcoded `http://127.0.0.1:3100` in `src/shared/logger.rs` with `app_config.infra.loki_url`.
   - Complete the Loki log stream push logic in `src/infrastructure/logging/loki_client.rs`.
4. **Wire Automatic In-App Notifications on Platform Events:**
   - When a deployment transitions to `Success` or `Failed`, dispatch an in-app notification record to the project creator.
   - When an organization invitation is sent, notify the invited user.

---

### Part 3: Normal (Not Required Now / Future Enhancements)

These items are defined as Phase 5 or Future Roadmap in the SRS and can be deferred until the core platform is stabilized:

1. **Git Provider OAuth Integration:**
   - Direct GitHub App / GitLab OAuth webhooks for automated commit push triggers (SRS Section 16).
2. **Kubernetes Orchestration:**
   - Generating Kubernetes manifests / Helm charts for cluster deployments (SRS Section 14 explicitly notes Kubernetes is out of scope for MVP).
3. **Object Storage Integration (S3 / MinIO):**
   - Storing built container artifacts or static build tarballs in S3-compatible buckets rather than local disk.
4. **Automated Load & Performance Benchmarking:**
   - Developing k6 test scripts to validate the 10,000+ concurrent connection requirement under high load.
5. **Team Audit Logs:**
   - Dedicated audit logging table tracking administrative actions (member removals, role updates, secret modifications).

---

## 5. Scorecard Summary

```
┌──────────────────────────────────────────────────────────────────┐
│                   FORGE PLATFORM AUDIT SCORECARD                 │
├──────────────────────────────────────────────────────────────────┤
│   DATABASE & SEAORM MIGRATIONS     :  9.7 / 10 (Production Ready)│
│   API SPEC & OPENAPI 3.0           :  9.8 / 10 (Exemplary)       │
│   ACCESS CONTROL (RBAC)            :  9.6 / 10 (Production Ready)│
│   ORGANIZATIONS & MEMBERS          :  9.5 / 10 (Production Ready)│
│   PROJECTS & ASSIGNMENTS           :  9.5 / 10 (Production Ready)│
│   DEPLOYMENTS LIFECYCLE            :  9.3 / 10 (Production Ready)│
│   RABBITMQ ASYNC MESSAGING         :  8.2 / 10 (Functional)      │
│   BUILD WORKER & RUNTIMES          :  7.8 / 10 (Gaps Identified) │
│   HEALTH & OBSERVABILITY           :  7.5 / 10 (Gaps Identified) │
│   ENVIRONMENT VARIABLES            :  7.5 / 10 (Regressed)       │
│   AUTHENTICATION & PROFILE         :  7.0 / 10 (Regressed)       │
│   APPLICATION ROUTER ALIASING      :  6.5 / 10 (Regressed)       │
│   REDIS CACHING & RATE LIMITING    :  0.0 / 10 (Not Started)     │
│   TRANSACTIONAL EMAIL DELIVERY     :  0.0 / 10 (Not Started)     │
├──────────────────────────────────────────────────────────────────┤
│   OVERALL COMPOSITE HEALTH SCORE   :  7.8 / 10 (Action Required) │
└──────────────────────────────────────────────────────────────────┘
```
