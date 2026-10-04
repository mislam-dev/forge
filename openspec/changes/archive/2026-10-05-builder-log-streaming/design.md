# Design

## Context

The Forge build worker executes deployment pipelines across multiple project types (`DockerContainer`, `NodeJs`, `Go`, `Rust`, `Python`, and `StaticFiles`). During pipeline execution, `BuildPipeline` emits high-level phase updates to `LogStream`, which dispatches them to `SseStreamTransporter` (publishing onto RabbitMQ `forge.deployments.log.jobs`) and `LokiStreamTransporter`.

Currently, `DockerBuilder` only partially logs stages, and cloning `LogStream` drops its registered transporters because `Clone` returns an empty vector. Furthermore, inner Docker image build events from Bollard are printed to stdout rather than passed to `LogStream`. The remaining language-specific builders (`NodeJsBuilder`, `GoBuilder`, etc.) do not yet integrate `LogStream`, and `base_builder.rs` remains an unpopulated stub.

See `proposal.md` for motivation and `specs/deployment-logs/spec.md` for requirements.

## Goals / Non-Goals

**Goals:**
- Make `LogStream` safely clonable by wrapping transporters in `Arc<dyn LogStreamTransporter>` so that cloned instances maintain active transporters.
- Forward Bollard Docker image build output stream chunks line-by-line into `LogStream` instead of printing directly to standard out.
- Unify lifecycle logging across all builders (`DockerBuilder`, `NodeJsBuilder`, `GoBuilder`, `PythonBuilder`, `RustBuilder`, and `StaticFilesBuilder`) for validation, file generation, package downloads, image building, deployment, execution, health checks, and cleanup.
- Leverage `base_builder.rs` or common builder helpers to reduce code duplication across builder implementations.
- Ensure pipeline failures capture errors and stream a terminal `is_end: true` log before returning.
- Restore unit test suites in `builders/mod.rs` updated for `BuilderConfig`.

**Non-Goals:**
- Changing RabbitMQ topology, exchange configurations, or message schemas.
- Modifying frontend SSE consumer logic or endpoint contracts in `logs/handlers.rs`.
- Redesigning language-specific file generation templates (`Dockerfile`, `.dockerignore`).

## Decisions

### Decision 1: Thread-safe Transporter Storage in LogStream
- **Choice**: Store transporters as `Vec<Arc<dyn LogStreamTransporter>>` inside `LogStream`, allowing `LogStream` to derive or implement `Clone` by cloning the `Arc` references.
- **Rationale**: `LogStream` needs to be passed into `BuilderConfig` and shared among pipeline phases and inner builders. Cloning must not discard the transporters registered during service startup.
- **Alternatives Considered**:
  - `Box<dyn LogStreamTransporter>` with custom clone trait: Requires cloneable traits for dynamic dispatch, which is clumsy with async traits.
  - Channel-based architecture: Unnecessary complexity given that transporters already implement `Send + Sync + async fn`.

### Decision 2: Streaming Image Build Output to LogStream
- **Choice**: Extend `DockerImage::create` (or provide `create_with_stream`) to accept a logging callback or `Option<&LogStream>` + `deployment_id`. As Bollard yields `BuildInfo` stream chunks, each non-empty line is forwarded to `LogStream` with `step: "Build"`, `level: LogLevel::Info`.
- **Rationale**: The frontend needs granular compiler / Docker build output to display real-time build progress to the user.
- **Alternatives Considered**:
  - Polling container logs post-build: Doesn't work for image build time, which happens prior to container launch.
  - Passing mpsc sender: Viable, but direct `LogStream` delegation is simpler and matches the rest of the worker architecture.

### Decision 3: Base Builder Composition / Helper
- **Choice**: Implement reusable lifecycle utilities in `src/modules/projects/build_worker/builders/base_builder.rs` (or a shared `BaseBuilder` struct) that encapsulates:
  - Docker container deployment (`deploy`)
  - Container execution and start (`run`)
  - Cleanup of temporary project workspace (`cleanup`)
  - Structured step logging helpers (`log_info`, `log_error`)
  Each specific builder (`DockerBuilder`, `NodeJsBuilder`, `GoBuilder`, etc.) embeds `BaseBuilder` or delegates common methods to it, only implementing unique logic for `validate()`, `create_files()`, `download_pkgs()`, and custom build configurations.
- **Rationale**: Currently, `deploy`, `run`, `cleanup`, and `health_check` are identical copy-pasted implementations across all 6 builders. Consolidating this eliminates duplicated boilerplate and guarantees uniform logging across all builders.
- **Alternatives Considered**:
  - Pure trait inheritance with default methods: Harder in Rust when fields (`docker_client`, `log_stream`, `project_path`) must be accessed. Composition via an inner struct is idiomatic and clean.

### Decision 4: Centralized Error Log Catching in BuildPipeline
- **Choice**: In `BuildPipeline::execute_pipeline()`, capture errors from any stage. When a stage returns `Err(e)`, invoke `log_error` on `LogStream` with `is_end: true` and status update to `Failed` before propagating the error.
- **Rationale**: Prevents frontend SSE streams from hanging without receiving failure details or the terminal `"done"` event.

## Risks / Trade-offs

- **[High message volume during dense Docker builds]** → Trim blank whitespace and split multi-line chunks cleanly to avoid spamming empty frames across RabbitMQ.
- **[Unit tests relying on live Docker / RabbitMQ]** → Provide a default/noop constructor for `LogStream` (`LogStream::empty()` or `LogStream::new(vec![])`) so builder unit tests run cleanly without external dependencies.
