# release-management Specification

## Purpose

Defines repository-level semantic release management standards, Keep a Changelog documentation conventions, and git release tagging procedures for the Forge platform.

## Requirements

### Requirement: Keep a Changelog Release Record
The repository SHALL maintain an authoritative `CHANGELOG.md` at the project root conforming strictly to Keep a Changelog (1.1.0) and Semantic Versioning (SemVer 2.0.0) conventions. The document MUST:
1. Provide an introductory description explaining the format and versioning principles.
2. Maintain an `[Unreleased]` comparison link section for tracking ongoing contributions.
3. Provide a release entry for `[0.1.0] - 2026-10-17` containing structured subheadings: `Added`, `Changed`, `Fixed`, and `Security`.
4. Document initial production milestones including 5-tier relational RBAC, Bollard Docker engine sandboxing, RabbitMQ competing worker pipelines, SeaORM 2.0 persistence, authenticated AES-256-GCM encryption, OpenAPI 3.0.3 documentation, and multi-tier k6 load testing suites.

#### Scenario: Inspecting v0.1.0 release entry
- **WHEN** an evaluator or package manager views `CHANGELOG.md`
- **THEN** the document cleanly details the full functional capabilities, architectural fixes, and security enhancements shipped in `v0.1.0`.

### Requirement: Git Release Tagging
The repository SHALL establish an immutable annotated git release tag `v0.1.0` associated with the release commit. The git tag MUST be visible via standard git tools (`git tag -l`) and correspond to the version declared in `Cargo.toml`.

#### Scenario: Verifying git tag
- **WHEN** `git tag -l` is executed on the repository
- **THEN** tag `v0.1.0` is listed and points to the corresponding release commit.
