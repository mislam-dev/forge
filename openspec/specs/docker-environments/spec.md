# docker-environments Specification

## Purpose
Provides containerized runtime, build, and test environments for the Forge platform across development, production, and testing lifecycles, integrating application services into Docker Compose with automated system limits and robust healthcheck dependencies.

## Requirements

### Requirement: Development Container Environment
The system SHALL provide a dedicated development container environment (`Dockerfile.dev`) configured for rapid developer feedback. The container image MUST include the Rust development toolchain, development dependencies, and file-watching utilities (`cargo-watch`). The container configuration SHALL support volume mounting of source code and caching of cargo registry/target build artifacts so incremental compilation remains fast across restarts.

#### Scenario: Running development service with hot reloading
- **WHEN** a developer starts the application using the development container configuration
- **THEN** the container SHALL mount local source files, compile in debug mode, and automatically recompile and restart the server upon source file changes without rebuilding the container image.

#### Scenario: Debug logging and diagnostic environment
- **WHEN** the development container is initialized
- **THEN** the environment SHALL enable verbose debug logging (`RUST_LOG=debug`) and full backtraces (`RUST_BACKTRACE=1`) by default.

### Requirement: Production Container Environment
The system SHALL provide an optimized multi-stage production container image (`Dockerfile.prod` or root `Dockerfile`). The build stage MUST compile a release binary with symbols stripped, and the runtime stage MUST use a minimal, hardened base image (such as `debian:bookworm-slim` or distroless) executing as an unprivileged non-root user. The production image MUST include a built-in container healthcheck probe targeting the HTTP `/health/live` endpoint.

#### Scenario: Building minimal production image
- **WHEN** the production Docker image is built
- **THEN** the resulting image MUST contain only the compiled release binary, required runtime libraries, CA certificates, and non-root user configuration, omitting compilers, cargo build caches, and unnecessary tooling.

#### Scenario: Container health verification in production
- **WHEN** the production container starts and enters steady state
- **THEN** the Docker daemon SHALL periodically query `/health/live`, marking the container healthy when HTTP 200 is received and unhealthy upon successive failures.

### Requirement: Containerized Testing Environment with Automated Limits
The system SHALL provide a self-contained testing container environment (`Dockerfile.test`) equipped with all dependencies necessary to execute Rust unit/integration tests, Newman API collection matrix tests, and k6 load tests. The test container configuration MUST automatically enforce elevated file descriptor and socket limits (setting `nofile` soft and hard limits to at least 65535) within the container specification, ensuring that neither developers nor CI runners need to manually tune host OS limits (such as macOS `ulimit -n` or sysctl settings).

#### Scenario: High concurrency testing without manual host limit configuration
- **WHEN** automated tests or high-concurrency test suites (such as k6 load tests or parallel cargo tests) are launched in the test container
- **THEN** the container process SHALL run with configured `nofile` limits of at least 65535, preventing `EMFILE` / "Too many open files" errors without requiring host OS `ulimit` modifications.

#### Scenario: Complete test suite execution within container
- **WHEN** the test container is invoked with test targets (`cargo test`, `just test-api`, or `just test-api-matrix`)
- **THEN** the tests MUST run against isolated container network services and write generated test reports or artifacts to a mounted output directory on the host.

### Requirement: Unified Docker Compose Service Orchestration
The system SHALL provide unified Docker Compose configuration orchestrating the Forge application alongside its required backing infrastructure (PostgreSQL, Redis, RabbitMQ, Loki, Grafana). The application service MUST declare explicit `depends_on` relationships with health check conditions (`service_healthy`) for all mandatory backing dependencies. The compose configuration SHALL support modular profiles (`dev`, `test`, `prod`) and configure container-level `ulimits` directly in Compose service definitions.

#### Scenario: Starting application stack with healthy dependencies
- **WHEN** the application service is started via Docker Compose
- **THEN** Docker Compose SHALL wait until PostgreSQL, Redis, RabbitMQ, and Loki pass their respective health checks before initializing the Forge application service.

#### Scenario: Running test profile via Docker Compose
- **WHEN** the test profile is triggered via Docker Compose
- **THEN** Docker Compose SHALL start backing services, run the database migrations and seed fixtures if specified, and execute the test runner container with elevated ulimits under the `backend` network.

### Requirement: Standardized CLI Workflow Targets
The system SHALL provide standardized recipes in `justfile` to build, run, and test the application within Docker environments.

#### Scenario: Executing container workflows via Just
- **WHEN** a developer executes `just docker-dev`, `just docker-test`, or `just docker-prod-up`
- **THEN** the corresponding Docker Compose profiles and services SHALL execute with appropriate environment variables, port mappings, and volume mounts.
