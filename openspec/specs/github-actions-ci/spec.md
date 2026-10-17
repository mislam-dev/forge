# github-actions-ci Specification

## Purpose
Automates continuous integration quality gates for the Forge platform via GitHub Actions, validating code formatting, compiler lints, workspace tests against backing services, and container image builds strictly on pull requests and pushes targeting the main branch.

## Requirements

### Requirement: Pull Request and Main Branch Push CI Trigger
The CI workflow SHALL be configured to run exclusively on pull requests targeting the `main` branch and on direct pushes to the `main` branch. No additional triggers (e.g., cron schedules, manual workflow dispatches, or tag pushes) SHALL be configured.

#### Scenario: Pull request targeting main triggers CI
- **WHEN** a pull request targeting the `main` branch is opened, synchronized, or reopened
- **THEN** the GitHub Actions workflow triggers and executes the configured quality gate jobs

#### Scenario: Direct push to main triggers CI
- **WHEN** commits are pushed directly to the `main` branch
- **THEN** the GitHub Actions workflow triggers and executes the configured quality gate jobs

#### Scenario: Branch push without pull request does not trigger CI
- **WHEN** commits are pushed directly to a feature branch that has no open pull request targeting `main`
- **THEN** the GitHub Actions workflow does not trigger

### Requirement: Rust Formatting Quality Gate
The CI workflow SHALL execute a dedicated `rustfmt` job that verifies workspace code formatting using `cargo fmt --check`.

#### Scenario: Properly formatted code passes
- **WHEN** code adheres to `rustfmt` formatting standards across all workspace members
- **THEN** the `rustfmt` job succeeds with exit code 0

#### Scenario: Unformatted code fails
- **WHEN** any Rust source file contains formatting discrepancies
- **THEN** the `rustfmt` job fails and reports the formatting diff

### Requirement: Clippy Lint Quality Gate
The CI workflow SHALL execute a dedicated `clippy` job that runs `cargo clippy --all-targets --all-features -- -D warnings` to enforce zero warnings across all workspace members.

#### Scenario: Clean code passes clippy
- **WHEN** workspace code compiles with zero compiler or clippy warnings across all targets and features
- **THEN** the `clippy` job completes successfully with exit code 0

#### Scenario: Lint warnings fail clippy
- **WHEN** code introduces any compiler or clippy lint warning
- **THEN** the `clippy` job fails with non-zero exit code

### Requirement: Workspace Test Execution with Backing Services
The CI workflow SHALL execute a dedicated `test` job running `cargo test --workspace` in an environment provisioned with healthy PostgreSQL and RabbitMQ service containers.

#### Scenario: Workspace tests execute successfully against service containers
- **WHEN** the `test` job initializes PostgreSQL 16 and RabbitMQ 3 service containers with healthchecks and executes `cargo test --workspace`
- **THEN** all unit and integration tests run to completion and the `test` job passes

#### Scenario: Service container connectivity failure fails test job
- **WHEN** backing service containers fail health checks or connection credentials are misconfigured
- **THEN** tests requiring backing services fail and the `test` job reports failure

### Requirement: Multi-Stage Docker Build Validation
The CI workflow SHALL execute a dedicated `docker-build` job that builds the root `Dockerfile` using Docker Buildx and validates `cargo-chef` dependency caching and binary stripping.

#### Scenario: Container image builds successfully
- **WHEN** the `docker-build` job executes `docker build` targeting `Dockerfile`
- **THEN** the multi-stage build completes all stages (chef, planner, builder, runtime) successfully
