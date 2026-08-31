# Security Architecture

> **Document:** Security Architecture  
> **Section:** 04 — Security  
> **Version:** 1.0  
> **Status:** Draft

---

## 1. Security Principles

The Forge platform is built with the following security principles:

| Principle | Implementation |
|-----------|---------------|
| **Authentication Required** | All endpoints except public health check and `/auth/*` require a valid JWT |
| **Principle of Least Privilege** | Every role is granted the minimum permissions needed; access escalates through well-defined role hierarchy |
| **Defense in Depth** | Multiple permission layers: system RBAC → org-level RBAC → project ownership check |
| **Secrets Never in Plaintext** | PAT tokens and secret env vars are encrypted with AES-256-GCM before storage; never returned in plaintext responses |
| **Immutable Audit Trail** | Deployment records carry `triggered_by` user ID; historical records cannot be modified |
| **Internal Service Isolation** | Build Worker uses service tokens, not user JWTs; internal endpoints are not exposed to external clients |

---

## 2. Authentication

### 2.1 JWT Access Tokens

- **Standard:** JSON Web Token (JWT), signed (HS256 or RS256).
- **Payload:** `user_id`, expiry (`exp`), issued-at (`iat`).
- **Lifetime:** Short-lived (e.g., 15 minutes).
- **Usage:** Passed as `Authorization: Bearer <token>` header on every protected request.
- **Validation:** Auth middleware validates signature and expiry on every request.

### 2.2 Refresh Tokens

- **Storage:** Persisted in the database (hashed or securely stored).
- **Purpose:** Exchange for a new access token without re-authenticating.
- **Invalidation:** Logout operation removes the refresh token session from the database.
- **Rotation:** Optionally rotate refresh token on every use (single-use refresh tokens).

### 2.3 Internal Service Token (Build Worker)

- Build Worker communicates with the Deployment API via a dedicated **service credential** — not a user JWT.
- This credential authorizes only:
  - `PATCH /deployments/:id/status` (status transitions)
  - `POST /deployments/:id/logs` (log writes)
  - `GET /projects/:id/env-vars/decrypt` (env var decryption for injection)
- External clients cannot use or obtain this service credential.

### 2.4 Login Flow

```mermaid
flowchart TD
    A[User] --> B[POST /auth/login email+password]
    B --> C[Verify password hash against DB]
    C --> D{Credentials Valid?}
    D -->|No| E[Return 401 Unauthorized]
    D -->|Yes| F[Issue signed JWT access_token]
    F --> G[Create refresh_token session in DB]
    G --> H[Return access_token + refresh_token]
```

---

## 3. Authorization — Multi-Tier RBAC & Dual Workspace Model

Forge supports two operating models: **Personal Workspaces** (individual developer projects) and **Organization Workspaces** (collaborative team multi-tenancy).

### Tier 1: System Access Control (Global Roles & Permissions)

- Managed by System Administrators via `/access-control/*` endpoints.
- Defines global `roles` (e.g., `admin`, `developer`, `viewer`) and fine-grained `permissions` (e.g., `projects:create`, `deployments:create`).
- Encoded into `JwtClaims` upon authentication.
- **For Personal Workspaces (`organization_id IS NULL`)**: This tier serves as the **primary gatekeeper** (e.g. evaluating `ProjectsCreatePolicy`, `DeploymentsCreatePolicy`).
- **For Platform Operators**: System Admin role bypasses tenant boundaries for system administration.

### Tier 2: Organization-Level RBAC

- Managed by Organization Owners and Admins within their specific tenant.
- Each organization member has a role: `Viewer`, `Developer`, `Admin`, `Owner`.
- Enforced by the **Org Permissions** sub-module on all organization-scoped resource operations (`organization_id IS NOT NULL`).

| Org Role | Capabilities |
|----------|-------------|
| **Viewer** | Read-only access to all org resources |
| **Developer** | Create projects (auto-assigned as owner), trigger deployments, manage own projects |
| **Admin** | Full CRUD on all org projects, members, and deployments |
| **Owner** | Unrestricted; includes rollback, member management, and org lifecycle |

### Tier 3: Resource Ownership & ABAC

- Enforced by the **Project Permissions** sub-module and contextual guards.
- **Personal Projects**: Full access granted to the creator (`owner_id == self.id`); denied to other users.
- **Organization Projects**: `Developer` role users can delete projects only if `project.owner_id == self.id`. `Admin` and `Owner` roles bypass project ownership constraints.

```mermaid
flowchart TD
    REQ[Request arrives] --> JWT[Validate JWT]
    JWT -->|Invalid| R401[401 Unauthorized]
    JWT -->|Valid| SYSROLE{System Admin?}
    
    SYSROLE -->|Yes| ALLOW[Authorize — Full Access]
    SYSROLE -->|No| WSCHECK{Is Resource Personal or Org?}
    
    WSCHECK -->|Personal: organization_id IS NULL| PGUARD{Check System Policy & owner_id}
    PGUARD -->|Policy + Owner Match| ALLOW
    PGUARD -->|Denied| R403[403 Forbidden]
    
    WSCHECK -->|Org: organization_id IS NOT NULL| ORGROLE[Check Org Membership Role]
    ORGROLE -->|No Membership| R403
    ORGROLE -->|Viewer| READONLY[Read-only allowed; write denied]
    ORGROLE -->|Developer/Admin/Owner| PROJCHECK[Project Ownership Check if applicable]
    PROJCHECK -->|Admin/Owner| ALLOW
    PROJCHECK -->|Developer + owner_id match| ALLOW
    PROJCHECK -->|Developer + no ownership| R403
```

---

## 4. Encryption

### 4.1 At-Rest Encryption

Two categories of sensitive data are encrypted before database storage:

| Data | Table | Column | Algorithm | Key Management |
|------|-------|--------|-----------|---------------|
| Git Personal Access Tokens (PAT) | `project_repositories` | `access_token_encrypted` | AES-256-GCM | Master secret key + project-scoped salt |
| Secret environment variable values | `project_environment_variables` | `value_encrypted` | AES-256-GCM | Master secret key + project ID salt |

**AES-256-GCM encryption process (for env vars):**

```mermaid
flowchart LR
    PL[Plaintext value] --> KEY[Derive key from master secret + project_id salt]
    KEY --> IV[Generate random 12-byte IV]
    IV --> ENC[AES-256-GCM encrypt]
    ENC --> OUT[Base64 encode: IV + Ciphertext + AuthTag]
    OUT --> DB[(Database)]
```

### 4.2 Decryption Authorization

| Actor | Can Decrypt? | Mechanism |
|-------|-------------|-----------|
| Public API user | ❌ Never | Masked as `••••••••` in responses |
| Project Owner (via API) | ✅ For their project | `GET /projects/:id/env-vars/decrypt` — restricted to Owner/System Admin |
| Build Worker | ✅ At runtime only | Internal service token; decryption happens in-memory, never logged |

### 4.3 PAT Handling Rules

> From Repository Module BR-003 and BR-004:
- PAT **must** be encrypted with AES-256-GCM before DB insertion.
- PAT **must never** be returned in API responses or written to log outputs.
- Git clone operations use temporary credential helpers or memory streams to prevent token exposure in CLI arguments.

---

## 5. API Security Controls

### 5.1 Endpoint Protection Matrix

| Endpoint Category | Auth Required | Role Minimum | Notes |
|-------------------|--------------|-------------|-------|
| `GET /health` | ❌ | None | Public health probe |
| `POST /auth/register` | ❌ | None | Public |
| `POST /auth/login` | ❌ | None | Public |
| `POST /auth/refresh` | ✅ (refresh token) | Any | Requires valid refresh token |
| `/access-control/*` | ✅ | System Admin | Admin-only |
| `/organizations/*` | ✅ | Authenticated | Org role checked per action |
| `/teams/*` | ✅ | Authenticated | Org role checked per action |
| `/projects/*` | ✅ | Authenticated | Personal: System Policy + Ownership; Org: Org Role + Ownership |
| `/deployments/*` (trigger) | ✅ | Developer / Policy | Personal: `deployments:create` + Owner; Org: Org Role + Project assignment |
| `PATCH /deployments/:id/status` | ✅ | Internal Service | Build Worker service token only |
| `/deployments/:id/logs/*` | ✅ | Viewer | Must have project access |
| `GET /dashboard` | ✅ | Authenticated | Scoped to user's orgs |
| `/users/*` | ✅ | Authenticated | Self or admin |

### 5.2 Input Sanitization

All write endpoints sanitize inputs to prevent injection attacks:
- POSIX key format validation for environment variables: `^[A-Z_][A-Z0-9_]*$`
- UUID validation on all ID path parameters
- URL format validation for repository URLs
- Runtime value enum validation (projects: `Node.js`, `Rust`, `Python`, `Go`, `Static Site`)

### 5.3 Internal-Only Endpoints

The following endpoints are **not** accessible via the public API surface; they require internal service credentials:

- `PATCH /deployments/:id/status` — Build Worker status updates
- `POST /deployments/:id/logs` — Build Worker log writes
- `GET /projects/:id/env-vars/decrypt` — Deployment runner secret injection

---

## 6. Security Boundaries

```mermaid
graph TB
    subgraph "Public Zone"
        EXT[External Client / Browser]
    end

    subgraph "API Zone (JWT-Protected)"
        API[REST API Server]
        AUTH[Auth Middleware]
        RBAC[RBAC Guard]
    end

    subgraph "Internal Zone (Service Token)"
        WORKER[Build Worker]
        DEPLOY_INTERNAL[Deployment Status API - Internal]
        ENV_DECRYPT[Env Vars Decrypt - Internal]
    end

    subgraph "Infrastructure Zone"
        DB[(PostgreSQL Database)]
        QUEUE[(Job Queue)]
        DOCKER[(Docker Runtime)]
        GIT[(Git Service)]
        PUBSUB[(Pub/Sub Broker)]
    end

    EXT -->|HTTPS + JWT| API
    API --> AUTH
    AUTH --> RBAC
    RBAC --> DB

    WORKER -->|Service Token| DEPLOY_INTERNAL
    WORKER -->|Service Token| ENV_DECRYPT
    WORKER --> DOCKER
    WORKER --> GIT
    WORKER --> PUBSUB
    WORKER --> DB

    API --> QUEUE
    QUEUE --> WORKER
```

---

## 7. Deployment Security

| Security Control | Requirement |
|-----------------|-------------|
| Build worker sandbox | Workers must run in isolated, sandboxed environments per build job |
| Env var decryption | Decryption only in worker's secure runtime context; never logged in plaintext |
| Workspace cleanup | Temporary build workspaces are deleted after every job (success or failure) |
| Non-root containers | Docker images should use non-root base users where possible |
| Log redaction | Log files must not contain plaintext secret env var values |
| Audit trail | All deployments carry `triggered_by` user ID; rollback/redeploy actions are logged |

---

## 8. Secret Redaction in Logs

The Build Worker (BR-004) enforces:
- Environment variables are decrypted only within the worker's secure runtime context.
- Decrypted values are passed to `docker run` via in-memory environment injection, **not** via command-line arguments (prevents exposure in process listings).
- Log storage (Grafana Loki / build logs per [ADR-005](../09-adr/ADR-005-use-loki-for-centralized-logging.md)) must never contain plaintext secret values.

---

**Document Version:** 1.0  
**Last Updated:** 2026-08-12  
**Author:** Backend Architecture Team
