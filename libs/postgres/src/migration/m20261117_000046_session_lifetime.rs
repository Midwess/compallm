use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let epoch = sea_orm::prelude::DateTimeUtc::UNIX_EPOCH.fixed_offset();
        manager
            .alter_table(
                Table::alter()
                    .table(UserSessions::Table)
                    .add_column(
                        ColumnDef::new(UserSessions::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(epoch),
                    )
                    .add_column(
                        ColumnDef::new(UserSessions::LastSeenAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(epoch),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_sessions_owner_activity")
                    .table(UserSessions::Table)
                    .col(UserSessions::UserId)
                    .col((UserSessions::LastSeenAt, IndexOrder::Desc))
                    .col((UserSessions::Id, IndexOrder::Desc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_sessions_owner_activity")
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(UserSessions::Table)
                    .drop_column(UserSessions::ExpiresAt)
                    .drop_column(UserSessions::LastSeenAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserSessions {
    Table,
    Id,
    UserId,
    ExpiresAt,
    LastSeenAt,
}
