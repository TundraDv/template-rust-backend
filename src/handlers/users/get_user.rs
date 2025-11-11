use crate::models::users;
use crate::utils::{TenantAccess, error::AppError};
use axum::{extract::Path, extract::State, response::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/tenants/{tenant_id}/users/{user_id}",
    tag = "Users",
    params(
        ("tenant_id" = String, Path, description = "Tenant ID"),
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User information", body = users::Model),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn get_user(
    State(db): State<Arc<DatabaseConnection>>,
    TenantAccess { tenant_id, .. }: TenantAccess,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .filter(users::Column::TenantId.eq(tenant_id))
        .one(db.as_ref())
        .await?
        .ok_or(AppError::UserNotFound)?;

    Ok(Json(serde_json::json!(user)))
}
