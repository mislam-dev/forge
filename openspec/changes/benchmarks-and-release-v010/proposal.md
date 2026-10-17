# Proposal

## Why

Forge requires formal performance benchmarking documentation and structured release lifecycle management to validate its production readiness and portfolio presentation. Currently, while multi-tier k6 load scripts exist and initial tests have demonstrated sub-4ms median latencies, the comprehensive benchmark results are not consolidated into a dedicated `docs/BENCHMARKS.md` report or linked in the root `README.md` (Task T-020). Furthermore, the repository lacks a standardized `CHANGELOG.md` and git release tag for the `v0.1.0` milestone (Task T-021), preventing automated release tracking and evaluation against Keep a Changelog standards.

## What Changes

- **Execute & Record Multi-Tier Benchmarks (T-020)**:
  - Execute k6 load tests for health probe and read query scenarios under concurrent virtual user load.
  - Record empirical performance metrics: p(50), p(95), p(99) latencies, requests per second (req/s), and error rates.
  - Author comprehensive `docs/BENCHMARKS.md` documenting methodology, environment specifications, CLI reproduction commands, and tabular benchmark results across concurrency tiers.
  - Update `README.md` to feature the benchmark report in the Documentation Map and badge/highlight sections.
- **Author CHANGELOG.md & Prepare v0.1.0 Release Tag (T-021)**:
  - Create root `CHANGELOG.md` adhering to Keep a Changelog standards and Semantic Versioning (SemVer 2.0.0).
  - Document `[0.1.0] - 2026-10-17` highlights, categorized under `Added`, `Changed`, `Fixed`, and `Security` (covering RBAC, SeaORM 2.0, RabbitMQ competing workers, Bollard Docker integration, AES-256-GCM encryption, OpenAPI 3.0.3, and multi-tier k6 suites).
  - Define procedure to create annotated git release tag `v0.1.0`.
- **Task Tracking Alignment**:
  - Update `docs/03-TASKS.md` marking T-020 and T-021 as completed, advancing the Resume & Portfolio track and overall task dashboard.

## Capabilities

### New Capabilities
- `release-management`: Manages semantic versioning, Keep a Changelog specifications (`CHANGELOG.md`), and git release tagging conventions for platform milestones.

### Modified Capabilities
- `project-documentation`: Extends project documentation requirements to include a dedicated `docs/BENCHMARKS.md` specification detailing empirical load testing results, environment setup, reproducible CLI commands, and bidirectional cross-links with `README.md`.

## Impact

- **Files Created**:
  - `docs/BENCHMARKS.md`
  - `CHANGELOG.md`
- **Files Modified**:
  - `README.md` (links and references to benchmarks)
  - `docs/03-TASKS.md` (progress tracking for T-020 and T-021)
- **External Dependencies & Tools**:
  - `k6` CLI (load generation)
  - `git` (tagging `v0.1.0`)
- **Compatibility**:
  - Non-breaking; additive documentation and release metadata.
