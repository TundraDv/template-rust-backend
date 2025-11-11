use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, String> {
        let url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL environment variable is not set".to_string())?;

        Ok(Self { url })
    }

    pub async fn connect(&self) -> Result<DatabaseConnection, sea_orm::DbErr> {
        let mut opt = sea_orm::ConnectOptions::new(&self.url);
        opt.sqlx_logging(true);
        opt.sqlx_logging_level(log::LevelFilter::Debug);
        Database::connect(opt).await
    }
}
