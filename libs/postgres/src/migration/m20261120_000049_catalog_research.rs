use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(include_str!("m20261120_000049_catalog_research/up.sql"))
            .await?;
        db.execute_unprepared(include_str!("m20261120_000049_catalog_research/views.sql"))
            .await?;
        db.execute_unprepared(include_str!(
            "m20261120_000049_catalog_research/validation.sql"
        ))
        .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261120_000049_catalog_research/down.sql"))
            .await?;
        Ok(())
    }
}
