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
            .drop_table(Table::drop().table(ApiKeys::Table).to_owned())
            .await
    }
}

impl Migration {
    fn table() -> TableCreateStatement {
        Table::create()
            .table(ApiKeys::Table)
            .col(
                ColumnDef::new(ApiKeys::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ApiKeys::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(ApiKeys::UserId).big_integer().not_null())
            .col(ColumnDef::new(ApiKeys::Value).binary().not_null())
            .col(ColumnDef::new(ApiKeys::Name).text().not_null())
            .col(
                ColumnDef::new(ApiKeys::ExpiresAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .col(
                ColumnDef::new(ApiKeys::RevokedAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .col(
                ColumnDef::new(ApiKeys::LastUsedAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .check(Expr::col(ApiKeys::Id).gt(0))
            .check(
                Func::char_length(Func::cust(Alias::new("btrim")).arg(Expr::col(ApiKeys::Name)))
                    .gt(0),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("api_keys_membership_fkey")
                    .from(ApiKeys::Table, (ApiKeys::OrganizationId, ApiKeys::UserId))
                    .to(
                        OrganizationMembers::Table,
                        (
                            OrganizationMembers::OrganizationId,
                            OrganizationMembers::UserId,
                        ),
                    )
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn indexes() -> [IndexCreateStatement; 3] {
        [
            Index::create()
                .name("api_keys_value_unique")
                .table(ApiKeys::Table)
                .col(ApiKeys::Value)
                .unique()
                .to_owned(),
            Index::create()
                .name("api_keys_organization_id_id_unique")
                .table(ApiKeys::Table)
                .col(ApiKeys::OrganizationId)
                .col(ApiKeys::Id)
                .unique()
                .to_owned(),
            Index::create()
                .name("api_keys_organization_user_id_unique")
                .table(ApiKeys::Table)
                .col(ApiKeys::OrganizationId)
                .col(ApiKeys::UserId)
                .col(ApiKeys::Id)
                .unique()
                .to_owned(),
        ]
    }
}

#[derive(DeriveIden)]
enum ApiKeys {
    Table,
    Id,
    OrganizationId,
    UserId,
    Value,
    Name,
    ExpiresAt,
    RevokedAt,
    LastUsedAt,
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
}
