use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(include_str!("m20261110_000028_restricted_reads/up.sql"))
            .await?;
        db.execute_unprepared(include_str!(
            "m20261110_000028_restricted_reads/policies.sql"
        ))
        .await?;
        db.execute_unprepared(include_str!("m20261110_000028_restricted_reads/guards.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261110_000028_restricted_reads/down.sql"))
            .await?;
        Ok(())
    }
}
