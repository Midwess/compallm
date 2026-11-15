use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(r#"
CREATE TYPE public.organization_member_invitation_status AS ENUM ('pending', 'accepted', 'revoked');
CREATE TABLE public.organization_member_invitations (
 organization_id BIGINT NOT NULL REFERENCES public.organizations(id) ON DELETE CASCADE,
 id BIGINT NOT NULL CHECK (id > 0),
 email TEXT NOT NULL CHECK (email = lower(btrim(email))),
 inviter_id BIGINT NOT NULL REFERENCES public.users(id),
 status public.organization_member_invitation_status NOT NULL,
 expires_at BIGINT NOT NULL CHECK (expires_at > 0),
 accepted_at BIGINT,
 accepted_by BIGINT REFERENCES public.users(id),
 revoked_at BIGINT,
 PRIMARY KEY (organization_id, id),
 CHECK ((status = 'pending' AND accepted_at IS NULL AND accepted_by IS NULL AND revoked_at IS NULL)
 OR (status = 'accepted' AND accepted_at IS NOT NULL AND accepted_by IS NOT NULL AND revoked_at IS NULL)
 OR (status = 'revoked' AND accepted_at IS NULL AND accepted_by IS NULL AND revoked_at IS NOT NULL))
);
CREATE INDEX organization_member_invitations_email ON public.organization_member_invitations (organization_id, email);
ALTER TABLE public.organization_member_invitations ENABLE ROW LEVEL SECURITY;
GRANT SELECT, INSERT ON public.organization_member_invitations TO compallm_restricted;
GRANT UPDATE (status, revoked_at) ON public.organization_member_invitations TO compallm_restricted;
CREATE POLICY invitation_scope ON public.organization_member_invitations AS RESTRICTIVE TO compallm_restricted
 USING (organization_id = (SELECT compallm_rls.organization_id()))
 WITH CHECK (organization_id = (SELECT compallm_rls.organization_id()));
CREATE POLICY invitation_read ON public.organization_member_invitations FOR SELECT TO compallm_restricted
 USING ((SELECT compallm_rls.organization_allowed('member.read')));
CREATE POLICY invitation_create ON public.organization_member_invitations FOR INSERT TO compallm_restricted
 WITH CHECK ((SELECT compallm_rls.organization_allowed('member.invite')) AND status = 'pending' AND inviter_id = (SELECT compallm_rls.user_id()) AND accepted_at IS NULL AND accepted_by IS NULL AND revoked_at IS NULL);
CREATE POLICY invitation_update ON public.organization_member_invitations FOR UPDATE TO compallm_restricted
 USING ((SELECT compallm_rls.organization_allowed('member.invite')) AND status = 'pending')
 WITH CHECK ((SELECT compallm_rls.organization_allowed('member.invite')) AND status = 'revoked' AND revoked_at IS NOT NULL AND accepted_at IS NULL AND accepted_by IS NULL);
ALTER TABLE public.organization_roles ADD COLUMN default_member BOOLEAN NOT NULL DEFAULT FALSE;
GRANT SELECT (default_member) ON public.organization_roles TO compallm_restricted;
UPDATE public.organization_roles SET default_member = TRUE WHERE name = 'Member' AND NOT org_owner;
CREATE UNIQUE INDEX organization_default_member_role ON public.organization_roles (organization_id) WHERE default_member;
"#).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared("DROP TABLE public.organization_member_invitations; DROP TYPE public.organization_member_invitation_status; ALTER TABLE public.organization_roles DROP COLUMN default_member;").await?;
        Ok(())
    }
}
