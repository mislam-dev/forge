# Proposal: GitHub Actions CI Pipeline

## Why

Forge currently lacks automated continuous integration to enforce code quality, compilation checks, test suite execution, and container build integrity on code contributions. Establishing an automated GitHub Actions CI pipeline triggered on pull requests and pushes to `main` ensures that formatting, clippy lints, unit/integration tests with backing services, and container builds are continuously validated before integration.

## What Changes

- Add GitHub Actions CI workflow in `.github/workflows/ci.yml`.
- Configure triggers restricted strictly to `pull_request` targeting `main` and direct `push` on `main`, with no additional event triggers.
- Configure quality gate jobs:
  - `rustfmt`: Verify formatting across all workspace files with `cargo fmt --check`.
  - `clippy`: Run compiler linter with `cargo clippy --all-targets --all-features -- -D warnings`.
  - `test`: Run workspace unit and integration tests (`cargo test --workspace`) with PostgreSQL and RabbitMQ service containers.
  - `docker-build`: Validate multi-stage Docker build caching (`cargo-chef`) and image build integrity against the root `Dockerfile`.

## Capabilities

### New Capabilities

- `github-actions-ci`: Automates CI validation on GitHub Actions triggered on pull requests to `main` and direct pushes to `main`, validating formatting, clippy lints, workspace tests with service containers, and container buildability.

### Modified Capabilities

*(None)*

## Impact

- **New files**: `.github/workflows/ci.yml`.
- **APIs and Runtime**: Zero runtime impact on application binaries or runtime APIs.
- **Dependencies**: Uses standard GitHub Actions runners (`ubuntu-latest`), actions (`actions/checkout@v4`, `dtolnay/rust-toolchain` or standard rust setup, cache actions, and Docker buildx actions).
- **Documentation**: Fulfills Task T-009 in `docs/03-TASKS.md`.
