use std::time::Instant;

pub struct DeploymentDurations {
    pub clone: i32,
    pub validate: i32,
    pub build: i32,
    pub deploy: i32,
    pub health_check: i32,
    pub start_time: Instant,
}

impl DeploymentDurations {
    pub fn new() -> Self {
        Self {
            clone: 0,
            validate: 0,
            build: 0,
            deploy: 0,
            health_check: 0,
            start_time: Instant::now(),
        }
    }

    pub fn set_start_time(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn set_clone_duration(&mut self) {
        self.clone = self.start_time.elapsed().as_millis() as i32;
    }

    pub fn set_validate_duration(&mut self) {
        self.validate = self.start_time.elapsed().as_millis() as i32;
    }

    pub fn set_build_duration(&mut self) {
        self.build = self.start_time.elapsed().as_millis() as i32;
    }

    pub fn set_deploy_duration(&mut self) {
        self.deploy = self.start_time.elapsed().as_millis() as i32;
    }

    pub fn set_health_check_duration(&mut self) {
        self.health_check = self.start_time.elapsed().as_millis() as i32;
    }
    pub fn total_duration(&self) -> i32 {
        self.start_time.elapsed().as_millis() as i32
    }
    pub fn deploy_duration(&self) -> i32 {
        self.total_duration() - self.build
    }
}
