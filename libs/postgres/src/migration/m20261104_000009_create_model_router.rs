use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(model_router_table()).await?;
        for index in model_router_indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelRouter::Table).to_owned())
            .await
    }
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
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
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
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn model_router_indexes() -> [IndexCreateStatement; 4] {
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
#[derive(DeriveIden)]
enum Models {
    Table,
    OrganizationId,
    Id,
}
#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    OrganizationId,
    Id,
}
