use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Assignments::Table)
                    .col(
                        ColumnDef::new(Assignments::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Assignments::GroupId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Assignments::RoleId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(Assignments::OrganizationId)
                            .col(Assignments::GroupId)
                            .col(Assignments::RoleId),
                    )
                    .check(Expr::col(Assignments::OrganizationId).gt(0))
                    .check(Expr::col(Assignments::GroupId).gt(0))
                    .check(Expr::col(Assignments::RoleId).gt(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("organization_group_roles_group_id_scope_fk")
                            .from(
                                Assignments::Table,
                                (Assignments::OrganizationId, Assignments::GroupId),
                            )
                            .to(
                                LeftParent::Table,
                                (LeftParent::OrganizationId, LeftParent::Id),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("organization_group_roles_role_id_scope_fk")
                            .from(
                                Assignments::Table,
                                (Assignments::OrganizationId, Assignments::RoleId),
                            )
                            .to(
                                RightParent::Table,
                                (RightParent::OrganizationId, RightParent::Id),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("organization_group_roles_reverse_idx")
                    .table(Assignments::Table)
                    .col(Assignments::OrganizationId)
                    .col(Assignments::RoleId)
                    .col(Assignments::GroupId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Assignments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Assignments {
    #[sea_orm(iden = "organization_group_roles")]
    Table,
    OrganizationId,
    GroupId,
    RoleId,
}

#[derive(DeriveIden)]
enum LeftParent {
    #[sea_orm(iden = "organization_groups")]
    Table,
    OrganizationId,
    Id,
}

#[derive(DeriveIden)]
enum RightParent {
    #[sea_orm(iden = "organization_roles")]
    Table,
    OrganizationId,
    Id,
}
