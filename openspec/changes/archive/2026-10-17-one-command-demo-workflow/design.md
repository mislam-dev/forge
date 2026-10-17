# Design

## Context

See `proposal.md` for motivation. Forge consists of an Axum HTTP server (`src/main.rs`), PostgreSQL database migrations (`sea-orm-cli`), RabbitMQ message broker for asynchronous build worker pipelines, Grafana Loki for log streaming, Redis for caching, and Swagger UI at `/docs`.

Currently, `justfile` defines isolated recipes (`docker-infra-up`, `db-up`, `seed`, `start`, `dev`), but lacks an automated end-to-end demonstration workflow. Running each step manually is error-prone for external evaluators, recruiters, or new contributors.

## Goals / Non-Goals

**Goals:**
- Provide a single, turnkey entry point: `just demo`.
- Automate backing container startup, database migration, and test account seeding.
- Poll API health probes (`/health/live` and `/health/ready`) with timeout guards.
- Execute an authenticated end-to-end API demonstration flow via `curl` (login $\rightarrow$ JWT token $\rightarrow$ create project $\rightarrow$ dispatch deployment).
- Launch Swagger UI (`http://localhost:9000/docs`) in the default system browser.
- Ensure the script runs smoothly cross-platform on macOS and Linux within 60 seconds.
- Provide graceful fallbacks if optional utilities (like `jq` or GUI browser openers) are unavailable.

**Non-Goals:**
- Replacing existing development recipes (`just dev`, `just test`, `just ci`).
- Managing cloud or remote infrastructure beyond local Docker Compose.
- Modifying core application business logic or API contracts.

## Decisions

### Decision 1: Dedicated Orchestration Script (`scripts/demo.sh`) vs Inline Just Recipe
- **Choice**: Implement the core orchestration in an executable POSIX/Bash script `scripts/demo.sh` and invoke it via `just demo`.
- **Rationale**: The demonstration flow requires process trapping, health polling loops, JSON parsing, API response validation, and cross-platform browser launching. Shell scripts provide superior error handling, trapped signals (`SIGINT`, `EXIT`), and testability compared to long inline multiline blocks in `justfile`.
- **Alternatives Considered**: Inlining everything in `justfile`. Rejected because multiline conditional logic, signal trapping, and retry loops in justfile recipes become brittle and harder to debug.

### Decision 2: API Lifecycle and Process Management
- **Choice**: The demonstration runner checks if an API instance is already running on port 9000. If running and healthy, it proceeds immediately. If not running, it starts the API in the background, logs output to a temporary log file (`.demo-api.log`), monitors `/health/ready` until responsive, executes the demo payloads, launches the browser, and informs the user of the active PID and log location.
- **Rationale**: Allows instant repeated runs if the API is already active, while guaranteeing zero-config automatic startup on clean clones.
- **Alternatives Considered**: Always killing and restarting the server. Rejected because compiling `cargo run` needlessly slows down repeat evaluations.

### Decision 3: Resilient JSON Parsing Without Mandatory `jq`
- **Choice**: Attempt to parse JSON with `jq` if available; otherwise gracefully fall back to a minimal Python (`python3 -c "import sys, json; ..."`) or `sed`/`grep` regex extractor.
- **Rationale**: External evaluators and hiring managers might clone the repo on clean machines lacking `jq`. The demo should never crash due to a missing auxiliary CLI tool.

### Decision 4: Cross-Platform Browser Launch
- **Choice**: Detect OS environment and invoke `open` (macOS), `xdg-open` (Linux), or print the link with fallback if in headless/SSH environments.
- **Rationale**: Prevents script failures in CI or containerized terminals where no display server is attached.

## Risks / Trade-offs

- **[Risk]** Port 9000 or 5432 already occupied by unrelated services.  
  $\rightarrow$ **Mitigation:** Pre-flight port check prints clear diagnostic message identifying the conflicting port before failing.
- **[Risk]** Slow initial compilation on machines with empty cargo cache exceeding 60s.  
  $\rightarrow$ **Mitigation:** Run `cargo check` and compile release/dev binary efficiently; print clear step-by-step progress timers so the evaluator sees active compilation progress.
- **[Risk]** Headless terminal crashes when trying to launch GUI browser.  
  $\rightarrow$ **Mitigation:** Suppress browser launch errors (`|| true`) and print clickable URL in console output.
