use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(Users::Role)
                            .string()
                            .not_null()
                            .default("regular"),
                    )
                    .col(
                        ColumnDef::new(Users::Status)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_tenant_id")
                            .from(Users::Table, Users::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_users_email")
                            .table(Users::Table)
                            .col(Users::Email)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx_users_tenant_id")
                            .table(Users::Table)
                            .col(Users::TenantId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    TenantId,
    Email,
    PasswordHash,
    Role,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
}

