# Tasks

## 1. Transporter Preservation in LogStream

- [x] 1.1 Update `LogStream` in `src/modules/projects/build_worker/log_stream/log_stream.rs` to store transporters as thread-safe shared references (`Vec<Arc<dyn LogStreamTransporter>>` or `Arc<Vec<Box<dyn LogStreamTransporter>>>`) so `Clone` preserves active transporters instead of dropping them. Verify with unit test that cloned `LogStream` instances retain transporters.
- [x] 1.2 Implement helper logging methods on `LogStream` (such as `log_info` and `log_error`) to simplify sending structured `LogItem` instances with step name, timestamp, and end status. Verify with unit tests.

## 2. Docker Image Build Output Streaming

- [x] 2.1 Update `DockerImage::create` in `src/modules/projects/build_worker/docker_client/image.rs` to support streaming Docker build events (stdout/stderr lines) to a log handler or `LogStream` instead of printing to stdout. Verify that build events are routed to the callback.
- [x] 2.2 Wire image build event streaming in `src/modules/projects/build_worker/builders/docker/builder/builder.rs` so that inner image compilation logs are streamed in real time to `LogStream`. Verify using `cargo check`.

## 3. Base Builder and Builder Suite Integration

- [x] 3.1 Implement `BaseBuilder` in `src/modules/projects/build_worker/builders/base_builder.rs` providing shared lifecycle implementations (`deploy`, `run`, `health_check`, `cleanup`) and structured logging helpers parameterized by `BuilderConfig`. Verify `base_builder.rs` compiles cleanly.
- [x] 3.2 Refactor `DockerBuilder` in `src/modules/projects/build_worker/builders/docker/docker.rs` to leverage `BaseBuilder` for common lifecycle methods and complete logging coverage across all phases (`validate`, `build`, `deploy`, `run`, `health_check`, `cleanup`).
- [x] 3.3 Integrate `LogStream` and `BaseBuilder` into `NodeJsBuilder` (`builders/nodejs/nodejs.rs`) and wire image build logging into `nodejs/builder/builder.rs`.
- [x] 3.4 Integrate `LogStream` and `BaseBuilder` into `GoBuilder` (`builders/go/go.rs`) and wire image build logging into `go/builder/builder.rs`.
- [x] 3.5 Integrate `LogStream` and `BaseBuilder` into `PythonBuilder` (`builders/python/python.rs`) and wire image build logging into `python/builder/builder.rs`.
- [x] 3.6 Integrate `LogStream` and `BaseBuilder` into `RustBuilder` (`builders/rust/rust.rs`) and wire image build logging into `rust/builder/builder.rs`.
- [x] 3.7 Integrate `LogStream` and `BaseBuilder` into `StaticFilesBuilder` (`builders/static_files/static_files.rs`) and wire image build logging into `static_files/builder/builder.rs`.

## 4. Pipeline Error Handling & Test Verification

- [x] 4.1 Update `BuildPipeline::execute_pipeline()` in `src/modules/projects/build_worker/pipeline.rs` to intercept any stage error, emit a structured ERROR log item with `is_end: true`, and transition deployment status to `Failed` before propagating the error.
- [x] 4.2 Restore and update unit tests in `src/modules/projects/build_worker/builders/mod.rs` to construct builders via `BuilderConfig` and test validation, file creation, and log dispatch.
- [x] 4.3 Run `cargo check` and `cargo test` across the workspace to ensure zero compilation warnings and all unit tests pass.
