# Forge — Architecture Decision Records (ADRs)

> **Document Set Version:** 1.0  
> **Status:** Active  
> **Section:** 09 — Architecture Decision Records

---

## Overview

This directory contains the **Architecture Decision Records (ADRs)** for the Forge Platform. ADRs capture key architectural and technology choices, their rationale, context, constraints, integration patterns, and consequences.

---

## Index of Architecture Decision Records

| ID                                                   | Title                               | Status   | Decision Type                           | Date       |
| ---------------------------------------------------- | ----------------------------------- | -------- | --------------------------------------- | ---------- |
| [ADR-001](./ADR-001-postgresql-primary-database.md)  | **PostgreSQL as Primary Database**                   | Accepted | Infrastructure / Data Persistence        | 2026-08-13 |
| [ADR-002](./ADR-002-seaorm-database-access-layer.md) | **SeaORM as Database Access Layer**                  | Accepted | Architecture / Data Access               | 2026-08-13 |
| [ADR-003](./ADR-003-redis-caching-layer.md)          | **Redis as Caching Layer**                           | Accepted | Architecture / Caching                   | 2026-08-13 |
| [ADR-004](./ADR-004-rabbitmq-message-broker.md)      | **RabbitMQ as Message Broker for Workflows**         | Accepted | Architecture / Messaging & Infrastructure | 2026-08-13 |
| [ADR-005](./ADR-005-use-loki-for-centralized-logging.md)| **Use Grafana Loki for Centralized Application & Build Logging** | Accepted | Infrastructure / Observability / Logging | 2026-08-13 |

---

## Document Conventions

- ADRs are numbered sequentially (`ADR-001`, `ADR-002`, `ADR-003`, `ADR-004`, `ADR-005`).
- All ADRs reference concrete entities, schemas, and architecture contracts defined in [`docs/system/`](../README.md).
- PostgreSQL is the sole authoritative source of truth for persistent platform business data across all records.
- Redis is strictly reserved for in-memory read caching, rate limiting, and session revocation.
- RabbitMQ manages all asynchronous background workflows, job queueing, and real-time log event streaming.
- Grafana Loki is the centralized logging platform for all operational logs (application logs and build/deployment logs).
