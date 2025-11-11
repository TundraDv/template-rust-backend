use crate::{
    config::Config,
    middleware::auth::Claims,
    services::auth_service::{AuthResponse, AuthService},
    utils::error::AppError,
};
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "Authentication",
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthResponse),
        (status = 401, description = "Token expired or invalid"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn refresh(
    State(db): State<Arc<DatabaseConnection>>,
    State(config): State<Arc<Config>>,
    claims: Claims,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Refresh token request for user_id={}", claims.user_id);
    let response = AuthService::refresh_token(
        &db,
        claims,
        &config.jwt_secret,
        config.jwt_expiration_minutes,
    )
    .await?;

    tracing::info!(
        "Token refreshed successfully: user_id={}, tenant_id={}",
        response.user.id,
        response.user.tenant_id
    );
    Ok(Json(serde_json::json!({
        "token": response.token,
        "user": response.user,
    })))
}
