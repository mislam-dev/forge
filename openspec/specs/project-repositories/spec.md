# Spec: Project Repositories

## Purpose

Provides Git repository management for projects, allowing users to connect, inspect, update, and disconnect repositories with automated PAT authentication and branch configuration.

## Requirements

### Requirement: Connect Project Git Repository
The system SHALL provide an endpoint `POST /projects/{id}/repository` to connect a Git repository to a project. If a repository is already connected to the project, the system SHALL update the existing repository connection configuration instead of returning a conflict error. The system SHALL automatically infer the repository authentication type (`pat` when an access token is provided, `none` otherwise) without requiring an explicit `auth_type` parameter. When `default_branch` is omitted in the request, the system SHALL default to `main`.

#### Scenario: Successfully connect a new repository
- **WHEN** user sends `POST /projects/{id}/repository` with `repository_url` and optional `access_token`
- **THEN** system saves the repository connection with status `connected`, masks the token in the response, and returns status 200

#### Scenario: Reconnect or overwrite an existing repository
- **WHEN** user sends `POST /projects/{id}/repository` for a project that already has a connected repository
- **THEN** system updates the repository details and returns the updated repository response without a 409 Conflict

### Requirement: Retrieve Connected Project Repository
The system SHALL provide an endpoint `GET /projects/{id}/repository` to retrieve the active repository configuration. The response SHALL include `id`, `project_id`, `repository_url`, `default_branch`, `status` (`connected` or `disconnected`), and masked `access_token` (`••••••••`).

#### Scenario: Get repository details
- **WHEN** authenticated user requests `GET /projects/{id}/repository`
- **THEN** system returns 200 OK with repository details and masked access token

### Requirement: Update Project Repository Configuration
The system SHALL provide an endpoint `PATCH /projects/{id}/repository` to update existing repository properties including `repository_url`, `access_token`, and `default_branch`. If an access token is updated, the system SHALL encrypt the token and set the authentication type to `pat`.

#### Scenario: Update default branch and access token
- **WHEN** user sends `PATCH /projects/{id}/repository` with a new `default_branch` or `access_token`
- **THEN** system updates the record, persists changes, and returns 200 OK with masked token

### Requirement: Disconnect Project Repository
The system SHALL provide an endpoint `DELETE /projects/{id}/repository` to disconnect the project repository. The system SHALL mark the repository status as `disconnected` and clear the stored access token.

#### Scenario: Disconnect repository
- **WHEN** user sends `DELETE /projects/{id}/repository`
- **THEN** system sets status to `disconnected`, clears token, and returns 200 OK
