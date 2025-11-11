use crate::enums::{UserRole, UserStatus};
use crate::models::users;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct UsersService;

impl UsersService {
    pub async fn change_user_status(
        db: &DatabaseConnection,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<users::Model, anyhow::Error> {
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(user_id))
            .filter(users::Column::TenantId.eq(tenant_id))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let new_status = match user.status {
            UserStatus::Active => UserStatus::Inactive,
            UserStatus::Inactive => UserStatus::Active,
        };

        let mut user: users::ActiveModel = user.into();
        user.status = Set(new_status);
        let user = user.update(db).await?;

        Ok(user)
    }

    pub async fn change_role(
        db: &DatabaseConnection,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<users::Model, anyhow::Error> {
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(user_id))
            .filter(users::Column::TenantId.eq(tenant_id))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let new_role = match user.role {
            UserRole::Admin => UserRole::Regular,
            UserRole::Regular => UserRole::Admin,
        };

        let mut user: users::ActiveModel = user.into();
        user.role = Set(new_role);
        let user = user.update(db).await?;

        Ok(user)
    }
}
