# Forge

<p align="center">
  <strong>High-performance, event-driven self-hosted developer platform (PaaS) built in Rust.</strong>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust" alt="Rust 2024"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License MIT"></a>
  <a href="./CHANGELOG.md"><img src="https://img.shields.io/badge/Release-v0.1.0-blue?logo=github" alt="Release v0.1.0"></a>
  <a href="./.github/workflows/ci.yml"><img src="https://img.shields.io/badge/CI-Passing-brightgreen?logo=github-actions" alt="CI Status"></a>
  <a href="./docs/API_TESTING_AND_MOCKING.md"><img src="https://img.shields.io/badge/Newman_API_Tests-93%2F93_Passing-brightgreen?logo=postman" alt="Newman Tests"></a>
  <a href="./Cargo.toml"><img src="https://img.shields.io/badge/Workspace_Tests-383_Passing-brightgreen?logo=rust" alt="Workspace Tests"></a>
  <a href="./Dockerfile"><img src="https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker" alt="Docker Ready"></a>
  <a href="./docs/system/05-api/openapi.yaml"><img src="https://img.shields.io/badge/OpenAPI-3.0.3-85EA2D?logo=openapi-initiative" alt="OpenAPI 3.0.3"></a>
  <a href="./docs/BENCHMARKS.md"><img src="https://img.shields.io/badge/k6_Benchmarks-2%2C499.8_req%2Fs-blueviolet?logo=k6" alt="k6 Benchmarks"></a>
</p>

---

## Overview

**Forge** is a self-hosted platform-as-a-service (PaaS) engineered in modern Rust (Axum 0.8, Tokio, SeaORM 2.0, PostgreSQL 16). It provides developers and engineering teams with a self-contained environment to manage Git repositories, execute containerized application builds via direct Docker Engine daemon integration (`bollard`), orchestrate deployments with RabbitMQ competing-consumer workers, and stream real-time build logs into Grafana Loki and client applications.

Designed as an enterprise-grade **Modular Monolith**, Forge couples strict domain boundaries with a 5-tier relational Role-Based Access Control (RBAC) model supporting personal and multi-tenant organization workspaces—delivering sub-4ms median API latencies and zero-garbage-collection predictability.

---

## System Architecture

Forge decouples synchronous user API traffic from asynchronous, compute-intensive container builds via an event-driven message bus:

```mermaid
flowchart TD
    subgraph Clients["Clients & Automation"]
        Dev["Developer / Browser UI"]
        CLI["API Client / Newman / k6"]
        GH["Git Webhooks / Providers"]
    end

    subgraph HTTP["HTTP / Axum 0.8 Gateway (Port 9000)"]
        Router["Axum Modular Router (/api/v1)"]
        MW["Middleware Pipeline\n(CORS, Request ID, Rate Limiter, JWT Extractor)"]
        Swagger["OpenAPI Swagger UI\n(/docs)"]
        Health["Health & Liveness Probes\n(/health, /health/live)"]
    end

    subgraph Data["Persistence & Caching"]
        PG[("PostgreSQL 16\n(SeaORM Pool, 18 Tables, RBAC)")]
        Redis[("Redis 7\n(Token Revocation & Cache)")]
    end

    subgraph Messaging["Message Broker"]
        RMQ{{"RabbitMQ (AMQP 2.1)\nExchange: forge.deployments.jobs\nQueue: forge.build_worker\nDLX: forge.dlx"}}
    end

    subgraph Workers["Asynchronous Build Workers"]
        Worker["Build Pipeline Worker\n(Competing Consumer, Prefetch=2)"]
        Git["Git2 Cloner & Tar Streamer"]
        Bollard["Docker Engine Client\n(Bollard Unix Socket)"]
    end

    subgraph Containers["Docker Daemon Sandbox"]
        BuildContainers["Ephemeral Build Containers\n(Resource-Limited, Isolated UID)"]
        AppContainers["Deployed App Containers\n(Managed Ports & Env Secrets)"]
    end

    subgraph Observability["Observability Stack"]
        Loki[("Grafana Loki\n(Log Ingestion)")]
        Grafana["Grafana Dashboards\n(Port 3000)"]
        Tracing["Tracing Subscriber\n(JSON Structured Tracing)"]
    end

    %% Flow connections
    Dev --> Router
    CLI --> Router
    GH --> Router
    Router --> MW
    MW --> Swagger
    MW --> Health

    MW --> PG
    MW --> Redis
    MW -- "Publish Deployment Job" --> RMQ

    RMQ -- "Consume Job" --> Worker
    Worker --> Git
    Worker --> Bollard
    Bollard --> BuildContainers
    Bollard --> AppContainers

    Worker -- "Log Stream (HTTP / AMQP)" --> Loki
    Worker -- "Status Callback (PUT /status)" --> Router
    Tracing --> Loki
    Loki --> Grafana
```

### Execution Flow
1. **API Ingestion & Authorization**: Requests traverse the middleware stack (Request ID injection, `axum-limit` rate-limiting, and `jsonwebtoken` claim extraction). Stateless services validate domain rules via SeaORM repositories.
2. **Asynchronous Build Dispatch**: Triggering `POST /api/v1/projects/{id}/deployments` transitions the deployment state machine to `Queued` in PostgreSQL and publishes an AMQP message to RabbitMQ exchange `forge.deployments.jobs`.
3. **Competing Worker Execution**: Background workers (`amqprs`) dequeue build jobs with prefetch limits to prevent memory exhaustion, clone source code into an in-memory tar stream using `git2`, and communicate with `/var/run/docker.sock` via `bollard`.
4. **Log Streaming & State Sync**: Container build output (stdout/stderr) streams in real time to Grafana Loki (`POST /push`) and client SSE endpoints, followed by authenticated internal status callbacks (`worker_service_token`) to seal the deployment record.

---

## Technical Highlights & Engineering Decisions

- **Modular Monolith Architecture**: Clean domain segregation (`auth`, `users`, `access_control`, `organization`, `teams`, `projects`, `build_worker`, `deployments`, `notifications`, `dashboard`) preserving compile-time type safety and ACID transactions without distributed RPC overhead.
- **Asynchronous Docker Daemon Control**: Replaces fragile shell commands (`docker build`) with direct asynchronous Docker socket communication via `bollard`, enabling fine-grained memory/CPU cgroup limits, dynamic port allocation, and automated cleanup.
- **5-Tier Relational RBAC**: Flexible multi-tenancy supporting both personal and organization scopes with hierarchical role resolution (`Owner` > `Admin` > `Developer` > `Viewer`), validated across 6 tenant permission matrices.
- **Authenticated AES-256-GCM Encryption**: Third-party Git Personal Access Tokens (PATs) and sensitive environment variables are encrypted at rest with authenticated AES-256-GCM using unique 12-byte random initialization vectors (nonces), with secret values systematically redacted as `"••••••••"` in read DTOs.
- **Rate Limiting & Security Defense**: Active `axum-limit` middleware (15 requests/minute per URI quota) returning standard HTTP 429 errors to safeguard public auth routes against brute-force credential stuffing.
- **Decoupled Internal Service Tokens**: Dedicated `worker_service_token` authenticates internal deployment worker status callbacks, eliminating bearer token cryptographic key reuse.
- **Robust Connection Pooling & Resilience**: SeaORM connection pool configured with connect/idle timeouts, exponential backoff reconnection, and RabbitMQ dead-letter exchange (`forge.dlx`) routing for failed workloads.

---

## Empirically Verified Benchmarks

The platform has been audited using [Grafana k6](https://k6.io/) load testing and [Postman / Newman](https://www.postman.com/) test automation. All metrics represent empirical measurements documented in [`docs/BENCHMARKS.md`](./docs/BENCHMARKS.md):

| Benchmark Scenario / Target | Concurrency | Requests / Throughput | Latency Metrics | Error Rate | Status |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Health Probe Smoke Test** | 10 VUs (10s) | 2,820 req (280+ req/s) | **820.5 µs median**, 5.76 ms p95, 18.42 ms p99 | **0.00 %** | **PASSED** |
| **Relational Read Queries (5 Endpoints)** | 20 VUs (10s) | 3,751 req (364+ req/s) | **5.66 ms median**, 14.72 ms p95, 30.15 ms p99 | **0.00 %** | **PASSED** |
| **Tier 1 High-Concurrency Stress Test** | 1,000 CVU (120s) | 326,456 req (**2,499.8 req/s**) | 1.82 ms min, 398 ms max (833 MB I/O) | **0.25 %** (99.75% Success) | **PASSED** |
| **Newman API Test Suite** | 1 Client | 93 requests (279 assertions) | **5 ms average response time** | **0.00 % (0 Failures)** | **PASSED** |
| **RBAC Matrix Suite (6 Roles / Tenants)** | 6 Profiles | 558 total assertions | Admin, Developer, Viewer roles verified | **0.00 % (0 Failures)** | **PASSED** |
| **Rust Workspace Test Suite** | Multi-threaded | **383 automated tests** | 274 unit + 109 integration in < 1.0s | **0.00 % (0 Failures)** | **PASSED** |

---

## Quick Start Guide

### Option A: One-Command Live Demo (< 60 Seconds)
If you have `just` installed, launch the entire backing stack, run migrations, seed data, verify health, deploy a sample project, and open the interactive Swagger UI with a single command:

```bash
git clone https://github.com/forge/forge.git
cd forge
just demo
```

---

### Option B: Step-by-Step Manual Setup

#### Prerequisites
- [Docker](https://docs.docker.com/get-docker/) & Docker Compose (v2.20+)
- [Rust toolchain](https://rustup.rs/) (edition 2024 / Rust 1.85+)
- Optional: [just](https://github.com/casey/just) command runner

#### 1. Clone & Configure Environment
```bash
git clone https://github.com/forge/forge.git
cd forge

# Copy documented environment configuration template
cp .env.example .env
```

#### 2. Start Backing Infrastructure
Launch PostgreSQL 16, Redis 7, RabbitMQ 3, Grafana Loki, and Grafana in the background:
```bash
# Using Just:
just docker-infra-up

# Or using Docker Compose directly:
docker compose up -d postgres redis rabbitmq loki grafana
```

#### 3. Run Database Migrations
Initialize database tables, foreign keys, and indexes:
```bash
# Using Just:
just db-up

# Or using SeaORM CLI:
sea-orm-cli migrate up --migration-dir src/database/migrations
```

#### 4. Seed Development Fixtures (Optional)
Populate sample users (admin, developer, viewer), organizations, and test projects:
```bash
# Using Just:
just seed

# Or using Cargo:
cargo run --bin seed
```

#### 5. Start the Forge API Server
```bash
# Using Just:
just start

# Or using Cargo:
cargo run
```

The API server binds to `http://localhost:9000`.

#### 6. Verify Service Health & Explore API
- **Liveness Probe**: `curl http://localhost:9000/health/live`  
  Response: `{"status":"ok"}`
- **Readiness Probe**: `curl http://localhost:9000/health/ready`  
  Response: `{"status":"ok","database":"connected","latency_ms":1.2}`
- **Interactive OpenAPI Documentation**: Visit `http://localhost:9000/docs` in your browser.

---

## Running Tests & Quality Gates

Forge maintains automated test suites covering unit logic, database migrations, API contracts, RBAC matrices, and load performance:

```bash
# 1. Run all Rust unit and integration tests across the workspace (383 tests)
cargo test --workspace

# 2. Run formatting check and Clippy strict linter with zero warnings tolerance
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings

# 3. Run full automated Newman API contract collection (93 requests, 279 assertions)
just test-api

# 4. Run multi-role RBAC matrix test across all 6 role & tenancy environments
just test-api-matrix

# 5. Run k6 load test smoke benchmark
just load-test-smoke

# 6. Run Tier-1 (1,000 CVU) concurrent load benchmark
just load-test-1k
```

---

## Documentation Map

Comprehensive technical specifications, Architecture Decision Records (ADRs), benchmark results, and audit reports are maintained under [`docs/`](./docs/README.md):

| Document | File Path | Focus |
| :--- | :--- | :--- |
| **Documentation Hub & Showcase** | [**`docs/README.md`**](./docs/README.md) | Centralized documentation portal with role-based reading journeys |
| **Technical Audit & Assessment** | [**`docs/01-ASSESSMENT.md`**](./docs/01-ASSESSMENT.md) | Technical audit, empirical verification, module ratings, and remaining backlog |
| **Gaps Analysis Report** | [**`docs/02-GAPS.md`**](./docs/02-GAPS.md) | Active architectural deficits, severity ratings, and resolution tracking |
| **Implementation Tasks Roadmap** | [**`docs/03-TASKS.md`**](./docs/03-TASKS.md) | Prioritized incomplete tasks (P1/P2), implementation steps, and acceptance criteria |
| **API Requests Summary** | [**`docs/API_REQUESTS_SUMMARY.md`**](./docs/API_REQUESTS_SUMMARY.md) | Complete reference of 83 endpoints, request DTOs, headers, and responses |
| **API Testing & Postman Guide** | [**`docs/API_TESTING_AND_MOCKING.md`**](./docs/API_TESTING_AND_MOCKING.md) | Postman Collection v2.1 guide, Newman automation, and RBAC matrix |
| **Empirical Benchmarks Report** | [**`docs/BENCHMARKS.md`**](./docs/BENCHMARKS.md) | Multi-tier empirical k6 load test results (1k VUs, latencies, req/s) |
| **Load Testing Guide** | [**`docs/load-testing-guide.md`**](./docs/load-testing-guide.md) | Multi-tier k6 benchmarking framework and OS tuning instructions |
| **System Requirements (SRS)** | [**`docs/system/00-requirements/srs-forge.md`**](./docs/system/00-requirements/srs-forge.md) | IEEE-830 PRD, functional specifications & NFRs |
| **System Architecture** | [**`docs/system/02-architecture/system-architecture.md`**](./docs/system/02-architecture/system-architecture.md) | Modular monolith topology, worker pipelines & security boundaries |
| **Database ERD Specification** | [**`docs/system/03-data/erd.md`**](./docs/system/03-data/erd.md) | 18-table Entity-Relationship model and relational constraints |
| **OpenAPI 3.0.3 Specification** | [**`docs/system/05-api/openapi.yaml`**](./docs/system/05-api/openapi.yaml) | Validated OpenAPI specification covering 83 operations |
| **Architecture Decisions (ADR)** | [**`docs/system/09-adr/README.md`**](./docs/system/09-adr/README.md) | Formal ADR records (SeaORM, RabbitMQ, Docker Daemon, Loki) |
| **Semantic Changelog** | [**`CHANGELOG.md`**](./CHANGELOG.md) | Semantic versioning changelog adhering to Keep a Changelog |

---

## License

This project is licensed under the [MIT License](./LICENSE).
