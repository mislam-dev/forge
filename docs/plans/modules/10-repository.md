# Module 10 — Repository

> **Module Type:** Sub-Module (Projects)
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Repository Module](../../modules/projects/repository-module.md)

---

## 1. Module Overview

### Purpose

The Repository sub-module manages the **Git repository connection** for a project. It stores the repository URL, authentication credentials (Personal Access Token, encrypted with AES-256-GCM), and provides branch listing and commit fetching from GitHub.

### Responsibilities

- Connect a GitHub repository to a project
- Store PAT token encrypted (AES-256-GCM)
- Update repository connection
- Disconnect repository
- List repository branches via GitHub API
- Fetch recent commits for a branch
- Switch the default deployment branch

### Scope

**Included:**
- `POST /projects/:project_id/repository` — connect repository
- `GET /projects/:project_id/repository` — get repository config (PAT masked)
- `PUT /projects/:project_id/repository` — update repository config
- `DELETE /projects/:project_id/repository` — disconnect repository
- `GET /projects/:project_id/repository/branches` — list branches
- `GET /projects/:project_id/repository/branches/:branch/commits` — list commits

**Excluded:**
- Build execution (Build Worker)
- CI/CD webhook handling (future)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/repositories/mod.rs` | Exists — empty stub |
| `src/infrastructure/github/mod.rs` | Exists — empty stub |
| Handlers | Not implemented |
| Service | Not implemented |
| Tests | None |

---

## 3. Dependencies

### Depends On
- **Projects** (repository belongs to a project)
- **Encryption** (PAT token encryption/decryption)
- **Authentication**
- GitHub API (external — `octocrab` or raw HTTP client)

### Used By
- **Build Worker** (reads repository URL + PAT to clone)
- **Deployments** (validates branch/commit before triggering)

---

## 4. Database Table

### `project_repositories`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| project_id | UUID | Unique, FK -> projects.id CASCADE, Not Null |
| repository_url | VARCHAR(500) | Not Null |
| auth_type | VARCHAR | CHECK(none, pat), Not Null |
| access_token_encrypted | TEXT | Nullable (required if auth_type=pat) |
| default_branch | VARCHAR(255) | Not Null (default: main) |
| status | VARCHAR | CHECK(connected, disconnected), Default connected |
| created_at | TIMESTAMP | Not Null |
| updated_at | TIMESTAMP | Not Null |

**Constraint:** One repository per project (`project_id` is UNIQUE).

---

## 5. API Implementation

### POST /projects/:project_id/repository

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** `{ repository_url, auth_type, access_token?, default_branch? }`
- **Service logic:**
  1. Verify project exists and user has write access
  2. Verify no repository already connected (`409 Conflict` if exists)
  3. If `auth_type=pat`: encrypt PAT using `EncryptionService::encrypt(pat, project_id)`
  4. Test GitHub API connectivity with provided credentials
  5. Insert `project_repositories` record
- **Response:** `201 { message, data: repository (PAT masked as "••••••••") }`

### GET /projects/:project_id/repository

- **Auth:** JWT + project member
- **Service logic:** Load repository config; **never return decrypted PAT**
- **Response:** `200 { message, data: { id, repository_url, auth_type, default_branch, status, access_token: "••••••••" } }`

### PUT /projects/:project_id/repository

- **Auth:** JWT + project owner OR org Admin/Owner
- **Request:** Same as POST (all optional)
- **Service logic:** If new PAT provided, encrypt and store. Test connectivity.
- **Response:** `200 { message, data: updated_repository }`

### DELETE /projects/:project_id/repository

- **Auth:** JWT + project owner OR org Admin/Owner
- **Service logic:** Set status to disconnected, clear access_token_encrypted
- **Response:** `200 { message: "Repository disconnected." }`

### GET /projects/:project_id/repository/branches

- **Auth:** JWT + project member
- **Service logic:**
  1. Load repository config (decrypt PAT internally)
  2. Call GitHub API: `GET /repos/{owner}/{repo}/branches`
  3. Return branch list
- **Response:** `200 { message, data: [{ name, sha }] }`

### GET /projects/:project_id/repository/branches/:branch/commits

- **Auth:** JWT + project member
- **Service logic:** Call GitHub API: `GET /repos/{owner}/{repo}/commits?sha={branch}&per_page=20`
- **Response:** `200 { message, data: [{ sha, message, author, date }] }`

---

## 6. Security Rules

- PAT tokens must NEVER be returned in plaintext via any API response
- PAT tokens must NEVER appear in logs
- PAT tokens must be encrypted at rest using `EncryptionService`
- In API responses, show `"access_token": "••••••••"` (8 bullet characters, regardless of actual length)
- Decryption only occurs internally in the service layer, never passed to handlers

---

## 7. GitHub API Client

The `src/infrastructure/github/mod.rs` stub should implement:

```rust
pub struct GitHubClient {
    http: reqwest::Client,
}

impl GitHubClient {
    pub async fn list_branches(&self, owner: &str, repo: &str, pat: &str) -> Result<Vec<Branch>, GitHubError>;
    pub async fn list_commits(&self, owner: &str, repo: &str, branch: &str, pat: &str) -> Result<Vec<Commit>, GitHubError>;
    pub async fn get_commit(&self, owner: &str, repo: &str, commit_sha: &str, pat: &str) -> Result<Commit, GitHubError>;
}
```

Parse owner/repo from repository_url (supports HTTPS GitHub URL format).

---

## 8. Testing

### Integration Tests
- [ ] `POST /repository` — valid GitHub repo with PAT: connected successfully
- [ ] `POST /repository` — already connected: 409 returned
- [ ] `GET /repository` — PAT never returned in plaintext
- [ ] `GET /repository/branches` — list returned
- [ ] `PUT /repository` — new PAT encrypted on update
- [ ] `DELETE /repository` — disconnected, PAT cleared

### Unit Tests
- [ ] PAT encrypt/decrypt round trip (via EncryptionService)
- [ ] GitHub URL parsing: extract owner/repo correctly
- [ ] Response DTO: PAT field masked

---

## 9. Implementation Tasks

### Infrastructure
- [ ] Add `reqwest` to Cargo.toml
- [ ] Implement `GitHubClient` in `src/infrastructure/github/mod.rs`
- [ ] Implement GitHub URL parser (extract owner/repo from HTTPS URL)

### Database
- [ ] Create `project_repositories` migration
- [ ] Generate SeaORM entity for `project_repositories`

### Service
- [ ] Implement `RepositoryService` in `src/modules/repositories/service.rs`
- [ ] PAT encryption on write
- [ ] PAT masking on read (never decrypt for API responses)
- [ ] PAT decryption for internal service use (branch/commit fetching)
- [ ] GitHub connectivity test on connect/update

### Handlers
- [ ] Implement handlers for all 6 repository endpoints
- [ ] Register routes in router

### Testing
- [ ] Write unit and integration tests

---

## 10. Definition of Done

- [ ] All 6 repository endpoints functional
- [ ] PAT never returned in plaintext from any endpoint
- [ ] PAT encrypted with AES-256-GCM before storage
- [ ] GitHub branches and commits retrieved successfully
- [ ] All tests pass

---

## 11. Estimated Effort

**Medium-Large (2–3 days)**

The GitHub API client, PAT encryption, and careful security testing make this module more complex than simple CRUD.

---

## 12. Recommendations

**Required:**
- PAT masking must be enforced in the response DTO struct, not just in the handler logic.
- GitHub API rate limiting: handle `403` / `429` responses gracefully.

**Recommended:**
- Cache branch list in Redis with short TTL (60s) to reduce GitHub API calls.
- Support SSH key auth type in addition to PAT (documented — may be future enhancement).

**Future Enhancement:**
- GitHub webhook integration for automatic deployment on push.
- Support for GitLab and Bitbucket repositories.
