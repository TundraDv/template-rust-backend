use crate::services::users_service::UsersService;
use crate::utils::{AdminRoleWithTenant, error::AppError};
use axum::{extract::Path, extract::State, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub async fn change_user_status(
    State(db): State<Arc<DatabaseConnection>>,
    AdminRoleWithTenant { tenant_id, .. }: AdminRoleWithTenant,
    Path((_, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, AppError> {
    let user = UsersService::change_user_status(&db, user_id, tenant_id).await?;

    Ok(Json(serde_json::json!({
        "id": user.id,
        "email": user.email,
        "status": user.status,
        "message": format!("User status changed to {:?} successfully", user.status)
    })))
}
