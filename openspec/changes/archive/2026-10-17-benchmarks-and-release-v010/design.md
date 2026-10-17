# Design

## Context

See `proposal.md` for problem motivation and scope.

The Forge platform backend runs an asynchronous Axum 0.8 / Tokio runtime with SeaORM connection pools, RabbitMQ message brokers, Redis token caching, and Grafana Loki structured log streaming. In previous audit phases, smoke tests confirmed baseline health latency around 1.42ms median. Multi-tier k6 test scripts are organized under `tests/load/` (`tiers/` and `scenarios/`).

To complete Task T-020 and Task T-021 from `docs/03-TASKS.md`, the platform requires executing and capturing multi-tier load benchmarks into `docs/BENCHMARKS.md`, linking it in `README.md`, publishing an authoritative `CHANGELOG.md` following Keep a Changelog standards, creating git tag `v0.1.0`, and synchronizing task progress tracking in `docs/03-TASKS.md`.

## Goals / Non-Goals

**Goals:**
- Execute k6 load test scenarios against the running API server (health probes and authenticated read queries) under virtual user load.
- Record concrete empirical metrics: p(50), p(95), p(99) latencies, requests per second (RPS), and error rates.
- Author `docs/BENCHMARKS.md` detailing system environment, testing methodology, concurrency tier performance, bottleneck analysis, and CLI reproduction commands.
- Update `README.md` to link to `docs/BENCHMARKS.md` in the Documentation Map, testing section, and badges.
- Author `CHANGELOG.md` following Keep a Changelog v1.1.0 and SemVer 2.0.0 for initial release `[0.1.0] - 2026-10-17`.
- Create annotated git release tag `v0.1.0` reflecting the release state.
- Update `docs/03-TASKS.md` moving T-020 and T-021 to Completed and recalculating the tracking statistics.

**Non-Goals:**
- Modifying Rust core application logic, database migrations, or route handlers.
- Executing distributed cloud-scale load generation beyond local hardware capabilities.
- Modifying cargo package version (already `0.1.0` in `Cargo.toml`).

## Decisions

### Decision 1: Benchmark Execution Approach & Scenario Selection
- **Choice**: Execute native k6 load tests using the established scenarios in `tests/load/scenarios/health.js` and `tests/load/scenarios/reads.js`, alongside the multi-stage 1,000 VU runner in `tests/load/tiers/tier_1k.js`.
- **Rationale**: Demonstrates both unauthenticated high-throughput edge probes and authenticated database-backed read query performance under concurrency.
- **Alternatives Considered**: Mocking the network layer or using synthetic estimates — rejected because empirical reproducibility is essential for hiring reviewer audits and portfolio credibility.

### Decision 2: Documentation Structure for `docs/BENCHMARKS.md`
- **Choice**: Structured format including:
  1. Executive Summary & Headline Metrics
  2. Test Environment Specification (Hardware, OS, Runtime, Database, Container daemon)
  3. Methodology & Workload Distribution (Traffic weighting, duration, ramp curves)
  4. Empirical Results Tables (Tier 1 Baseline, Tier 2 Target, Tier 3 Saturation thresholds & actuals)
  5. Per-Endpoint Latency Distribution (`/health`, `/health/live`, `/api/v1/organizations`, `/api/v1/projects`)
  6. OS Socket & System Tuning Guide
  7. Exact CLI Reproduction Commands
- **Rationale**: Meets recruiter and senior staff engineer expectations for a production-grade infrastructure project.

### Decision 3: Keep a Changelog Compliance for `v0.1.0`
- **Choice**: Author `CHANGELOG.md` matching Keep a Changelog (1.1.0) with standard categories: `Added`, `Changed`, `Fixed`, and `Security`.
- **Rationale**: Standardized release documentation is expected for open-source and enterprise-grade software. All completed tasks (T-001 through T-018, T-020, T-021) will be categorized clearly.
- **Alternatives Considered**: Conventional Commits auto-generated changelog — rejected because curated, domain-rich release notes provide significantly better clarity on architectural capabilities and security improvements.

### Decision 4: Git Tag Annotation
- **Choice**: Create annotated git tag `git tag -a v0.1.0 -m "Release v0.1.0 - Initial production milestone for Forge developer platform"`.
- **Rationale**: Annotated tags include author metadata, date, and release description, providing an immutable reference in git history.

## Risks / Trade-offs

- **[Risk] Host File Descriptor / Ephemeral Port Exhaustion during high VU tests**  
  → *Mitigation*: Ensure `ulimit -n 65536` is applied for load testing sessions. Document OS tuning commands directly in `docs/BENCHMARKS.md`.
- **[Risk] Benchmark Numbers Varying Across Environments**  
  → *Mitigation*: Specify exact test runner hardware configuration (Apple M-series, memory, local Docker daemon) and include standard reproduction commands so reviewers can run the tests on their own machines.
- **[Risk] Git tag conflicts or uncommitted changes**  
  → *Mitigation*: Ensure all artifacts (`CHANGELOG.md`, `docs/BENCHMARKS.md`, `README.md`, `docs/03-TASKS.md`) are committed prior to tagging `v0.1.0`.
