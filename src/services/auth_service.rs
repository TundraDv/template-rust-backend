use crate::enums::{UserRole, UserStatus};
use crate::models::users;
use crate::utils::error::AppError;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub exp: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    pub tenant_id: Uuid,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: users::Model,
}

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal)?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::Internal)?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
        tracing::debug!("Verifying token with secret length: {}", secret.len());
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Internal)?;
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
    ) -> Result<String, AppError> {
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
        )
        .map_err(|_| AppError::Internal)?;
        Ok(token)
    }

    pub async fn register(
        db: &DatabaseConnection,
        req: RegisterRequest,
        secret: &str,
        expiration_minutes: i64,
    ) -> Result<AuthResponse, AppError> {
        let existing_user = users::Entity::find()
            .filter(users::Column::Email.eq(&req.email))
            .filter(users::Column::TenantId.eq(req.tenant_id))
            .one(db)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
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
    ) -> Result<AuthResponse, AppError> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&req.email))
            .one(db)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        if !Self::verify_password(&req.password, &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        if user.status != UserStatus::Active {
            return Err(AppError::UserNotValidated);
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
    ) -> Result<AuthResponse, AppError> {
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(claims.user_id))
            .one(db)
            .await?
            .ok_or(AppError::UserNotFound)?;

        if user.status != UserStatus::Active {
            return Err(AppError::UserNotValidated);
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
    ) -> Result<UserRole, AppError> {
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
