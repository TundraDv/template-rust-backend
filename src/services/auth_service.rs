use crate::enums::{UserRole, UserStatus};
use crate::models::users;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub exp: i64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub tenant_id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: users::Model,
}

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, anyhow::Error> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Failed to parse password hash: {}", e))?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, anyhow::Error> {
        tracing::debug!("Verifying token with secret length: {}", secret.len());
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::error!("JWT decode error: {:?}", e);
            anyhow::anyhow!("Token verification failed: {:?}", e)
        })?;
        tracing::debug!(
            "Token decoded successfully, claims: user_id={}, tenant_id={}, email={}",
            token_data.claims.user_id,
            token_data.claims.tenant_id,
            token_data.claims.email
        );
        Ok(token_data.claims)
    }

    pub fn generate_token(
        user_id: Uuid,
        tenant_id: Uuid,
        email: String,
        role: UserRole,
        secret: &str,
        expiration_minutes: i64,
    ) -> Result<String, anyhow::Error> {
        let exp = (Utc::now() + Duration::minutes(expiration_minutes)).timestamp();
        let claims = Claims {
            user_id,
            tenant_id,
            email,
            role,
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        Ok(token)
    }

    pub async fn register(
        db: &DatabaseConnection,
        req: RegisterRequest,
        secret: &str,
        expiration_minutes: i64,
    ) -> Result<AuthResponse, anyhow::Error> {
        let existing_user = users::Entity::find()
            .filter(users::Column::Email.eq(&req.email))
            .filter(users::Column::TenantId.eq(req.tenant_id))
            .one(db)
            .await?;

        if existing_user.is_some() {
            return Err(anyhow::anyhow!("USER_ALREADY_EXISTS_FOR_TENANT"));
        }

        let role = Self::determine_user_role(db, req.tenant_id).await?;

        let password_hash = Self::hash_password(&req.password)?;

        let user = users::ActiveModel {
            id: Set(uuid::Uuid::now_v7()),
            tenant_id: Set(req.tenant_id),
            email: Set(req.email),
            password_hash: Set(password_hash),
            role: Set(role),
            status: Set(UserStatus::Active),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        };

        let user = user.insert(db).await?;

        let token = Self::generate_token(
            user.id,
            user.tenant_id,
            user.email.clone(),
            user.role,
            secret,
            expiration_minutes,
        )?;

        Ok(AuthResponse { token, user })
    }

    pub async fn login(
        db: &DatabaseConnection,
        req: LoginRequest,
        secret: &str,
        expiration_minutes: i64,
    ) -> Result<AuthResponse, anyhow::Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&req.email))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("INVALID_CREDENTIALS"))?;

        if !Self::verify_password(&req.password, &user.password_hash)? {
            return Err(anyhow::anyhow!("INVALID_CREDENTIALS"));
        }

        if user.status != UserStatus::Active {
            return Err(anyhow::anyhow!("USER_NOT_VALIDATED"));
        }

        let token = Self::generate_token(
            user.id,
            user.tenant_id,
            user.email.clone(),
            user.role,
            secret,
            expiration_minutes,
        )?;

        Ok(AuthResponse { token, user })
    }

    pub async fn refresh_token(
        db: &DatabaseConnection,
        claims: Claims,
        secret: &str,
        expiration_minutes: i64,
    ) -> Result<AuthResponse, anyhow::Error> {
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(claims.user_id))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        if user.status != UserStatus::Active {
            return Err(anyhow::anyhow!("USER_NOT_VALIDATED"));
        }

        let token = Self::generate_token(
            user.id,
            user.tenant_id,
            user.email.clone(),
            user.role,
            secret,
            expiration_minutes,
        )?;

        Ok(AuthResponse { token, user })
    }

    async fn determine_user_role(
        db: &DatabaseConnection,
        tenant_id: Uuid,
    ) -> Result<UserRole, anyhow::Error> {
        let has_tenant_users = users::Entity::find()
            .filter(users::Column::TenantId.eq(tenant_id))
            .one(db)
            .await?
            .is_some();

        Ok(if has_tenant_users {
            UserRole::Regular
        } else {
            UserRole::Admin
        })
    }
}
