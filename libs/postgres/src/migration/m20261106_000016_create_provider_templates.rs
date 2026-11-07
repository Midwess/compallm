use sea_orm_migration::prelude::*;
use serde_json::json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProviderTemplates::Table)
                    .col(
                        ColumnDef::new(ProviderTemplates::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProviderTemplates::Name)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProviderTemplates::Description).text().null())
                    .col(ColumnDef::new(ProviderTemplates::IconKey).text().null())
                    .col(
                        ColumnDef::new(ProviderTemplates::Protocol)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderTemplates::BaseUrl)
                            .string_len(512)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderTemplates::Headers)
                            .json_binary()
                            .not_null()
                            .default(json!({}))
                            .check(
                                Func::cust(Alias::new("jsonb_typeof"))
                                    .arg(Expr::col(ProviderTemplates::Headers))
                                    .eq("object"),
                            ),
                    )
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim")).arg(Expr::col(ProviderTemplates::Id)),
                        )
                        .gt(0),
                    )
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim")).arg(Expr::col(ProviderTemplates::Name)),
                        )
                        .gt(0),
                    )
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim"))
                                .arg(Expr::col(ProviderTemplates::BaseUrl)),
                        )
                        .gt(0),
                    )
                    .check(Expr::col(ProviderTemplates::Protocol).is_in([
                        "openai_compatible",
                        "anthropic",
                        "provider_specific",
                    ]))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderTemplates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProviderTemplates {
    Table,
    Id,
    Name,
    Description,
    IconKey,
    Protocol,
    BaseUrl,
    Headers,
}
