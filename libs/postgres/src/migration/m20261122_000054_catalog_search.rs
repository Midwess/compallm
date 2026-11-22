use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261122_000054_catalog_search/views.sql"))
            .await?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261122_000054_catalog_search/up.sql"))
            .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261122_000054_catalog_search/down.sql"))
            .await?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261122_000054_catalog_search/restore.sql"))
            .await?;
        Ok(())
    }
}
