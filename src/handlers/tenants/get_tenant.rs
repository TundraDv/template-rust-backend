use crate::services::tenants_service::TenantsService;
use crate::utils::TenantAccess;
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn get_tenant(
    State(db): State<Arc<DatabaseConnection>>,
    TenantAccess { tenant_id, .. }: TenantAccess,
) -> Result<Json<Value>, StatusCode> {
    let tenant = TenantsService::get_by_id(&db, tenant_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!(tenant)))
}
