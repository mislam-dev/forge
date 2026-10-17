# Spec Delta

## Purpose

Defines repository-level documentation and open-source licensing contracts for Forge, ensuring transparent architectural communication, streamlined developer onboarding, and formal open-source governance.

## ADDED Requirements

### Requirement: Production-Grade Root README Specification
The repository SHALL maintain an authoritative, production-grade `README.md` at the project root documenting the Forge platform. The document MUST contain:
1. Status and technology badges including Rust 2024 edition, MIT License, Newman API test suite (93/93 passing), Docker readiness, and OpenAPI 3.0.3 specification.
2. A concise problem statement and elevator pitch describing Forge as an event-driven, self-hosted developer platform (PaaS) built in Rust for containerized application deployments.
3. An architectural diagram illustrating the request lifecycle and asynchronous build pipeline across the Axum HTTP layer, SeaORM PostgreSQL persistence, RabbitMQ message broker, Bollard Docker engine integration, and Grafana Loki log aggregation.
4. A Quick Start guide with executable commands for starting backing infrastructure via Docker Compose, applying database migrations, starting the server, and exploring the interactive OpenAPI Swagger UI.
5. Technical architectural highlights detailing Tokio asynchronous execution, SeaORM connection pooling, relational multi-tenant RBAC, and Bollard container sandboxing.
6. Empirically verified performance benchmark metrics derived from k6 load testing (such as sub-4ms median response times and 265+ req/s throughput).
7. Navigation links pointing to formal repository documentation including the IEEE-830 SRS, ADR records, OpenAPI specification, and load testing guide.

#### Scenario: Browsing repository on GitHub
- **WHEN** a prospective developer or recruiter visits the root repository page on GitHub
- **THEN** the `README.md` SHALL render cleanly formatted markdown with working badges, readable architecture diagrams, accurate benchmark numbers, and working cross-reference links.

#### Scenario: Executing local quickstart onboarding
- **WHEN** a developer follows the Quick Start instructions on a fresh clone with Docker installed
- **THEN** the documented commands (`docker compose up -d`, database migration, `cargo run`) SHALL successfully boot the platform and expose healthy `/health` and `/swagger-ui` endpoints.

#### Scenario: Inspecting architectural highlights and benchmarks
- **WHEN** an evaluator reviews the architecture and performance sections
- **THEN** the document SHALL provide concrete technical explanations of the modular monolith design, async worker concurrency, and empirical k6 latency statistics without placeholder tokens.

### Requirement: Open Source MIT License Declaration
The repository SHALL establish formal open-source licensing terms under the MIT License. The system MUST provide an authoritative `LICENSE` file at the repository root containing the standard MIT License legal text with appropriate copyright attribution. Furthermore, `Cargo.toml` package metadata MUST declare `license = "MIT"` to enable automated license detection by package managers, security scanners, and repository indexers.

#### Scenario: Validating root license file
- **WHEN** an automated license scanner or user inspects the repository root
- **THEN** a `LICENSE` file containing the standard MIT License text SHALL be present and readable.

#### Scenario: Validating Cargo package license metadata
- **WHEN** `cargo read-manifest` or `cargo metadata` parses `Cargo.toml`
- **THEN** the package license field SHALL evaluate to `"MIT"`.
