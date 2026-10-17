# Proposal

## Why

Evaluating or showcasing the Forge self-hosted PaaS platform currently requires manual orchestration: developers or technical recruiters must review README instructions, sequentially start Docker Compose infrastructure services, run SeaORM migrations, invoke the seed binary, boot the Axum HTTP server in a separate terminal, execute curl commands to obtain auth tokens and trigger deployments, and manually navigate to the Swagger UI in a browser.

Providing a turnkey, one-command demonstration workflow (`just demo`) eliminates onboarding friction, guarantees reproducibility on clean clones in under 60 seconds, and provides a polished developer and recruiter evaluation experience showcasing the platform's full stack—database, messaging, migrations, authentication, deployment queueing, and OpenAPI documentation.

## What Changes

- Add a standalone, robust `demo` orchestration recipe to `justfile` that coordinates the entire demonstration lifecycle.
- Add a helper automation script (or self-contained recipe runner) `scripts/demo.sh` to handle health check polling, authentication, sample project creation, deployment dispatch, and browser opening cross-platform (macOS/Linux).
- Automate backing infrastructure startup (`postgres`, `rabbitmq`, `loki`, `redis`) via Docker Compose with health-status readiness waiting.
- Automate database migrations (`just db-up`) and database seeding (`just seed`) with clear console feedback and error handling.
- Verify API server readiness via `/health/ready` or `/health/live` before executing demonstration payloads.
- Execute an end-to-end sample project deployment flow:
  1. Authenticate against `POST /api/v1/auth/login` using seeded credentials (`admin1@forge.local` / `Password123!`).
  2. Extract JWT bearer token.
  3. Create or inspect a sample project via `POST /api/v1/projects`.
  4. Trigger a sample deployment via `POST /api/v1/projects/{id}/deployments`.
  5. Log the queued deployment response with deployment ID and status.
- Automatically open Swagger UI (`http://localhost:9000/docs`) in the default system browser upon successful demonstration deployment.
- Update `docs/03-TASKS.md` to reflect task `T-019` implementation status and verification notes.

## Capabilities

### New Capabilities
- `one-command-demo`: Automated single-command local demonstration and evaluation workflow orchestrating infrastructure startup, database migrations and seeding, API readiness checks, automated sample project deployment dispatch, and Swagger UI browser launch within 60 seconds.

### Modified Capabilities
<!-- None: existing capability requirements remain unchanged. -->

## Impact

- **Affected files / tooling**: `justfile`, `scripts/demo.sh` (new helper), `docs/03-TASKS.md`.
- **Dependencies**: Relies on existing Docker Compose services (`docker-compose.yaml`), SeaORM CLI (`sea-orm-cli`), `cargo`, `curl`, and standard platform browser openers (`open` on macOS / `xdg-open` on Linux).
- **APIs**: Consumes existing `/health`, `/api/v1/auth/login`, `/api/v1/projects`, and `/api/v1/projects/{id}/deployments` endpoints.
