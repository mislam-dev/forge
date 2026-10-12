# Design: Parameterize Login Credentials Across Environments and Login Request

## Context

See [proposal.md](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/openspec/changes/parameterize-login-credentials/proposal.md) for background and motivation.

The Forge Platform repository maintains Postman environment configurations in three places:
1. Root Postman environment (`environment.json`).
2. Standalone Newman/CLI export JSON files (`postman/environments/*.environment.json`).
3. Postman v12 workspace YAML environment files (`postman/postman/environments/*.environment.yaml`).

Currently, these files define role-specific variables (`adminEmail`, `adminPassword`, `devEmail`, `devPassword`, `viewerEmail`, `viewerPassword`, `userEmail`, `userPassword`), but lack standard `email` and `password` variables corresponding directly to the login request body keys. Furthermore, `postman/postman/collections/Forge Platform API/auth/login/Authenticate user credentials.request.yaml` uses a static JSON payload with hardcoded values (`viewer1@forge.local` and `Password123!`), preventing seamless multi-role test execution.

## Goals / Non-Goals

**Goals:**
- Add `email` and `password` variables to all Postman environment configurations across YAML, JSON, and root formats.
- Populate each environment's `email` and `password` with the credentials corresponding to that environment's target role.
- Update the `POST /auth/login` request body payload in `Authenticate user credentials.request.yaml` to dynamically reference `{{email}}` and `{{password}}`.
- Preserve existing credential variables (`adminEmail`, `userEmail`, etc.) to guarantee backward compatibility with existing tests and scripts.

**Non-Goals:**
- Modify backend Rust authentication handlers or database seed data.
- Parameterize other authentication endpoint bodies (e.g. registration, forgot password) that require distinct input patterns.

## Decisions

### Decision 1: Standardize on `email` and `password` Environment Variable Keys
- **Choice**: Introduce top-level `email` and `password` variables matching the exact property names in the login request payload.
- **Rationale**: Postman template syntax `{{email}}` and `{{password}}` directly mirrors the JSON keys `{"email": "{{email}}", "password": "{{password}}"}`, which is clean, idiomatic in Postman, and requires no script transformation.
- **Alternative Considered**: Reusing `{{userEmail}}` and `{{userPassword}}` directly in the login body. Rejected because the explicit requirement is to add `email` and `password` variables to each environment, and `email`/`password` provides a standard contract for the active login identity.

### Decision 2: Credential Mapping Matrix Across Environments
Each environment will configure `email` and `password` according to its target role:
- **Admin - With Organization & Admin - Without Organization**:
  - `email`: `"admin1@forge.local"`
  - `password`: `"Password123!"`
- **Developer - With Organization & Developer - Without Organization**:
  - `email`: `"dev1@forge.local"`
  - `password`: `"Password123!"`
- **Viewer - With Organization & Viewer - Without Organization**:
  - `email`: `"viewer1@forge.local"`
  - `password`: `"Password123!"`
- **Development, Local Development, & root environment.json**:
  - `email`: `"admin1@forge.local"`
  - `password`: `"Password123!"`
- **Production**:
  - `email`: `""`
  - `password`: `""` (configured with `secret: true` in YAML and `"type": "secret"` in JSON)

### Decision 3: Update `Authenticate user credentials.request.yaml` Body
- **Choice**: Replace hardcoded email and password strings in `body.content` with `{{email}}` and `{{password}}`:
  ```json
  {
    "email": "{{email}}",
    "password": "{{password}}"
  }
  ```
- **Rationale**: Postman automatically interpolates `{{...}}` variable templates at runtime against the active environment.

## Risks / Trade-offs

- **[Risk] Missing variables in custom environments cause unauthenticated failures**: If a user runs the collection with a custom or third-party environment lacking `email` and `password`, the unresolved template strings `"{{email}}"` will be sent, resulting in 401 Unauthorized.
  - **Mitigation**: Update all standard environments in the repository (root `environment.json`, all 8 files in `postman/environments/`, and all 9 files in `postman/postman/environments/`), and document the required variables in the specification.
- **[Risk] Production credential exposure**:
  - **Mitigation**: Ensure `password` in Production environment files is explicitly flagged as secret (`secret: true` / `"type": "secret"`) and initialized with empty strings.
