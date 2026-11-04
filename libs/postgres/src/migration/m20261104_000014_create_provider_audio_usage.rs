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
        manager.create_type(usage_direction_type()).await?;
        manager.create_table(provider_audio_usage_table()).await?;
        for index in provider_audio_usage_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderAudioUsage::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(UsageDirection::Enum).to_owned())
            .await
    }
}

fn provider_audio_usage_table() -> TableCreateStatement {
    Table::create()
        .table(ProviderAudioUsage::Table)
        .col(
            ColumnDef::new(ProviderAudioUsage::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::ModelUsageId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::AttemptId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::ModelRouterId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::ProviderModelId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::ProviderId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::ProviderRequestId)
                .text()
                .null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::Status)
                .custom(ProviderUsageStatus::Enum)
                .not_null()
                .default("in_progress"),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::PricingSnapshot)
                .json_binary()
                .not_null()
                .check(
                    Func::cust(Alias::new("jsonb_typeof"))
                        .arg(Expr::col(ProviderAudioUsage::PricingSnapshot))
                        .eq("object"),
                ),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::Direction)
                .custom(UsageDirection::Enum)
                .not_null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::DurationSeconds)
                .decimal_len(24, 9)
                .null(),
        )
        .col(
            ColumnDef::new(ProviderAudioUsage::CharacterCount)
                .big_integer()
                .null(),
        )
        .check(Expr::col(ProviderAudioUsage::Id).gt(0))
        .check(Expr::col(ProviderAudioUsage::AttemptId).gt(0))
        .check(Expr::col(ProviderAudioUsage::DurationSeconds).gte(0))
        .check(Expr::col(ProviderAudioUsage::CharacterCount).gte(0))
        .foreign_key(
            ForeignKey::create()
                .name("provider_audio_usage_model_usage_scope_fkey")
                .from(
                    ProviderAudioUsage::Table,
                    (
                        ProviderAudioUsage::OrganizationId,
                        ProviderAudioUsage::ModelUsageId,
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
                .name("provider_audio_usage_route_scope_fkey")
                .from(
                    ProviderAudioUsage::Table,
                    (
                        ProviderAudioUsage::OrganizationId,
                        ProviderAudioUsage::ProviderModelId,
                        ProviderAudioUsage::ModelRouterId,
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
                .name("provider_audio_usage_provider_model_scope_fkey")
                .from(
                    ProviderAudioUsage::Table,
                    (
                        ProviderAudioUsage::OrganizationId,
                        ProviderAudioUsage::ProviderId,
                        ProviderAudioUsage::ProviderModelId,
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
                .name("provider_audio_usage_provider_scope_fkey")
                .from(
                    ProviderAudioUsage::Table,
                    (
                        ProviderAudioUsage::OrganizationId,
                        ProviderAudioUsage::ProviderId,
                    ),
                )
                .to(Providers::Table, (Providers::OrganizationId, Providers::Id))
                .on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn provider_audio_usage_indexes() -> [IndexCreateStatement; 5] {
    [
        Index::create()
            .name("provider_audio_usage_attempt_direction_unique")
            .table(ProviderAudioUsage::Table)
            .col(ProviderAudioUsage::OrganizationId)
            .col(ProviderAudioUsage::AttemptId)
            .col(ProviderAudioUsage::Direction)
            .unique()
            .to_owned(),
        Index::create()
            .name("provider_audio_usage_model_usage_idx")
            .table(ProviderAudioUsage::Table)
            .col(ProviderAudioUsage::OrganizationId)
            .col(ProviderAudioUsage::ModelUsageId)
            .to_owned(),
        Index::create()
            .name("provider_audio_usage_provider_model_idx")
            .table(ProviderAudioUsage::Table)
            .col(ProviderAudioUsage::OrganizationId)
            .col(ProviderAudioUsage::ProviderModelId)
            .to_owned(),
        Index::create()
            .name("provider_audio_usage_route_idx")
            .table(ProviderAudioUsage::Table)
            .col(ProviderAudioUsage::OrganizationId)
            .col(ProviderAudioUsage::ModelRouterId)
            .to_owned(),
        Index::create()
            .name("provider_audio_usage_request_idx")
            .table(ProviderAudioUsage::Table)
            .col(ProviderAudioUsage::OrganizationId)
            .col(ProviderAudioUsage::ProviderId)
            .col(ProviderAudioUsage::ProviderRequestId)
            .to_owned(),
    ]
}

fn usage_direction_type() -> TypeCreateStatement {
    Type::create()
        .as_enum(UsageDirection::Enum)
        .values([UsageDirection::Input, UsageDirection::Output])
        .to_owned()
}

#[derive(DeriveIden)]
enum UsageDirection {
    #[sea_orm(iden = "usage_direction")]
    Enum,
    Input,
    Output,
}
#[derive(DeriveIden)]
enum ProviderUsageStatus {
    #[sea_orm(iden = "provider_usage_status")]
    Enum,
}
#[derive(DeriveIden)]
enum ProviderAudioUsage {
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
    PricingSnapshot,
    Direction,
    DurationSeconds,
    CharacterCount,
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
