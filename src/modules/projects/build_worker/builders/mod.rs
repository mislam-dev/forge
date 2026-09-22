pub mod common;
pub mod docker;
pub mod go;
pub mod nodejs;
pub mod python;
pub mod rust;
pub mod static_files;

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
    use crate::modules::projects::build_worker::traits::ProjectBuilder;

    fn temp_test_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("builder_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn create_test_path(dir: &std::path::Path) -> DeploymentPath {
        let source = dir.join("source");
        std::fs::create_dir_all(&source).unwrap();
        DeploymentPath {
            root: dir.to_path_buf(),
            source,
            build: dir.join("build"),
            logs: dir.join("logs"),
            meta: dir.join("meta.json"),
            static_content: dir.join("static"),
        }
    }

    #[tokio::test]
    async fn test_docker_builder_validate() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("Dockerfile"), "FROM alpine\n").unwrap();

        let builder = DockerBuilder::new(
            path,
            "prj_1".to_string(),
            vec![],
            "usr_1".to_string(),
            "dep_1".to_string(),
            "app".to_string(),
        )
        .unwrap();

        assert!(builder.validate().await.is_ok());
    }

    #[tokio::test]
    async fn test_go_builder_validate_and_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("go.mod"), "module example.com/app\n").unwrap();

        let builder = GoBuilder::new(
            path.clone(),
            "prj_1".to_string(),
            vec![],
            "usr_1".to_string(),
            "dep_1".to_string(),
            "app".to_string(),
        )
        .unwrap();

        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_rust_builder_validate_and_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("Cargo.toml"), "[package]\nname=\"app\"\n").unwrap();

        let builder = RustBuilder::new(
            path.clone(),
            "prj_1".to_string(),
            vec![],
            "usr_1".to_string(),
            "dep_1".to_string(),
            "app".to_string(),
        )
        .unwrap();

        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_python_builder_validate_and_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("requirements.txt"), "flask==3.0.0\n").unwrap();

        let builder = PythonBuilder::new(
            path.clone(),
            "prj_1".to_string(),
            vec![],
            "usr_1".to_string(),
            "dep_1".to_string(),
            "app".to_string(),
        )
        .unwrap();

        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_static_files_builder_create_files() {
        let temp = temp_test_dir();
        let path = create_test_path(&temp);
        std::fs::write(path.source.join("index.html"), "<h1>Hello</h1>").unwrap();

        let builder = StaticFilesBuilder::new(
            path.clone(),
            "prj_1".to_string(),
            vec![],
            "usr_1".to_string(),
            "dep_1".to_string(),
            "app".to_string(),
        )
        .unwrap();

        assert!(builder.validate().await.is_ok());
        assert!(builder.create_files().await.is_ok());
        assert!(path.source.join("Dockerfile").exists());
    }
}
