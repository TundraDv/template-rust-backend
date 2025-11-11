use utoipa::OpenApi;

use crate::{
    handlers::health,
    models,
    services::auth_service::{AuthResponse, LoginRequest, RegisterRequest},
    utils::error::ErrorResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
        crate::handlers::auth::register::register,
        crate::handlers::auth::login::login,
        crate::handlers::auth::refresh::refresh,
        crate::handlers::users::me::me,
        crate::handlers::users::get_user::get_user,
        crate::handlers::users::get_users::get_users,
        crate::handlers::tenants::get_tenants::list_tenants,
        crate::handlers::tenants::get_tenant::get_tenant
    ),
    components(
        schemas(
            health::HealthResponse,
            RegisterRequest,
            LoginRequest,
            AuthResponse,
            models::users::Model,
            models::tenants::Model,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Tenants", description = "Tenant management endpoints"),
    ),
    info(
        title = "Rust Backend Template API",
        description = "A production-ready Rust backend API with multi-tenancy, JWT authentication, and comprehensive error handling",
        version = "1.0.0"
    ),
    servers(
        (url = "http://localhost:8070", description = "Local development server"),
    )
)]
pub struct ApiDoc;
