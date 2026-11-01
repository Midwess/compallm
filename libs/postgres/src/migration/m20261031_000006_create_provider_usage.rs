use sea_orm_migration::{
    prelude::*,
    sea_query::{
        IndexCreateStatement, TableCreateStatement,
        extension::postgres::{Type, TypeCreateStatement},
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Self::status_type()).await?;
        manager.create_table(Self::table()).await?;
        for index in Self::indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderUsage::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ProviderUsageStatus::Enum).to_owned())
            .await
    }
}

impl Migration {
    fn status_type() -> TypeCreateStatement {
        Type::create()
            .as_enum(ProviderUsageStatus::Enum)
            .values([
                ProviderUsageStatus::InProgress,
                ProviderUsageStatus::Succeeded,
                ProviderUsageStatus::Failed,
                ProviderUsageStatus::Cancelled,
                ProviderUsageStatus::Incomplete,
            ])
            .to_owned()
    }

    fn table() -> TableCreateStatement {
        Table::create()
            .table(ProviderUsage::Table)
            .col(
                ColumnDef::new(ProviderUsage::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ProviderUsage::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::ModelUsageId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::ModelRouterId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::ProviderModelId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::ProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::ProviderRequestId)
                    .text()
                    .null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::Status)
                    .custom(ProviderUsageStatus::Enum)
                    .not_null()
                    .default("in_progress"),
            )
            .col(
                ColumnDef::new(ProviderUsage::InputTokens)
                    .big_integer()
                    .null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::CachedInputTokens)
                    .big_integer()
                    .null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::OutputTokens)
                    .big_integer()
                    .null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::InputPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::OutputPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProviderUsage::CachedPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .check(Expr::col(ProviderUsage::Id).gt(0))
            .check(Expr::col(ProviderUsage::InputTokens).gte(0))
            .check(Expr::col(ProviderUsage::CachedInputTokens).gte(0))
            .check(
                Expr::col(ProviderUsage::CachedInputTokens)
                    .lte(Expr::col(ProviderUsage::InputTokens)),
            )
            .check(Expr::col(ProviderUsage::OutputTokens).gte(0))
            .check(Expr::col(ProviderUsage::InputPricePerMillionTokens).gte(0))
            .check(Expr::col(ProviderUsage::OutputPricePerMillionTokens).gte(0))
            .check(Expr::col(ProviderUsage::CachedPricePerMillionTokens).gte(0))
            .foreign_key(
                ForeignKey::create()
                    .name("provider_usage_organization_fkey")
                    .from(ProviderUsage::Table, ProviderUsage::OrganizationId)
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("provider_usage_model_usage_scope_fkey")
                    .from(
                        ProviderUsage::Table,
                        (ProviderUsage::OrganizationId, ProviderUsage::ModelUsageId),
                    )
                    .to(
                        ModelUsage::Table,
                        (ModelUsage::OrganizationId, ModelUsage::Id),
                    )
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("provider_usage_route_provider_model_scope_fkey")
                    .from(
                        ProviderUsage::Table,
                        (
                            ProviderUsage::OrganizationId,
                            ProviderUsage::ProviderModelId,
                            ProviderUsage::ModelRouterId,
                        ),
                    )
                    .to(
                        ModelRouter::Table,
                        (
                            ModelRouter::OrganizationId,
                            ModelRouter::ProviderModelId,
                            ModelRouter::Id,
                        ),
                    )
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("provider_usage_provider_model_scope_fkey")
                    .from(
                        ProviderUsage::Table,
                        (
                            ProviderUsage::OrganizationId,
                            ProviderUsage::ProviderId,
                            ProviderUsage::ProviderModelId,
                        ),
                    )
                    .to(
                        ProviderModels::Table,
                        (
                            ProviderModels::OrganizationId,
                            ProviderModels::ProviderId,
                            ProviderModels::Id,
                        ),
                    )
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("provider_usage_provider_scope_fkey")
                    .from(
                        ProviderUsage::Table,
                        (ProviderUsage::OrganizationId, ProviderUsage::ProviderId),
                    )
                    .to(Providers::Table, (Providers::OrganizationId, Providers::Id))
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .to_owned()
    }

    fn indexes() -> [IndexCreateStatement; 4] {
        [
            Index::create()
                .name("provider_usage_organization_model_usage_idx")
                .table(ProviderUsage::Table)
                .col(ProviderUsage::OrganizationId)
                .col(ProviderUsage::ModelUsageId)
                .to_owned(),
            Index::create()
                .name("provider_usage_organization_provider_model_idx")
                .table(ProviderUsage::Table)
                .col(ProviderUsage::OrganizationId)
                .col(ProviderUsage::ProviderModelId)
                .to_owned(),
            Index::create()
                .name("provider_usage_organization_route_idx")
                .table(ProviderUsage::Table)
                .col(ProviderUsage::OrganizationId)
                .col(ProviderUsage::ModelRouterId)
                .to_owned(),
            Index::create()
                .name("provider_usage_organization_provider_request_id_idx")
                .table(ProviderUsage::Table)
                .col(ProviderUsage::OrganizationId)
                .col(ProviderUsage::ProviderId)
                .col(ProviderUsage::ProviderRequestId)
                .to_owned(),
        ]
    }
}

#[derive(DeriveIden)]
enum ProviderUsageStatus {
    #[sea_orm(iden = "provider_usage_status")]
    Enum,
    InProgress,
    Succeeded,
    Failed,
    Cancelled,
    Incomplete,
}

#[derive(DeriveIden)]
enum ProviderUsage {
    Table,
    Id,
    OrganizationId,
    ModelUsageId,
    ModelRouterId,
    ProviderModelId,
    ProviderId,
    ProviderRequestId,
    Status,
    InputTokens,
    CachedInputTokens,
    OutputTokens,
    InputPricePerMillionTokens,
    OutputPricePerMillionTokens,
    CachedPricePerMillionTokens,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ModelUsage {
    Table,
    Id,
    OrganizationId,
}

#[derive(DeriveIden)]
enum ModelRouter {
    Table,
    Id,
    OrganizationId,
    ProviderModelId,
}

#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    Id,
    OrganizationId,
    ProviderId,
}

#[derive(DeriveIden)]
enum Providers {
    Table,
    Id,
    OrganizationId,
}
