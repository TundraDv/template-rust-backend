use crate::config::Config;
use axum::http::HeaderValue;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn create_cors_layer(config: &Config) -> CorsLayer {
    let cors_origin = if config.environment == "dev" || config.environment == "development" {
        AllowOrigin::predicate(
            |_origin: &HeaderValue, _request_head: &axum::http::request::Parts| true,
        )
    } else {
        if let Some(frontend_url) = &config.frontend_url {
            match frontend_url.parse::<HeaderValue>() {
                Ok(header_value) => AllowOrigin::exact(header_value),
                Err(_) => {
                    tracing::warn!(
                        "FRONTEND_URL '{}' is invalid, defaulting to no CORS",
                        frontend_url
                    );
                    AllowOrigin::list(vec![])
                }
            }
        } else {
            tracing::warn!("FRONTEND_URL not set in production, defaulting to no CORS");
            AllowOrigin::list(vec![])
        }
    };

    CorsLayer::new()
        .allow_origin(cors_origin)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
            axum::http::Method::PATCH,
        ])
        .allow_headers(tower_http::cors::Any)
        .expose_headers(tower_http::cors::Any)
        .max_age(std::time::Duration::from_secs(3600))
}
