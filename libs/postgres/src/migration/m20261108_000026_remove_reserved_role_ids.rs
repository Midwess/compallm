//! Default-role IDs have no runtime privilege or reservation semantics.
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DROP TRIGGER IF EXISTS protect_default_organization_role ON organization_roles; DROP FUNCTION IF EXISTS protect_default_organization_role();"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Restore the preceding schema on rollback without changing role data.
        manager.get_connection().execute_unprepared(r#"CREATE FUNCTION protect_default_organization_role() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP='UPDATE' THEN
   IF OLD.id IN (1,2,3) OR NEW.id IN (1,2,3) THEN RAISE EXCEPTION 'default role cannot be modified'; END IF;
   RETURN NEW;
 END IF;
 IF OLD.id IN (1,2,3) AND EXISTS(SELECT 1 FROM organizations WHERE id=OLD.organization_id) THEN
   RAISE EXCEPTION 'default role cannot be deleted';
 END IF;
 RETURN OLD;
END $$;
CREATE TRIGGER protect_default_organization_role BEFORE UPDATE OR DELETE ON organization_roles
 FOR EACH ROW EXECUTE FUNCTION protect_default_organization_role();"#).await?;
        Ok(())
    }
}
