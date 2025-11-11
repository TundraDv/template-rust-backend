use crate::models::tenants;
use crate::services::tenants_service::TenantsService;
use crate::utils::error::AppError;
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/tenants",
    tag = "Tenants",
    responses(
        (status = 200, description = "List of all tenants", body = Vec<tenants::Model>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_tenants(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<Value>, AppError> {
    let tenants = TenantsService::list_all(&db).await?;

    Ok(Json(serde_json::json!(tenants)))
}
