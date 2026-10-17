# Design: GitHub Actions CI Pipeline

## Context

Forge is an event-driven self-hosted developer platform written in Rust (Rust 2024 edition workspace). The platform requires PostgreSQL and RabbitMQ for integration testing, and utilizes a multi-stage Dockerfile with `cargo-chef` for release compilation. Currently, quality gates (`cargo fmt`, `cargo clippy`, `cargo test`) are executed manually via `justfile` recipes.

See `proposal.md` for motivation and `specs/github-actions-ci/spec.md` for formal requirement contracts.

## Goals / Non-Goals

**Goals:**
- Provide `.github/workflows/ci.yml` defining an automated quality gate pipeline on GitHub Actions.
- Restrict workflow triggers strictly to `pull_request` targeting `main` and direct `push` on `main`.
- Parallelize independent checks (`rustfmt`, `clippy`, `docker-build`, `test`) to minimize turnaround time.
- Attach PostgreSQL 16 and RabbitMQ 3 service containers with robust health checks to the `test` job.
- Leverage Rust build caching (`Swatinem/rust-cache`) to ensure fast incremental workflow execution.
- Validate release image packaging via Docker Buildx against the root `Dockerfile`.

**Non-Goals:**
- No automated deployments, container registry publishing, or release tagging in this workflow.
- No cron schedules, manual `workflow_dispatch`, or triggers on other branches.
- No modifications to application code or existing integration test implementations.

## Decisions

### 1. Workflow Triggers
- **Decision**: Define triggers strictly as:
  ```yaml
  on:
    pull_request:
      branches: [ main ]
    push:
      branches: [ main ]
  ```
- **Rationale**: Directly aligns with the user constraint ("write github workflow on pull request, directo push on the main branch. nothing else").
- **Alternatives considered**: Triggering on all branch pushes (rejected: creates noisy runs for work-in-progress topic branches before PR creation).

### 2. Rust Toolchain & Caching Strategy
- **Decision**: Use `dtolnay/rust-toolchain@stable` with `rustfmt` and `clippy` components, paired with `Swatinem/rust-cache@v2`.
- **Rationale**: Rust 2024 edition requires Rust >= 1.85.0. Stable GitHub runners track current stable Rust. `Swatinem/rust-cache` handles Cargo index, target directory, and dependency cache hashing automatically across jobs.
- **Alternatives considered**: `actions-rust-lang/setup-rust-toolchain` (also valid, but `dtolnay/rust-toolchain` + `Swatinem/rust-cache` is industry standard and offers fine-grained cache control).

### 3. Service Containers for Integration Tests
- **Decision**: Use GitHub Actions native `services` in the `test` job:
  - `postgres`: `image: postgres:16-alpine`, ports `5432:5432`, healthcheck `pg_isready -U postgres`.
  - `rabbitmq`: `image: rabbitmq:3-management-alpine`, ports `5672:5672`, healthcheck `rabbitmq-diagnostics -q ping`.
- **Rationale**: Service containers run natively alongside runner steps, eliminating the overhead of managing `docker compose` lifecycle scripts.
- **Alternatives considered**: Running `docker compose up -d` in a bash step (rejected: requires managing compose process state, log aggregation, and cleanup hooks).

### 4. Job Matrix & Separation of Concerns
- **Decision**: Define four distinct jobs:
  1. `rustfmt`: Runs `cargo fmt --check` (fast fail, <1m).
  2. `clippy`: Runs `cargo clippy --all-targets --all-features -- -D warnings`.
  3. `test`: Runs `cargo test --workspace` with service containers and env configuration matching `.env.example`.
  4. `docker-build`: Uses `docker/setup-buildx-action@v3` and runs `docker build -t forge:ci .`.
- **Rationale**: Decoupled jobs execute in parallel on GitHub runners, providing instant granular status reporting on PR checks.
- **Alternatives considered**: Single monolithic job running all steps sequentially (rejected: slow feedback loop; failure in formatting would block test execution).

## Risks / Trade-offs

- **[Risk] Service Container Readiness Delays**: RabbitMQ startup can take several seconds, potentially causing connection failures if tests run before ports are open.
  - **Mitigation**: Configure explicit service container healthchecks with `interval: 10s`, `timeout: 5s`, and `retries: 5`.
- **[Risk] Missing Test Environment Variables**: Integration tests may expect specific secrets or fallback values.
  - **Mitigation**: Populate environment variables on the `test` step mirroring `.env.example` (`DATABASE_URL`, `RABBITMQ_URL`, `JWT_SECRET`, `MASTER_ENCRYPTION_KEY`, `WORKER_SERVICE_TOKEN`).
- **[Risk] Cache Invalidation on Large Target Directories**: Cargo target directories can exceed GitHub cache quota (10GB).
  - **Mitigation**: `Swatinem/rust-cache` automatically purges outdated build artifacts and caches only shared dependency build artifacts.
