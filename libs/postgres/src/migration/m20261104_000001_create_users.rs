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
        manager.create_type(user_status_type()).await?;
        manager.create_table(users_table()).await?;
        manager.create_index(email_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(UserStatus::Enum).to_owned())
            .await
    }
}

fn users_table() -> TableCreateStatement {
    Table::create()
        .table(Users::Table)
        .col(
            ColumnDef::new(Users::Id)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(Users::Email).text().not_null())
        .col(ColumnDef::new(Users::DisplayName).text().not_null())
        .col(
            ColumnDef::new(Users::Status)
                .custom(UserStatus::Enum)
                .not_null()
                .default("active"),
        )
        .col(
            ColumnDef::new(Users::EmailVerifiedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(Users::LastLoginAt)
                .timestamp_with_time_zone()
                .null(),
        )
        .check(Expr::col(Users::Id).gt(0))
        .check(Expr::col(Users::Email).ne(""))
        .to_owned()
}

fn user_status_type() -> TypeCreateStatement {
    Type::create()
        .as_enum(UserStatus::Enum)
        .values([UserStatus::Active, UserStatus::Disabled])
        .to_owned()
}

fn email_index() -> IndexCreateStatement {
    Index::create()
        .name("users_email_unique")
        .table(Users::Table)
        .col(Func::lower(Expr::col(Users::Email)))
        .unique()
        .to_owned()
}

#[derive(DeriveIden)]
enum UserStatus {
    #[sea_orm(iden = "user_status")]
    Enum,
    Active,
    Disabled,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    DisplayName,
    Status,
    EmailVerifiedAt,
    LastLoginAt,
}
