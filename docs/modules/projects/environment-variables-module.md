# Introduction

> **Module Type:** Sub-Module (Projects)
> **Version:** 1.0
> **Status:** Draft
> **Priority:** Critical
> **Owner:** Backend Team

---

# 1. Module Overview

## Purpose

The Environment Variables sub-module manages environment variables for projects within the [Projects Module](./projects-module.md). It allows configuring key-value pairs scoped across target environments (`Development`, `Preview`, `Production`), with support for secret encryption at rest, value masking, and full variable lifecycle operations (Create, Update, Delete, Encrypt Secrets).

## Scope

### Included

- Managing `key`/`value` environment variables per project
- Scoping variables by target environment: `Development`, `Preview`, `Production`
- Automatic AES-256-GCM encryption of secret values at rest
- Value masking (`••••••••`) in public list responses to protect sensitive credentials
- Operations: **Create**, **Update**, **Delete**, **Encrypt Secrets**

### Excluded

- Runtime environment variable injection into container / server instances (handled in Deployment pipeline)
- User authentication and authorization guard rules (handled in Auth & Project Permissions modules)
- Project lifecycle management (handled in [Projects Module](./projects-module.md))

---

# 2. Actors & Responsibilities

| Actor / Entity    | Access & Responsibilities                                                                         |
| ----------------- | ------------------------------------------------------------------------------------------------- |
| Project Owner     | Full authority to create, update, decrypt, and delete environment variables for all environments. |
| Org Admin / Dev   | Create, edit, and delete environment variables according to project permission roles.             |
| Project Viewer    | View masked list of environment variable keys (read-only, secret values remain hidden).           |
| Deployment Engine | Internal service allowed to decrypt and inject environment variables during build & deployment.   |
| System Admin      | Global access to project environment configurations.                                              |

---

# 3. Business Goals

- Provide project-level environment variable configuration with explicit target environment isolation (`Development`, `Preview`, `Production`).
- Guarantee high security for secret environment variables via AES-256-GCM encryption at rest.
- Provide clean API interfaces for managing variable creation, updates, deletions, and decrypted injection into deployment runners.

---

# 4. Functional Requirements

## FR-001 Create Environment Variable

### Description

Creates a new environment variable for a project scoped to a specific environment (`Development`, `Preview`, `Production`).

### Inputs

| Field       | Required | Descriptions                                                    |
| ----------- | -------- | --------------------------------------------------------------- |
| project_id  | Yes      | UUID of the target project                                      |
| key         | Yes      | Variable key name (e.g. `DATABASE_URL`, `API_KEY`)              |
| value       | Yes      | Plaintext value of the environment variable                     |
| environment | Yes      | Target environment (`Development`, `Preview`, `Production`)     |
| is_secret   | No       | Boolean flag indicating if value is secret (defaults to `true`) |

### Process

1. Validate payload format:
   - Key format must conform to POSIX uppercase variable naming (`^[A-Z_][A-Z0-9_]*$`).
   - Environment must be one of `Development`, `Preview`, `Production`.
2. Check for key duplication within the same `project_id` and `environment`.
3. If `is_secret == true`, encrypt `value` using AES-256-GCM with project master key.
4. Save record to `project_environment_variables`.

### Success Response

- Environment variable created successfully.

### Failure Cases

- Invalid key name format (`ENV_001`).
- Duplicate key in target environment (`ENV_002`).
- Invalid environment specified (`ENV_003`).

---

## FR-002 Get / List Environment Variables

### Description

Retrieves the list of environment variables for a project, optionally filtered by environment. Secret values are masked by default.

### Inputs

| Field       | Required | Descriptions                                                   |
| ----------- | -------- | -------------------------------------------------------------- |
| project_id  | Yes      | UUID of the target project                                     |
| environment | No       | Filter by environment (`Development`, `Preview`, `Production`) |

### Process

1. Query `project_environment_variables` matching `project_id` and optional `environment`.
2. If `is_secret == true`, mask the value as `••••••••` in API response.
3. Return list of environment variable descriptors.

### Success Response

- Environment variables list retrieved.

### Failure Cases

- Project not found.

---

## FR-003 Update Environment Variable

### Description

Updates an existing environment variable's key, value, environment, or secret status.

### Inputs

| Field       | Required | Descriptions                                                 |
| ----------- | -------- | ------------------------------------------------------------ |
| id          | Yes      | UUID of the target environment variable record               |
| key         | No       | Updated key name                                             |
| value       | No       | Updated plaintext value                                      |
| environment | No       | Updated environment (`Development`, `Preview`, `Production`) |
| is_secret   | No       | Updated secret flag                                          |

### Process

1. Validate variable existence.
2. If key or environment changed, verify uniqueness within target scope (`ENV_002`).
3. If value updated and `is_secret == true`, encrypt new value with AES-256-GCM.
4. Update record in `project_environment_variables`.

### Success Response

- Environment variable updated.

### Failure Cases

- Variable not found (`ENV_004`).
- Duplicate key error (`ENV_002`).

---

## FR-004 Delete Environment Variable

### Description

Deletes an environment variable from a project.

### Inputs

| Field | Required | Descriptions                               |
| ----- | -------- | ------------------------------------------ |
| id    | Yes      | UUID of the environment variable to delete |

### Process

1. Validate record existence.
2. Delete record from `project_environment_variables`.

### Success Response

- Environment variable deleted successfully.

### Failure Cases

- Variable not found (`ENV_004`).

---

## FR-005 Encrypt Secret Values

### Description

Utility process that encrypts raw string values using AES-256-GCM before writing to storage.

### Inputs

| Field      | Required | Descriptions                    |
| ---------- | -------- | ------------------------------- |
| plaintext  | Yes      | Raw string value                |
| project_id | Yes      | UUID for key derivation context |

### Process

1. Derive encryption key for project context using master secret key + project ID salt.
2. Generate random 12-byte Initialization Vector (IV).
3. Perform AES-256-GCM encryption.
4. Output Base64 string containing `IV + Ciphertext + AuthTag`.

### Success Response

- Encrypted ciphertext returned.

### Failure Cases

- Encryption failure (`ENV_005`).

---

# 5. Business Rules

| ID     | Rule                                                                                                                                      |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| BR-001 | Variable keys must contain uppercase alphanumeric characters and underscores (`^[A-Z_][A-Z0-9_]*$`).                                      |
| BR-002 | Target environments are strictly limited to: `Development`, `Preview`, `Production`.                                                      |
| BR-003 | Variable key MUST be unique per project and environment combination (`project_id`, `environment`, `key`).                                 |
| BR-004 | Secret values MUST be encrypted at rest using AES-256-GCM.                                                                                |
| BR-005 | Public list/get endpoints MUST mask secret values as `••••••••`. Plaintext secret retrieval is restricted to authorized internal runners. |

---

# 6. Validation Rules

## Environment Variable

| Field       | Validation                                                      |
| ----------- | --------------------------------------------------------------- |
| project_id  | Required, valid UUID                                            |
| key         | Required, must match regex `^[A-Z_][A-Z0-9_]*$`                 |
| value       | Required string                                                 |
| environment | Required, must be one of `Development`, `Preview`, `Production` |
| is_secret   | Optional boolean, default `true`                                |

---

# 7. Authorization Matrix

| Route                                 | Action          | Viewer | Developer | Admin | Owner | System Admin | Internal Runner |
| ------------------------------------- | --------------- | ------ | --------- | ----- | ----- | ------------ | --------------- |
| POST /projects/:id/env-vars           | Create Variable | No     | Yes       | Yes   | Yes   | Yes          | No              |
| GET /projects/:id/env-vars            | List Variables  | Yes    | Yes       | Yes   | Yes   | Yes          | Yes             |
| PUT /projects/:id/env-vars/:env_id    | Update Variable | No     | Yes       | Yes   | Yes   | Yes          | No              |
| DELETE /projects/:id/env-vars/:env_id | Delete Variable | No     | Yes       | Yes   | Yes   | Yes          | No              |
| GET /projects/:id/env-vars/decrypt    | Decrypt Value   | No     | No        | No    | Yes   | Yes          | Yes (Internal)  |

---

# 8. Workflow

## Create & Encrypt Environment Variable Workflow

```mermaid
flowchart TD
    A[User] --> B[Submit Variable Key & Value]
    B --> C[Validate Key Regex: ^[A-Z_][A-Z0-9_]*$]
    C --> D{Is Key Format Valid?}
    D -->|No| E[Return ENV_001: Invalid Key Format]
    D -->|Yes| F{Does Key Already Exist in Environment?}
    F -->|Yes| G[Return ENV_002: Duplicate Key]
    F -->|No| H{Is is_secret == true?}
    H -->|Yes| I[Encrypt Value with AES-256-GCM]
    H -->|No| J[Use Raw Value]
    I --> K[Save to project_environment_variables]
    J --> K
    K --> L[Return Success Response]
```

---

# 9. Sequence Diagram

---

# 10. Database Design

## Project Environment Variables Table (`project_environment_variables`)

| Field           | Type      | Constraints                                |
| --------------- | --------- | ------------------------------------------ |
| id              | UUID      | Primary                                    |
| project_id      | UUID      | Foreign Key to `projects`                  |
| key             | VARCHAR   | Variable key (e.g. `DATABASE_URL`)         |
| value_encrypted | TEXT      | Encrypted ciphertext payload (AES-256-GCM) |
| environment     | VARCHAR   | `Development`, `Preview`, `Production`     |
| is_secret       | BOOLEAN   | Default `true`                             |
| created_at      | TIMESTAMP |                                            |
| updated_at      | TIMESTAMP |                                            |

---

# 11. API Endpoints

| Method | Endpoint                       | Description                           |
| ------ | ------------------------------ | ------------------------------------- |
| POST   | /projects/:id/env-vars         | Create environment variable           |
| GET    | /projects/:id/env-vars         | List environment variables (masked)   |
| PUT    | /projects/:id/env-vars/:env_id | Update environment variable key/value |
| DELETE | /projects/:id/env-vars/:env_id | Delete environment variable           |

---

# 12. API Examples

## Create Environment Variable

```json
POST /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/env-vars
{
  "key": "DATABASE_URL",
  "value": "postgres://user:secretpass@db.example.com:5432/production",
  "environment": "Production",
  "is_secret": true
}
```

### Success Response

```json
{
  "message": "Environment variable created successfully.",
  "data": {
    "id": "env-12345678-8e8c-44c1-942c-3004f5a6c5b6",
    "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
    "key": "DATABASE_URL",
    "value": "••••••••",
    "environment": "Production",
    "is_secret": true,
    "created_at": "2026-08-08T00:00:00Z",
    "updated_at": "2026-08-08T00:00:00Z"
  }
}
```

## List Environment Variables

```json
GET /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/env-vars?environment=Production
```

### Success Response

```json
{
  "message": "Environment variables retrieved.",
  "data": [
    {
      "id": "env-12345678-8e8c-44c1-942c-3004f5a6c5b6",
      "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
      "key": "DATABASE_URL",
      "value": "••••••••",
      "environment": "Production",
      "is_secret": true,
      "created_at": "2026-08-08T00:00:00Z"
    },
    {
      "id": "env-87654321-8e8c-44c1-942c-3004f5a6c5b6",
      "project_id": "07c0060e-8e8c-44c1-942c-3004f5a6c5b6",
      "key": "PORT",
      "value": "8080",
      "environment": "Production",
      "is_secret": false,
      "created_at": "2026-08-08T00:00:00Z"
    }
  ]
}
```

## Update Environment Variable

```json
PUT /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/env-vars/env-12345678-8e8c-44c1-942c-3004f5a6c5b6
{
  "value": "postgres://user:newpassword@db.example.com:5432/production"
}
```

### Success Response

```json
{
  "message": "Environment variable updated successfully.",
  "data": {
    "id": "env-12345678-8e8c-44c1-942c-3004f5a6c5b6",
    "key": "DATABASE_URL",
    "value": "••••••••",
    "environment": "Production",
    "is_secret": true,
    "updated_at": "2026-08-08T00:00:00Z"
  }
}
```

## Delete Environment Variable

```json
DELETE /projects/07c0060e-8e8c-44c1-942c-3004f5a6c5b6/env-vars/env-12345678-8e8c-44c1-942c-3004f5a6c5b6
```

### Success Response

```json
{
  "message": "Environment variable deleted successfully."
}
```

---

# 13. Error Codes

| Code    | Description                                                     |
| ------- | --------------------------------------------------------------- |
| ENV_001 | Invalid Key Format (Must match POSIX ^[A-Z\_][A-Z0-9_]\*$)      |
| ENV_002 | Duplicate Variable Key for Environment                          |
| ENV_003 | Invalid Target Environment (Must be Development, Preview, Prod) |
| ENV_004 | Environment Variable Not Found                                  |
| ENV_005 | Encryption / Decryption Error                                   |

---

# 14. Security Requirements

- All secret environment values (`is_secret == true`) MUST be encrypted using AES-256-GCM prior to database persistence.
- Public responses MUST mask secret values as `••••••••`.
- Plaintext value decryption is strictly allowed only for authorized deployment injection runners.

---

# 15. Non-Functional Requirements

| Requirement                 | Target |
| --------------------------- | ------ |
| Variable Read Response Time | <50 ms |
| Decryption & Injection Time | <30 ms |

---

# 16. Acceptance Criteria

- Users can manage environment variables scoped to `Development`, `Preview`, `Production`.
- Key validation enforces standard POSIX naming formats.
- Secret values are encrypted at rest with AES-256-GCM and masked in API list responses.

---

# 17. Dependencies

- [Projects Module](./projects-module.md)
- Database
- Encryption Key Management Service

---

# 18. Assumptions

- Application deployment runner has access to project master encryption keys for variable injection.

---

# 19. Future Enhancements

- Import / export `.env` files.
- Variable inheritance across environments (e.g. fallback from Production to Preview).

---

# 20. Appendix

## Related Documents

- [Projects Module](./projects-module.md)
- [Repository Sub-Module](./repository-module.md)
- [Project Permissions Sub-Module](./project-permissions-module.md)
- System Architecture
- API Documentation
- Security Policy

---

**Document Version:** 1.0
**Last Updated:** 2026-08-12
**Author:** Monirul Islam
