# Spec: Deployment Logs

## Purpose

Provides real-time log event streaming and stage tracking across project deployment builds, allowing clients to monitor build execution and container initialization over Server-Sent Events.

## Requirements

### Requirement: Real-Time Build Log Streaming
The system SHALL stream structured build log entries for all supported project types (Docker, Node.js, Go, Python, Rust, and Static Files) across all pipeline phases, including validation, package dependency fetching, configuration generation, image building, deployment, container start, and health checking. Each emitted log entry SHALL contain `step`, `timestamp`, `level`, `message`, `deployment_id`, and `is_end`.

#### Scenario: Builder emits phase progression logs
- **WHEN** a build worker executes a builder lifecycle phase (such as validation, file generation, building, or deployment)
- **THEN** the builder streams structured log items with step names and status messages via the configured LogStream transporters

#### Scenario: Real-time image build output streaming
- **WHEN** Docker builds an image for any supported project builder
- **THEN** stdout and stderr chunks from the Docker build engine are forwarded in real time through LogStream as info or log lines rather than only written to local console stdout

### Requirement: Error Logging on Build Failure
When any build worker stage or builder lifecycle execution encounters an error, the system SHALL stream an error log item with level ERROR containing the failure explanation before terminating the pipeline execution.

#### Scenario: Build worker step failure emits error log
- **WHEN** a build validation, build compilation, or deployment operation returns an error
- **THEN** the system logs an ERROR level event to the deployment log stream with details of the failure

### Requirement: Finalizing Log Stream on Completion
Upon completing or failing the deployment pipeline execution, the system SHALL emit a terminal log item with `is_end: true` so that downstream Server-Sent Event consumers can dispatch a final termination event to connected clients and close the stream.

#### Scenario: Pipeline completes successfully
- **WHEN** the build worker finishes all stages successfully
- **THEN** a final log entry with `is_end: true` is emitted, prompting downstream SSE subscribers to receive a completion notification
