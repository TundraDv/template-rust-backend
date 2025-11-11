use axum_test::TestServer;
use template_rust_backend::routes;
use crate::common::*;

#[tokio::test]
#[ignore] // Ignore until test DB is set up
async fn test_health_check_success() {
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/health").await;
    response.assert_status_ok();
    response.assert_json_contains(serde_json::json!({
        "status": "healthy",
        "database": "connected"
    }));
}

#[tokio::test]
#[ignore]
async fn test_health_check_database_disconnected() {
    // Test case for when database is disconnected
    // Would require mocking or disconnecting the DB
}

