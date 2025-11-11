use crate::{
    config::Config,
    middleware::auth::BearerToken,
    services::auth_service::{AuthService, LoginRequest},
};
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn login(
    State(db): State<Arc<DatabaseConnection>>,
    State(config): State<Arc<Config>>,
    _bearer_token: BearerToken,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Value>, StatusCode> {
    let email = payload.email.clone();
    tracing::info!("Login attempt for email: {}", email);
    let result = AuthService::login(
        &db,
        payload,
        &config.jwt_secret,
        config.jwt_expiration_minutes,
    )
    .await;

    match result {
        Ok(response) => {
            tracing::info!(
                "Login successful: user_id={}, tenant_id={}",
                response.user.id,
                response.user.tenant_id
            );
            Ok(Json(json!({
                "token": response.token,
                "user": response.user,
            })))
        }
        Err(e) => {
            let error_msg = e.to_string();
            tracing::warn!("Login failed for email: {}, error: {}", email, error_msg);
            if error_msg == "USER_NOT_VALIDATED" {
                return Ok(Json(json!({
                    "error": "USER_NOT_VALIDATED"
                })));
            }
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
