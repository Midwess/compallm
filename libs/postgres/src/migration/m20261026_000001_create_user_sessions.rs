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
                ColumnDef::new(UserSessions::OrganizationId)
                    .big_integer()
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
            .foreign_key(
                ForeignKey::create()
                    .name("user_sessions_organization_fk")
                    .from(UserSessions::Table, UserSessions::OrganizationId)
                    .to(Alias::new("organizations"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade),
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
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Self::table()).await?;
        manager.create_index(Self::scope_index()).await
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
    OrganizationId,
    RevokedAt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_sessions_store_revocation_and_scope_without_duplicating_jwt_expiry() {
        let sql = Migration::table().to_string(PostgresQueryBuilder);
        assert!(sql.contains("\"id\" bigint NOT NULL PRIMARY KEY"));
        assert!(!sql.to_lowercase().contains("serial"));
        assert!(
            sql.contains(
                "FOREIGN KEY (\"user_id\") REFERENCES \"users\" (\"id\") ON DELETE CASCADE"
            )
        );
        assert!(sql.contains("\"revoked_at\" timestamp with time zone NULL"));
        assert!(sql.contains("\"organization_id\" bigint NOT NULL"));
        assert!(sql.contains("REFERENCES \"organizations\" (\"id\")"));
        assert!(!sql.contains("last_used_at"));
        assert!(!sql.contains("expires_at"));
        let index = Migration::scope_index().to_string(PostgresQueryBuilder);
        assert!(index.contains("(\"organization_id\", \"user_id\")"));
    }
}
