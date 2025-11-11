use crate::enums::UserRole;
use crate::middleware::auth::Claims;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

pub fn check_tenant_access(claims: &Claims, tenant_id: Uuid) -> Result<(), StatusCode> {
    if claims.tenant_id != tenant_id {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(())
}

pub fn check_role(claims: &Claims, required_role: UserRole) -> Result<(), StatusCode> {
    if claims.role != required_role {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(())
}

pub fn check_tenant_and_role(
    claims: &Claims,
    tenant_id: Uuid,
    required_role: UserRole,
) -> Result<(), StatusCode> {
    check_tenant_access(claims, tenant_id)?;
    check_role(claims, required_role)?;
    Ok(())
}

pub struct AdminRole(pub Claims);

impl<S> FromRequestParts<S> for AdminRole
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, _state)
            .await
            .map_err(|e| e.into_response())?;

        if claims.role != UserRole::Admin {
            let body = json!({
                "error": "FORBIDDEN",
                "message": "ADMIN_ROLE_REQUIRED"
            });
            return Err((StatusCode::FORBIDDEN, axum::Json(body)).into_response());
        }

        Ok(AdminRole(claims))
    }
}

pub struct AdminRoleWithTenant {
    pub claims: Claims,
    pub tenant_id: Uuid,
}

impl<S> FromRequestParts<S> for AdminRoleWithTenant
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, _state)
            .await
            .map_err(|e| e.into_response())?;

        if claims.role != UserRole::Admin {
            let body = json!({
                "error": "FORBIDDEN",
                "message": "ADMIN_ROLE_REQUIRED"
            });
            return Err((StatusCode::FORBIDDEN, axum::Json(body)).into_response());
        }

        let tenant_id = axum::extract::Path::<Uuid>::from_request_parts(parts, _state)
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({"error": "INVALID_TENANT_ID"})),
                )
                    .into_response()
            })?;

        if claims.tenant_id != *tenant_id {
            let body = json!({
                "error": "FORBIDDEN",
                "message": "ACCESS_DENIED_FOR_THIS_TENANT"
            });
            return Err((StatusCode::FORBIDDEN, axum::Json(body)).into_response());
        }

        Ok(AdminRoleWithTenant {
            claims,
            tenant_id: *tenant_id,
        })
    }
}

pub struct TenantAccess {
    pub claims: Claims,
    pub tenant_id: Uuid,
}

impl<S> FromRequestParts<S> for TenantAccess
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, _state)
            .await
            .map_err(|e| e.into_response())?;

        let tenant_id = axum::extract::Path::<Uuid>::from_request_parts(parts, _state)
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({"error": "INVALID_TENANT_ID"})),
                )
                    .into_response()
            })?;

        if claims.tenant_id != *tenant_id {
            let body = json!({
                "error": "FORBIDDEN",
                "message": "ACCESS_DENIED_FOR_THIS_TENANT"
            });
            return Err((StatusCode::FORBIDDEN, axum::Json(body)).into_response());
        }

        Ok(TenantAccess {
            claims,
            tenant_id: *tenant_id,
        })
    }
}
