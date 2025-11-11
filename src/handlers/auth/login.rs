use crate::{
    config::Config,
    middleware::{auth::BearerToken, validation::validate_request},
    services::auth_service::{AuthService, LoginRequest},
    utils::error::AppError,
};
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;

pub async fn login(
    State(db): State<Arc<DatabaseConnection>>,
    State(config): State<Arc<Config>>,
    _bearer_token: BearerToken,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    let payload = validate_request(payload)?;
    let email = payload.email.clone();
    tracing::info!("Login attempt for email: {}", email);

    let response = AuthService::login(
        &db,
        payload,
        &config.jwt_secret,
        config.jwt_expiration_minutes,
    )
    .await?;

    tracing::info!(
        "Login successful: user_id={}, tenant_id={}",
        response.user.id,
        response.user.tenant_id
    );

    Ok(Json(serde_json::json!({
        "token": response.token,
        "user": response.user,
    })))
}
