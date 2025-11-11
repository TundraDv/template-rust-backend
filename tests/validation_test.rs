use template_rust_backend::middleware::validation::validate_request;
use template_rust_backend::services::auth_service::{LoginRequest, RegisterRequest};
use template_rust_backend::utils::error::AppError;
use uuid::Uuid;

#[test]
fn test_validate_register_request_valid() {
    let req = RegisterRequest {
        tenant_id: Uuid::now_v7(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    assert!(validate_request(req).is_ok());
}

#[test]
fn test_validate_register_request_invalid_email() {
    let req = RegisterRequest {
        tenant_id: Uuid::now_v7(),
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
    };
    let result = validate_request(req);
    assert!(result.is_err());
    if let Err(AppError::Validation(msg)) = result {
        assert!(msg.contains("email"));
    }
}

#[test]
fn test_validate_register_request_short_password() {
    let req = RegisterRequest {
        tenant_id: Uuid::now_v7(),
        email: "test@example.com".to_string(),
        password: "short".to_string(),
    };
    let result = validate_request(req);
    assert!(result.is_err());
    if let Err(AppError::Validation(msg)) = result {
        assert!(msg.contains("password"));
    }
}

#[test]
fn test_validate_register_request_long_password() {
    let req = RegisterRequest {
        tenant_id: Uuid::now_v7(),
        email: "test@example.com".to_string(),
        password: "a".repeat(101),
    };
    let result = validate_request(req);
    assert!(result.is_err());
}

#[test]
fn test_validate_login_request_valid() {
    let req = LoginRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    assert!(validate_request(req).is_ok());
}

#[test]
fn test_validate_login_request_invalid_email() {
    let req = LoginRequest {
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
    };
    let result = validate_request(req);
    assert!(result.is_err());
}

#[test]
fn test_validate_login_request_empty_password() {
    let req = LoginRequest {
        email: "test@example.com".to_string(),
        password: "".to_string(),
    };
    let result = validate_request(req);
    assert!(result.is_err());
}
