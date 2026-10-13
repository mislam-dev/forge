# Design

## Context

The Forge API is an asynchronous HTTP service written in Rust with Axum, SeaORM (PostgreSQL), and Redis caching. While comprehensive integration and Postman API tests validate functional correctness, no automated load test harnesses exist to verify throughput and latency under concurrent user demand. SRS Section 6, 13, and NFR-001 target 10,000+ concurrent requests.

See `proposal.md` for motivation and `specs/load-testing/spec.md` for functional requirements.

## Goals / Non-Goals

**Goals:**
- Provide a modular k6-based load testing framework located in `tests/load/`.
- Establish three isolated execution profiles for the three user concurrency tiers:
  - Tier 1: 1,000 Virtual Users (baseline / smoke test)
  - Tier 2: 10,000 Virtual Users (SRS NFR-001 compliance target)
  - Tier 3: 100,000 Virtual Users (stress, saturation, and graceful degradation limit)
- Implement realistic workload scenarios (health probes, auth login, authenticated read flows, and project mutations).
- Implement token pre-generation/caching to avoid bcrypt CPU bottlenecks during high-concurrency read/write benchmarks.
- Provide frictionless developer workflows via `justfile` recipes and Docker containerized runner commands.
- Include structured reporting output (console summary and JSON export) with threshold validations.

**Non-Goals:**
- Setting up a multi-node distributed Kubernetes load generator cluster (Tier 3 on a single workstation will utilize stage-ramped virtual users with connection pooling and OS tuning guidelines).
- Altering the core Rust application architecture or database schema as part of this test harness addition.

## Decisions

### 1. Load Testing Tool: Grafana k6
- **Decision:** Use Grafana k6 for all load test scripts and execution runners.
- **Rationale:** k6 is written in Go, provides excellent concurrent execution performance with low CPU/memory overhead, supports standard ES6 JavaScript for complex scenario orchestration, and features built-in threshold enforcement (`http_req_duration`, `http_req_failed`). It can be run as a standalone CLI tool or via official Docker images (`grafana/k6`) without introducing Python or node package dependencies into the repository.
- **Alternatives Considered:**
  - *Locust (Python):* Rejected due to Python GIL concurrency limits requiring multi-worker process management for 10k+ users.
  - *wrk / vegeta / oha:* Rejected because while fast, they lack multi-step user journey scripting (e.g., authenticate, extract token, query organization, query projects).
  - *Custom Rust Tokio runner:* Rejected due to higher maintenance overhead compared to standard tooling with rich metric reporting.

### 2. Directory Layout & Architecture
- **Decision:** Structure load testing assets under `tests/load/`:
  ```
  tests/load/
  ├── package.json (or config meta)
  ├── config.js               # Shared thresholds, base URLs, tier definitions
  ├── helpers/
  │   ├── auth.js             # Token acquisition, token caching, auth headers
  │   └── metrics.js          # Custom Trend and Counter metric definitions
  ├── scenarios/
  │   ├── health.js           # Lightweight throughput baseline
  │   ├── auth.js             # Login endpoint stress test
  │   ├── reads.js            # Cached and un-cached read endpoints
  │   └── full_journey.js     # Realistic mixed read/write developer flow
  └── tiers/
      ├── tier_1k.js          # 1,000 VUs ramp-up and execution profile
      ├── tier_10k.js         # 10,000 VUs target execution profile
      └── tier_100k.js        # 100,000 VUs stress & degradation profile
  ```
- **Rationale:** Separating scenario logic from concurrency tier execution profiles allows scenarios to be reused across different user scales or run independently.

### 3. Authentication & Password Hashing Mitigation
- **Decision:** Use a dual-strategy for authentication in load tests:
  1. Dedicated Auth Scenario: Tests `POST /api/v1/auth/login` with realistic request arrival rates to measure bcrypt performance and rate-limiting.
  2. High-Concurrency Scenarios (1k, 10k, 100k): Pre-fetch or generate JWT bearer tokens during k6's `setup()` lifecycle hook and distribute them across virtual users.
- **Rationale:** If 10,000 virtual users each invoke bcrypt password verification on every HTTP request, server CPU will be consumed 100% by bcrypt hashing rather than testing web server concurrency, database connection pools, or Redis caching.

### 4. Tier 3 (100,000 VUs) Execution Strategy & OS Constraints
- **Decision:** Tier 3 uses aggressive connection reuse (`k6 run --no-connection-reuse=false`), staged ramp-up, and configurable duration. Documentation will provide the required system parameter adjustments:
  - File descriptor limit: `ulimit -n 1048576`
  - Ephemeral port expansion and TCP recycling settings (`sysctl -w net.inet.ip.portrange.first=1024` on macOS / Linux TCP tw settings).
  - Dockerized runner option with increased container limits.

## Risks / Trade-offs

- **[Risk]** Local developer workstations running 100,000 VUs may exhaust file descriptors or ephemeral TCP ports before the API server hits saturation.
  - **Mitigation:** Document OS socket limits (`ulimit -n`) and provide staged execution ramping with connection pooling. Provide Docker-based invocation in `justfile` so container network stacks can be isolated.
- **[Risk]** Intensive write testing at 10,000+ VUs could flood the local test database with excess test records.
  - **Mitigation:** Weighted scenarios allocate 90% of traffic to read operations (`/health`, `/user/profile`, `/organizations`, `/projects`) and 10% to write operations, using dedicated cleanup routines or transactional test tenants.
- **[Risk]** Threshold false positives when executed on underpowered local laptops.
  - **Mitigation:** Allow threshold configurations to be overridden via environment variables (e.g. `LOAD_TEST_STRICT=false` or custom threshold multipliers).

## Migration Plan

No migration or production disruption is required:
1. Load test files are added to `tests/load/`.
2. `justfile` is updated with `load-test-1k`, `load-test-10k`, and `load-test-100k` recipes.
3. CI/CD or staging environments can execute tiers as part of performance validation pipelines.
