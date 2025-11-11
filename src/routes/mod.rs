use crate::{
    config::{Config, create_cors_layer},
    handlers::{auth, health, tenants, users},
    middleware::auth::AuthState,
    middleware::tracing_middleware,
};
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<Config>,
}

impl FromRef<AppState> for Arc<DatabaseConnection> {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

pub fn create_router(db: Arc<DatabaseConnection>, config: Arc<Config>) -> Router {
    let auth_state = Arc::new(AuthState {
        secret: config.jwt_secret.clone(),
        bearer_token: config.jwt_secret.clone(),
    });

    let app_state = AppState {
        db,
        config: config.clone(),
    };

    let cors = create_cors_layer(&config);

    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/tenants", get(tenants::list_tenants));

    let authenticated_routes = Router::new()
        .route("/api/auth/refresh", post(auth::refresh))
        .route("/api/me", get(users::me))
        .route("/api/tenants/{tenant_id}", get(tenants::get_tenant))
        .route(
            "/api/tenants/{tenant_id}/users/{user_id}",
            get(users::get_user),
        );

    let admin_routes = Router::new()
        .route("/api/tenants/{tenant_id}/users", get(users::get_users))
        .route(
            "/api/tenants/{tenant_id}/users/{user_id}/change-status",
            put(users::change_user_status),
        )
        .route(
            "/api/tenants/{tenant_id}/users/{user_id}/change-role",
            put(users::change_role),
        );

    Router::new()
        .merge(public_routes)
        .merge(authenticated_routes)
        .merge(admin_routes)
        .layer(axum::middleware::from_fn(tracing_middleware))
        .layer(cors)
        .layer(axum::Extension(auth_state))
        .with_state(app_state)
}
