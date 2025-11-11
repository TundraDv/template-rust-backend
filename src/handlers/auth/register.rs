use crate::{
    config::Config,
    middleware::{auth::BearerToken, validation::validate_request},
    services::auth_service::{AuthResponse, AuthService, RegisterRequest},
    utils::error::AppError,
};
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    State(db): State<Arc<DatabaseConnection>>,
    State(config): State<Arc<Config>>,
    _bearer_token: BearerToken,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<Value>, AppError> {
    let payload = validate_request(payload)?;
    tracing::info!("Register request for email: {}", payload.email);
    let response = AuthService::register(
        &db,
        payload,
        &config.jwt_secret,
        config.jwt_expiration_minutes,
    )
    .await?;

    tracing::info!(
        "User registered successfully: user_id={}, tenant_id={}",
        response.user.id,
        response.user.tenant_id
    );
    Ok(Json(serde_json::json!({
        "token": response.token,
        "user": response.user,
    })))
}
