use axum_test::TestServer;
use template_rust_backend::routes;
use crate::common::*;

#[tokio::test]
#[ignore] // Ignore until test DB is set up
async fn test_list_tenants() {
    let app = setup_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/api/tenants").await;
    response.assert_status_ok();
    // Would assert on actual tenant data
}

#[tokio::test]
#[ignore]
async fn test_get_tenant_by_id() {
    // Test GET /api/tenants/{tenant_id}
    // Requires authentication and tenant access
}

