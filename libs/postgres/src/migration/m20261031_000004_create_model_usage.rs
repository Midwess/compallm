use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Self::table()).await?;
        for index in Self::indexes() {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelUsage::Table).to_owned())
            .await
    }
}

impl Migration {
    fn table() -> TableCreateStatement {
        Table::create()
            .table(ModelUsage::Table)
            .col(
                ColumnDef::new(ModelUsage::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ModelUsage::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(ModelUsage::UserId).big_integer().null())
            .col(ColumnDef::new(ModelUsage::ApiKeyId).big_integer().null())
            .col(ColumnDef::new(ModelUsage::ModelId).big_integer().null())
            .check(Expr::col(ModelUsage::Id).gt(0))
            .foreign_key(
                ForeignKey::create()
                    .name("model_usage_organization_fkey")
                    .from(ModelUsage::Table, ModelUsage::OrganizationId)
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_usage_user_fkey")
                    .from(ModelUsage::Table, ModelUsage::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_usage_membership_fkey")
                    .from(
                        ModelUsage::Table,
                        (ModelUsage::OrganizationId, ModelUsage::UserId),
                    )
                    .to(
                        OrganizationMembers::Table,
                        (
                            OrganizationMembers::OrganizationId,
                            OrganizationMembers::UserId,
                        ),
                    )
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_usage_api_key_scope_fkey")
                    .from(
                        ModelUsage::Table,
                        (ModelUsage::OrganizationId, ModelUsage::ApiKeyId),
                    )
                    .to(ApiKeys::Table, (ApiKeys::OrganizationId, ApiKeys::Id))
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_usage_api_key_owner_fkey")
                    .from(
                        ModelUsage::Table,
                        (
                            ModelUsage::OrganizationId,
                            ModelUsage::UserId,
                            ModelUsage::ApiKeyId,
                        ),
                    )
                    .to(
                        ApiKeys::Table,
                        (ApiKeys::OrganizationId, ApiKeys::UserId, ApiKeys::Id),
                    )
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_usage_model_scope_fkey")
                    .from(
                        ModelUsage::Table,
                        (ModelUsage::OrganizationId, ModelUsage::ModelId),
                    )
                    .to(Models::Table, (Models::OrganizationId, Models::Id))
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .to_owned()
    }

    fn indexes() -> [IndexCreateStatement; 4] {
        [
            Index::create()
                .name("model_usage_organization_id_id_unique")
                .table(ModelUsage::Table)
                .col(ModelUsage::OrganizationId)
                .col(ModelUsage::Id)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_usage_organization_user_id_unique")
                .table(ModelUsage::Table)
                .col(ModelUsage::OrganizationId)
                .col(ModelUsage::UserId)
                .col(ModelUsage::Id)
                .unique()
                .to_owned(),
            Index::create()
                .name("model_usage_organization_api_key_idx")
                .table(ModelUsage::Table)
                .col(ModelUsage::OrganizationId)
                .col(ModelUsage::ApiKeyId)
                .to_owned(),
            Index::create()
                .name("model_usage_organization_model_idx")
                .table(ModelUsage::Table)
                .col(ModelUsage::OrganizationId)
                .col(ModelUsage::ModelId)
                .to_owned(),
        ]
    }
}

#[derive(DeriveIden)]
enum ModelUsage {
    Table,
    Id,
    OrganizationId,
    UserId,
    ApiKeyId,
    ModelId,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
}

#[derive(DeriveIden)]
enum ApiKeys {
    Table,
    Id,
    OrganizationId,
    UserId,
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
    OrganizationId,
}
