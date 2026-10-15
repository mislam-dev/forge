# Proposal

## Why

The user requested to revert the OpenAPI Schemathesis property-based fuzz testing changes introduced across `openapi-schemathesis-fuzz-testing` and `docker-api-fuzz-testing`, restoring the codebase, documentation, and task runner to their clean prior state.

## What Changes

- **Revert `justfile`**: Remove the Property-Based API Fuzz Testing recipe block (`fuzz-api`, `fuzz-api-auth`, `fuzz-api-report`, `fuzz-api-deep`).
- **Revert `docs/API_TESTING_AND_MOCKING.md`**: Remove Section 6 ("OpenAPI Property-Based Fuzz Testing").
- **Revert `.gitignore`**: Remove the `.schemathesis/` entry.
- **Clean up generated artifacts**: Remove `reports/fuzz/` directory, `schema-coverage.html`, and unarchived planning directories for `openapi-schemathesis-fuzz-testing` and `docker-api-fuzz-testing`.

## Capabilities

### New Capabilities
<!-- None: skip_specs is enabled as this is a pure tooling/configuration revert -->

### Modified Capabilities
<!-- None: no durable specs in openspec/specs/ were modified -->

## Impact

- Restores `justfile`, `docs/API_TESTING_AND_MOCKING.md`, and `.gitignore` to their exact pre-fuzz testing state.
- Removes temporary test reports and unarchived fuzz testing changes.
