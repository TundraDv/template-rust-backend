use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Auth(#[from] AuthError),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists for tenant")]
    UserAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not validated")]
    UserNotValidated,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal server error")]
    Internal,

    #[error("Tenant not found")]
    TenantNotFound,

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Token expired")]
    ExpiredToken,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Missing token")]
    MissingToken,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            AppError::Auth(AuthError::ExpiredToken) => (
                StatusCode::UNAUTHORIZED,
                "TOKEN_EXPIRED",
                "Token has expired".to_string(),
            ),
            AppError::Auth(AuthError::InvalidToken) => (
                StatusCode::UNAUTHORIZED,
                "INVALID_TOKEN",
                "Invalid authentication token".to_string(),
            ),
            AppError::Auth(AuthError::MissingToken) => (
                StatusCode::UNAUTHORIZED,
                "MISSING_TOKEN",
                "Authentication token required".to_string(),
            ),
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "USER_NOT_FOUND",
                "User not found".to_string(),
            ),
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "USER_ALREADY_EXISTS",
                "User already exists for this tenant".to_string(),
            ),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Invalid email or password".to_string(),
            ),
            AppError::UserNotValidated => (
                StatusCode::FORBIDDEN,
                "USER_NOT_VALIDATED",
                "User account is not validated".to_string(),
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                format!("Forbidden: {}", msg),
            ),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                "Database operation failed".to_string(),
            ),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Internal server error".to_string(),
            ),
            AppError::TenantNotFound => (
                StatusCode::NOT_FOUND,
                "TENANT_NOT_FOUND",
                "Tenant not found".to_string(),
            ),
            AppError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                "Service is currently unavailable".to_string(),
            ),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg),
        };

        let body = json!({
            "error": error_code,
            "message": message
        });

        (status, axum::Json(body)).into_response()
    }
}
