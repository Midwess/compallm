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
        manager.create_table(provider_video_usage_table()).await?;
        for index in provider_video_usage_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderVideoUsage::Table).to_owned())
            .await
    }
}

fn provider_video_usage_table() -> TableCreateStatement {
    Table::create()
        .table(ProviderVideoUsage::Table)
        .col(
            ColumnDef::new(ProviderVideoUsage::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::ModelUsageId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::AttemptId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::ModelRouterId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::ProviderModelId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::ProviderId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::ProviderRequestId)
                .text()
                .null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::Status)
                .custom(ProviderUsageStatus::Enum)
                .not_null()
                .default("in_progress"),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::UsdPerSecond)
                .decimal_len(20, 8)
                .null()
                .check(Expr::col(ProviderVideoUsage::UsdPerSecond).gte(0)),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::DurationSeconds)
                .decimal_len(24, 9)
                .null(),
        )
        .col(
            ColumnDef::new(ProviderVideoUsage::Conditions)
                .json_binary()
                .not_null()
                .default(json!({}))
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderVideoUsage::Conditions))
                        .eq("object"),
                ),
        )
        .check(Expr::col(ProviderVideoUsage::Id).gt(0))
        .check(Expr::col(ProviderVideoUsage::AttemptId).gt(0))
        .check(Expr::col(ProviderVideoUsage::DurationSeconds).gte(0))
        .foreign_key(
            ForeignKey::create()
                .name("provider_video_usage_model_usage_scope_fkey")
                .from(
                    ProviderVideoUsage::Table,
                    (
                        ProviderVideoUsage::OrganizationId,
                        ProviderVideoUsage::ModelUsageId,
                    ),
                )
                .to(
                    ModelUsage::Table,
                    (ModelUsage::OrganizationId, ModelUsage::Id),
                )
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .foreign_key(
            ForeignKey::create()
                .name("provider_video_usage_route_scope_fkey")
                .from(
                    ProviderVideoUsage::Table,
                    (
                        ProviderVideoUsage::OrganizationId,
                        ProviderVideoUsage::ProviderModelId,
                        ProviderVideoUsage::ModelRouterId,
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
                .on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::NoAction),
        )
        .foreign_key(
            ForeignKey::create()
                .name("provider_video_usage_provider_model_scope_fkey")
                .from(
                    ProviderVideoUsage::Table,
                    (
                        ProviderVideoUsage::OrganizationId,
                        ProviderVideoUsage::ProviderId,
                        ProviderVideoUsage::ProviderModelId,
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
                .on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::NoAction),
        )
        .foreign_key(
            ForeignKey::create()
                .name("provider_video_usage_provider_scope_fkey")
                .from(
                    ProviderVideoUsage::Table,
                    (
                        ProviderVideoUsage::OrganizationId,
                        ProviderVideoUsage::ProviderId,
                    ),
                )
                .to(Providers::Table, (Providers::OrganizationId, Providers::Id))
                .on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn provider_video_usage_indexes() -> [IndexCreateStatement; 5] {
    [
        Index::create()
            .name("provider_video_usage_attempt_idx")
            .table(ProviderVideoUsage::Table)
            .col(ProviderVideoUsage::OrganizationId)
            .col(ProviderVideoUsage::AttemptId)
            .to_owned(),
        Index::create()
            .name("provider_video_usage_model_usage_idx")
            .table(ProviderVideoUsage::Table)
            .col(ProviderVideoUsage::OrganizationId)
            .col(ProviderVideoUsage::ModelUsageId)
            .to_owned(),
        Index::create()
            .name("provider_video_usage_provider_model_idx")
            .table(ProviderVideoUsage::Table)
            .col(ProviderVideoUsage::OrganizationId)
            .col(ProviderVideoUsage::ProviderModelId)
            .to_owned(),
        Index::create()
            .name("provider_video_usage_route_idx")
            .table(ProviderVideoUsage::Table)
            .col(ProviderVideoUsage::OrganizationId)
            .col(ProviderVideoUsage::ModelRouterId)
            .to_owned(),
        Index::create()
            .name("provider_video_usage_request_idx")
            .table(ProviderVideoUsage::Table)
            .col(ProviderVideoUsage::OrganizationId)
            .col(ProviderVideoUsage::ProviderId)
            .col(ProviderVideoUsage::ProviderRequestId)
            .to_owned(),
    ]
}

#[derive(DeriveIden)]
enum ProviderUsageStatus {
    #[sea_orm(iden = "provider_usage_status")]
    Enum,
}
#[derive(DeriveIden)]
enum ProviderVideoUsage {
    Table,
    Id,
    OrganizationId,
    ModelUsageId,
    AttemptId,
    ModelRouterId,
    ProviderModelId,
    ProviderId,
    ProviderRequestId,
    Status,
    UsdPerSecond,
    DurationSeconds,
    Conditions,
}
#[derive(DeriveIden)]
enum ModelUsage {
    Table,
    OrganizationId,
    Id,
}
#[derive(DeriveIden)]
enum ModelRouter {
    Table,
    OrganizationId,
    ProviderModelId,
    Id,
}
#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    OrganizationId,
    ProviderId,
    Id,
}
#[derive(DeriveIden)]
enum Providers {
    Table,
    OrganizationId,
    Id,
}
