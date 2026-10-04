# Proposal

## Why

During project deployment builds, users require real-time visibility into the build and container deployment lifecycle (validation, package fetching, docker image compilation, container startup, and health checking) via Server-Sent Events (SSE). Currently, `DockerBuilder` only partially integrates `LogStream`, drops transporter instances upon cloning due to an empty clone implementation, does not forward granular Docker build output streams to SSE, and the remaining builders (`NodeJsBuilder`, `GoBuilder`, `PythonBuilder`, `RustBuilder`, and `StaticFilesBuilder`) lack `LogStream` integration and lifecycle logging entirely.

## What Changes

- **Transporter Preservation in LogStream**: Fix `LogStream` cloning so that transporters (such as `SseStreamTransporter` and `LokiStreamTransporter`) are safely shared across clones (using thread-safe reference counting like `Arc`) rather than discarded.
- **Docker Image Build Output Streaming**: Connect Bollard Docker image build output stream chunks directly into `LogStream`, enabling line-by-line build output streaming to the frontend in real time instead of printing only to stdout.
- **Consistent Builder Logging Architecture**: Standardize structured log emitting across all builder stages (`validate`, `create_files`, `download_pkgs`, `build`, `deploy`, `run`, `health_check`, `cleanup`), either through a reusable base builder / common helper or uniform builder implementation.
- **Full Builder Suite Integration**: Integrate `LogStream` into all builders (`DockerBuilder`, `NodeJsBuilder`, `GoBuilder`, `PythonBuilder`, `RustBuilder`, and `StaticFilesBuilder`) accepting `BuilderConfig`.
- **Error and Termination Handling**: Ensure build errors and failure states emit error logs to `LogStream` before exiting, and mark terminal stages accurately.
- **Test Suite Updates**: Restore and update unit tests in `builders/mod.rs` to construct builders with `BuilderConfig` and verify log streaming behavior.

## Capabilities

### New Capabilities
- `deployment-logs`: Real-time streaming and structured progression logging across all build worker builder pipelines via LogStream and SSE.

### Modified Capabilities
*(None - existing project-deployments and other capabilities maintain their current HTTP API contracts)*

## Impact

- **Affected Modules**:
  - `src/modules/projects/build_worker/log_stream/log_stream.rs` and `traits.rs`
  - `src/modules/projects/build_worker/docker_client/image.rs`
  - `src/modules/projects/build_worker/builders/base_builder.rs` / `builders/common/`
  - `src/modules/projects/build_worker/builders/docker/` (`docker.rs`, `builder/builder.rs`)
  - `src/modules/projects/build_worker/builders/nodejs/`
  - `src/modules/projects/build_worker/builders/go/`
  - `src/modules/projects/build_worker/builders/python/`
  - `src/modules/projects/build_worker/builders/rust/`
  - `src/modules/projects/build_worker/builders/static_files/`
  - `src/modules/projects/build_worker/builders/mod.rs`
- **Dependencies**: No new external crate dependencies required; leverages existing `tokio`, `bollard`, `bytes`, `futures_util` / `tokio_stream`, and `serde_json`.
- **Breaking Changes**: None to external APIs; internal builder constructor signatures are unified under `BuilderConfig`.
