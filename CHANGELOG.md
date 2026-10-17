# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-17

### Added

- **Modular Monolith Core Architecture:** High-performance self-hosted developer platform (PaaS) engineered in modern Rust 2024 edition, powered by Axum 0.8 and Tokio multi-threaded asynchronous runtime.
- **Relational Persistence & Connection Pooling:** Integrated SeaORM 2.0 with PostgreSQL 16, managing 18 normalized database tables, migration versioning, and auto-scaling connection pooling.
- **5-Tier Role-Based Access Control (RBAC):** Relational RBAC system supporting multi-tenant organizations and personal workspaces with granular permissions across `System Admin`, `Admin`, `Developer`, `Maintainer`, and `Viewer` roles.
- **Docker Engine Daemon Integration:** Containerized build execution engine interacting directly with `/var/run/docker.sock` via `bollard` with resource quotas, ephemeral build containers, and runtime sandboxing.
- **Event-Driven Messaging Bus:** Competing-consumer background worker pipeline powered by RabbitMQ (AMQP 2.1) exchange `forge.deployments.jobs` and consumer prefetch throttling (`basic.qos = 2`).
- **Distributed Log Streaming:** Real-time stdout/stderr build log ingestion into Grafana Loki (`POST /push`) and client-side Server-Sent Events (SSE) streaming.
- **Interactive OpenAPI Documentation:** Validated OpenAPI 3.0.3 specification covering 83 operations, rendered via interactive Swagger UI at `/swagger-ui`.
- **Multi-Tier Load Benchmark Suite:** Automated Grafana k6 performance testing framework supporting 1,000, 10,000, and 100,000 virtual users with empirical validation documented in `docs/BENCHMARKS.md`.
- **Headless Postman / Newman Automation:** Complete API collection test automation with 93 operations validated across 6 RBAC tenancy environments with visual HTML reports.
- **Continuous Integration Pipeline:** GitHub Actions workflow (`.github/workflows/ci.yml`) enforcing `rustfmt`, strict `clippy`, workspace test suites, and Docker multi-stage caching builds.

### Changed

- **Standardized API Base Path:** Centralized all modular service routers under canonical `/api/v1` namespace with root-level `/health` public probe endpoints.
- **Dynamic Observability Endpoints:** Converted hardcoded Grafana Loki endpoints in tracing subscriber to dynamic resolution from `LOKI_URL` environment variables.
- **Global Error Envelope Harmonization:** Standardized 404 Not Found error responses to conform uniformly to `{ "code": "NOT_FOUND", "message": "The requested resource was not found" }`.

### Fixed

- **Docker Healthcheck Probing:** Fixed container healthcheck instruction to target public unauthenticated `/health/live` probe.
- **Clippy Compiler Lints:** Addressed and resolved 138 Clippy lints across all modules, tests, and database migration crates to achieve zero-warning compiler status.
- **User Profile Deletion Lifecycle:** Aligned user profile deletion with account cascading semantics, removing redundant isolated profile deletion test cases.
- **Postman RBAC Refresh Assertion:** Parameterized JWT refresh variables and assertions across multi-role test environments.

### Security

- **Authenticated AES-256-GCM Encryption:** Implemented NIST-compliant AES-256-GCM symmetric cipher with 12-byte random nonces (`nonce:ciphertext`) for GitHub Personal Access Tokens and project environment secrets.
- **Internal Worker Token Decoupling:** Decoupled internal build worker status callbacks from `MASTER_ENCRYPTION_KEY`, introducing dedicated `WORKER_SERVICE_TOKEN` secrets.
- **Rate Limiting & Healthcheck Exemption:** Applied `axum-limit` rate-limiting middleware to protect core API endpoints while guaranteeing unthrottled access for infrastructure health monitors.
