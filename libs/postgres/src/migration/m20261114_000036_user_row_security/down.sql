REVOKE UPDATE (navigation, sidebar, member_display, calendar, theme, typography,
  accessibility, message_composition, code_appearance) ON public.user_preferences FROM compallm_restricted;
REVOKE SELECT, INSERT ON public.user_preferences FROM compallm_restricted;
DROP POLICY compallm_preference_update ON public.user_preferences;
DROP POLICY compallm_preference_insert ON public.user_preferences;
DROP POLICY compallm_preference_read ON public.user_preferences;
ALTER TABLE public.user_preferences DISABLE ROW LEVEL SECURITY;

REVOKE UPDATE (display_name, username, avatar_path) ON public.users FROM compallm_restricted;
REVOKE SELECT ON public.users FROM compallm_restricted;
DROP POLICY compallm_user_update ON public.users;
DROP POLICY compallm_user_read ON public.users;

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
DROP FUNCTION compallm_rls.user_id();
