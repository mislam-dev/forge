use bollard::Docker;
pub struct NodejsDockerClient;

impl NodejsDockerClient {
    pub fn new() -> Docker {
        let docker = Docker::connect_with_local_defaults().unwrap();

        docker
    }
}
