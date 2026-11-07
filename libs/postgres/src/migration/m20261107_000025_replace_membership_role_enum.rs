//! Preserve legacy built-in assignments while enabling custom multi-role membership.
//! The Owner source is an immutable snapshot; later changes require new migrations.
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
const OWNER_SOURCE: &str = r###"// Application-owned initial Owner policy; organization/role IDs are trusted numeric values.
permit (
    principal in Compallm::Role::"__ORG__:1",
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"organization.delete", Compallm::Action::"member.list", Compallm::Action::"member.invite", Compallm::Action::"member.metadata.read", Compallm::Action::"member.remove", Compallm::Action::"provider.list", Compallm::Action::"provider.create", Compallm::Action::"provider.connection.test", Compallm::Action::"provider.configuration.read", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.list", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.read", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.list", Compallm::Action::"model.create", Compallm::Action::"model.metadata.read", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.list", Compallm::Action::"role.create", Compallm::Action::"role.metadata.read", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"role.assign", Compallm::Action::"role.unassign", Compallm::Action::"group.list", Compallm::Action::"group.create", Compallm::Action::"group.metadata.read", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in Compallm::Organization::"__ORG__"
);

permit (
    principal in Compallm::Role::"__ORG__:1",
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in Compallm::Organization::"__ORG__"
) when { context.role.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in Compallm::Role::"__ORG__:1",
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.member.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in Compallm::Role::"__ORG__:1",
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

forbid(principal in Compallm::Role::"__ORG__:1", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") unless { resource.enabled };
"###;
const ADMIN_SOURCE: &str = r###"// Application-owned initial Admin policy; organization/role IDs are trusted numeric values.
permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"member.list", Compallm::Action::"member.invite", Compallm::Action::"member.metadata.read", Compallm::Action::"member.remove", Compallm::Action::"provider.list", Compallm::Action::"provider.create", Compallm::Action::"provider.connection.test", Compallm::Action::"provider.configuration.read", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.list", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.read", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.list", Compallm::Action::"model.create", Compallm::Action::"model.metadata.read", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.list", Compallm::Action::"role.create", Compallm::Action::"role.metadata.read", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"group.list", Compallm::Action::"group.create", Compallm::Action::"group.metadata.read", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in Compallm::Organization::"__ORG__"
);

permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in Compallm::Organization::"__ORG__"
) when { context.role.organization == Compallm::Organization::"__ORG__" && context.role != Compallm::Role::"__ORG__:1" };

permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.member.organization == Compallm::Organization::"__ORG__" }
unless { resource in Compallm::Role::"__ORG__:1" };

permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

permit (
 principal in Compallm::Role::"__ORG__:2",
 action in [Compallm::Action::"role.assign", Compallm::Action::"role.unassign"],
 resource is Compallm::Role in Compallm::Organization::"__ORG__"
) unless { resource == Compallm::Role::"__ORG__:1" };

forbid(principal in Compallm::Role::"__ORG__:2", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") unless { resource.enabled };
"###;
const MEMBER_SOURCE: &str = r###"// Default members discover models and invoke enabled logical models only.
permit(principal in Compallm::Role::"__ORG__:3", action == Compallm::Action::"model.list",
 resource == Compallm::Organization::"__ORG__");
permit(principal in Compallm::Role::"__ORG__:3", action == Compallm::Action::"model.metadata.read",
 resource is Compallm::Model in Compallm::Organization::"__ORG__");
permit(principal in Compallm::Role::"__ORG__:3", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") when { resource.enabled };
"###;
const UP: &str = r#"
INSERT INTO organization_roles(organization_id,id,name,description,cedar_source)
 SELECT o.id,seed.id,seed.name,'',replace(seed.source,'__ORG__',o.id::text)
 FROM organizations o CROSS JOIN (VALUES (1::bigint,'Owner',$1),(2::bigint,'Admin',$2),(3::bigint,'Member',$3)) seed(id,name,source);
INSERT INTO organization_member_roles(organization_id,user_id,role_id)
 SELECT m.organization_id,m.user_id,CASE m.role::text WHEN 'owner' THEN 1 WHEN 'admin' THEN 2 WHEN 'member' THEN 3 END
 FROM organization_members m;
ALTER TABLE organization_members DROP COLUMN role;
DROP TYPE organization_role;
CREATE FUNCTION protect_default_organization_role() RETURNS trigger LANGUAGE plpgsql AS $$
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
 FOR EACH ROW EXECUTE FUNCTION protect_default_organization_role();
"#;
const DOWN: &str = r#"
DO $$ BEGIN
 IF EXISTS (SELECT 1 FROM organization_group_members) OR EXISTS (
   SELECT 1 FROM organization_members m LEFT JOIN organization_member_roles mr
    ON mr.organization_id=m.organization_id AND mr.user_id=m.user_id
   GROUP BY m.organization_id,m.user_id HAVING count(mr.role_id)<>1 OR bool_or(mr.role_id NOT IN (1,2,3))
 ) THEN RAISE EXCEPTION 'cannot downgrade multi-role/custom/group memberships'; END IF;
END $$;
CREATE TYPE organization_role AS ENUM ('owner','admin','member');
ALTER TABLE organization_members ADD COLUMN role organization_role;
UPDATE organization_members m SET role=(CASE mr.role_id WHEN 1 THEN 'owner' WHEN 2 THEN 'admin' WHEN 3 THEN 'member' END)::organization_role
 FROM organization_member_roles mr WHERE mr.organization_id=m.organization_id AND mr.user_id=m.user_id;
ALTER TABLE organization_members ALTER COLUMN role SET NOT NULL;
DROP TRIGGER protect_default_organization_role ON organization_roles;
DROP FUNCTION protect_default_organization_role();
DELETE FROM organization_roles WHERE id IN (1,2,3);
"#;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Only trusted immutable source is quoted here: PostgreSQL prepared statements
        // cannot contain this multi-command migration batch. No caller input is accepted.
        let quote = |source: &str| format!("'{}'", source.replace(char::from(39), "''"));
        manager
            .get_connection()
            .execute_unprepared(
                &UP.replace("$1", &quote(OWNER_SOURCE))
                    .replace("$2", &quote(ADMIN_SOURCE))
                    .replace("$3", &quote(MEMBER_SOURCE)),
            )
            .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(DOWN).await?;
        Ok(())
    }
}
