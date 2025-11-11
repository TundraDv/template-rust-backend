use crate::{
    config::Config,
    handlers::{auth, tenants, users},
    middleware::auth::AuthState,
};
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

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

    let app_state = AppState { db, config };

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
            axum::http::Method::PATCH,
        ])
        .allow_headers(tower_http::cors::Any)
        .expose_headers(tower_http::cors::Any)
        .max_age(std::time::Duration::from_secs(3600));

    let public_routes = Router::new()
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
        .layer(cors)
        .layer(axum::Extension(auth_state))
        .with_state(app_state)
}
