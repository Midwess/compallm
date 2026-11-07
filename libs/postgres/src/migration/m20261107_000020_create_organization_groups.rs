use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationGroups::Table)
                    .col(
                        ColumnDef::new(OrganizationGroups::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationGroups::Id)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrganizationGroups::Name).text().not_null())
                    .col(
                        ColumnDef::new(OrganizationGroups::Description)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(OrganizationGroups::Revision)
                            .big_integer()
                            .not_null()
                            .default(1_i64),
                    )
                    .primary_key(
                        Index::create()
                            .col(OrganizationGroups::OrganizationId)
                            .col(OrganizationGroups::Id),
                    )
                    .index(
                        Index::create()
                            .name("organization_groups_organization_name_key")
                            .unique()
                            .col(OrganizationGroups::OrganizationId)
                            .col(OrganizationGroups::Name),
                    )
                    .check(Expr::col(OrganizationGroups::OrganizationId).gt(0))
                    .check(Expr::col(OrganizationGroups::Id).gt(0))
                    .check(Expr::col(OrganizationGroups::Revision).gt(0))
                    .check(
                        Func::char_length(
                            Func::cust(Alias::new("btrim"))
                                .arg(Expr::col(OrganizationGroups::Name)),
                        )
                        .gt(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("organization_groups_organization_fk")
                            .from(
                                OrganizationGroups::Table,
                                OrganizationGroups::OrganizationId,
                            )
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
            .drop_table(Table::drop().table(OrganizationGroups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrganizationGroups {
    Table,
    OrganizationId,
    Id,
    Name,
    Description,
    Revision,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
