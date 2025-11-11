use crate::services::tenants_service::TenantsService;
use crate::utils::error::AppError;
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;

pub async fn list_tenants(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<Value>, AppError> {
    let tenants = TenantsService::list_all(&db).await?;

    Ok(Json(serde_json::json!(tenants)))
}
