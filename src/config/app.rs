use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration_minutes: i64,
    pub server_host: String,
    pub server_port: u16,
    pub environment: String,
    pub frontend_url: Option<String>,
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

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8070".to_string())
            .parse::<u16>()
            .map_err(|_| "SERVER_PORT must be a valid number".to_string())?;

        tracing::info!("Server will bind to {}:{}", server_host, server_port);

        let environment = env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "production".to_string())
            .to_lowercase();

        let frontend_url = env::var("FRONTEND_URL").ok();

        if environment == "production" || environment == "prod" {
            if frontend_url.is_none() {
                tracing::warn!(
                    "ENVIRONMENT is production but FRONTEND_URL is not set - CORS will be restrictive"
                );
            } else {
                tracing::info!(
                    "CORS configured for production with FRONTEND_URL: {}",
                    frontend_url.as_ref().unwrap()
                );
            }
        } else {
            tracing::info!("CORS configured for development - allowing all origins");
        }

        Ok(Self {
            jwt_secret,
            jwt_expiration_minutes,
            server_host,
            server_port,
            environment,
            frontend_url,
        })
    }
}
