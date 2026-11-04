use sea_orm_migration::{
    prelude::*,
    sea_query::{
        TableCreateStatement,
        extension::postgres::{Type, TypeCreateStatement},
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(organization_status_type()).await?;
        manager.create_table(organizations_table()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organizations::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OrganizationStatus::Enum).to_owned())
            .await
    }
}

fn organizations_table() -> TableCreateStatement {
    Table::create()
        .table(Organizations::Table)
        .col(
            ColumnDef::new(Organizations::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(Organizations::Name).text().not_null())
        .col(
            ColumnDef::new(Organizations::Status)
                .custom(OrganizationStatus::Enum)
                .not_null()
                .default("active"),
        )
        .check(Expr::col(Organizations::Id).gt(0))
        .check(
            Func::char_length(Func::cust(Alias::new("btrim")).arg(Expr::col(Organizations::Name)))
                .gt(0),
        )
        .to_owned()
}

fn organization_status_type() -> TypeCreateStatement {
    Type::create()
        .as_enum(OrganizationStatus::Enum)
        .values([OrganizationStatus::Active, OrganizationStatus::Disabled])
        .to_owned()
}

#[derive(DeriveIden)]
enum OrganizationStatus {
    #[sea_orm(iden = "organization_status")]
    Enum,
    Active,
    Disabled,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
    Name,
    Status,
}
