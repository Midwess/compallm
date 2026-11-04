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
        manager.create_type(organization_role_type()).await?;
        manager.create_table(organization_members_table()).await?;
        manager.create_index(organization_index()).await?;
        manager.create_index(user_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationMembers::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OrganizationRole::Enum).to_owned())
            .await
    }
}

fn organization_members_table() -> TableCreateStatement {
    Table::create()
        .table(OrganizationMembers::Table)
        .col(
            ColumnDef::new(OrganizationMembers::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(OrganizationMembers::UserId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(OrganizationMembers::Role)
                .custom(OrganizationRole::Enum)
                .not_null(),
        )
        .col(
            ColumnDef::new(OrganizationMembers::Status)
                .custom(UserStatus::Enum)
                .not_null()
                .default("active"),
        )
        .primary_key(
            Index::create()
                .name("organization_members_pkey")
                .col(OrganizationMembers::OrganizationId)
                .col(OrganizationMembers::UserId),
        )
        .foreign_key(
            ForeignKey::create()
                .name("organization_members_organization_id_fkey")
                .from(
                    OrganizationMembers::Table,
                    OrganizationMembers::OrganizationId,
                )
                .to(Organizations::Table, Organizations::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .foreign_key(
            ForeignKey::create()
                .name("organization_members_user_id_fkey")
                .from(OrganizationMembers::Table, OrganizationMembers::UserId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn user_index() -> IndexCreateStatement {
    Index::create()
        .name("organization_members_user_id_idx")
        .table(OrganizationMembers::Table)
        .col(OrganizationMembers::UserId)
        .to_owned()
}

fn organization_index() -> IndexCreateStatement {
    Index::create()
        .name("organization_members_organization_id_idx")
        .table(OrganizationMembers::Table)
        .col(OrganizationMembers::OrganizationId)
        .to_owned()
}

fn organization_role_type() -> TypeCreateStatement {
    Type::create()
        .as_enum(OrganizationRole::Enum)
        .values([
            OrganizationRole::Owner,
            OrganizationRole::Admin,
            OrganizationRole::Member,
        ])
        .to_owned()
}

#[derive(DeriveIden)]
enum OrganizationRole {
    #[sea_orm(iden = "organization_role")]
    Enum,
    Owner,
    Admin,
    Member,
}
#[derive(DeriveIden)]
enum UserStatus {
    #[sea_orm(iden = "user_status")]
    Enum,
}
#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
    Role,
    Status,
}
#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
