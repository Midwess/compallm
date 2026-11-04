use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(user_sessions_table()).await?;
        manager.create_index(scope_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSessions::Table).to_owned())
            .await
    }
}

fn user_sessions_table() -> TableCreateStatement {
    Table::create()
        .table(UserSessions::Table)
        .col(
            ColumnDef::new(UserSessions::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(UserSessions::UserId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(UserSessions::OrganizationId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(UserSessions::RevokedAt)
                .timestamp_with_time_zone()
                .null(),
        )
        .check(Expr::col(UserSessions::Id).gt(0))
        .foreign_key(
            ForeignKey::create()
                .name("user_sessions_organization_fk")
                .from(UserSessions::Table, UserSessions::OrganizationId)
                .to(Organizations::Table, Organizations::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .foreign_key(
            ForeignKey::create()
                .name("user_sessions_user_fk")
                .from(UserSessions::Table, UserSessions::UserId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn scope_index() -> IndexCreateStatement {
    Index::create()
        .name("user_sessions_organization_user_idx")
        .table(UserSessions::Table)
        .col(UserSessions::OrganizationId)
        .col(UserSessions::UserId)
        .to_owned()
}

#[derive(DeriveIden)]
enum UserSessions {
    Table,
    Id,
    UserId,
    OrganizationId,
    RevokedAt,
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
