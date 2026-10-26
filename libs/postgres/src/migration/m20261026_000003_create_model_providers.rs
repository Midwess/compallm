use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(Self::provider_identity_scope_index())
            .await?;
        manager.create_table(Self::table()).await?;
        manager.create_index(Self::model_provider_index()).await?;
        manager.create_index(Self::model_priority_index()).await?;
        manager.create_index(Self::provider_scope_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelProviders::Table).to_owned())
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("providers_id_organization_id_unique")
                    .table(Providers::Table)
                    .to_owned(),
            )
            .await
    }
}

impl Migration {
    fn table() -> TableCreateStatement {
        Table::create()
            .table(ModelProviders::Table)
            // Hero generates the route ID in the application.
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
            .check(Expr::cust("length(btrim(upstream_model_name)) > 0"))
            .foreign_key(
                ForeignKey::create()
                    .name("model_providers_model_scope_fkey")
                    .from(
                        ModelProviders::Table,
                        (ModelProviders::ModelId, ModelProviders::OrganizationId),
                    )
                    .to(Models::Table, (Models::Id, Models::OrganizationId))
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("model_providers_provider_scope_fkey")
                    .from(
                        ModelProviders::Table,
                        (ModelProviders::ProviderId, ModelProviders::OrganizationId),
                    )
                    .to(Providers::Table, (Providers::Id, Providers::OrganizationId))
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn provider_identity_scope_index() -> IndexCreateStatement {
        // Required by the composite route-to-provider foreign key.
        Index::create()
            .name("providers_id_organization_id_unique")
            .table(Providers::Table)
            .col(Providers::Id)
            .col(Providers::OrganizationId)
            .unique()
            .to_owned()
    }

    fn model_provider_index() -> IndexCreateStatement {
        Index::create()
            .name("model_providers_model_id_provider_id_unique")
            .table(ModelProviders::Table)
            .col(ModelProviders::ModelId)
            .col(ModelProviders::ProviderId)
            .unique()
            .to_owned()
    }

    fn model_priority_index() -> IndexCreateStatement {
        // Also supports loading a model's routes in fallback order.
        Index::create()
            .name("model_providers_model_id_priority_unique")
            .table(ModelProviders::Table)
            .col(ModelProviders::ModelId)
            .col(ModelProviders::Priority)
            .unique()
            .to_owned()
    }

    fn provider_scope_index() -> IndexCreateStatement {
        Index::create()
            .name("model_providers_provider_id_organization_id_idx")
            .table(ModelProviders::Table)
            .col(ModelProviders::ProviderId)
            .col(ModelProviders::OrganizationId)
            .to_owned()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_stores_order_and_provider_specific_model_name() {
        let sql = Migration::table().to_string(PostgresQueryBuilder);

        assert!(sql.contains(r#""id" bigint NOT NULL PRIMARY KEY"#));
        assert!(!sql.to_lowercase().contains("serial"));
        assert!(sql.contains(r#""upstream_model_name" text NOT NULL"#));
        assert!(sql.contains(r#""priority" integer NOT NULL"#));
        assert!(sql.contains(r#""enabled" bool NOT NULL DEFAULT TRUE"#));
        assert!(sql.contains(r#""priority" > 0"#));
        assert!(sql.contains("length(btrim(upstream_model_name)) > 0"));
    }

    #[test]
    fn route_cannot_cross_organization_boundaries() {
        let sql = Migration::table().to_string(PostgresQueryBuilder);

        assert!(sql.contains(
            r#"FOREIGN KEY ("model_id", "organization_id") REFERENCES "models" ("id", "organization_id") ON DELETE CASCADE"#
        ));
        assert!(sql.contains(
            r#"FOREIGN KEY ("provider_id", "organization_id") REFERENCES "providers" ("id", "organization_id") ON DELETE CASCADE"#
        ));
        assert_eq!(
            Migration::provider_identity_scope_index().to_string(PostgresQueryBuilder),
            r#"CREATE UNIQUE INDEX "providers_id_organization_id_unique" ON "providers" ("id", "organization_id")"#
        );
    }

    #[test]
    fn provider_and_priority_are_unique_within_a_model() {
        assert_eq!(
            Migration::model_provider_index().to_string(PostgresQueryBuilder),
            r#"CREATE UNIQUE INDEX "model_providers_model_id_provider_id_unique" ON "model_providers" ("model_id", "provider_id")"#
        );
        assert_eq!(
            Migration::model_priority_index().to_string(PostgresQueryBuilder),
            r#"CREATE UNIQUE INDEX "model_providers_model_id_priority_unique" ON "model_providers" ("model_id", "priority")"#
        );
    }

    #[test]
    fn provider_reverse_lookup_has_an_index() {
        assert_eq!(
            Migration::provider_scope_index().to_string(PostgresQueryBuilder),
            r#"CREATE INDEX "model_providers_provider_id_organization_id_idx" ON "model_providers" ("provider_id", "organization_id")"#
        );
    }
}
