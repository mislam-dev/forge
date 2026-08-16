use crate::database::DbConfig;
use dotenvy::dotenv;
use std::{
    env as EnvConfig,
    net::{IpAddr, Ipv4Addr},
};

#[derive(Debug, Clone)]
pub struct InfraConnectionUrls {
    pub db: DbConfig,
    pub redis_url: String,
    pub rabbitmq_url: String,
    pub loki_url: String,
}

#[derive(Debug, Clone)]
pub struct Secrets {
    pub jwt_secret: String,
    pub master_encryption_key: String,
    pub jwt_expiry_seconds: u32,        // default 3600
    pub refresh_token_expiry_days: u16, // default 7
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub rust_log: bool,
    pub server_port: u16,    // default 3000
    pub server_host: IpAddr, // default
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub infra: InfraConnectionUrls,
    pub secrets: Secrets,
    pub server_config: ServerConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        Ok(Self {
            secrets: Self::get_secrets()?,
            infra: Self::get_infra()?,
            server_config: Self::get_server_config()?,
        })
    }

    pub fn get_db_config() -> Result<DbConfig, Box<dyn std::error::Error>> {
        let database_url = EnvConfig::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set in the environment")?;

        Ok(DbConfig {
            max_connections: EnvConfig::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            min_connections: EnvConfig::var("DB_MIN_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1),
            connect_timeout_secs: EnvConfig::var("DB_CONNECT_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            idle_timeout_secs: EnvConfig::var("DB_IDLE_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300),
            db_url: database_url,
        })
    }

    fn get_infra() -> Result<InfraConnectionUrls, Box<dyn std::error::Error>> {
        let redis_url = EnvConfig::var("REDIS_URL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(String::from("redis://localhost:6379"));
        let rabbitmq_url = EnvConfig::var("RABBITMQ_URL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(String::from("amqp://username:password@hostname:port/vhost"));

        let loki_url = EnvConfig::var("LOKI_URL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(String::from("http://localhost:3100"));

        Ok(InfraConnectionUrls {
            db: Self::get_db_config()?,
            redis_url,
            rabbitmq_url,
            loki_url,
        })
    }

    fn get_secrets() -> Result<Secrets, Box<dyn std::error::Error>> {
        let jwt_secret = EnvConfig::var("JWT_SECRET")
            .map_err(|_| "JWT_SECRET must be set in the environment!")?;

        let jwt_expiry_seconds: u32 = EnvConfig::var("JWT_EXPIRY_SECONDS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3600);

        let refresh_token_expiry_days: u16 = EnvConfig::var("REFRESH_TOKEN_EXPIRY_DAYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(7);
        let master_encryption_key = EnvConfig::var("MASTER_ENCRYPTION_KEY")
            .map_err(|_| "MASTER_ENCRYPTION_KEY is required!")?;

        Ok(Secrets {
            jwt_secret,
            jwt_expiry_seconds,
            refresh_token_expiry_days,
            master_encryption_key,
        })
    }

    fn get_server_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
        let rust_log: bool = EnvConfig::var("RUST_LOG")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        let server_port: u16 = EnvConfig::var("SERVER_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);
        let server_host: IpAddr = EnvConfig::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string())
            .parse()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        Ok(ServerConfig {
            rust_log,
            server_port,
            server_host,
        })
    }
}
