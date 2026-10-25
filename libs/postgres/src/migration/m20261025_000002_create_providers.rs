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
        manager.create_type(Self::api_protocol_type()).await?;
        manager.create_table(Self::providers_table()).await?;
        manager.create_index(Self::organization_name_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Providers::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ApiProtocol::Enum).to_owned())
            .await
    }
}

impl Migration {
    fn api_protocol_type() -> TypeCreateStatement {
        Type::create()
            .as_enum(ApiProtocol::Enum)
            .values([
                ApiProtocol::Anthropic,
                ApiProtocol::Openai,
                ApiProtocol::OpenaiCompatible,
            ])
            .to_owned()
    }

    fn providers_table() -> TableCreateStatement {
        Table::create()
            .table(Providers::Table)
            // Hero generates the ID in the application; no database sequence.
            .col(
                ColumnDef::new(Providers::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Providers::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Providers::Name).text().not_null())
            .col(ColumnDef::new(Providers::Url).text().not_null())
            // Application code must encrypt the API key before persistence.
            .col(ColumnDef::new(Providers::ApiKey).text().not_null())
            .col(
                ColumnDef::new(Providers::ApiProtocol)
                    .custom(ApiProtocol::Enum)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Providers::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .check(Expr::col(Providers::Id).gt(0))
            .check(Expr::cust("length(btrim(name)) > 0"))
            .check(Expr::cust("length(btrim(url)) > 0"))
            .check(Expr::cust("length(api_key) > 0"))
            .foreign_key(
                ForeignKey::create()
                    .name("providers_organization_id_fkey")
                    .from(Providers::Table, Providers::OrganizationId)
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn organization_name_index() -> IndexCreateStatement {
        Index::create()
            .name("providers_organization_id_name_unique")
            .table(Providers::Table)
            .col(Providers::OrganizationId)
            .col(Providers::Name)
            .unique()
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum ApiProtocol {
    #[sea_orm(iden = "api_protocol")]
    Enum,
    Anthropic,
    Openai,
    OpenaiCompatible,
}

#[derive(DeriveIden)]
enum Providers {
    Table,
    Id,
    OrganizationId,
    Name,
    Url,
    ApiKey,
    ApiProtocol,
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
    fn provider_is_an_organization_scoped_connection() {
        let sql = Migration::providers_table().to_string(PostgresQueryBuilder);

        assert!(sql.contains(r#""id" bigint NOT NULL PRIMARY KEY"#));
        assert!(!sql.to_lowercase().contains("serial"));
        assert!(sql.contains(r#""name" text NOT NULL"#));
        assert!(sql.contains(r#""url" text NOT NULL"#));
        assert!(sql.contains(r#""api_key" text NOT NULL"#));
        assert!(sql.contains(r#""api_protocol" api_protocol NOT NULL"#));
        assert!(sql.contains(
            r#"FOREIGN KEY ("organization_id") REFERENCES "organizations" ("id") ON DELETE CASCADE"#
        ));
    }

    #[test]
    fn api_protocol_is_a_native_postgres_enum() {
        assert_eq!(
            Migration::api_protocol_type().to_string(PostgresQueryBuilder),
            r#"CREATE TYPE "api_protocol" AS ENUM ('anthropic', 'openai', 'openai_compatible')"#
        );
    }

    #[test]
    fn provider_name_is_unique_within_an_organization() {
        assert_eq!(
            Migration::organization_name_index().to_string(PostgresQueryBuilder),
            r#"CREATE UNIQUE INDEX "providers_organization_id_name_unique" ON "providers" ("organization_id", "name")"#
        );
    }
}
