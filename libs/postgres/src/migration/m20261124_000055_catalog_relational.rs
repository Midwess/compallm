use sea_orm_migration::prelude::*;
#[path = "m20261122_000054_catalog_search/import.rs"]
mod import;
#[path = "m20261122_000054_catalog_search/relational.rs"]
mod relational;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let functions = include_str!("m20261121_000052_capability_pricing/constraints.sql")
            .split("ALTER TABLE")
            .next()
            .ok_or_else(|| DbErr::Custom("Missing capability validation functions".into()))?
            .replace("CREATE FUNCTION", "CREATE OR REPLACE FUNCTION");
        manager
            .get_connection()
            .execute_unprepared(&functions)
            .await?;
        if manager.has_table("catalog_schema_rollback").await? {
            return Ok(());
        }
        import::up(manager).await?;
        relational::up(manager).await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        relational::down(manager).await?;
        import::down(manager).await?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261122_000054_catalog_search/views.sql"))
            .await?;
        Ok(())
    }
}
