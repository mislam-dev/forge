# Forge — System-Level Technical Documentation

> **Document Set Version:** 2.0  
> **Status:** Final  
> **Platform:** Forge — Developer Deployment & Project Management Platform  
> **Author:** Senior Backend Architecture & Technical Documentation Team

---

## About This Documentation Set

This directory contains the **project-level (system-level) technical documentation** for the Forge platform. It synthesizes and integrates all module-level documentation into a coherent architectural view of the entire system.

> **Important:** These documents describe the *system*, not individual modules. For module-specific details, consult the individual module documentation files in the `docs/modules/` directory.

---

## Document Map

| # | Document | Description |
|---|----------|-------------|
| 00 | [System Requirements (SRS)](./00-requirements/srs-forge.md) | Platform PRD and IEEE-830 style System Requirements Specification |
| 01 | [System Overview](./01-overview/system-overview.md) | Platform purpose, goals, architectural philosophy, and actor catalog |
| 02 | [Module Catalog & Dependency Map](./02-architecture/module-catalog.md) | All modules, their types, cross-module dependencies, and ownership |
| 03 | [System Architecture](./02-architecture/system-architecture.md) | Layered architecture, deployment model, communication patterns |
| 04 | [Cross-Module Data Flow](./02-architecture/cross-module-data-flow.md) | How data moves across module boundaries for key platform workflows |
| 05 | [Database Schema Overview](./03-data/database-schema-overview.md) | Consolidated schema reference: all tables, relations, and ownership |
| 06 | [Database ERD Specification](./03-data/erd.md) | Complete 18-table Entity-Relationship Diagram and DDL constraints |
| 07 | [Security Architecture](./04-security/security-architecture.md) | Authentication, authorization, encryption, and trust boundaries |
| 08 | [API Surface Map](./05-api/api-surface-map.md) | Complete endpoint inventory across all modules |
| 09 | [API Reference Documentation](./05-api/api-documentation.md) | Human-readable API documentation covering endpoints, parameters, and error codes |
| 10 | [OpenAPI 3.0 Specification](./05-api/openapi.yaml) | Validated, machine-readable OpenAPI 3.0.3 spec (83 operations, 0 lint errors) |
| 11 | [Internal Integration Points](./06-integrations/internal-integration-points.md) | Async workers, queues, pub/sub, and internal service contracts |
| 12 | [Observability & Health](./07-operations/observability-and-health.md) | Health probes, liveness checks, and observability architecture |
| 13 | [Documentation Evaluation Report](./08-analysis/documentation-evaluation-report.md) | Pure documentation quality, completeness, and architecture audit |
| 14 | [ERD & API Validation Report](./08-analysis/erd-api-validation-report.md) | 18-table database ERD and 83-endpoint API traceability report |
| 15 | [OpenAPI Consistency Audit Report](./08-analysis/openapi-module-consistency-report.md) | OpenAPI specification remediation and lint validation report |
| 16 | [Architecture Decision Records (ADR)](./09-adr/README.md) | Index of formal ADR records (PostgreSQL, SeaORM, Redis, RabbitMQ, Loki) |

---

## Document Conventions

- All cross-references use relative links within this `docs/system/` tree.
- Mermaid diagrams are used throughout for architecture and flow visualization.
- Tables are used to express structured relationships (e.g., dependency matrices, schema columns).
- Module documentation files under `docs/modules/` are the authoritative source for module-specific behavioral details; this set documents system-level integration and architecture only.

---

**Last Updated:** 2026-08-13  
**Maintainer:** Backend Architecture Team
