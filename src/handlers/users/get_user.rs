use crate::models::users;
use crate::utils::{TenantAccess, error::AppError};
use axum::{extract::Path, extract::State, response::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_user(
    State(db): State<Arc<DatabaseConnection>>,
    TenantAccess { tenant_id, .. }: TenantAccess,
    Path((_, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .filter(users::Column::TenantId.eq(tenant_id))
        .one(db.as_ref())
        .await?
        .ok_or(AppError::UserNotFound)?;

    Ok(Json(serde_json::json!(user)))
}
