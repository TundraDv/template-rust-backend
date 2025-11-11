use crate::middleware::auth::Claims;
use crate::models::users;
use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn me(
    State(db): State<Arc<DatabaseConnection>>,
    claims: Claims,
) -> Result<Json<Value>, StatusCode> {
    tracing::info!(
        "GET /me request from user_id={}, tenant_id={}",
        claims.user_id,
        claims.tenant_id
    );
    let user = users::Entity::find_by_id(claims.user_id)
        .one(db.as_ref())
        .await
        .map_err(|e| {
            tracing::error!("Database error fetching user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            tracing::warn!("User not found: user_id={}", claims.user_id);
            StatusCode::NOT_FOUND
        })?;

    tracing::debug!(
        "User data retrieved: email={}, tenant_id={}",
        user.email,
        user.tenant_id
    );
    Ok(Json(json!({
        "id": user.id,
        "email": user.email,
        "tenant_id": user.tenant_id,
        "status": user.status,
        "role": user.role,
        "created_at": user.created_at
    })))
}
