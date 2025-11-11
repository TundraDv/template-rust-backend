use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub environment: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, String> {
        let url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL environment variable is not set".to_string())?;

        let environment = env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "production".to_string())
            .to_lowercase();

        // Environment-based pool configuration
        let (max_connections, min_connections, connect_timeout, idle_timeout, max_lifetime) =
            if environment == "dev" || environment == "development" {
                // Development: More permissive settings
                (
                    env::var("DB_MAX_CONNECTIONS")
                        .unwrap_or_else(|_| "10".to_string())
                        .parse::<u32>()
                        .unwrap_or(10),
                    env::var("DB_MIN_CONNECTIONS")
                        .unwrap_or_else(|_| "2".to_string())
                        .parse::<u32>()
                        .unwrap_or(2),
                    env::var("DB_CONNECT_TIMEOUT_SECS")
                        .unwrap_or_else(|_| "10".to_string())
                        .parse::<u64>()
                        .unwrap_or(10),
                    env::var("DB_IDLE_TIMEOUT_SECS")
                        .unwrap_or_else(|_| "600".to_string())
                        .parse::<u64>()
                        .unwrap_or(600),
                    env::var("DB_MAX_LIFETIME_SECS")
                        .unwrap_or_else(|_| "1800".to_string())
                        .parse::<u64>()
                        .unwrap_or(1800),
                )
            } else {
                // Production: Stricter, optimized settings
                (
                    env::var("DB_MAX_CONNECTIONS")
                        .unwrap_or_else(|_| "20".to_string())
                        .parse::<u32>()
                        .unwrap_or(20),
                    env::var("DB_MIN_CONNECTIONS")
                        .unwrap_or_else(|_| "5".to_string())
                        .parse::<u32>()
                        .unwrap_or(5),
                    env::var("DB_CONNECT_TIMEOUT_SECS")
                        .unwrap_or_else(|_| "5".to_string())
                        .parse::<u64>()
                        .unwrap_or(5),
                    env::var("DB_IDLE_TIMEOUT_SECS")
                        .unwrap_or_else(|_| "300".to_string())
                        .parse::<u64>()
                        .unwrap_or(300),
                    env::var("DB_MAX_LIFETIME_SECS")
                        .unwrap_or_else(|_| "1800".to_string())
                        .parse::<u64>()
                        .unwrap_or(1800),
                )
            };

        tracing::info!(
            "Database pool configuration (environment: {}): max={}, min={}, connect_timeout={}s, idle_timeout={}s, max_lifetime={}s",
            environment,
            max_connections,
            min_connections,
            connect_timeout,
            idle_timeout,
            max_lifetime
        );

        Ok(Self {
            url,
            environment,
            max_connections,
            min_connections,
            connect_timeout_secs: connect_timeout,
            idle_timeout_secs: idle_timeout,
            max_lifetime_secs: max_lifetime,
        })
    }

    pub async fn connect(&self) -> Result<DatabaseConnection, sea_orm::DbErr> {
        let mut opt = sea_orm::ConnectOptions::new(&self.url);

        // Connection pool settings
        opt.max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect_timeout(Duration::from_secs(self.connect_timeout_secs))
            .idle_timeout(Duration::from_secs(self.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(self.max_lifetime_secs));

        // Logging configuration
        if self.environment == "dev" || self.environment == "development" {
            opt.sqlx_logging(true);
            opt.sqlx_logging_level(log::LevelFilter::Debug);
        } else {
            opt.sqlx_logging(false);
        }

        tracing::info!(
            "Connecting to database with pool: max={}, min={}",
            self.max_connections,
            self.min_connections
        );

        Database::connect(opt).await
    }
}
