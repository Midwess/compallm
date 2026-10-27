use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableAlterStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::add_model_columns()).await?;
        manager.alter_table(Self::add_provider_columns()).await?;
        manager
            .create_index(Self::provider_identity_scope_index())
            .await?;
        manager.create_table(Self::table()).await?;
        manager.create_index(Self::model_provider_index()).await?;
        manager.create_index(Self::model_priority_index()).await?;
        manager
            .create_index(Self::organization_provider_index())
            .await
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
            .await?;
        manager.alter_table(Self::drop_provider_columns()).await?;
        manager.alter_table(Self::drop_model_columns()).await
    }
}

impl Migration {
    fn add_model_columns() -> TableAlterStatement {
        Table::alter()
            .table(Models::Table)
            .add_column(
                ColumnDef::new(Models::Enabled)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .add_column(
                ColumnDef::new(Models::Hidden)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .add_column(
                ColumnDef::new(Models::Settings)
                    .json_binary()
                    .not_null()
                    .default(Expr::cust("'{}'::jsonb")),
            )
            .to_owned()
    }

    fn drop_model_columns() -> TableAlterStatement {
        Table::alter()
            .table(Models::Table)
            .drop_column(Models::Settings)
            .drop_column(Models::Hidden)
            .drop_column(Models::Enabled)
            .to_owned()
    }

    fn add_provider_columns() -> TableAlterStatement {
        Table::alter()
            .table(Providers::Table)
            .add_column(
                ColumnDef::new(Providers::Enabled)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .to_owned()
    }

    fn drop_provider_columns() -> TableAlterStatement {
        Table::alter()
            .table(Providers::Table)
            .drop_column(Providers::Enabled)
            .to_owned()
    }

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
        // PostgreSQL requires this exact pair to be unique before
        // model_providers can reference (provider_id, organization_id) and
        // thereby reject routes whose provider belongs to another organization.
        // https://www.postgresql.org/docs/current/sql-createtable.html#SQL-CREATETABLE-PARMS-REFERENCES
        Index::create()
            .name("providers_id_organization_id_unique")
            .table(Providers::Table)
            .col(Providers::Id)
            .col(Providers::OrganizationId)
            .unique()
            .to_owned()
    }

    fn model_provider_index() -> IndexCreateStatement {
        // Prevent one logical model from routing to the same provider more than
        // once, which would create duplicate route configuration.
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

    fn organization_provider_index() -> IndexCreateStatement {
        // Support organization-scoped route listing and lookup by provider.
        // This is a query index; organization integrity is enforced by the
        // composite foreign keys above.
        Index::create()
            .name("model_providers_organization_id_provider_id_idx")
            .table(ModelProviders::Table)
            .col(ModelProviders::OrganizationId)
            .col(ModelProviders::ProviderId)
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
    Enabled,
    Hidden,
    Settings,
}

#[derive(DeriveIden)]
enum Providers {
    Table,
    Id,
    OrganizationId,
    Enabled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_lifecycle_and_settings_columns_are_added_together() {
        assert_eq!(
            Migration::add_model_columns().to_string(PostgresQueryBuilder),
            concat!(
                r#"ALTER TABLE "models" ADD COLUMN "enabled" bool NOT NULL DEFAULT TRUE, "#,
                r#"ADD COLUMN "hidden" bool NOT NULL DEFAULT FALSE, "#,
                r#"ADD COLUMN "settings" jsonb NOT NULL DEFAULT ('{}'::jsonb)"#
            )
        );
        assert_eq!(
            Migration::drop_model_columns().to_string(PostgresQueryBuilder),
            r#"ALTER TABLE "models" DROP COLUMN "settings", DROP COLUMN "hidden", DROP COLUMN "enabled""#
        );
    }

    #[test]
    fn provider_availability_defaults_to_enabled() {
        assert_eq!(
            Migration::add_provider_columns().to_string(PostgresQueryBuilder),
            r#"ALTER TABLE "providers" ADD COLUMN "enabled" bool NOT NULL DEFAULT TRUE"#
        );
        assert_eq!(
            Migration::drop_provider_columns().to_string(PostgresQueryBuilder),
            r#"ALTER TABLE "providers" DROP COLUMN "enabled""#
        );
    }

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
    fn organization_scoped_provider_lookup_has_an_index() {
        assert_eq!(
            Migration::organization_provider_index().to_string(PostgresQueryBuilder),
            r#"CREATE INDEX "model_providers_organization_id_provider_id_idx" ON "model_providers" ("organization_id", "provider_id")"#
        );
    }
}
