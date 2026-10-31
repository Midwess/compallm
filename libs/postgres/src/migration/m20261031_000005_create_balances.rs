use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Self::table()).await?;
        for index in Self::indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Balances::Table).to_owned())
            .await
    }
}

impl Migration {
    fn table() -> TableCreateStatement {
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
            .to_owned()
    }

    fn indexes() -> [IndexCreateStatement; 2] {
        [
            Index::create()
                .name("balances_model_usage_unique")
                .table(Balances::Table)
                .col(Balances::ModelUsageId)
                .unique()
                .to_owned(),
            Index::create()
                .name("balances_member_idx")
                .table(Balances::Table)
                .col(Balances::OrganizationId)
                .col(Balances::UserId)
                .to_owned(),
        ]
    }
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
