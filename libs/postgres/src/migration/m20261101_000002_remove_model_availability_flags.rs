use sea_orm_migration::{prelude::*, sea_query::TableAlterStatement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::drop_availability_flags()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::add_availability_flags()).await
    }
}

impl Migration {
    /// Availability and visibility belong to the router bindings and to
    /// Gateway access rules, so the model keeps no flags of its own.
    fn drop_availability_flags() -> TableAlterStatement {
        Table::alter()
            .table(Models::Table)
            .drop_column(Models::Hidden)
            .drop_column(Models::Enabled)
            .to_owned()
    }

    fn add_availability_flags() -> TableAlterStatement {
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
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Enabled,
    Hidden,
}
