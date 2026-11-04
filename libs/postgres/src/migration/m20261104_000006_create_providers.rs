use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};
use serde_json::json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(providers_table()).await?;
        for index in providers_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Providers::Table).to_owned())
            .await
    }
}

fn providers_table() -> TableCreateStatement {
    Table::create()
        .table(Providers::Table)
        .col(
            ColumnDef::new(Providers::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(Providers::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(ColumnDef::new(Providers::Name).text().not_null())
        .col(
            ColumnDef::new(Providers::Connection)
                .json_binary()
                .not_null()
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(Providers::Connection))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(Providers::Headers)
                .json_binary()
                .not_null()
                .default(json!({}))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(Providers::Headers))
                        .eq("object"),
                ),
        )
        .col(ColumnDef::new(Providers::Credentials).text().not_null())
        .check(Expr::col(Providers::Id).gt(0))
        .check(
            Func::char_length(Func::cust(Alias::new("btrim")).arg(Expr::col(Providers::Name)))
                .gt(0),
        )
        .foreign_key(
            ForeignKey::create()
                .name("providers_organization_id_fkey")
                .from(Providers::Table, Providers::OrganizationId)
                .to(Organizations::Table, Organizations::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn providers_indexes() -> [IndexCreateStatement; 3] {
    [
        Index::create()
            .name("providers_organization_id_id_unique")
            .table(Providers::Table)
            .col(Providers::OrganizationId)
            .col(Providers::Id)
            .unique()
            .to_owned(),
        Index::create()
            .name("providers_organization_id_name_unique")
            .table(Providers::Table)
            .col(Providers::OrganizationId)
            .col(Providers::Name)
            .unique()
            .to_owned(),
        Index::create()
            .name("providers_organization_id_index")
            .table(Providers::Table)
            .col(Providers::OrganizationId)
            .to_owned(),
    ]
}

#[derive(DeriveIden)]
enum Providers {
    Table,
    Id,
    OrganizationId,
    Name,
    Connection,
    Headers,
    Credentials,
}
#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
