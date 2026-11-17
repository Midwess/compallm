use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserSessions::Table)
                    .add_column(
                        ColumnDef::new(UserSessions::IpAddress)
                            .string_len(45)
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(UserSessions::UserAgent)
                            .string_len(512)
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(UserSessions::Location)
                            .string_len(256)
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserSessions::Table)
                    .drop_column(UserSessions::Location)
                    .drop_column(UserSessions::UserAgent)
                    .drop_column(UserSessions::IpAddress)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserSessions {
    Table,
    IpAddress,
    UserAgent,
    Location,
}
