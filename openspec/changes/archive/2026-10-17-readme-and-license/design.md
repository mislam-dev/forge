# Design

## Context

Forge is an event-driven, self-hosted developer platform (PaaS) built in Rust (Axum 0.8, SeaORM 2.0, Tokio 1.53, RabbitMQ, Docker Engine via Bollard, Grafana Loki). While the internal system documentation in `docs/system/` is comprehensive (IEEE-830 SRS, ADRs, OpenAPI 3.0 spec), the repository root lacks:
1. An authoritative `README.md` that serves as the entry point for technical evaluators, recruiters, and contributors.
2. A formal `LICENSE` file.
3. Package license metadata in `Cargo.toml`.

See `proposal.md` for motivation and `specs/project-documentation/spec.md` for requirements.

## Goals / Non-Goals

**Goals:**
- Provide a production-grade, GitHub-ready root `README.md` featuring:
  - Tech stack & test suite badges (Rust 2024, MIT License, Newman 93 tests, Docker Ready, OpenAPI 3.0.3).
  - High-impact problem statement and architectural positioning (self-hosted Heroku/Vercel alternative).
  - Native GitHub Mermaid diagram detailing HTTP request routing, PostgreSQL persistence, RabbitMQ event bus, Bollard Docker daemon worker, and Loki log streaming.
  - Step-by-step Quick Start instructions using verified workspace commands (`docker compose up -d`, `just db-up` / migrations, `cargo run`, and Swagger UI access).
  - Core technical highlights (Tokio async concurrency, SeaORM connection pooling, relational RBAC, Bollard containerization).
  - Empirically verified k6 benchmark numbers from the architectural audit.
  - Navigation links to SRS, ADRs, OpenAPI specs, and test guides.
- Create an official `LICENSE` file containing the standard MIT License text.
- Add `license = "MIT"` and supporting descriptive metadata (`description`, `readme`, `repository`) to `Cargo.toml`.
- Prepare task tracking updates for `docs/03-TASKS.md` for tasks T-016 and T-017.

**Non-Goals:**
- Editing application runtime code, middleware, or database migrations.
- Implementing GitHub Actions CI (T-009) or the `just demo` workflow (T-019), which are scheduled in later roadmap phases.
- Modifying Postman collections or k6 test scenarios.

## Decisions

### Decision 1: GitHub-Native Mermaid Architectural Diagrams
- **Choice:** Use native GitHub-flavored Markdown Mermaid blocks (`flowchart TD`) for the system architecture diagram.
- **Rationale:** Renders immediately on GitHub web and modern Markdown readers without external image dependencies, SVG hosting, or binary asset drift.
- **Alternatives Considered:**
  - *Pre-rendered PNG / SVG assets:* Requires maintaining an image pipeline or binary files in git; difficult to update when architectural components evolve.
  - *ASCII art only:* Less visually striking for portfolio and recruiter presentation.

### Decision 2: Open Source License Selection (MIT)
- **Choice:** Standard MIT License.
- **Rationale:** Permissive, universally recognized, and matches task T-017 specifications in `docs/03-TASKS.md`. Enables GitHub repository insights to surface the "License: MIT" badge.
- **Alternatives Considered:**
  - *Apache 2.0 / Dual MIT + Apache 2.0:* Common in library crates, but MIT is standard and straightforward for self-hosted developer tooling and application templates.

### Decision 3: Documenting Empirically Audited Metrics
- **Choice:** Incorporate the exact benchmark results captured in `docs/01-ASSESSMENT.md` and `docs/03-TASKS.md`:
  - 2,679 requests in 10s (265.45 req/s sustained)
  - 0.00% error rate under concurrent load
  - 3.59ms average latency (1.42ms median, 13.02ms p95, 35.22ms p99)
  - 275 unit tests (100% pass) and 93 Newman API tests (100% pass, 5ms avg response)
- **Rationale:** Technical hiring reviewers and senior engineers value measured, reproducible empirical data over speculative claims.
- **Alternatives Considered:**
  - *Generic statements without numbers:* Less credible to senior technical evaluators.

## Risks / Trade-offs

- **[Risk: Stale commands or drift between README and actual CLI recipes]**  
  → *Mitigation:* Explicitly verify all documented commands against `justfile` (`just up`, `just db-up`, `cargo run`) and confirm port allocations (e.g., port 9000 for Axum, port 5432 for Postgres, port 5672/15672 for RabbitMQ).
- **[Risk: `Cargo.toml` formatting or validation errors]**  
  → *Mitigation:* Validate crate metadata with `cargo check` and `cargo fmt --check` after updating `Cargo.toml`.
