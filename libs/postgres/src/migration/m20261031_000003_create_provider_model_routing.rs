use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, Query, SelectStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ModelProviders::Table)
                    .drop_foreign_key("model_providers_model_scope_fkey")
                    .drop_foreign_key("model_providers_provider_scope_fkey")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("models_id_organization_id_unique")
                    .table(Models::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("providers_id_organization_id_unique")
                    .table(Providers::Table)
                    .to_owned(),
            )
            .await?;
        manager.create_index(Self::model_scope_index()).await?;
        manager.create_index(Self::provider_scope_index()).await?;

        manager.create_table(Self::provider_models_table()).await?;
        for index in Self::provider_model_indexes() {
            manager.create_index(index).await?;
        }
        manager.exec_stmt(Self::provider_model_backfill()).await?;

        manager.create_table(Self::model_router_table()).await?;
        for index in Self::router_indexes() {
            manager.create_index(index).await?;
        }
        manager.exec_stmt(Self::router_backfill()).await?;
        manager
            .drop_table(Table::drop().table(ModelProviders::Table).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Self::legacy_table()).await?;
        manager.exec_stmt(Self::legacy_backfill()).await?;
        manager
            .drop_table(Table::drop().table(ModelRouter::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProviderModels::Table).to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("models_organization_id_id_unique")
                    .table(Models::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("providers_organization_id_id_unique")
                    .table(Providers::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(Self::legacy_model_scope_index())
            .await?;
        manager
            .create_index(Self::legacy_provider_scope_index())
            .await?;
        manager.alter_table(Self::legacy_foreign_keys()).await?;
        for index in Self::legacy_route_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }
}

impl Migration {
    fn model_scope_index() -> IndexCreateStatement {
        Index::create()
            .name("models_organization_id_id_unique")
            .table(Models::Table)
            .col(Models::OrganizationId)
            .col(Models::Id)
            .unique()
            .to_owned()
    }

    fn provider_scope_index() -> IndexCreateStatement {
        Index::create()
            .name("providers_organization_id_id_unique")
            .table(Providers::Table)
            .col(Providers::OrganizationId)
            .col(Providers::Id)
            .unique()
            .to_owned()
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
                ColumnDef::new(ProviderModels::Enabled)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(ProviderModels::InputPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderModels::OutputPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderModels::CachedPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .check(Expr::col(ProviderModels::Id).gt(0))
            .check(
                Func::char_length(
                    Func::cust(Alias::new("btrim")).arg(Expr::col(ProviderModels::Name)),
                )
                .gt(0),
            )
            .check(Expr::col(ProviderModels::InputPricePerMillionTokens).gte(0))
            .check(Expr::col(ProviderModels::OutputPricePerMillionTokens).gte(0))
            .check(Expr::col(ProviderModels::CachedPricePerMillionTokens).gte(0))
            .foreign_key(
                ForeignKey::create()
                    .name("provider_models_provider_scope_fkey")
                    .from(
                        ProviderModels::Table,
                        (ProviderModels::OrganizationId, ProviderModels::ProviderId),
                    )
                    .to(Providers::Table, (Providers::OrganizationId, Providers::Id))
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn provider_model_indexes() -> [IndexCreateStatement; 3] {
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
                .name("provider_models_organization_provider_name_unique")
                .table(ProviderModels::Table)
                .col(ProviderModels::OrganizationId)
                .col(ProviderModels::ProviderId)
                .col(ProviderModels::Name)
                .unique()
                .to_owned(),
        ]
    }

    fn provider_model_backfill() -> InsertStatement {
        let select = Query::select()
            .expr(Func::min(Expr::col(ModelProviders::Id)))
            .column(ModelProviders::OrganizationId)
            .column(ModelProviders::ProviderId)
            .column(ModelProviders::UpstreamModelName)
            .expr(Expr::value(true))
            .expr(Expr::value(0))
            .expr(Expr::value(0))
            .expr(Expr::value(0))
            .from(ModelProviders::Table)
            .group_by_columns([
                ModelProviders::OrganizationId,
                ModelProviders::ProviderId,
                ModelProviders::UpstreamModelName,
            ])
            .to_owned();
        let mut insert = Query::insert();
        insert
            .into_table(ProviderModels::Table)
            .columns([
                ProviderModels::Id,
                ProviderModels::OrganizationId,
                ProviderModels::ProviderId,
                ProviderModels::Name,
                ProviderModels::Enabled,
                ProviderModels::InputPricePerMillionTokens,
                ProviderModels::OutputPricePerMillionTokens,
                ProviderModels::CachedPricePerMillionTokens,
            ])
            .select_from(select)
            .expect("backfill columns match");
        insert.to_owned()
    }

    fn model_router_table() -> TableCreateStatement {
        Table::create()
            .table(ModelRouter::Table)
            .col(
                ColumnDef::new(ModelRouter::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ModelRouter::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelRouter::ModelId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelRouter::ProviderModelId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(ModelRouter::Priority).integer().not_null())
            .col(
                ColumnDef::new(ModelRouter::Enabled)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .check(Expr::col(ModelRouter::Id).gt(0))
            .check(Expr::col(ModelRouter::Priority).gt(0))
            .foreign_key(
                ForeignKey::create()
                    .name("model_router_model_scope_fkey")
                    .from(
                        ModelRouter::Table,
                        (ModelRouter::OrganizationId, ModelRouter::ModelId),
                    )
                    .to(Models::Table, (Models::OrganizationId, Models::Id))
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_router_provider_model_scope_fkey")
                    .from(
                        ModelRouter::Table,
                        (ModelRouter::OrganizationId, ModelRouter::ProviderModelId),
                    )
                    .to(
                        ProviderModels::Table,
                        (ProviderModels::OrganizationId, ProviderModels::Id),
                    )
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn router_indexes() -> [IndexCreateStatement; 4] {
        [
            Index::create()
                .name("model_router_organization_provider_model_id_unique")
                .table(ModelRouter::Table)
                .col(ModelRouter::OrganizationId)
                .col(ModelRouter::ProviderModelId)
                .col(ModelRouter::Id)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_router_model_id_provider_model_id_unique")
                .table(ModelRouter::Table)
                .col(ModelRouter::ModelId)
                .col(ModelRouter::ProviderModelId)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_router_model_id_priority_unique")
                .table(ModelRouter::Table)
                .col(ModelRouter::ModelId)
                .col(ModelRouter::Priority)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_router_provider_model_id_model_id_idx")
                .table(ModelRouter::Table)
                .col(ModelRouter::ProviderModelId)
                .col(ModelRouter::ModelId)
                .to_owned(),
        ]
    }

    fn router_backfill() -> InsertStatement {
        let select = Query::select()
            .column((ModelProviders::Table, ModelProviders::Id))
            .column((ModelProviders::Table, ModelProviders::OrganizationId))
            .column((ModelProviders::Table, ModelProviders::ModelId))
            .column((ProviderModels::Table, ProviderModels::Id))
            .column((ModelProviders::Table, ModelProviders::Priority))
            .column((ModelProviders::Table, ModelProviders::Enabled))
            .from(ModelProviders::Table)
            .join(
                JoinType::InnerJoin,
                ProviderModels::Table,
                Expr::col((ProviderModels::Table, ProviderModels::OrganizationId))
                    .equals((ModelProviders::Table, ModelProviders::OrganizationId))
                    .and(
                        Expr::col((ProviderModels::Table, ProviderModels::ProviderId))
                            .equals((ModelProviders::Table, ModelProviders::ProviderId)),
                    )
                    .and(
                        Expr::col((ProviderModels::Table, ProviderModels::Name))
                            .equals((ModelProviders::Table, ModelProviders::UpstreamModelName)),
                    ),
            )
            .to_owned();
        let mut insert = Query::insert();
        insert
            .into_table(ModelRouter::Table)
            .columns([
                ModelRouter::Id,
                ModelRouter::OrganizationId,
                ModelRouter::ModelId,
                ModelRouter::ProviderModelId,
                ModelRouter::Priority,
                ModelRouter::Enabled,
            ])
            .select_from(select)
            .expect("backfill columns match");
        insert.to_owned()
    }

    fn legacy_table() -> TableCreateStatement {
        Table::create()
            .table(ModelProviders::Table)
            .col(
                ColumnDef::new(ModelProviders::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ModelProviders::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelProviders::ModelId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelProviders::ProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelProviders::UpstreamModelName)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelProviders::Priority)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ModelProviders::Enabled)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(ModelProviders::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ModelProviders::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .check(Expr::col(ModelProviders::Id).gt(0))
            .check(Expr::col(ModelProviders::Priority).gt(0))
            .check(
                Func::char_length(
                    Func::cust(Alias::new("btrim"))
                        .arg(Expr::col(ModelProviders::UpstreamModelName)),
                )
                .gt(0),
            )
            .to_owned()
    }

    fn legacy_backfill() -> InsertStatement {
        let mut select: SelectStatement = Query::select()
            .column((ModelRouter::Table, ModelRouter::Id))
            .column((ModelRouter::Table, ModelRouter::OrganizationId))
            .column((ModelRouter::Table, ModelRouter::ModelId))
            .column((ProviderModels::Table, ProviderModels::ProviderId))
            .column((ProviderModels::Table, ProviderModels::Name))
            .column((ModelRouter::Table, ModelRouter::Priority))
            .column((ModelRouter::Table, ModelRouter::Enabled))
            .from(ModelRouter::Table)
            .join(
                JoinType::InnerJoin,
                ProviderModels::Table,
                Expr::col((ProviderModels::Table, ProviderModels::OrganizationId))
                    .equals((ModelRouter::Table, ModelRouter::OrganizationId))
                    .and(
                        Expr::col((ProviderModels::Table, ProviderModels::Id))
                            .equals((ModelRouter::Table, ModelRouter::ProviderModelId)),
                    ),
            )
            .to_owned();
        select
            .distinct_on(vec![
                (Alias::new("model_router"), Alias::new("model_id")),
                (Alias::new("provider_models"), Alias::new("provider_id")),
            ])
            .order_by((ModelRouter::Table, ModelRouter::ModelId), Order::Asc)
            .order_by(
                (ProviderModels::Table, ProviderModels::ProviderId),
                Order::Asc,
            )
            .order_by((ModelRouter::Table, ModelRouter::Priority), Order::Asc)
            .order_by((ModelRouter::Table, ModelRouter::Id), Order::Asc);
        let mut insert = Query::insert();
        insert
            .into_table(ModelProviders::Table)
            .columns([
                ModelProviders::Id,
                ModelProviders::OrganizationId,
                ModelProviders::ModelId,
                ModelProviders::ProviderId,
                ModelProviders::UpstreamModelName,
                ModelProviders::Priority,
                ModelProviders::Enabled,
            ])
            .select_from(select)
            .expect("backfill columns match");
        insert.to_owned()
    }

    fn legacy_model_scope_index() -> IndexCreateStatement {
        Index::create()
            .name("models_id_organization_id_unique")
            .table(Models::Table)
            .col(Models::Id)
            .col(Models::OrganizationId)
            .unique()
            .to_owned()
    }

    fn legacy_provider_scope_index() -> IndexCreateStatement {
        Index::create()
            .name("providers_id_organization_id_unique")
            .table(Providers::Table)
            .col(Providers::Id)
            .col(Providers::OrganizationId)
            .unique()
            .to_owned()
    }

    fn legacy_foreign_keys() -> TableAlterStatement {
        Table::alter()
            .table(ModelProviders::Table)
            .add_foreign_key(
                &TableForeignKey::new()
                    .name("model_providers_model_scope_fkey")
                    .from_tbl(ModelProviders::Table)
                    .from_col(ModelProviders::ModelId)
                    .from_col(ModelProviders::OrganizationId)
                    .to_tbl(Models::Table)
                    .to_col(Models::Id)
                    .to_col(Models::OrganizationId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .add_foreign_key(
                &TableForeignKey::new()
                    .name("model_providers_provider_scope_fkey")
                    .from_tbl(ModelProviders::Table)
                    .from_col(ModelProviders::ProviderId)
                    .from_col(ModelProviders::OrganizationId)
                    .to_tbl(Providers::Table)
                    .to_col(Providers::Id)
                    .to_col(Providers::OrganizationId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .to_owned()
    }

    fn legacy_route_indexes() -> [IndexCreateStatement; 3] {
        [
            Index::create()
                .name("model_providers_model_id_provider_id_unique")
                .table(ModelProviders::Table)
                .col(ModelProviders::ModelId)
                .col(ModelProviders::ProviderId)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_providers_model_id_priority_unique")
                .table(ModelProviders::Table)
                .col(ModelProviders::ModelId)
                .col(ModelProviders::Priority)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_providers_organization_id_provider_id_idx")
                .table(ModelProviders::Table)
                .col(ModelProviders::OrganizationId)
                .col(ModelProviders::ProviderId)
                .to_owned(),
        ]
    }
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
    OrganizationId,
}
#[derive(DeriveIden)]
enum Providers {
    Table,
    Id,
    OrganizationId,
}
#[derive(DeriveIden)]
enum ModelProviders {
    Table,
    Id,
    OrganizationId,
    ModelId,
    ProviderId,
    UpstreamModelName,
    Priority,
    Enabled,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    Id,
    OrganizationId,
    ProviderId,
    Name,
    Enabled,
    InputPricePerMillionTokens,
    OutputPricePerMillionTokens,
    CachedPricePerMillionTokens,
}
#[derive(DeriveIden)]
enum ModelRouter {
    Table,
    Id,
    OrganizationId,
    ModelId,
    ProviderModelId,
    Priority,
    Enabled,
}
