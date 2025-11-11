use crate::models::tenants;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct TenantsService;

impl TenantsService {
    pub async fn list_all(db: &DatabaseConnection) -> Result<Vec<tenants::Model>, anyhow::Error> {
        let tenants_list = tenants::Entity::find().all(db).await?;
        Ok(tenants_list)
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        tenant_id: Uuid,
    ) -> Result<tenants::Model, anyhow::Error> {
        let tenant = tenants::Entity::find_by_id(tenant_id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Tenant not found"))?;
        Ok(tenant)
    }
}
