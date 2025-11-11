use crate::models::users;
use crate::utils::AdminRoleWithTenant;
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn get_users(
    State(db): State<Arc<DatabaseConnection>>,
    AdminRoleWithTenant { tenant_id, .. }: AdminRoleWithTenant,
) -> Result<Json<Value>, StatusCode> {
    let users_list = users::Entity::find()
        .filter(users::Column::TenantId.eq(tenant_id))
        .order_by_desc(users::Column::CreatedAt)
        .all(db.as_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to list users: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!(users_list)))
}
