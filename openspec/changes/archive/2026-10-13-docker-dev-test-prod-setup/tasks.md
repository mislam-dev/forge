# Tasks

## 1. Build Context and Ignore Rules

- [x] 1.1 Create `.dockerignore` file excluding `target/`, `node_modules/`, `reports/`, `logs/`, `.git/`, `.env`, and IDE artifacts to keep build contexts fast and clean. Verify file is created and covers all artifact directories.

## 2. Dockerfiles Implementation

- [x] 2.1 Implement `docker/Dockerfile.dev` with `rust:1.85-slim-bookworm`, essential build tools, `cargo-watch` installation, and development entrypoint. Verify file builds successfully with `docker build -t forge:dev -f docker/Dockerfile.dev .`.
- [x] 2.2 Implement `docker/Dockerfile.prod` (and root `Dockerfile`) using a multi-stage `cargo-chef` pipeline (planner, builder, runtime) with a minimal Debian Bookworm runtime, unprivileged `appuser` (UID 10001), and a curl/health check probe. Verify production image builds and binary is stripped.
- [x] 2.3 Implement `docker/Dockerfile.test` containing Rust toolchain, Node.js, Newman, and test dependencies. Verify test runner image builds with `docker build -t forge:test -f docker/Dockerfile.test .`.

## 3. Docker Compose Integration & Limits

- [x] 3.1 Update `docker-compose.yaml` to define application services (`forge-dev`, `forge-api`, `forge-test`) using Docker Compose profiles (`dev`, `prod`, `test`). Verify `docker compose config` validates cleanly.
- [x] 3.2 Configure explicit container `ulimits` (`nofile: 65535`) on `forge-test` and application services in `docker-compose.yaml` so high-concurrency tests execute without manual host OS tweaks. Verify ulimits in container via `docker compose --profile test run --rm forge-test ulimit -n`.
- [x] 3.3 Configure `depends_on` service health dependencies (`condition: service_healthy`) for `postgres`, `redis`, `rabbitmq`, and `loki` so application and test containers wait for dependency readiness before booting.

## 4. Workflow Automation in Justfile

- [x] 4.1 Add container lifecycle recipes to `justfile` (`docker-dev`, `docker-test`, `docker-test-api`, `docker-prod-up`, `docker-down`). Verify `just --list` displays the new container commands.

## 5. End-to-End Verification

- [x] 5.1 Validate development container hot reloading and verify compilation in `forge-dev`.
- [x] 5.2 Validate automated test execution in `forge-test` (Cargo test and Newman API tests) without host limit errors.
- [x] 5.3 Validate full production stack boot in `forge-api` with health checks passing against containerized PostgreSQL, Redis, RabbitMQ, and Loki.
