# Spec Delta: Docker Environments

## MODIFIED Requirements

### Requirement: Production Container Environment
The system SHALL provide an optimized multi-stage production container image (`Dockerfile.prod` or root `Dockerfile`). The build stage MUST compile a release binary with symbols stripped, and the runtime stage MUST use a minimal, hardened base image (such as `debian:bookworm-slim` or distroless) executing as an unprivileged non-root user. The production image MUST include a built-in container healthcheck probe targeting the HTTP `/health/live` endpoint.

#### Scenario: Building minimal production image
- **WHEN** the production Docker image is built
- **THEN** the resulting image MUST contain only the compiled release binary, required runtime libraries, CA certificates, and non-root user configuration, omitting compilers, cargo build caches, and unnecessary tooling.

#### Scenario: Container health verification in production
- **WHEN** the production container starts and enters steady state
- **THEN** the Docker daemon SHALL periodically query `/health/live`, marking the container healthy when HTTP 200 is received and unhealthy upon successive failures.
