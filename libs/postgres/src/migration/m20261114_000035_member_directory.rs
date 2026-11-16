use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(r#"
ALTER TABLE public.users ENABLE ROW LEVEL SECURITY;
CREATE POLICY compallm_member_directory ON public.users FOR SELECT TO compallm_restricted
USING ((SELECT compallm_rls.organization_allowed('member.read')) AND EXISTS (
 SELECT 1 FROM public.organization_members m
 WHERE m.organization_id = (SELECT compallm_rls.organization_id()) AND m.user_id = users.id
));
CREATE POLICY compallm_member_directory_guard ON public.users AS RESTRICTIVE FOR SELECT TO compallm_restricted
USING ((SELECT compallm_rls.organization_allowed('member.read')) AND EXISTS (
 SELECT 1 FROM public.organization_members m
 WHERE m.organization_id = (SELECT compallm_rls.organization_id()) AND m.user_id = users.id
));
GRANT SELECT (id, display_name, email, status) ON public.users TO compallm_restricted;
"#).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
REVOKE SELECT (id, display_name, email, status) ON public.users FROM compallm_restricted;
DROP POLICY compallm_member_directory_guard ON public.users;
DROP POLICY compallm_member_directory ON public.users;
ALTER TABLE public.users DISABLE ROW LEVEL SECURITY;
"#,
            )
            .await?;
        Ok(())
    }
}
