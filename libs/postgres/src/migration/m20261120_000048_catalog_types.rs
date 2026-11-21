use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(include_str!(
            "m20261120_000048_catalog_types/model_info.sql"
        ))
        .await?;
        db.execute_unprepared(include_str!("m20261120_000048_catalog_types/up.sql"))
            .await?;
        db.execute_unprepared(include_str!("m20261120_000048_catalog_types/semantics.sql"))
            .await?;
        db.execute_unprepared(include_str!(
            "m20261120_000048_catalog_types/constraints.sql"
        ))
        .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(include_str!("m20261120_000048_catalog_types/down.sql"))
            .await?;
        let info = include_str!("m20261117_000047_provider_catalog/model_info_constraints.sql");
        db.execute_unprepared(
            &info[..info.find("ALTER TABLE").unwrap()]
                .replace("CREATE FUNCTION", "CREATE OR REPLACE FUNCTION"),
        )
        .await?;
        let previous = include_str!("m20261117_000047_provider_catalog/catalog_semantics.sql");
        db.execute_unprepared(
            &previous[previous.find("CREATE FUNCTION").unwrap()..]
                .replace("CREATE FUNCTION", "CREATE OR REPLACE FUNCTION"),
        )
        .await?;
        db.execute_unprepared(include_str!("m20261120_000048_catalog_types/restore.sql"))
            .await?;
        Ok(())
    }
}
