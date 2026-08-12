# Cross-Module Data Flow

> **Document:** Cross-Module Data Flow  
> **Section:** 02 — Architecture  
> **Version:** 1.0  
> **Status:** Draft

This document describes how data moves across module boundaries during key platform workflows. It focuses on *integration points* between modules rather than within-module processing.

---

## 1. User Registration & First Login

**Modules involved:** Users → Auth → Access Control

```mermaid
sequenceDiagram
    actor User
    participant UsersAPI as Users Module
    participant AuthAPI as Auth Module
    participant AC as Access Control
    participant DB as Database

    User->>UsersAPI: POST /auth/register (name, email, password)
    UsersAPI->>DB: Create user record (hashed password)
    UsersAPI->>AC: Assign default system role
    UsersAPI-->>User: 201 User created

    User->>AuthAPI: POST /auth/login (email, password)
    AuthAPI->>DB: Verify credentials
    AuthAPI->>DB: Create refresh token session
    AuthAPI-->>User: {access_token, refresh_token}
```

**Key data crossings:**
- `users.id` is referenced by virtually all other modules as a foreign key.
- System roles (from Access Control) are assigned at user creation.
- JWT payload carries `user_id` across all subsequent requests.

---

## 2. Organization Setup & Member Onboarding

**Modules involved:** Organization → Org Members → Org Permissions → Teams

```mermaid
sequenceDiagram
    actor Owner
    participant OrgAPI as Organization Module
    participant MemberAPI as Org Members
    participant TeamAPI as Teams Module
    participant DB as Database

    Owner->>OrgAPI: POST /organizations (name, slug)
    OrgAPI->>DB: Create organization record
    OrgAPI->>MemberAPI: Auto-assign creator as Owner role
    OrgAPI-->>Owner: Organization created

    Owner->>MemberAPI: POST /organizations/:id/members (user_id, role)
    MemberAPI->>DB: Create organization_members record (user_id, org_id, role)
    MemberAPI-->>Owner: Member added

    Owner->>TeamAPI: POST /organizations/:id/teams (name)
    TeamAPI->>DB: Create team record
    Owner->>TeamAPI: POST /teams/:id/members (user_id)
    TeamAPI->>DB: Add user to team_members
```

**Key data crossings:**
- `organization_members` table links `users.id` + `organizations.id` + role.
- Every subsequent operation on org resources (projects, deployments) checks `organization_members` for role resolution.
- Teams are scoped to organizations (`teams.organization_id`).

---

## 3. Project Creation & Repository Connection

**Modules involved:** Projects → Organization → Repository → Environment Variables

```mermaid
sequenceDiagram
    actor Developer
    participant ProjAPI as Projects Module
    participant OrgPerms as Org Permissions
    participant RepoAPI as Repository Module
    participant EnvAPI as Env Variables Module
    participant DB as Database

    Developer->>ProjAPI: POST /projects (org_id, name, type, runtime)
    ProjAPI->>OrgPerms: Check Developer/Admin/Owner role in org
    OrgPerms-->>ProjAPI: Authorized
    ProjAPI->>DB: Create project record (owner_id = developer.id)
    ProjAPI-->>Developer: Project created

    Developer->>RepoAPI: POST /projects/:id/repository/validate (repo_url, auth_type, pat)
    RepoAPI->>GitService: git ls-remote (validate credentials)
    GitService-->>RepoAPI: Valid
    RepoAPI-->>Developer: Validation success

    Developer->>RepoAPI: POST /projects/:id/repository (repo_url, auth_type, pat)
    RepoAPI->>Crypto: AES-256-GCM encrypt PAT
    RepoAPI->>DB: Create project_repositories record
    RepoAPI-->>Developer: Repository connected

    Developer->>EnvAPI: POST /projects/:id/env-vars (key, value, environment, is_secret)
    EnvAPI->>Crypto: AES-256-GCM encrypt secret value
    EnvAPI->>DB: Create project_environment_variables record
    EnvAPI-->>Developer: Variable created
```

**Key data crossings:**
- `projects.organization_id` → `organizations.id` (org must exist).
- `projects.owner_id` → `users.id` (project creator).
- `project_repositories.project_id` → `projects.id`.
- `project_environment_variables.project_id` → `projects.id`.
- PAT and secret env vars are encrypted before DB write; plaintext never persists.

---

## 4. Triggering a Deployment

**Modules involved:** Deployments → Projects → Build Worker → Environment Variables → Live Build Logs

```mermaid
sequenceDiagram
    actor Developer
    participant DeployAPI as Deployments Module
    participant ProjAPI as Projects Module
    participant Queue as Job Queue
    participant Worker as Build Worker
    participant EnvAPI as Env Variables Module
    participant GitSvc as Git Service
    participant Docker as Docker Runtime
    participant LogStore as Log Store
    participant PubSub as Pub/Sub
    participant LogsAPI as Live Build Logs API

    Developer->>DeployAPI: POST /deployments (project_id, branch, commit_hash)
    DeployAPI->>ProjAPI: Verify project exists and is active
    ProjAPI-->>DeployAPI: Project active
    DeployAPI->>DB: Create deployment (status=Queued, triggered_by=user.id)
    DeployAPI->>Queue: Enqueue build job (deployment_id, project_id)
    DeployAPI-->>Developer: 201 {status: Queued}

    Queue-->>Worker: Dispatch job
    Worker->>DeployAPI: PATCH /deployments/:id/status (Building)
    Worker->>GitSvc: Clone repo @ commit_hash (using stored credentials)
    Worker->>Docker: docker build -t image_tag
    Worker->>PubSub: Publish build log lines
    PubSub->>LogsAPI: Forward to SSE subscribers

    Worker->>DeployAPI: PATCH /deployments/:id/status (Deploying)
    Worker->>EnvAPI: GET /projects/:id/env-vars/decrypt (internal runner auth)
    EnvAPI-->>Worker: Decrypted env vars
    Worker->>Docker: docker run --env-file (inject decrypted vars)
    Worker->>Docker: HTTP health check poll
    Docker-->>Worker: 200 OK
    Worker->>DeployAPI: PATCH /deployments/:id/status (Success, build_duration, deploy_duration)
    Worker->>LogStore: Write all structured log lines
```

**Key data crossings:**
- `deployments.project_id` → `projects.id` (project must be active).
- `deployments.triggered_by` → `users.id` (audit trail).
- Build Worker fetches encrypted credentials from `project_repositories` and decrypts them at runtime.
- Build Worker fetches encrypted env vars from `project_environment_variables` and decrypts them at runtime.
- All status transitions go back through `PATCH /deployments/:id/status` using an internal service token.
- Log lines are published to Pub/Sub (for live streaming) and written to `build_logs` (for storage).

---

## 5. Rollback Operation

**Modules involved:** Deployment History → Deployments → Build Worker

```mermaid
sequenceDiagram
    actor Admin
    participant HistAPI as Deployment History API
    participant DeployAPI as Deployments Module
    participant DB as Database
    participant Queue as Job Queue

    Admin->>HistAPI: POST /projects/:id/rollback (branch?)
    HistAPI->>DB: Query most recent deployment where status=Success for project+branch
    DB-->>HistAPI: Last successful deployment record
    HistAPI->>DeployAPI: POST /deployments (project_id, branch, commit_hash=last_success_commit)
    DeployAPI->>DB: Create new deployment record (status=Queued)
    DeployAPI->>Queue: Enqueue build job
    DeployAPI-->>HistAPI: New deployment record
    HistAPI-->>Admin: {rollback_from, new_deployment: {status: Queued}}
```

**Key data crossings:**
- Rollback reads `deployments` table to find last `Success` state.
- Creates a new `deployments` record (original record is immutable — BR-004).
- `rollback_from` references the previous deployment UUID for audit purposes.

---

## 6. Dashboard Data Aggregation

**Modules involved:** Dashboard → Projects, Deployments, Organizations, Users (read-only)

```mermaid
sequenceDiagram
    actor User
    participant DashAPI as Dashboard Module
    participant ProjDB as projects table
    participant DeployDB as deployments table
    participant OrgDB as organizations table

    User->>DashAPI: GET /dashboard
    DashAPI->>ProjDB: Count active/archived/draft projects
    DashAPI->>DeployDB: Count recent deployments by status
    DashAPI->>DeployDB: Fetch latest deployments
    DashAPI->>OrgDB: Fetch org summary
    DashAPI-->>User: Aggregated dashboard response
```

**Key facts:**
- Dashboard owns **no tables**. All data is read from other modules' tables.
- Dashboard does not write any state.
- This is a pure read-aggregation pattern.

---

## 7. Health Check Aggregation

**Modules involved:** Health → All service dependencies

```mermaid
sequenceDiagram
    actor Monitor
    participant HealthAPI as Health Module
    participant DB as Database
    participant Queue as Job Queue
    participant Auth as Auth Module health endpoint
    participant Deploy as Deployments health endpoint

    Monitor->>HealthAPI: GET /health
    par Probe all dependencies
        HealthAPI->>DB: Test connection
        HealthAPI->>Queue: Ping broker
        HealthAPI->>Auth: GET /auth/health
        HealthAPI->>Deploy: GET /deployments/health
    end
    HealthAPI-->>Monitor: Aggregated health report (status: ok/degraded/critical)
```

**Key facts:**
- Health module classifies dependencies as **critical** or **non-critical**.
- If a **critical** dependency is down, the platform status is `critical`.
- If a **non-critical** dependency is down, status is `degraded`.
- Health probes do not modify any state; all checks are read/ping-only.

---

## 8. Notification Dispatch

**Modules involved:** Notifications → Users (cross-module event consumer)

Notifications are triggered by events in other modules (e.g., deployment status changes, org membership changes). The notification module:

1. Receives event payloads (user_id, event type, message).
2. Persists notification records to the `notifications` table.
3. Delivers to users via in-app notification feed.

**Key data crossings:**
- `notifications.user_id` → `users.id` (recipient must exist).
- Notification events originate from Deployments (status change), Org Members (invitation), Teams (membership), and Projects (ownership changes).

---

**Document Version:** 1.0  
**Last Updated:** 2026-08-12  
**Author:** Backend Architecture Team
