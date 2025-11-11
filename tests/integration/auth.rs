use crate::common::*;
use axum_test::TestServer;
use template_rust_backend::routes;
use uuid::Uuid;

#[tokio::test]
#[ignore] // Ignore until test DB is set up
async fn test_register_success() {
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/auth/register")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "tenant_id": Uuid::new_v4().to_string(),
            "email": "test@example.com",
            "password": "password123"
        }))
        .await;

    response.assert_status_ok();
    response.assert_json_contains(serde_json::json!({
        "user": {
            "email": "test@example.com"
        }
    }));
}

#[tokio::test]
#[ignore]
async fn test_register_validation_error() {
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/auth/register")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "tenant_id": Uuid::new_v4().to_string(),
            "email": "invalid-email",
            "password": "short"
        }))
        .await;

    response.assert_status_code(400);
    response.assert_json_contains(serde_json::json!({
        "error": "VALIDATION_ERROR"
    }));
}

#[tokio::test]
#[ignore]
async fn test_register_duplicate_user() {
    // Test registering the same user twice
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();
    let tenant_id = Uuid::new_v4();

    // First registration
    let _response1 = server
        .post("/api/auth/register")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "tenant_id": tenant_id.to_string(),
            "email": "duplicate@example.com",
            "password": "password123"
        }))
        .await;

    // Second registration with same email and tenant
    let response2 = server
        .post("/api/auth/register")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "tenant_id": tenant_id.to_string(),
            "email": "duplicate@example.com",
            "password": "password123"
        }))
        .await;

    response2.assert_status_code(409);
    response2.assert_json_contains(serde_json::json!({
        "error": "USER_ALREADY_EXISTS"
    }));
}

#[tokio::test]
#[ignore]
async fn test_login_success() {
    // First register a user
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();
    let tenant_id = Uuid::new_v4();

    // Register
    let _register_response = server
        .post("/api/auth/register")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "tenant_id": tenant_id.to_string(),
            "email": "login@example.com",
            "password": "password123"
        }))
        .await;

    // Login
    let login_response = server
        .post("/api/auth/login")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "email": "login@example.com",
            "password": "password123"
        }))
        .await;

    login_response.assert_status_ok();
    login_response.assert_json_contains(serde_json::json!({
        "user": {
            "email": "login@example.com"
        }
    }));
}

#[tokio::test]
#[ignore]
async fn test_login_invalid_credentials() {
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/auth/login")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "email": "nonexistent@example.com",
            "password": "wrongpassword"
        }))
        .await;

    response.assert_status_code(401);
    response.assert_json_contains(serde_json::json!({
        "error": "INVALID_CREDENTIALS"
    }));
}

#[tokio::test]
#[ignore]
async fn test_login_validation_error() {
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/auth/login")
        .add_header(
            "Authorization",
            &format!("Bearer {}", get_test_bearer_token()),
        )
        .json(&serde_json::json!({
            "email": "invalid-email",
            "password": ""
        }))
        .await;

    response.assert_status_code(400);
    response.assert_json_contains(serde_json::json!({
        "error": "VALIDATION_ERROR"
    }));
}
