use crate::models::users;
use crate::utils::TenantAccess;
use axum::{extract::Path, extract::State, http::StatusCode, response::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::{Value, json};
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_user(
    State(db): State<Arc<DatabaseConnection>>,
    TenantAccess { tenant_id, .. }: TenantAccess,
    Path((_, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, StatusCode> {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .filter(users::Column::TenantId.eq(tenant_id))
        .one(db.as_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            tracing::warn!("User not found: user_id={}", user_id);
            StatusCode::NOT_FOUND
        })?;

    Ok(Json(json!(user)))
}
