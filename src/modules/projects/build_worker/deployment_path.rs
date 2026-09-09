use std::{fs, io, path::PathBuf};

pub enum OwnerType {
    Org,
    User,
}
pub struct Owner {
    pub id: String,
    pub owner_type: OwnerType,
}

impl Owner {
    pub fn user(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            owner_type: OwnerType::User,
        }
    }
    pub fn org(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            owner_type: OwnerType::Org,
        }
    }

    pub fn segment(&self) -> &'static str {
        match self.owner_type {
            OwnerType::User => "users",
            OwnerType::Org => "orgs",
        }
    }
}

pub struct DeploymentPath {
    pub root: PathBuf,
    pub source: PathBuf,
    pub build: PathBuf,
    pub logs: PathBuf,
    pub meta: PathBuf,
    pub static_content: PathBuf,
}

const META_FILE_NAME: &str = "meta.json";
const SOURCE_DIR: &str = "source";
const BUILD_DIR: &str = "build";
const LOGS_DIR: &str = "logs";
const STATIC_DIR: &str = "static";
const DEPLOYMENTS_DIR: &str = "deployments";

pub struct DeploymentPathDTO {
    pub project_id: String,
    pub deployment_id: String,
    pub owner: Owner,
    pub base: Option<String>,
}

impl DeploymentPath {
    pub fn new(dto: DeploymentPathDTO) -> Self {
        let base = dto.base.as_deref().unwrap_or(DEPLOYMENTS_DIR);

        let root = PathBuf::from(base)
            .join(dto.owner.segment())
            .join(dto.owner.id)
            .join(dto.project_id)
            .join(dto.deployment_id);

        Self {
            source: root.join(SOURCE_DIR),
            build: root.join(BUILD_DIR),
            logs: root.join(LOGS_DIR),
            meta: root.join(META_FILE_NAME),
            static_content: root.join(STATIC_DIR),
            root,
        }
    }

    pub fn create_all(&self) -> Result<(), io::Error> {
        fs::create_dir_all(&self.source)?;
        fs::create_dir_all(&self.build)?;
        fs::create_dir_all(&self.logs)?;
        fs::create_dir_all(&self.static_content)?;

        Ok(())
    }

    pub fn clean_deployment(
        owner: Owner,
        project_id: String,
        deployment_id: String,
        base: Option<&str>,
    ) -> Result<(), io::Error> {
        let base = base.unwrap_or(DEPLOYMENTS_DIR);

        let root = PathBuf::from(base)
            .join(owner.segment())
            .join(owner.id)
            .join(project_id)
            .join(deployment_id);

        fs::remove_dir_all(root)?;

        Ok(())
    }

    pub fn clean_project(
        owner: &Owner,
        project_id: &String,
        base: Option<&str>,
    ) -> Result<(), io::Error> {
        let base = base.unwrap_or(DEPLOYMENTS_DIR);

        let root = PathBuf::from(base)
            .join(owner.segment())
            .join(owner.id.clone())
            .join(project_id);

        fs::remove_dir_all(root)?;

        Ok(())
    }

    pub fn clean_owner(owner: &Owner, base: Option<&str>) -> Result<(), io::Error> {
        let base = base.unwrap_or(DEPLOYMENTS_DIR);

        let root = PathBuf::from(base)
            .join(owner.segment())
            .join(owner.id.clone());

        fs::remove_dir_all(root)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_user_and_org() {
        let user_owner = Owner::user("usr_123");
        assert_eq!(user_owner.id, "usr_123");
        assert_eq!(user_owner.segment(), "users");

        let org_owner = Owner::org("org_456");
        assert_eq!(org_owner.id, "org_456");
        assert_eq!(org_owner.segment(), "orgs");
    }

    #[test]
    fn test_deployment_path_construction() {
        let dto = DeploymentPathDTO {
            project_id: "prj_abc".to_string(),
            deployment_id: "dep_xyz".to_string(),
            owner: Owner::user("usr_123"),
            base: Some("/tmp/test_deployments".to_string()),
        };

        let path = DeploymentPath::new(dto);
        assert_eq!(
            path.root,
            PathBuf::from("/tmp/test_deployments/users/usr_123/prj_abc/dep_xyz")
        );
        assert_eq!(path.source, path.root.join("source"));
        assert_eq!(path.build, path.root.join("build"));
        assert_eq!(path.logs, path.root.join("logs"));
        assert_eq!(path.meta, path.root.join("meta.json"));
        assert_eq!(path.static_content, path.root.join("static"));
    }

    #[test]
    fn test_deployment_path_create_all_and_clean_deployment() {
        let temp_dir = std::env::temp_dir().join(format!("forge_dp_test_{}", uuid::Uuid::new_v4()));
        let base_path = temp_dir.to_str().unwrap().to_string();

        let dto = DeploymentPathDTO {
            project_id: "prj_1".to_string(),
            deployment_id: "dep_1".to_string(),
            owner: Owner::user("usr_1"),
            base: Some(base_path.clone()),
        };

        let path = DeploymentPath::new(dto);
        path.create_all().expect("create_all should succeed");

        assert!(path.source.exists());
        assert!(path.build.exists());
        assert!(path.logs.exists());
        assert!(path.static_content.exists());

        // clean deployment
        DeploymentPath::clean_deployment(
            Owner::user("usr_1"),
            "prj_1".to_string(),
            "dep_1".to_string(),
            Some(&base_path),
        )
        .expect("clean_deployment should succeed");

        assert!(!path.root.exists());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_deployment_path_clean_project_and_clean_owner() {
        let temp_dir = std::env::temp_dir().join(format!("forge_dp_test_{}", uuid::Uuid::new_v4()));
        let base_path = temp_dir.to_str().unwrap().to_string();

        let owner = Owner::org("org_1");
        let project_id = "prj_2".to_string();

        let dto = DeploymentPathDTO {
            project_id: project_id.clone(),
            deployment_id: "dep_2".to_string(),
            owner: Owner::org("org_1"),
            base: Some(base_path.clone()),
        };

        let path = DeploymentPath::new(dto);
        path.create_all().expect("create_all should succeed");
        assert!(path.root.exists());

        DeploymentPath::clean_project(&owner, &project_id, Some(&base_path))
            .expect("clean_project should succeed");
        assert!(!path.root.exists());

        // Recreate and test clean_owner
        let dto2 = DeploymentPathDTO {
            project_id: project_id.clone(),
            deployment_id: "dep_3".to_string(),
            owner: Owner::org("org_1"),
            base: Some(base_path.clone()),
        };
        let path2 = DeploymentPath::new(dto2);
        path2.create_all().expect("create_all should succeed");

        DeploymentPath::clean_owner(&owner, Some(&base_path)).expect("clean_owner should succeed");
        assert!(!temp_dir.join("orgs").join("org_1").exists());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
