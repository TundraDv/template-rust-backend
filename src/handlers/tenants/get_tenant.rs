use crate::models::tenants;
use crate::services::tenants_service::TenantsService;
use crate::utils::{TenantAccess, error::AppError};
use axum::{extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/tenants/{tenant_id}",
    tag = "Tenants",
    params(
        ("tenant_id" = String, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "Tenant information", body = tenants::Model),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Tenant not found")
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn get_tenant(
    State(db): State<Arc<DatabaseConnection>>,
    TenantAccess { tenant_id, .. }: TenantAccess,
) -> Result<Json<Value>, AppError> {
    let tenant = TenantsService::get_by_id(&db, tenant_id).await?;

    Ok(Json(serde_json::json!(tenant)))
}
