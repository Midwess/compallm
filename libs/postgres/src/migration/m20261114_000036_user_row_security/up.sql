CREATE FUNCTION compallm_rls.user_id() RETURNS bigint
LANGUAGE plpgsql STABLE SET search_path = pg_catalog AS $$
DECLARE value bigint;
BEGIN
  IF current_setting('app.rls_version', true) IS DISTINCT FROM '1' THEN RETURN NULL; END IF;
  value := nullif(current_setting('app.user_id', true), '')::bigint;
  IF value > 0 THEN RETURN value; END IF;
  RETURN NULL;
EXCEPTION WHEN invalid_text_representation OR numeric_value_out_of_range THEN RETURN NULL;
END $$;
REVOKE ALL ON FUNCTION compallm_rls.user_id() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION compallm_rls.user_id() TO compallm_restricted;

DROP POLICY compallm_member_directory_guard ON public.users;
DROP POLICY compallm_member_directory ON public.users;
REVOKE SELECT (id, display_name, email, status) ON public.users FROM compallm_restricted;

CREATE POLICY compallm_user_read ON public.users FOR SELECT TO compallm_restricted
USING ((SELECT compallm_rls.user_id()) IS NOT NULL);
CREATE POLICY compallm_user_update ON public.users FOR UPDATE TO compallm_restricted
USING (id = (SELECT compallm_rls.user_id()))
WITH CHECK (id = (SELECT compallm_rls.user_id()));
GRANT SELECT ON public.users TO compallm_restricted;
GRANT UPDATE (display_name, username, avatar_path) ON public.users TO compallm_restricted;

ALTER TABLE public.user_preferences ENABLE ROW LEVEL SECURITY;
CREATE POLICY compallm_preference_read ON public.user_preferences FOR SELECT TO compallm_restricted
USING (user_id = (SELECT compallm_rls.user_id()));
CREATE POLICY compallm_preference_insert ON public.user_preferences FOR INSERT TO compallm_restricted
WITH CHECK (user_id = (SELECT compallm_rls.user_id()));
CREATE POLICY compallm_preference_update ON public.user_preferences FOR UPDATE TO compallm_restricted
USING (user_id = (SELECT compallm_rls.user_id()))
WITH CHECK (user_id = (SELECT compallm_rls.user_id()));
GRANT SELECT, INSERT ON public.user_preferences TO compallm_restricted;
GRANT UPDATE (navigation, sidebar, member_display, calendar, theme, typography,
  accessibility, message_composition, code_appearance) ON public.user_preferences TO compallm_restricted;
