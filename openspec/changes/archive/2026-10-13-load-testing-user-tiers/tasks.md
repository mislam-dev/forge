# Tasks

## 1. Foundation and Configuration Setup

- [x] 1.1 Create `tests/load/` directory structure (`helpers/`, `scenarios/`, `tiers/`) and configuration in `tests/load/config.js` defining target endpoints, connection parameters, and tier threshold definitions. Verify by checking file existence and syntax.
- [x] 1.2 Implement helper utilities in `tests/load/helpers/auth.js` and `tests/load/helpers/metrics.js` for session token caching, auth header generation, and custom trend/counter metrics. Verify helper module syntax and exports.

## 2. Workload Scenario Scripts

- [x] 2.1 Implement `tests/load/scenarios/health.js` for lightweight throughput baseline probing (`GET /health` and system probes). Verify script syntax and validate with a single-VU smoke run.
- [x] 2.2 Implement `tests/load/scenarios/auth.js` for authentication load benchmarking (`POST /api/v1/auth/login`) with realistic arrival rates and status verification. Verify token generation and response validation.
- [x] 2.3 Implement `tests/load/scenarios/reads.js` for authenticated cached queries (`/user/profile`, `/organizations`, `/projects`, `/teams`). Verify headers, token propagation, and 200 OK responses.
- [x] 2.4 Implement `tests/load/scenarios/full_journey.js` orchestrating a realistic developer lifecycle workflow with mixed read/write actions. Verify multi-step request flow and error checks.

## 3. Concurrency Tier Runners

- [x] 3.1 Implement Tier 1 profile in `tests/load/tiers/tier_1k.js` with staged ramp-up to 1,000 virtual users, sustained load, and thresholds (p95 < 50ms, error rate < 0.1%). Verify stage configuration and threshold rules.
- [x] 3.2 Implement Tier 2 profile in `tests/load/tiers/tier_10k.js` with staged ramp-up to 10,000 virtual users and thresholds (p95 < 200ms, error rate < 1.0%) for SRS NFR-001 compliance. Verify stage configuration and threshold rules.
- [x] 3.3 Implement Tier 3 profile in `tests/load/tiers/tier_100k.js` simulating 100,000 virtual users with backpressure tracking, graceful degradation checks (429/503 monitoring), and post-load recovery timing. Verify stage configuration and threshold rules.

## 4. Automation & Developer Tooling

- [x] 4.1 Add `justfile` recipes (`load-test-1k`, `load-test-10k`, `load-test-100k`, and Dockerized k6 runner) for one-command test execution. Verify recipes appear in `just --list`.
- [x] 4.2 Create developer documentation in `docs/load-testing-guide.md` covering execution instructions, OS socket/ulimit tuning, benchmark reporting, and threshold analysis. Verify markdown document is created and formatted.
