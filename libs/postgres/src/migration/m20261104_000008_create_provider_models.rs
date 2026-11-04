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
        manager.create_table(provider_models_table()).await?;
        for index in provider_models_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderModels::Table).to_owned())
            .await
    }
}

fn provider_models_table() -> TableCreateStatement {
    Table::create()
        .table(ProviderModels::Table)
        .col(
            ColumnDef::new(ProviderModels::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(ProviderModels::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderModels::ProviderId)
                .big_integer()
                .not_null(),
        )
        .col(ColumnDef::new(ProviderModels::Name).text().not_null())
        .col(
            ColumnDef::new(ProviderModels::ServiceKind)
                .text()
                .not_null(),
        )
        .col(ColumnDef::new(ProviderModels::Protocol).text().not_null())
        .col(
            ColumnDef::new(ProviderModels::Enabled)
                .boolean()
                .not_null()
                .default(true),
        )
        .col(
            ColumnDef::new(ProviderModels::BaseUrl).text().null().check(
                Func::char_length(
                    Func::cust(Alias::new("btrim")).arg(Expr::col(ProviderModels::BaseUrl)),
                )
                .gt(0),
            ),
        )
        .col(
            ColumnDef::new(ProviderModels::Connection)
                .json_binary()
                .null()
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderModels::Connection))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(ProviderModels::Headers)
                .json_binary()
                .not_null()
                .default(json!({}))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderModels::Headers))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(ProviderModels::Capabilities)
                .json_binary()
                .not_null()
                .default(json!([]))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderModels::Capabilities))
                        .eq("array"),
                ),
        )
        .col(
            ColumnDef::new(ProviderModels::Limits)
                .json_binary()
                .not_null()
                .default(json!({}))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderModels::Limits))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(ProviderModels::Thinking)
                .json_binary()
                .null()
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderModels::Thinking))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(ProviderModels::Pricing)
                .json_binary()
                .not_null()
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderModels::Pricing))
                        .eq("object"),
                ),
        )
        .check(Expr::col(ProviderModels::Id).gt(0))
        .check(
            Func::char_length(Func::cust(Alias::new("btrim")).arg(Expr::col(ProviderModels::Name)))
                .gt(0),
        )
        .foreign_key(
            ForeignKey::create()
                .name("provider_models_provider_scope_fkey")
                .from(
                    ProviderModels::Table,
                    (ProviderModels::OrganizationId, ProviderModels::ProviderId),
                )
                .to(Providers::Table, (Providers::OrganizationId, Providers::Id))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn provider_models_indexes() -> [IndexCreateStatement; 3] {
    [
        Index::create()
            .name("provider_models_organization_id_id_unique")
            .table(ProviderModels::Table)
            .col(ProviderModels::OrganizationId)
            .col(ProviderModels::Id)
            .unique()
            .to_owned(),
        Index::create()
            .name("provider_models_organization_provider_id_unique")
            .table(ProviderModels::Table)
            .col(ProviderModels::OrganizationId)
            .col(ProviderModels::ProviderId)
            .col(ProviderModels::Id)
            .unique()
            .to_owned(),
        Index::create()
            .name("provider_models_org_provider_name_service_protocol_unique")
            .table(ProviderModels::Table)
            .col(ProviderModels::OrganizationId)
            .col(ProviderModels::ProviderId)
            .col(ProviderModels::Name)
            .col(ProviderModels::ServiceKind)
            .col(ProviderModels::Protocol)
            .unique()
            .to_owned(),
    ]
}

#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    Id,
    OrganizationId,
    ProviderId,
    Name,
    ServiceKind,
    Protocol,
    Enabled,
    BaseUrl,
    Connection,
    Headers,
    Capabilities,
    Limits,
    Thinking,
    Pricing,
}
#[derive(DeriveIden)]
enum Providers {
    Table,
    OrganizationId,
    Id,
}
