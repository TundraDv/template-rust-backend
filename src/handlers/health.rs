use crate::utils::error::{AppError, ErrorResponse};
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;
use utoipa::ToSchema;
#[derive(ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unavailable", body = ErrorResponse)
    )
)]
pub async fn health_check(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<Value>, AppError> {
    let db_status = sea_orm::sqlx::query("SELECT 1")
        .execute(db.get_postgres_connection_pool())
        .await
        .is_ok();

    let status = if db_status { "healthy" } else { "unhealthy" };

    let response = json!({
        "status": status,
        "database": if db_status { "connected" } else { "disconnected" }
    });

    if db_status {
        Ok(Json(response))
    } else {
        Err(AppError::ServiceUnavailable)
    }
}
