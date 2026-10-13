# Design: Containerized Application Environments and Unified Docker Compose

## Context

Forge is a high-performance backend application built with Rust (Axum, SeaORM, Tokio) and integrated with PostgreSQL, Redis, RabbitMQ, and Grafana Loki. Testing encompasses Cargo unit/integration tests, Newman API collection matrix tests (Node.js), and multi-tier k6 load tests (up to 100k virtual users).

Currently, backing services run in Docker Compose (`docker-compose.yaml`), but the application itself runs directly on host environments. On macOS and certain Linux configurations, default file descriptor limits (`ulimit -n = 256` or `1024`) cause socket and file exhaustion (`EMFILE: too many open files`) during parallel integration tests and k6 load tests. Furthermore, disparity between local developer environments, CI environments, and production targets creates operational friction.

## Goals / Non-Goals

**Goals:**
- Provide specialized Dockerfiles for Development (`docker/Dockerfile.dev`), Production (`docker/Dockerfile.prod` or root `Dockerfile`), and Testing (`docker/Dockerfile.test`).
- Provide automated kernel/container-level `ulimits` (`nofile: 65535`) in testing containers to eliminate manual host-level tuning on developer workstations.
- Integrate application containers into a unified Docker Compose setup with service health checks (`depends_on: condition: service_healthy`) and Docker Compose Profiles (`dev`, `test`, `prod`).
- Optimize build speed through efficient layer caching (using `cargo-chef` for production release builds and volume-mounted cache for development).
- Minimize production attack surface with non-root runtime user and minimal Debian slim base.
- Add standard `justfile` commands for containerized execution.

**Non-Goals:**
- Kubernetes manifests, Helm charts, or cloud-specific deployment pipelines.
- Replacing host-based fast iteration for developers who prefer running native `cargo run` / `cargo test` directly.

## Decisions

### 1. Base Operating System: Debian Bookworm Slim vs. Alpine Linux
- **Choice**: `rust:1.85-slim-bookworm` (build) and `debian:bookworm-slim` (runtime).
- **Rationale**: Forge utilizes `git2` with `vendored-openssl`, SeaORM with `sqlx-postgres`, and various C-linked libraries. Compiling on Alpine with `musl` requires complex toolchain overrides, increases compile time, and can exhibit memory allocator performance penalties under extreme concurrency compared to `glibc` with jemalloc/ptmalloc. Debian Slim provides compatibility, stability, and small footprint (~80-100MB runtime).
- **Alternative Considered**: Alpine / musl. Rejected due to build friction with `git2` and potential allocator latency degradation under high-load benchmarks.

### 2. Production Build Optimization: `cargo-chef` Multi-Stage Pipeline
- **Choice**: Use `cargo-chef` in `docker/Dockerfile.prod` with 3 stages:
  1. `planner`: Prepares `recipe.json` analyzing workspace dependencies.
  2. `builder`: Caches and compiles external crate dependencies separately.
  3. `runtime`: Hardened `debian:bookworm-slim` containing only the stripped binary, CA certificates, and unprivileged user `appuser` (UID 10001).
- **Rationale**: Rebuilding production images when only application code changes takes seconds instead of re-compiling all 100+ dependencies.
- **Alternative Considered**: Plain `cargo build --release` without cargo-chef. Rejected due to poor caching (any line change causes complete crate rebuild).

### 3. Testing Container with Automated Limits
- **Choice**: Dedicated `docker/Dockerfile.test` equipped with Rust toolchain, Node.js, Newman, and test execution scripts. Configured with:
  ```yaml
  ulimits:
    nofile:
      soft: 65535
      hard: 65535
  ```
- **Rationale**: Completely isolates tests from host OS limits. High-concurrency tests (k6 load tiers and multi-threaded Cargo test suites) will no longer hit `EMFILE` or require developers to alter macOS system configs (`launchd` or `ulimit`).
- **Alternative Considered**: Running tests on the host with shell wrapper scripts checking `ulimit`. Rejected because macOS system limits require root privileges or SIP overrides for large file descriptor increases.

### 4. Docker Compose Orchestration with Compose Profiles
- **Choice**: Maintain a single cohesive `docker-compose.yaml` utilizing Compose Profiles:
  - Default (no profile): Backing infrastructure (`postgres`, `redis`, `rabbitmq`, `loki`, `grafana`).
  - `profile: ["dev"]`: Service `forge-dev` (hot-reloading with `cargo-watch`, mounted code and named cargo caches).
  - `profile: ["prod"]`: Service `forge-api` (built with production Dockerfile, exposed on port 8080).
  - `profile: ["test"]`: Service `forge-test` (runs tests with elevated ulimits and mounted `./reports`).
- **Rationale**: Avoids maintaining multiple diverging compose files (`docker-compose.dev.yml`, `docker-compose.prod.yml`, etc.) while allowing developers to start exactly what they need (`docker compose --profile dev up` or `docker compose --profile test run forge-test`).
- **Alternative Considered**: Multiple compose files with `-f docker-compose.yml -f docker-compose.dev.yml`. Rejected because Compose Profiles are native, standard, and less prone to flag omission.

### 5. Hot Reloading in Development via Named Cache Volumes
- **Choice**: In `forge-dev`, mount local `./src` into `/app/src`, but use named volumes for `/app/target` and `/usr/local/cargo/registry`.
- **Rationale**: Sharing the macOS host `target/` directory with a Linux container causes platform binary collisions. Named volumes keep Linux build artifacts inside the Docker VM while retaining persistent caching across container restarts.

## Risks / Trade-offs

- **[Risk] Initial Docker build time for Rust dependencies**: First-time compilation of 100+ crates inside Docker takes a few minutes.
  - *Mitigation*: Use `cargo-chef` in production and persistent Docker volumes (`cargo_cache`, `cargo_target`) in development so external dependencies are compiled once.
- **[Risk] Docker Desktop resource constraints on macOS**: Running PostgreSQL, Redis, RabbitMQ, Loki, Grafana, and Rust compiler simultaneously requires adequate memory in Docker Desktop (recommended 4-6GB).
  - *Mitigation*: Document recommended Docker Desktop resource settings in README / Justfile comments.
- **[Risk] Database migration timing**: Application containers starting before PostgreSQL is accepting connections.
  - *Mitigation*: Use `depends_on: postgres: condition: service_healthy` so the application only boots once Postgres passes `pg_isready`.

## Migration Plan

1. Create `.dockerignore` to filter out build artifacts, logs, node_modules, and git history from Docker build contexts.
2. Create `docker/Dockerfile.dev`, `docker/Dockerfile.prod`, and `docker/Dockerfile.test`.
3. Update `docker-compose.yaml` to define `forge-dev`, `forge-api`, and `forge-test` with health checks, networks, and ulimits.
4. Add `justfile` helper recipes for running container profiles.
5. Validate container builds and verify test execution in container without host limit tweaking.
