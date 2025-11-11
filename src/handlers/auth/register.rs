use crate::{
    config::Config,
    middleware::auth::BearerToken,
    services::auth_service::{AuthService, RegisterRequest},
};
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn register(
    State(db): State<Arc<DatabaseConnection>>,
    State(config): State<Arc<Config>>,
    _bearer_token: BearerToken,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Register request for email: {}", payload.email);
    let response = AuthService::register(
        &db,
        payload,
        &config.jwt_secret,
        config.jwt_expiration_minutes,
    )
    .await
    .map_err(|e| {
        tracing::error!("Registration failed: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    tracing::info!(
        "User registered successfully: user_id={}, tenant_id={}",
        response.user.id,
        response.user.tenant_id
    );
    Ok(Json(json!({
        "token": response.token,
        "user": response.user,
    })))
}
