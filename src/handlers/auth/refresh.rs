use crate::{config::Config, middleware::auth::Claims, services::auth_service::AuthService};
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn refresh(
    State(db): State<Arc<DatabaseConnection>>,
    State(config): State<Arc<Config>>,
    claims: Claims,
) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Refresh token request for user_id={}", claims.user_id);
    let response = AuthService::refresh_token(
        &db,
        claims,
        &config.jwt_secret,
        config.jwt_expiration_minutes,
    )
    .await
    .map_err(|e| {
        tracing::warn!("Token refresh failed: {:?}", e);
        StatusCode::UNAUTHORIZED
    })?;

    tracing::info!(
        "Token refreshed successfully: user_id={}, tenant_id={}",
        response.user.id,
        response.user.tenant_id
    );
    Ok(Json(json!({
        "token": response.token,
        "user": response.user,
    })))
}
