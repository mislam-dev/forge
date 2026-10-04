pub mod base_builder;
pub mod common;
pub mod docker;
pub mod go;
pub mod nodejs;
pub mod python;
pub mod rust;
pub mod static_files;

#[allow(unused_imports)]
pub use base_builder::BaseBuilder;
pub use docker::DockerBuilder;
pub use go::GoBuilder;
pub use nodejs::NodeJsBuilder;
pub use python::PythonBuilder;
pub use rust::RustBuilder;
pub use static_files::StaticFilesBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::projects::build_worker::DeploymentPath;
    use crate::modules::projects::build_worker::log_stream::LogStream;
    use crate::modules::projects::build_worker::traits::{BuilderConfig, ProjectBuilder};

    fn temp_test_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("builder_test_{}", uuid::Uuid::new_v4()));
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    fn create_test_path(dir: &std::path::Path) -> DeploymentPath {
        let source = dir.join("source");
        let _ = std::fs::create_dir_all(&source);
        DeploymentPath {
            root: dir.to_path_buf(),
            source,
            build: dir.join("build"),
            logs: dir.join("logs"),
            meta: dir.join("meta.json"),
            static_content: dir.join("static"),
        }
    }

    fn create_test_config(path: DeploymentPath) -> BuilderConfig {
        BuilderConfig {
            project_path: path,
            project_id: "prj_1".to_string(),
            env_vars: vec![],
            org_or_user_id: "usr_1".to_string(),
            deployment_id: "dep_1".to_string(),
            app_name: "app".to_string(),
            log_stream: LogStream::empty(),
        }
    }

    #[tokio::test]
    async fn test_docker_builder_validate() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("Dockerfile"), "FROM alpine\n").unwrap();

        let builder = DockerBuilder::new(create_test_config(path)).unwrap();
        assert!(builder.validate().await.is_ok());
    }

    #[tokio::test]
    async fn test_go_builder_validate_and_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("go.mod"), "module example.com/app\n").unwrap();

        let builder = GoBuilder::new(create_test_config(path.clone())).unwrap();
        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_rust_builder_validate_and_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("Cargo.toml"), "[package]\nname=\"app\"\n").unwrap();

        let builder = RustBuilder::new(create_test_config(path.clone())).unwrap();
        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_python_builder_validate_and_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("requirements.txt"), "flask==3.0.0\n").unwrap();

        let builder = PythonBuilder::new(create_test_config(path.clone())).unwrap();
        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_static_files_builder_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("index.html"), "<h1>Hello</h1>").unwrap();

        let builder = StaticFilesBuilder::new(create_test_config(path.clone())).unwrap();
        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_builder_logs_dispatched() {
        use crate::modules::projects::build_worker::log_stream::{LogItem, LogStreamTransporter};
        use async_trait::async_trait;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct MockTransporter {
            count: AtomicUsize,
        }
        #[async_trait]
        impl LogStreamTransporter for MockTransporter {
            async fn store(&self, _item: LogItem) {}
            async fn stream(&self, _item: LogItem) {
                self.count.fetch_add(1, Ordering::SeqCst);
            }
        }

        let transporter = Arc::new(MockTransporter {
            count: AtomicUsize::new(0),
        });
        let log_stream = LogStream::new(vec![transporter.clone()]);

        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("Dockerfile"), "FROM alpine\n").unwrap();

        let mut config = create_test_config(path);
        config.log_stream = log_stream;

        let builder = DockerBuilder::new(config).unwrap();
        assert!(builder.validate().await.is_ok());
        assert!(transporter.count.load(Ordering::SeqCst) > 0);
    }
}
