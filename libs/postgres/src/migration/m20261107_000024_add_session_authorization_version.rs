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
                        ColumnDef::new(UserSessions::AuthorizationVersion)
                            .big_integer()
                            .not_null()
                            .default(1_i64)
                            .check(
                                Expr::col(UserSessions::AuthorizationVersion)
                                    .between(1_i64, 4_294_967_295_i64),
                            ),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("user_sessions_organization_session_user_idx")
                    .table(UserSessions::Table)
                    .col(UserSessions::OrganizationId)
                    .col(UserSessions::Id)
                    .col(UserSessions::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("user_sessions_organization_session_user_idx")
                    .table(UserSessions::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(UserSessions::Table)
                    .drop_column(UserSessions::AuthorizationVersion)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserSessions {
    Table,
    Id,
    OrganizationId,
    UserId,
    AuthorizationVersion,
}
