# Proposal

## Why

Forge lacks a root repository `README.md` and an open-source license declaration (`LICENSE`), making it difficult for technical recruiters, reviewers, and external developers to evaluate the platform's architectural design, understand setup steps, or verify its open-source license terms. Establishing a production-grade root `README.md` and standard MIT `LICENSE` fulfills portfolio and open-source readiness requirements (T-016 and T-017).

## What Changes

- Add a production-grade root `README.md` containing:
  - Project status and tech stack badges (Rust 2024, MIT License, Newman 93 tests, Docker Ready, OpenAPI 3.0).
  - Concise problem statement and target use cases (self-hosted developer platform / PaaS for containerized applications).
  - Architecture overview with an ASCII / Mermaid diagram detailing Axum HTTP layer, PostgreSQL persistence, RabbitMQ event bus, Bollard Docker daemon worker, and Loki log aggregation.
  - Quickstart guide covering local prerequisites, backing service orchestration (`docker compose up -d`), database migrations, and server startup (`cargo run`).
  - Key technical highlights and architectural decisions (modular monolith, SeaORM connection pooling, relational RBAC, asynchronous build worker pipeline).
  - Performance benchmarks and empirical test metrics (sub-4ms median response latency, 265+ req/s sustained throughput under k6 load tests, 100% unit and Newman contract test pass rates).
  - Navigation links to SRS, ADRs, OpenAPI specs, and testing guides.
- Add an open-source `LICENSE` file (MIT License) at the repository root.
- Update `Cargo.toml` package metadata to declare `license = "MIT"` and relevant package descriptive metadata.

## Capabilities

### New Capabilities
- `project-documentation`: Defines documentation contracts for the repository root `README.md` (architecture diagrams, quickstart guides, technical highlights, benchmark summaries) and open-source licensing terms via root `LICENSE` and crate metadata.

### Modified Capabilities
None.

## Impact

- Affected files: `README.md` (new file), `LICENSE` (new file), `Cargo.toml` (package metadata addition).
- No runtime application code, APIs, or database migrations are altered.
