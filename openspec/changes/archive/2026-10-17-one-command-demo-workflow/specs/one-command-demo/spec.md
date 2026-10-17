# Spec Delta

## Purpose

Provides a turnkey, single-command demonstration workflow (`just demo`) that bootstraps infrastructure, applies database migrations, seeds foundational platform accounts, verifies API health readiness, dispatches a sample project deployment, and launches the interactive Swagger UI for seamless technical evaluation.

## ADDED Requirements

### Requirement: Single-Command Demonstration Recipe
The system SHALL provide a standardized `just demo` recipe in the root `justfile` that automates end-to-end platform orchestration, database preparation, deployment triggering, and documentation launching without requiring manual developer intervention.

#### Scenario: User runs demo recipe
- **WHEN** user executes `just demo` from the repository root
- **THEN** the system executes the demonstration sequence, validates each milestone, and prints progress indicators to stdout

### Requirement: Automated Backing Infrastructure Lifecycle
The demonstration workflow SHALL verify and start the required containerized backing services (`postgres`, `redis`, `rabbitmq`, `loki`) using Docker Compose, waiting until all critical service healthchecks report healthy before proceeding to database operations.

#### Scenario: Infrastructure containers are stopped
- **WHEN** the demonstration workflow starts while backing containers are offline
- **THEN** the workflow starts the containers in the background, polls their readiness, and proceeds once PostgreSQL and RabbitMQ are responsive

#### Scenario: Infrastructure containers are already running
- **WHEN** the demonstration workflow starts while backing containers are already healthy
- **THEN** the workflow detects healthy services immediately without tearing down or disrupting existing container state

### Requirement: Automated Schema Migration and Data Seeding
The demonstration workflow SHALL execute pending database schema migrations using SeaORM migration commands and populate initial baseline access control roles, permissions, and test user accounts using the project's seed runner.

#### Scenario: Database schema and seed population
- **WHEN** the database container becomes healthy during the demonstration workflow
- **THEN** SeaORM migrations are applied up to date and the seed runner creates baseline accounts including `admin1@forge.local` without exiting with an error

### Requirement: API Health Readiness Polling
The demonstration workflow SHALL verify that the Forge API server is reachable and reports healthy status by probing `/health/live` and `/health/ready` endpoints over HTTP within a timeout limit (default 30 seconds).

#### Scenario: API server boots and responds
- **WHEN** the API server starts up and responds with HTTP 200 on `/health/ready`
- **THEN** the workflow acknowledges API readiness and proceeds to the deployment demonstration step

#### Scenario: API server fails to become healthy within timeout
- **WHEN** the API server fails to respond with HTTP 200 within the timeout limit
- **THEN** the workflow aborts with an informative diagnostic error message indicating service startup failure

### Requirement: Automated Demonstration Project and Deployment Dispatch
The demonstration workflow SHALL authenticate against `POST /api/v1/auth/login` using seeded administrator credentials (`admin1@forge.local` / `Password123!`), acquire a valid JWT bearer token, ensure a demonstration project exists via `POST /api/v1/projects`, trigger an asynchronous deployment via `POST /api/v1/projects/{id}/deployments`, and display the deployment ID and queued status in stdout.

#### Scenario: Successful demonstration deployment trigger
- **WHEN** the API server is healthy and authenticated credentials are provided
- **THEN** the demonstration workflow receives an HTTP 200 login token, creates or identifies the demo project, dispatches a deployment job with HTTP 201/200, and displays deployment metadata in the console

### Requirement: Interactive Swagger UI Launch
Upon successful execution of the demonstration deployment dispatch, the workflow SHALL open the interactive Swagger UI (`http://localhost:9000/docs`) in the user's default web browser using the host operating system's default browser opener (`open` on macOS or `xdg-open` on Linux).

#### Scenario: Browser launch on demonstration completion
- **WHEN** the deployment dispatch milestone succeeds
- **THEN** the workflow invokes the platform browser utility to open `http://localhost:9000/docs` and outputs the direct documentation URL to the console

### Requirement: Clean Execution Under 60 Seconds
The demonstration workflow SHALL complete all bootstrapping, database migration, seeding, health polling, deployment dispatch, and documentation launching steps within 60 seconds on a system with cached container images and compiled target binaries.

#### Scenario: Demonstration performance evaluation
- **WHEN** the workflow is executed on a clean git workspace with dependencies present
- **THEN** the entire sequence from command invocation to browser launch completes in under 60 seconds
