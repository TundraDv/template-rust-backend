use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::json;
use std::sync::Arc;

pub use crate::services::auth_service::Claims;

pub struct AuthState {
    pub secret: String,
    pub bearer_token: String,
}

pub struct BearerToken;

impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_state_arc = parts
            .extensions
            .get::<Arc<AuthState>>()
            .ok_or_else(|| {
                tracing::error!("AuthState not found in request extensions");
                let body =
                    json!({"error": "INTERNAL_ERROR", "message": "Server configuration error"});
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(body),
                )
                    .into_response()
            })?
            .clone();

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                tracing::warn!("No Authorization header found in request");
                let body =
                    json!({"error": "MISSING_TOKEN", "message": "Authentication token required"});
                (axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            tracing::warn!("Authorization header does not start with 'Bearer '");
            let body = json!({"error": "INVALID_TOKEN", "message": "Invalid authentication token"});
            (axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
        })?;

        if token != auth_state_arc.bearer_token {
            tracing::warn!("Bearer token mismatch");
            let body = json!({"error": "INVALID_TOKEN", "message": "Invalid authentication token"});
            return Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response());
        }

        tracing::info!("Bearer token validated successfully");
        Ok(BearerToken)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_state_arc = parts
            .extensions
            .get::<Arc<AuthState>>()
            .ok_or_else(|| {
                tracing::error!("AuthState not found in request extensions");
                let body =
                    json!({"error": "INTERNAL_ERROR", "message": "Server configuration error"});
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(body),
                )
                    .into_response()
            })?
            .clone();

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                tracing::warn!("No Authorization header found in request");
                let body =
                    json!({"error": "MISSING_TOKEN", "message": "Authentication token required"});
                (axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            tracing::warn!("Authorization header does not start with 'Bearer '");
            let body = json!({"error": "INVALID_TOKEN", "message": "Invalid authentication token"});
            (axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
        })?;

        tracing::debug!("Validating JWT token for protected endpoint");

        let mut validation = Validation::default();
        validation.validate_exp = true;

        let claims_result = jsonwebtoken::decode::<crate::services::auth_service::Claims>(
            token,
            &DecodingKey::from_secret(auth_state_arc.secret.as_ref()),
            &validation,
        );

        match claims_result {
            Ok(token_data) => {
                tracing::debug!(
                    "JWT token verified successfully for user_id: {}, tenant_id: {}",
                    token_data.claims.user_id,
                    token_data.claims.tenant_id
                );
                Ok(token_data.claims)
            }
            Err(e) => {
                tracing::error!("JWT token verification failed: {:?}", e);

                let (error_code, message) = match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        ("TOKEN_EXPIRED", "Token has expired")
                    }
                    _ => ("INVALID_TOKEN", "Invalid authentication token"),
                };

                let body = json!({
                    "error": error_code,
                    "message": message
                });
                Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response())
            }
        }
    }
}
