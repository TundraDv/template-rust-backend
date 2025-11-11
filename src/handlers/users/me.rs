use crate::middleware::auth::Claims;
use crate::models::users;
use crate::utils::error::AppError;
use axum::{extract::State, response::Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::Value;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/me",
    tag = "Users",
    responses(
        (status = 200, description = "Current user information", body = users::Model),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn me(
    State(db): State<Arc<DatabaseConnection>>,
    claims: Claims,
) -> Result<Json<Value>, AppError> {
    tracing::info!(
        "GET /me request from user_id={}, tenant_id={}",
        claims.user_id,
        claims.tenant_id
    );
    let user = users::Entity::find_by_id(claims.user_id)
        .one(db.as_ref())
        .await?
        .ok_or(AppError::UserNotFound)?;

    tracing::debug!(
        "User data retrieved: email={}, tenant_id={}",
        user.email,
        user.tenant_id
    );
    Ok(Json(serde_json::json!({
        "id": user.id,
        "email": user.email,
        "tenant_id": user.tenant_id,
        "status": user.status,
        "role": user.role,
        "created_at": user.created_at
    })))
}
