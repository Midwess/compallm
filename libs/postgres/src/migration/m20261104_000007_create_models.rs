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
        manager.create_table(models_table()).await?;
        for index in models_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Models::Table).to_owned())
            .await
    }
}

fn models_table() -> TableCreateStatement {
    Table::create()
        .table(Models::Table)
        .col(
            ColumnDef::new(Models::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(Models::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(ColumnDef::new(Models::Name).text().not_null())
        .col(ColumnDef::new(Models::ServiceKind).text().not_null())
        .col(
            ColumnDef::new(Models::Enabled)
                .boolean()
                .not_null()
                .default(true),
        )
        .col(
            ColumnDef::new(Models::Capabilities)
                .json_binary()
                .not_null()
                .default(json!([]))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(Models::Capabilities))
                        .eq("array"),
                ),
        )
        .col(
            ColumnDef::new(Models::Limits)
                .json_binary()
                .not_null()
                .default(json!({}))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(Models::Limits))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(Models::Thinking).json_binary().null().check(
                Func::cust(Alias::new("jsonb_typeof"))
                    .arg(Expr::col(Models::Thinking))
                    .eq("object"),
            ),
        )
        .col(
            ColumnDef::new(Models::Pricing)
                .json_binary()
                .not_null()
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(Models::Pricing))
                        .eq("object"),
                ),
        )
        .check(Expr::col(Models::Id).gt(0))
        .check(
            Func::char_length(Func::cust(Alias::new("btrim")).arg(Expr::col(Models::Name))).gt(0),
        )
        .foreign_key(
            ForeignKey::create()
                .name("models_organization_id_fkey")
                .from(Models::Table, Models::OrganizationId)
                .to(Organizations::Table, Organizations::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn models_indexes() -> [IndexCreateStatement; 2] {
    [
        Index::create()
            .name("models_organization_id_id_unique")
            .table(Models::Table)
            .col(Models::OrganizationId)
            .col(Models::Id)
            .unique()
            .to_owned(),
        Index::create()
            .name("models_organization_id_name_unique")
            .table(Models::Table)
            .col(Models::OrganizationId)
            .col(Models::Name)
            .unique()
            .to_owned(),
    ]
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
    OrganizationId,
    Name,
    ServiceKind,
    Enabled,
    Capabilities,
    Limits,
    Thinking,
    Pricing,
}
#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
