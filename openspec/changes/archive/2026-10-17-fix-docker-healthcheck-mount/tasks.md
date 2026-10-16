# Tasks

## 1. Application Routing & Health Mount

- [x] 1.1 In `src/app/app.rs`, mount `health_router()` at root `.nest("/health", health_router())` on the outer application router alongside the `/api/v1` routes and verify compilation with `cargo check`.
- [x] 1.2 Verify public unauthenticated access to `/health`, `/health/live`, and `/health/ready` by running `cargo test --test health_tests`.

## 2. Docker Container Healthcheck Configuration

- [x] 2.1 Update the `HEALTHCHECK` instruction in `Dockerfile` line 59 to curl `http://localhost:9000/health/live || exit 1`.
- [x] 2.2 Update the `HEALTHCHECK` instruction in `docker/Dockerfile.prod` line 59 to curl `http://localhost:9000/health/live || exit 1`.

## 3. Verification & Documentation

- [x] 3.1 Execute `cargo test --test health_tests` and verify all root `/health` probe test cases pass.
- [x] 3.2 Update `docs/03-TASKS.md` to mark task `T-002` as completed.
