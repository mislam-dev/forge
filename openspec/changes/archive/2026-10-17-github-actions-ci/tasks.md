# Tasks

## 1. CI Workflow Setup & Triggers

- [x] 1.1 Create `.github/workflows/ci.yml` with triggers restricted strictly to `pull_request` targeting `main` and direct `push` on `main`
- [x] 1.2 Configure `rustfmt` quality gate job running `cargo fmt --check` with Rust stable toolchain

## 2. Linter & Build Validation Jobs

- [x] 2.1 Configure `clippy` quality gate job running `cargo clippy --all-targets --all-features -- -D warnings` with caching enabled
- [x] 2.2 Configure `docker-build` job to build the root `Dockerfile` using Docker Buildx and verify cache stage generation

## 3. Test Suite & Service Containers

- [x] 3.1 Configure `test` job with PostgreSQL 16 and RabbitMQ 3 service containers, healthchecks, and environment variables
- [x] 3.2 Add `cargo test --workspace` step to `test` job and verify execution against service containers

## 4. Verification & Documentation

- [x] 4.1 Validate workflow YAML syntax and verify jobs, steps, and trigger boundaries against specifications
- [x] 4.2 Update `docs/03-TASKS.md` to update Task T-009 status and progress dashboard
