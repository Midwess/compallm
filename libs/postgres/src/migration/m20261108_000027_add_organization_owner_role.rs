use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "ALTER TABLE organization_roles ADD COLUMN org_owner boolean NOT NULL DEFAULT false; \
             UPDATE organization_roles SET org_owner = true WHERE name = 'Owner'; \
             CREATE UNIQUE INDEX organization_roles_one_owner_idx ON organization_roles (organization_id) WHERE org_owner;"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DROP INDEX organization_roles_one_owner_idx; ALTER TABLE organization_roles DROP COLUMN org_owner;"
        ).await?;
        Ok(())
    }
}
