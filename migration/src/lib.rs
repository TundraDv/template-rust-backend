use sea_orm_migration::prelude::*;

pub use sea_orm_migration::MigratorTrait;

mod m20240101000001_create_tenants;
mod m20240101000002_create_users;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101000001_create_tenants::Migration),
            Box::new(m20240101000002_create_users::Migration),
        ]
    }
}
