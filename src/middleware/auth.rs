use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

pub use crate::services::auth_service::Claims;

#[derive(Debug)]
pub enum AuthError {
    ExpiredToken,
    InvalidToken,
    MissingToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "INVALID_TOKEN"),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "MISSING_TOKEN"),
        };

        let body = json!({
            "error": error_message
        });

        (status, axum::Json(body)).into_response()
    }
}

pub struct AuthState {
    pub secret: String,
    pub bearer_token: String,
}

pub struct BearerToken;

impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_state_arc = parts
            .extensions
            .get::<Arc<AuthState>>()
            .ok_or_else(|| {
                tracing::error!("AuthState not found in request extensions");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .clone();

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                tracing::warn!("No Authorization header found in request");
                StatusCode::UNAUTHORIZED
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            tracing::warn!("Authorization header does not start with 'Bearer '");
            StatusCode::UNAUTHORIZED
        })?;

        tracing::info!(
            "Received token (first 20 chars): {}",
            if token.len() > 20 {
                format!("{}...", &token[..20])
            } else {
                token.to_string()
            }
        );
        tracing::info!(
            "Expected Bearer Token (first 20 chars): {}",
            if auth_state_arc.bearer_token.len() > 20 {
                format!("{}...", &auth_state_arc.bearer_token[..20])
            } else {
                auth_state_arc.bearer_token.clone()
            }
        );
        tracing::info!(
            "Token length: {}, Expected length: {}",
            token.len(),
            auth_state_arc.bearer_token.len()
        );

        if token != auth_state_arc.bearer_token {
            tracing::warn!("Bearer token mismatch - tokens do not match");
            return Err(StatusCode::UNAUTHORIZED);
        }

        tracing::info!("Bearer token validated successfully");
        Ok(BearerToken)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_state_arc = parts
            .extensions
            .get::<Arc<AuthState>>()
            .ok_or_else(|| {
                tracing::error!("AuthState not found in request extensions");
                AuthError::InvalidToken
            })?
            .clone();

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                tracing::warn!("No Authorization header found in request");
                AuthError::MissingToken
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            tracing::warn!("Authorization header does not start with 'Bearer '");
            AuthError::InvalidToken
        })?;

        tracing::debug!("Validating JWT token for protected endpoint");

        let mut validation = jsonwebtoken::Validation::default();
        validation.validate_exp = false;

        let claims_result = jsonwebtoken::decode::<crate::services::auth_service::Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(auth_state_arc.secret.as_ref()),
            &validation,
        );

        let claims = match claims_result {
            Ok(token_data) => token_data.claims,
            Err(e) => {
                let error_str = format!("{:?}", e);
                tracing::error!("JWT token verification failed: {:?}", e);

                if error_str.contains("ExpiredSignature")
                    || error_str.contains("expired")
                    || error_str.contains("Expired")
                    || error_str.to_lowercase().contains("expired")
                {
                    tracing::warn!("Token expired, returning TOKEN_EXPIRED error");
                    return Err(AuthError::ExpiredToken);
                } else {
                    return Err(AuthError::InvalidToken);
                }
            }
        };

        tracing::debug!(
            "JWT token verified successfully for user_id: {}, tenant_id: {}",
            claims.user_id,
            claims.tenant_id
        );
        Ok(claims)
    }
}
