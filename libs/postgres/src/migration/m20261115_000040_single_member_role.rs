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
ALTER TABLE organization_members ADD COLUMN role_id BIGINT;
UPDATE organization_members m SET role_id = a.role_id FROM organization_member_roles a
 WHERE (a.organization_id,a.user_id)=(m.organization_id,m.user_id);
ALTER TABLE organization_members ALTER COLUMN role_id SET NOT NULL;
ALTER TABLE organization_members ADD CONSTRAINT organization_members_role_scope_fk
 FOREIGN KEY(organization_id,role_id) REFERENCES organization_roles(organization_id,id);
CREATE INDEX organization_members_role_idx ON organization_members(organization_id,role_id);
DROP TABLE organization_member_roles;
DO $$ BEGIN
 IF to_regprocedure('compallm_rls.entity_in(jsonb,jsonb)') IS NOT NULL THEN
  EXECUTE replace(pg_get_functiondef('compallm_rls.entity_in(jsonb,jsonb)'::regprocedure),
   'public.organization_member_roles r', 'public.organization_members r');
 END IF;
END $$;
DO $policy$ BEGIN
 IF EXISTS(SELECT 1 FROM pg_policies WHERE schemaname='public' AND tablename='organization_members' AND policyname='compallm_insert') THEN
  ALTER POLICY compallm_insert ON organization_members WITH CHECK (
   (SELECT compallm_rls.organization_allowed('member.invite'))
   AND compallm_rls.allowed('member.role.assign', compallm_rls.entity('Member',user_id,organization_id,NULL,NULL,user_id), compallm_rls.load_entity('Role',organization_id,role_id))
   AND compallm_rls.allowed('role.assign', compallm_rls.load_entity('Role',organization_id,role_id)));
  ALTER POLICY compallm_insert_guard ON organization_members WITH CHECK (
   (SELECT compallm_rls.organization_allowed('member.invite'))
   AND compallm_rls.allowed('member.role.assign', compallm_rls.entity('Member',user_id,organization_id,NULL,NULL,user_id), compallm_rls.load_entity('Role',organization_id,role_id))
   AND compallm_rls.allowed('role.assign', compallm_rls.load_entity('Role',organization_id,role_id)));
 END IF;
END $policy$;
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
CREATE TABLE organization_member_roles (
 organization_id BIGINT NOT NULL, user_id BIGINT NOT NULL, role_id BIGINT NOT NULL,
 PRIMARY KEY(organization_id,user_id,role_id),
 CONSTRAINT organization_member_roles_user_id_scope_fk FOREIGN KEY(organization_id,user_id)
 REFERENCES organization_members(organization_id,user_id) ON DELETE CASCADE,
 CONSTRAINT organization_member_roles_role_id_scope_fk FOREIGN KEY(organization_id,role_id)
 REFERENCES organization_roles(organization_id,id) ON DELETE CASCADE
);
CREATE INDEX organization_member_roles_reverse_idx ON organization_member_roles(organization_id,role_id,user_id);
INSERT INTO organization_member_roles SELECT organization_id,user_id,role_id FROM organization_members;
DO $$ BEGIN
 IF to_regprocedure('compallm_rls.entity_in(jsonb,jsonb)') IS NOT NULL THEN
  EXECUTE replace(pg_get_functiondef('compallm_rls.entity_in(jsonb,jsonb)'::regprocedure),
   'public.organization_members r', 'public.organization_member_roles r');
 END IF;
END $$;
DO $policy$ BEGIN
 IF EXISTS(SELECT 1 FROM pg_policies WHERE schemaname='public' AND tablename='organization_members' AND policyname='compallm_insert') THEN
  ALTER POLICY compallm_insert ON organization_members WITH CHECK ((SELECT compallm_rls.organization_allowed('member.invite')));
  ALTER POLICY compallm_insert_guard ON organization_members WITH CHECK ((SELECT compallm_rls.organization_allowed('member.invite')));
 END IF;
END $policy$;
ALTER TABLE organization_members DROP COLUMN role_id;
DO $restore$ BEGIN IF to_regnamespace('compallm_rls') IS NOT NULL AND EXISTS(SELECT 1 FROM pg_roles WHERE rolname='compallm_restricted') THEN
ALTER TABLE public.organization_member_roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY compallm_scope ON public.organization_member_roles AS RESTRICTIVE FOR ALL TO compallm_restricted USING (organization_id = (SELECT compallm_rls.organization_id())) WITH CHECK (organization_id = (SELECT compallm_rls.organization_id()));
CREATE POLICY compallm_read ON public.organization_member_roles FOR SELECT TO compallm_restricted USING ((SELECT compallm_rls.organization_allowed('member.read')));
CREATE POLICY compallm_read_guard ON public.organization_member_roles AS RESTRICTIVE FOR SELECT TO compallm_restricted USING ((SELECT compallm_rls.organization_allowed('member.read')));
CREATE POLICY compallm_insert ON public.organization_member_roles FOR INSERT TO compallm_restricted WITH CHECK (compallm_rls.allowed('member.role.assign', compallm_rls.load_entity('Member', organization_id, user_id), compallm_rls.load_entity('Role', organization_id, role_id)) AND compallm_rls.allowed('role.assign', compallm_rls.load_entity('Role', organization_id, role_id)));
CREATE POLICY compallm_insert_guard ON public.organization_member_roles AS RESTRICTIVE FOR INSERT TO compallm_restricted WITH CHECK (compallm_rls.allowed('member.role.assign', compallm_rls.load_entity('Member', organization_id, user_id), compallm_rls.load_entity('Role', organization_id, role_id)) AND compallm_rls.allowed('role.assign', compallm_rls.load_entity('Role', organization_id, role_id)));
CREATE POLICY compallm_delete ON public.organization_member_roles FOR DELETE TO compallm_restricted USING (compallm_rls.allowed('member.role.unassign', compallm_rls.load_entity('Member', organization_id, user_id), compallm_rls.load_entity('Role', organization_id, role_id)) AND compallm_rls.allowed('role.unassign', compallm_rls.load_entity('Role', organization_id, role_id)));
CREATE POLICY compallm_delete_guard ON public.organization_member_roles AS RESTRICTIVE FOR DELETE TO compallm_restricted USING (compallm_rls.allowed('member.role.unassign', compallm_rls.load_entity('Member', organization_id, user_id), compallm_rls.load_entity('Role', organization_id, role_id)) AND compallm_rls.allowed('role.unassign', compallm_rls.load_entity('Role', organization_id, role_id)));
GRANT SELECT, INSERT, DELETE ON public.organization_member_roles TO compallm_restricted;
END IF; END $restore$;
"#,
            )
            .await
            .map(|_| ())
    }
}
