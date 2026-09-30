# Spec: Project Deployments

## Purpose

Provides deployment lifecycle management for projects, allowing authorized users to trigger builds, view paginated history, inspect deployment details, re-trigger previous deployments, initiate rollbacks, and enable build workers to update status.

## Requirements

### Requirement: Trigger Project Deployment
The system SHALL provide an endpoint `POST /projects/{id}/deployments` to trigger a new deployment for the specified project. The request SHALL accept optional `branch` and `commit_hash` parameters in the JSON body. The project ID SHALL be provided via the path parameter `{id}`. The endpoint SHALL require JWT authentication. On success, the system SHALL return HTTP status 201 Created with the queued deployment details wrapped in an ApiResponse structure.

#### Scenario: Successfully trigger a deployment
- **WHEN** authenticated user sends `POST /projects/{id}/deployments` with optional `branch` or `commit_hash`
- **THEN** system queues the deployment, returns status 201 Created with message "Deployment triggered successfully.", and provides the created deployment object with status "Queued"

#### Scenario: Trigger without JWT token
- **WHEN** unauthenticated request is sent to `POST /projects/{id}/deployments`
- **THEN** system rejects the request with HTTP status 401 Unauthorized

### Requirement: List Project Deployments
The system SHALL provide an endpoint `GET /projects/{id}/deployments` to retrieve a paginated history of deployments for the specified project. The endpoint SHALL support query parameters `page`, `limit`, `status`, and `branch`. The endpoint SHALL require JWT authentication and viewer access to the project. The response SHALL include pagination metadata and the list of deployment items.

#### Scenario: Retrieve paginated deployment list
- **WHEN** authenticated user requests `GET /projects/{id}/deployments?page=1&limit=20`
- **THEN** system returns HTTP status 200 OK with paginated deployment records and metadata

#### Scenario: Filter deployments by branch and status
- **WHEN** authenticated user requests `GET /projects/{id}/deployments?branch=main&status=Success`
- **THEN** system returns HTTP status 200 OK with records matching the specified branch and status

### Requirement: Retrieve Single Deployment Details
The system SHALL provide an endpoint `GET /projects/{id}/deployments/{deployment_id}` to retrieve metadata for a single deployment specified by its `deployment_id` under project `{id}`. The endpoint SHALL require JWT authentication and viewer access. On success, the response SHALL return the deployment's status, commit details, timestamps, durations, and any error message.

#### Scenario: Get deployment details by ID
- **WHEN** authenticated user requests `GET /projects/{id}/deployments/{deployment_id}`
- **THEN** system returns HTTP status 200 OK with message "Deployment details retrieved successfully." and the deployment payload

#### Scenario: Deployment does not belong to project
- **WHEN** authenticated user requests `GET /projects/{id}/deployments/{deployment_id}` where deployment is not associated with project `{id}`
- **THEN** system returns HTTP status 404 Not Found

### Requirement: Redeploy Past Deployment
The system SHALL provide an endpoint `POST /projects/{id}/deployments/{deployment_id}/redeploy` to re-trigger a deployment using the identical commit and configuration of a past deployment. The endpoint SHALL require JWT authentication and editor access. The system SHALL create a new deployment entry and return HTTP status 201 Created.

#### Scenario: Successfully redeploy a past deployment
- **WHEN** authenticated user sends `POST /projects/{id}/deployments/{deployment_id}/redeploy`
- **THEN** system queues a new deployment using the past commit and returns HTTP status 201 Created with message "Redeploy triggered successfully."

### Requirement: Rollback Project Deployment
The system SHALL provide an endpoint `POST /projects/{id}/deployments/rollback` to rollback the project to its most recent successful deployment. The endpoint SHALL require JWT authentication and administrator access. On success, the system SHALL create a new rollback deployment and return HTTP status 201 Created.

#### Scenario: Successfully rollback to latest successful deployment
- **WHEN** authenticated admin sends `POST /projects/{id}/deployments/rollback`
- **THEN** system identifies the latest successful deployment, queues a rollback deployment, and returns HTTP status 201 Created with message "Rollback deployment triggered successfully."

### Requirement: Update Deployment Status Internally
The system SHALL provide an endpoint `PUT /projects/internal/deployments/{deployment_id}/status` allowing internal build workers to update deployment status and execution durations. The endpoint SHALL authenticate incoming requests via an `x-service-token` header validated against the master encryption key. The request body SHALL require `status` (valid enum: "Queued", "Building", "Deploying", "Running", "Failed", "Success") and allow optional `build_duration`, `deploy_duration`, and `error_message`.

#### Scenario: Build worker updates status with valid service token
- **WHEN** worker sends `PUT /projects/internal/deployments/{deployment_id}/status` with valid `x-service-token` and valid status payload
- **THEN** system transitions the deployment status, persists durations, and returns HTTP status 200 OK with message "Deployment status updated successfully."

#### Scenario: Reject status update without valid service token
- **WHEN** request is sent to `PUT /projects/internal/deployments/{deployment_id}/status` with missing or invalid `x-service-token`
- **THEN** system rejects the request with HTTP status 401 Unauthorized
