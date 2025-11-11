use crate::models::users;
use crate::utils::{AdminRoleWithTenant, error::AppError};
use axum::{extract::State, response::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde_json::Value;
use std::sync::Arc;

pub async fn get_users(
    State(db): State<Arc<DatabaseConnection>>,
    AdminRoleWithTenant { tenant_id, .. }: AdminRoleWithTenant,
) -> Result<Json<Value>, AppError> {
    let users_list = users::Entity::find()
        .filter(users::Column::TenantId.eq(tenant_id))
        .order_by_desc(users::Column::CreatedAt)
        .all(db.as_ref())
        .await?;

    Ok(Json(serde_json::json!(users_list)))
}
