# Proposal: Containerized Application Environments (Dev, Test, Prod) & Unified Docker Compose

## Why

Currently, Forge relies on Docker Compose exclusively for backing infrastructure (PostgreSQL, Redis, RabbitMQ, Loki, Grafana), while the Rust application itself and its test suites run directly on the host machine. This causes environment discrepancies across developer workstations, requires manual adjustment of OS file descriptor limits (`ulimit -n`) on host systems (especially macOS) to avoid socket/file exhaustion during multi-threaded integration and load testing, and complicates end-to-end local deployment and CI execution.

Containerizing the application across tailored Development, Production, and Testing Docker environments—and integrating them seamlessly into Docker Compose with healthchecks, dependency orchestration, and elevated resource/descriptor limits—ensures reproducible, robust, zero-manual-tuning workflows for developers, CI pipelines, and production deployments.

## What Changes

- **Development Dockerfile (`docker/Dockerfile.dev`)**: Fast, developer-friendly container image with volume-mounted source code, cargo caching, `cargo-watch` hot-reloading, and debug logging configuration.
- **Production Dockerfile (`docker/Dockerfile` / `docker/Dockerfile.prod`)**: Secure, minimal multi-stage build image compiling a stripped release binary onto a hardened runtime base (`debian:bookworm-slim` or distroless) with a non-root user, minimal attack surface, and container health probe.
- **Testing Dockerfile (`docker/Dockerfile.test`)**: Comprehensive test runner image incorporating the Rust toolchain, Node.js runtime, Newman, and test dependencies. Configured specifically to bypass host OS limit constraints (setting `nofile` ulimits to 65535+ and tuning network/socket parameters) so integration, API matrix, and load tests execute reliably without manual host configuration.
- **Unified Docker Compose Architecture (`docker-compose.yaml` / Compose profiles)**:
  - Add containerized application services (`forge-api`, `forge-dev`, `forge-test`) wired into the existing `backend` network.
  - Implement robust dependency management (`depends_on` with `service_healthy` conditions for PostgreSQL, Redis, RabbitMQ, and Loki).
  - Explicitly define container-level `ulimits` (`nofile: 65535`) in Docker Compose to eliminate manual host-level limit configurations for testing and high-concurrency workloads.
  - Support Docker Compose profiles (e.g., `dev`, `test`, `prod`) to cleanly isolate developer iteration, test execution, and full stack deployment.
- **Developer Workflow Automation (`justfile`)**:
  - Add convenient `just` targets for building, running, and testing inside Docker containers (`docker-dev`, `docker-test`, `docker-prod`, `docker-down`).

## Capabilities

### New Capabilities
- `docker-environments`: Defines requirements for containerized application environments across development, testing, and production, including multi-stage image optimization, automated resource/descriptor limits for testing containers, and unified Docker Compose orchestration with service health dependencies.

### Modified Capabilities
None. (Existing capabilities remain functionally identical; this capability provides containerization and orchestration infrastructure.)

## Impact

- **Infrastructure & Configuration**:
  - `docker/Dockerfile.dev`, `docker/Dockerfile.prod` (or root `Dockerfile`), and `docker/Dockerfile.test`.
  - `.dockerignore` for efficient, secure build contexts.
  - `docker-compose.yaml` updated with service definitions, profiles, and ulimit configurations.
  - `justfile` recipes for container lifecycle management.
- **Developer Experience**:
  - Developers can run unit, integration, Newman API contract, and load tests entirely in Docker containers without manually modifying host system limits (`ulimit`).
- **Dependencies & Host Isolation**:
  - Standardizes runtime dependencies across all environments, eliminating "works on my machine" issues and simplifying CI integration.
