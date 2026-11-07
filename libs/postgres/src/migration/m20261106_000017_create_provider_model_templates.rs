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
                    .table(ProviderModelTemplates::Table)
                    .col(
                        ColumnDef::new(ProviderModelTemplates::ProviderTemplateId)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProviderModelTemplates::Id).text().not_null())
                    .col(
                        ColumnDef::new(ProviderModelTemplates::Name)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::Protocols)
                            .json_binary()
                            .not_null()
                            .default(json!([]))
                            .check(
                                Func::cust(Alias::new("jsonb_typeof"))
                                    .arg(Expr::col(ProviderModelTemplates::Protocols))
                                    .eq("array"),
                            ),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::Paths)
                            .json_binary()
                            .not_null()
                            .default(json!({}))
                            .check(
                                Func::cust(Alias::new("jsonb_typeof"))
                                    .arg(Expr::col(ProviderModelTemplates::Paths))
                                    .eq("object"),
                            ),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::Capabilities)
                            .json_binary()
                            .not_null()
                            .default(json!([]))
                            .check(
                                Func::cust(Alias::new("jsonb_typeof"))
                                    .arg(Expr::col(ProviderModelTemplates::Capabilities))
                                    .eq("array"),
                            ),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::MaxContextTokens)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::EffortMapping)
                            .json_binary()
                            .not_null()
                            .default(json!({}))
                            .check(
                                Func::cust(Alias::new("jsonb_typeof"))
                                    .arg(Expr::col(ProviderModelTemplates::EffortMapping))
                                    .eq("object"),
                            ),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::InputPrice)
                            .decimal_len(20, 8)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::OutputPrice)
                            .decimal_len(20, 8)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProviderModelTemplates::CachedPrice)
                            .decimal_len(20, 8)
                            .null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ProviderModelTemplates::ProviderTemplateId)
                            .col(ProviderModelTemplates::Id),
                    )
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim"))
                                .arg(Expr::col(ProviderModelTemplates::Id)),
                        )
                        .gt(0),
                    )
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim"))
                                .arg(Expr::col(ProviderModelTemplates::Name)),
                        )
                        .gt(0),
                    )
                    .check(Expr::col(ProviderModelTemplates::MaxContextTokens).gt(0))
                    .check(
                        Expr::col(ProviderModelTemplates::MaxContextTokens).lte(4_294_967_295_i64),
                    )
                    .check(Expr::col(ProviderModelTemplates::InputPrice).gte(0))
                    .check(Expr::col(ProviderModelTemplates::OutputPrice).gte(0))
                    .check(Expr::col(ProviderModelTemplates::CachedPrice).gte(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("provider_model_templates_provider_template_id_fkey")
                            .from(
                                ProviderModelTemplates::Table,
                                ProviderModelTemplates::ProviderTemplateId,
                            )
                            .to(ProviderTemplates::Table, ProviderTemplates::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ProviderModelTemplates::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ProviderModelTemplates {
    Table,
    ProviderTemplateId,
    Id,
    Name,
    Description,
    Protocols,
    Paths,
    Capabilities,
    MaxContextTokens,
    EffortMapping,
    InputPrice,
    OutputPrice,
    CachedPrice,
}
#[derive(DeriveIden)]
enum ProviderTemplates {
    Table,
    Id,
}
