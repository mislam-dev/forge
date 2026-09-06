use core::fmt;

#[derive(Clone)]
pub struct RabbitMqConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub virtual_host: String,
    pub connection_timeout: u64,
    pub max_retries: u32,
}

impl RabbitMqConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("RABBITMQ_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("RABBITMQ_PORT")
            .unwrap_or_else(|_| "5672".to_string())
            .parse()
            .unwrap();
        let username = std::env::var("RABBITMQ_USERNAME").unwrap_or_else(|_| "guest".to_string());
        let password = std::env::var("RABBITMQ_PASSWORD").unwrap_or_else(|_| "guest".to_string());
        let virtual_host =
            std::env::var("RABBITMQ_VIRTUAL_HOST").unwrap_or_else(|_| "/".to_string());
        let connection_timeout = std::env::var("RABBITMQ_CONNECTION_TIMEOUT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap();
        let max_retries = std::env::var("RABBITMQ_MAX_RETRIES")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap();
        Self {
            host,
            port,
            username,
            password,
            virtual_host,
            connection_timeout,
            max_retries,
        }
    }

    pub fn url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.virtual_host,
        )
    }
}

impl fmt::Debug for RabbitMqConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RabbitMqConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("virtual_host", &self.virtual_host)
            .field("connection_timeout", &self.connection_timeout)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}
