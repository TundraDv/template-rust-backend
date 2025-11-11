use crate::services::tenants_service::TenantsService;
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn list_tenants(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<Value>, StatusCode> {
    let tenants = TenantsService::list_all(&db).await.map_err(|e| {
        tracing::error!("Failed to list tenants: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!(tenants)))
}
