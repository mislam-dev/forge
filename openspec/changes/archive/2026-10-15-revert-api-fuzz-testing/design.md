# Design

## Context

See `proposal.md`. The user evaluated Schemathesis property-based fuzz testing and requested a complete revert of all related changes, leaving the original Postman API tests and k6 load tests intact.

## Goals / Non-Goals

**Goals:**
- Surgically remove the Property-Based API Fuzz Testing recipe block from `justfile`.
- Revert `docs/API_TESTING_AND_MOCKING.md` to remove Section 6.
- Revert `.gitignore` by removing `.schemathesis/`.
- Delete generated report artifacts (`reports/fuzz/`, `schema-coverage.html`).
- Remove the temporary unarchived changes `openapi-schemathesis-fuzz-testing` and `docker-api-fuzz-testing` from `openspec/changes/`.

**Non-Goals:**
- Affecting Newman/Postman contract test suites or k6 load tests.
- Reverting any unrelated workspace changes.

## Decisions

### Decision 1: Surgical File Restoration
- Revert the files (`justfile`, `docs/API_TESTING_AND_MOCKING.md`, `.gitignore`) using explicit block replacement/git restore so that unrelated lines (like `docker compose` dev profile flag) are preserved as intended.

### Decision 2: OpenSpec Change Cleanup
- Remove `openspec/changes/openapi-schemathesis-fuzz-testing` and `openspec/changes/docker-api-fuzz-testing` so `openspec list` only tracks active, valid changes.

## Risks / Trade-offs

- **[Risk] Unintended removal of shared recipes**
  → *Mitigation*: Verify `just --list` shows all existing Postman, database, and load testing recipes intact after removal.
