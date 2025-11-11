use axum::Router;
use std::sync::Arc;
use template_rust_backend::config;

/// Setup a test application router
/// Note: This requires a test database to be set up
/// For now, this is a placeholder that will panic
/// In production, you'd use testcontainers or a test DB
pub async fn setup_test_app() -> Router {
    // TODO: Implement actual test database setup
    // 1. Connect to test database (or use testcontainers)
    // 2. Run migrations
    // 3. Create test config
    // 4. Return router

    // For now, this will fail - tests are marked #[ignore] until implemented
    panic!("Test database setup not implemented. Use testcontainers or set up test DB connection.")
}

/// Get test configuration
pub fn get_test_config() -> Arc<config::Config> {
    Arc::new(config::Config {
        jwt_secret: "test-secret-key-for-testing-only".to_string(),
        jwt_expiration_minutes: 10,
        server_host: "127.0.0.1".to_string(),
        server_port: 0, // Use 0 for random port in tests
        environment: "test".to_string(),
        frontend_url: Some("http://localhost:3000".to_string()),
    })
}

/// Get test bearer token (same as JWT secret for testing)
pub fn get_test_bearer_token() -> String {
    "test-secret-key-for-testing-only".to_string()
}
