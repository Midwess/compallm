use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
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
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("model_usage_limit_organization_user_unique")
                    .table(ModelUsageLimit::Table)
                    .col(ModelUsageLimit::OrganizationId)
                    .col(ModelUsageLimit::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Balances::Table).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Balances::Table)
                    .col(
                        ColumnDef::new(Balances::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Balances::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Balances::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Balances::ModelUsageId).big_integer().null())
                    .col(
                        ColumnDef::new(Balances::Amount)
                            .decimal_len(24, 12)
                            .not_null(),
                    )
                    .check(Expr::col(Balances::Id).gt(0))
                    .check(Expr::col(Balances::Amount).ne(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("balances_membership_fkey")
                            .from(
                                Balances::Table,
                                (Balances::OrganizationId, Balances::UserId),
                            )
                            .to(
                                OrganizationMembers::Table,
                                (
                                    OrganizationMembers::OrganizationId,
                                    OrganizationMembers::UserId,
                                ),
                            )
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("balances_model_usage_fkey")
                            .from(
                                Balances::Table,
                                (
                                    Balances::OrganizationId,
                                    Balances::UserId,
                                    Balances::ModelUsageId,
                                ),
                            )
                            .to(
                                ModelUsage::Table,
                                (
                                    ModelUsage::OrganizationId,
                                    ModelUsage::UserId,
                                    ModelUsage::Id,
                                ),
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("balances_model_usage_unique")
                    .table(Balances::Table)
                    .col(Balances::ModelUsageId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("balances_member_idx")
                    .table(Balances::Table)
                    .col(Balances::OrganizationId)
                    .col(Balances::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ModelUsageLimit::Table).to_owned())
            .await
    }
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
enum Balances {
    Table,
    Id,
    OrganizationId,
    UserId,
    ModelUsageId,
    Amount,
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
}

#[derive(DeriveIden)]
enum ModelUsage {
    Table,
    Id,
    OrganizationId,
    UserId,
}
