# Spec Delta

## Purpose

Provides environment variable management for projects, supporting single creation, bulk upserting, patch updates, environment-scoped filtering, and secure deletion across development, staging, and production environments.

## ADDED Requirements

### Requirement: Create Single Project Environment Variable
The system SHALL provide an endpoint `POST /projects/{id}/env-vars` to create a project environment variable. The request payload MUST specify `key`, `value`, and `environment` (which MUST be one of `development`, `staging`, or `production`). If `is_secret` is omitted, it SHALL default to `false`. If a variable with the same key already exists within the specified environment for the project, the system SHALL return a 409 Conflict.

#### Scenario: Create environment variable successfully
- **WHEN** user posts a valid payload with `key`, `value`, and `environment` (`development`)
- **THEN** system saves the variable and returns 201 Created with the created environment variable details

#### Scenario: Conflict on existing key in same environment
- **WHEN** user attempts to create a variable with a key and environment that already exists for the project
- **THEN** system returns 409 Conflict with an error message indicating the variable already exists

### Requirement: Bulk Upsert Project Environment Variables
The system SHALL provide an endpoint `POST /projects/{id}/env-vars/bulk` accepting a JSON payload with a `vars` array. Each item in `vars` MUST include `key`, `value`, and `environment` (`development`, `staging`, or `production`), and optional `is_secret`. If a variable with the specified key and environment already exists, the system SHALL update its value and metadata (upsert behavior).

#### Scenario: Bulk create and update environment variables
- **WHEN** user posts `POST /projects/{id}/env-vars/bulk` with multiple items across different environments, including existing and new keys
- **THEN** system updates existing records, creates new records within a single transaction, and returns 201 Created with the list of environment variables

### Requirement: Update Project Environment Variable
The system SHALL provide an endpoint `PATCH /projects/{id}/env-vars/{env_id}` allowing partial updates to an existing environment variable, including its `value` or `is_secret` flag.

#### Scenario: Patch environment variable value
- **WHEN** user sends `PATCH /projects/{id}/env-vars/{env_id}` with an updated `value`
- **THEN** system updates the variable and returns 200 OK with the updated record

### Requirement: Delete Project Environment Variable
The system SHALL provide an endpoint `DELETE /projects/{id}/env-vars/{env_id}` to permanently delete an environment variable.

#### Scenario: Delete variable successfully
- **WHEN** user sends `DELETE /projects/{id}/env-vars/{env_id}`
- **THEN** system removes the record and returns 200 OK

### Requirement: List Project Environment Variables
The system SHALL provide an endpoint `GET /projects/{id}/env-vars` to retrieve environment variables for a project. The endpoint SHALL support an optional `environment` query parameter (`development`, `staging`, `production`) to filter variables.

#### Scenario: List variables with environment filter
- **WHEN** user sends `GET /projects/{id}/env-vars?environment=production`
- **THEN** system returns 200 OK with only variables belonging to the `production` environment
