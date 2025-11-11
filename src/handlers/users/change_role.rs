use crate::services::users_service::UsersService;
use crate::utils::AdminRoleWithTenant;
use axum::{extract::Path, extract::State, http::StatusCode, response::Json};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;
use uuid::Uuid;

pub async fn change_role(
    State(db): State<Arc<DatabaseConnection>>,
    AdminRoleWithTenant { tenant_id, .. }: AdminRoleWithTenant,
    Path((_, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, StatusCode> {
    let user = UsersService::change_role(&db, user_id, tenant_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "id": user.id,
        "email": user.email,
        "role": user.role,
        "message": format!("User role changed to {:?} successfully", user.role)
    })))
}
