use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationRoles::Table)
                    .col(
                        ColumnDef::new(OrganizationRoles::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationRoles::Id)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrganizationRoles::Name).text().not_null())
                    .col(
                        ColumnDef::new(OrganizationRoles::Description)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(OrganizationRoles::CedarSource)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationRoles::Revision)
                            .big_integer()
                            .not_null()
                            .default(1_i64),
                    )
                    .primary_key(
                        Index::create()
                            .col(OrganizationRoles::OrganizationId)
                            .col(OrganizationRoles::Id),
                    )
                    .index(
                        Index::create()
                            .name("organization_roles_organization_name_key")
                            .unique()
                            .col(OrganizationRoles::OrganizationId)
                            .col(OrganizationRoles::Name),
                    )
                    .check(Expr::col(OrganizationRoles::OrganizationId).gt(0))
                    .check(Expr::col(OrganizationRoles::Id).gt(0))
                    .check(Expr::col(OrganizationRoles::Revision).gt(0))
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim")).arg(Expr::col(OrganizationRoles::Name)),
                        )
                        .gt(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("organization_roles_organization_fk")
                            .from(OrganizationRoles::Table, OrganizationRoles::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrganizationRoles {
    Table,
    OrganizationId,
    Id,
    Name,
    Description,
    CedarSource,
    Revision,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
