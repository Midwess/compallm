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
        manager
            .create_index(Self::organization_name_index())
            .await?;
        manager.create_index(Self::identity_scope_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Models::Table).to_owned())
            .await
    }
}

impl Migration {
    fn table() -> TableCreateStatement {
        Table::create()
            .table(Models::Table)
            // Hero generates the ID in the application; no database sequence.
            .col(
                ColumnDef::new(Models::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Models::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Models::Name).text().not_null())
            .col(
                ColumnDef::new(Models::InputPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Models::OutputPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Models::CachedPricePerMillionTokens)
                    .decimal_len(20, 8)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Models::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Models::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .check(Expr::col(Models::Id).gt(0))
            .check(Expr::cust("length(btrim(name)) > 0"))
            .check(Expr::col(Models::InputPricePerMillionTokens).gte(0))
            .check(Expr::col(Models::OutputPricePerMillionTokens).gte(0))
            .check(Expr::col(Models::CachedPricePerMillionTokens).gte(0))
            .foreign_key(
                ForeignKey::create()
                    .name("models_organization_id_fkey")
                    .from(Models::Table, Models::OrganizationId)
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn organization_name_index() -> IndexCreateStatement {
        // A logical model name identifies at most one model within an
        // organization, while another organization may reuse the same name.
        Index::create()
            .name("models_organization_id_name_unique")
            .table(Models::Table)
            .col(Models::OrganizationId)
            .col(Models::Name)
            .unique()
            .to_owned()
    }

    fn identity_scope_index() -> IndexCreateStatement {
        // PostgreSQL requires this exact pair to be unique before
        // model_providers can reference (model_id, organization_id) and thereby
        // reject routes whose model belongs to another organization.
        // https://www.postgresql.org/docs/current/sql-createtable.html#SQL-CREATETABLE-PARMS-REFERENCES
        Index::create()
            .name("models_id_organization_id_unique")
            .table(Models::Table)
            .col(Models::Id)
            .col(Models::OrganizationId)
            .unique()
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
    OrganizationId,
    Name,
    InputPricePerMillionTokens,
    OutputPricePerMillionTokens,
    CachedPricePerMillionTokens,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_is_owned_by_an_organization_and_uses_an_application_id() {
        let sql = Migration::table().to_string(PostgresQueryBuilder);

        assert!(sql.contains(r#""id" bigint NOT NULL PRIMARY KEY"#));
        assert!(!sql.to_lowercase().contains("serial"));
        assert!(sql.contains(r#""organization_id" bigint NOT NULL"#));
        assert!(sql.contains(r#""name" text NOT NULL"#));
        assert!(sql.contains(r#""input_price_per_million_tokens" decimal(20, 8) NOT NULL"#));
        assert!(sql.contains(r#""output_price_per_million_tokens" decimal(20, 8) NOT NULL"#));
        assert!(sql.contains(r#""cached_price_per_million_tokens" decimal(20, 8) NOT NULL"#));
        assert!(sql.contains("length(btrim(name)) > 0"));
        assert!(sql.contains(r#""input_price_per_million_tokens" >= 0"#));
        assert!(sql.contains(r#""output_price_per_million_tokens" >= 0"#));
        assert!(sql.contains(r#""cached_price_per_million_tokens" >= 0"#));
        assert!(sql.contains(
            r#"FOREIGN KEY ("organization_id") REFERENCES "organizations" ("id") ON DELETE CASCADE"#
        ));
        assert!(sql.contains(
            r#""created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP"#
        ));
        assert!(sql.contains(
            r#""updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP"#
        ));
    }

    #[test]
    fn model_name_is_unique_within_an_organization() {
        assert_eq!(
            Migration::organization_name_index().to_string(PostgresQueryBuilder),
            r#"CREATE UNIQUE INDEX "models_organization_id_name_unique" ON "models" ("organization_id", "name")"#
        );
    }

    #[test]
    fn model_identity_can_be_referenced_with_its_organization() {
        assert_eq!(
            Migration::identity_scope_index().to_string(PostgresQueryBuilder),
            r#"CREATE UNIQUE INDEX "models_id_organization_id_unique" ON "models" ("id", "organization_id")"#
        );
    }
}
