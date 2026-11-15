use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(r#"
DO $$ BEGIN
 IF EXISTS (SELECT 1 FROM organization_member_roles GROUP BY organization_id,user_id HAVING count(*)>1) THEN
  RAISE EXCEPTION 'Resolve multiple direct roles before applying the single-role migration';
 END IF;
END $$;
DO $$ BEGIN
 IF EXISTS (SELECT 1 FROM organization_members m WHERE NOT EXISTS
 (SELECT 1 FROM organization_member_roles a WHERE (a.organization_id,a.user_id)=(m.organization_id,m.user_id))) THEN
  RAISE EXCEPTION 'Resolve missing direct roles before applying the single-role migration';
 END IF;
END $$;
ALTER TABLE organization_member_roles DROP CONSTRAINT organization_member_roles_pkey;
ALTER TABLE organization_member_roles ADD CONSTRAINT organization_member_roles_pkey PRIMARY KEY(organization_id,user_id);
ALTER TABLE organization_members ADD CONSTRAINT organization_member_required_role
 FOREIGN KEY(organization_id,user_id) REFERENCES organization_member_roles(organization_id,user_id) DEFERRABLE INITIALLY DEFERRED;
UPDATE user_sessions s SET authorization_version=authorization_version+1
 WHERE revoked_at IS NULL AND EXISTS (
 SELECT 1 FROM organization_group_members gm JOIN organization_group_roles gr USING(organization_id,group_id)
 JOIN organization_roles r ON (r.organization_id,r.id)=(gr.organization_id,gr.role_id)
 WHERE r.org_owner AND (gm.organization_id,gm.user_id)=(s.organization_id,s.user_id));
DELETE FROM organization_group_roles gr USING organization_roles r
 WHERE (r.organization_id,r.id)=(gr.organization_id,gr.role_id) AND r.org_owner;
ALTER TABLE organization_roles ADD CONSTRAINT organization_role_owner_identity UNIQUE(organization_id,id,org_owner);
ALTER TABLE organization_group_roles ADD COLUMN owner_role BOOLEAN NOT NULL DEFAULT FALSE CHECK (NOT owner_role);
ALTER TABLE organization_group_roles ADD CONSTRAINT organization_group_role_not_owner
 FOREIGN KEY(organization_id,role_id,owner_role) REFERENCES organization_roles(organization_id,id,org_owner);
"#).await.map(|_| ())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
ALTER TABLE organization_group_roles DROP CONSTRAINT organization_group_role_not_owner;
ALTER TABLE organization_group_roles DROP COLUMN owner_role;
ALTER TABLE organization_roles DROP CONSTRAINT organization_role_owner_identity;
ALTER TABLE organization_members DROP CONSTRAINT organization_member_required_role;
ALTER TABLE organization_member_roles DROP CONSTRAINT organization_member_roles_pkey;
ALTER TABLE organization_member_roles ADD CONSTRAINT organization_member_roles_pkey PRIMARY KEY(organization_id,user_id,role_id);
"#,
            )
            .await
            .map(|_| ())
    }
}
