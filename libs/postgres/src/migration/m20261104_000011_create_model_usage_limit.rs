use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(model_usage_limit_table()).await?;
        manager.create_index(membership_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelUsageLimit::Table).to_owned())
            .await
    }
}

fn model_usage_limit_table() -> TableCreateStatement {
    Table::create()
        .table(ModelUsageLimit::Table)
        .col(
            ColumnDef::new(ModelUsageLimit::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(ModelUsageLimit::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ModelUsageLimit::UserId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ModelUsageLimit::LimitUsd)
                .decimal_len(24, 12)
                .not_null(),
        )
        .col(
            ColumnDef::new(ModelUsageLimit::CurrentUsageUsd)
                .decimal_len(24, 12)
                .not_null(),
        )
        .check(Expr::col(ModelUsageLimit::Id).gt(0))
        .check(Expr::col(ModelUsageLimit::LimitUsd).gte(0))
        .check(Expr::col(ModelUsageLimit::CurrentUsageUsd).gte(0))
        .foreign_key(
            ForeignKey::create()
                .name("model_usage_limit_membership_fkey")
                .from(
                    ModelUsageLimit::Table,
                    (ModelUsageLimit::OrganizationId, ModelUsageLimit::UserId),
                )
                .to(
                    OrganizationMembers::Table,
                    (
                        OrganizationMembers::OrganizationId,
                        OrganizationMembers::UserId,
                    ),
                )
                .on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn membership_index() -> IndexCreateStatement {
    Index::create()
        .name("model_usage_limit_organization_user_unique")
        .table(ModelUsageLimit::Table)
        .col(ModelUsageLimit::OrganizationId)
        .col(ModelUsageLimit::UserId)
        .unique()
        .to_owned()
}

#[derive(DeriveIden)]
enum ModelUsageLimit {
    Table,
    Id,
    OrganizationId,
    UserId,
    LimitUsd,
    CurrentUsageUsd,
}
#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
}
