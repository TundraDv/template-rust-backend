use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration_minutes: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let jwt_secret = env::var("BEARER_TOKEN")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

        tracing::info!("BEARER_TOKEN loaded from environment");
        tracing::debug!("BEARER_TOKEN length: {}", jwt_secret.len());
        tracing::debug!(
            "BEARER_TOKEN (first 10 chars): {}",
            &jwt_secret.chars().take(10).collect::<String>()
        );
        if jwt_secret == "your-secret-key-change-in-production" {
            tracing::warn!("Using default BEARER_TOKEN - this should be changed in production!");
        }

        let jwt_expiration_minutes = env::var("JWT_EXPIRATION_MINUTES")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| "JWT_EXPIRATION_MINUTES must be a valid number".to_string())?;

        tracing::info!("JWT expiration minutes: {}", jwt_expiration_minutes);

        Ok(Self {
            jwt_secret,
            jwt_expiration_minutes,
        })
    }
}
