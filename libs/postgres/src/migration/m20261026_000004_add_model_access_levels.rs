use sea_orm_migration::{prelude::*, sea_query::TableAlterStatement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::add_membership_level()).await?;
        manager.alter_table(Self::add_model_min_level()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Models::Table)
                    .drop_column(Models::MinLevel)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OrganizationMembers::Table)
                    .drop_column(OrganizationMembers::Level)
                    .to_owned(),
            )
            .await
    }
}

impl Migration {
    fn add_membership_level() -> TableAlterStatement {
        Table::alter()
            .table(OrganizationMembers::Table)
            .add_column(
                ColumnDef::new(OrganizationMembers::Level)
                    .integer()
                    .not_null()
                    .default(0)
                    .check((
                        "organization_members_level_nonnegative",
                        Expr::col(OrganizationMembers::Level).gte(0),
                    )),
            )
            .to_owned()
    }

    fn add_model_min_level() -> TableAlterStatement {
        Table::alter()
            .table(Models::Table)
            .add_column(
                ColumnDef::new(Models::MinLevel)
                    .integer()
                    .not_null()
                    .default(0)
                    .check((
                        "models_min_level_nonnegative",
                        Expr::col(Models::MinLevel).gte(0),
                    )),
            )
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    Level,
}

#[derive(DeriveIden)]
enum Models {
    Table,
    MinLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_and_future_memberships_default_to_level_zero() {
        assert_eq!(
            Migration::add_membership_level().to_string(PostgresQueryBuilder),
            r#"ALTER TABLE "organization_members" ADD COLUMN "level" integer NOT NULL DEFAULT 0 CONSTRAINT "organization_members_level_nonnegative" CHECK ("level" >= 0)"#
        );
    }

    #[test]
    fn existing_and_future_models_default_to_minimum_level_zero() {
        assert_eq!(
            Migration::add_model_min_level().to_string(PostgresQueryBuilder),
            r#"ALTER TABLE "models" ADD COLUMN "min_level" integer NOT NULL DEFAULT 0 CONSTRAINT "models_min_level_nonnegative" CHECK ("min_level" >= 0)"#
        );
    }
}
