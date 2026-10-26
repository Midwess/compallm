use sea_orm_migration::{
    prelude::*,
    sea_query::{IndexCreateStatement, TableCreateStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn table() -> TableCreateStatement {
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
                ColumnDef::new(UserSessions::RefreshTokenId)
                    .string_len(43)
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserSessions::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserSessions::LastUsedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserSessions::ExpiresAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserSessions::RevokedAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("user_sessions_user_fk")
                    .from(UserSessions::Table, UserSessions::UserId)
                    .to(Alias::new("users"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .check(Expr::col(UserSessions::Id).gt(0))
            .check(Expr::cust("refresh_token_id ~ '^[A-Za-z0-9_-]{43}$'"))
            .check(Expr::col(UserSessions::LastUsedAt).gte(Expr::col(UserSessions::CreatedAt)))
            .check(Expr::col(UserSessions::ExpiresAt).gt(Expr::col(UserSessions::LastUsedAt)))
            .check(
                Expr::col(UserSessions::RevokedAt)
                    .is_null()
                    .or(Expr::col(UserSessions::RevokedAt).gte(Expr::col(UserSessions::CreatedAt))),
            )
            .to_owned()
    }

    fn history_index() -> IndexCreateStatement {
        Index::create()
            .name("user_sessions_user_created_at_idx")
            .table(UserSessions::Table)
            .col(UserSessions::UserId)
            .col(UserSessions::CreatedAt)
            .to_owned()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Self::table()).await?;
        manager.create_index(Self::history_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserSessions {
    Table,
    Id,
    UserId,
    RefreshTokenId,
    CreatedAt,
    LastUsedAt,
    ExpiresAt,
    RevokedAt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_sessions_keep_login_and_activity_separate_without_storing_tokens() {
        let sql = Migration::table().to_string(PostgresQueryBuilder);
        assert!(sql.contains("\"id\" bigint NOT NULL PRIMARY KEY"));
        assert!(!sql.to_lowercase().contains("serial"));
        assert!(
            sql.contains(
                "FOREIGN KEY (\"user_id\") REFERENCES \"users\" (\"id\") ON DELETE CASCADE"
            )
        );
        assert!(sql.contains("\"refresh_token_id\" varchar(43) NOT NULL"));
        assert!(sql.contains("\"created_at\" timestamp with time zone NOT NULL"));
        assert!(sql.contains("\"last_used_at\" timestamp with time zone NOT NULL"));
        assert!(sql.contains("\"expires_at\" > \"last_used_at\""));
        assert!(sql.contains("\"revoked_at\" timestamp with time zone NULL"));
        let index = Migration::history_index().to_string(PostgresQueryBuilder);
        assert!(index.contains("(\"user_id\", \"created_at\")"));
    }
}
