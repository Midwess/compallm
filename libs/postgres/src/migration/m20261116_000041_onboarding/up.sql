CREATE TABLE public.organization_onboarding_settings (
 organization_id BIGINT PRIMARY KEY REFERENCES public.organizations(id) ON DELETE CASCADE,
 settings JSONB NOT NULL CHECK (jsonb_typeof(settings) = 'object')
);
CREATE TABLE public.organization_member_onboarding (
 organization_id BIGINT NOT NULL,
 user_id BIGINT NOT NULL,
 progress JSONB NOT NULL CHECK (jsonb_typeof(progress) = 'object'),
 completed BOOLEAN NOT NULL DEFAULT FALSE,
 PRIMARY KEY (organization_id, user_id),
 FOREIGN KEY (organization_id, user_id) REFERENCES public.organization_members(organization_id, user_id) ON DELETE CASCADE
);
CREATE TABLE public.analytic_users_profile (
 user_id BIGINT NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
 key VARCHAR(64) NOT NULL CHECK (char_length(key) >= 1),
 value VARCHAR(256) NOT NULL,
 PRIMARY KEY (user_id, key)
);
ALTER TABLE public.organization_onboarding_settings ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.organization_member_onboarding ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.analytic_users_profile ENABLE ROW LEVEL SECURITY;
GRANT SELECT, INSERT, UPDATE ON public.organization_onboarding_settings TO compallm_restricted;
GRANT SELECT ON public.organization_member_onboarding TO compallm_restricted;
GRANT UPDATE (progress) ON public.organization_member_onboarding TO compallm_restricted;
GRANT SELECT, INSERT, UPDATE ON public.analytic_users_profile TO compallm_restricted;
CREATE POLICY onboarding_settings_scope ON public.organization_onboarding_settings AS RESTRICTIVE TO compallm_restricted
 USING (organization_id = (SELECT compallm_rls.organization_id())) WITH CHECK (organization_id = (SELECT compallm_rls.organization_id()));
CREATE POLICY onboarding_settings_read ON public.organization_onboarding_settings FOR SELECT TO compallm_restricted
 USING ((SELECT compallm_rls.organization_allowed('organization.onboarding.read')) OR (SELECT compallm_rls.organization_allowed('organization.onboarding.update')));
CREATE POLICY onboarding_settings_insert ON public.organization_onboarding_settings FOR INSERT TO compallm_restricted
 WITH CHECK ((SELECT compallm_rls.organization_allowed('organization.onboarding.update')));
CREATE POLICY onboarding_settings_update ON public.organization_onboarding_settings FOR UPDATE TO compallm_restricted
 USING ((SELECT compallm_rls.organization_allowed('organization.onboarding.update'))) WITH CHECK ((SELECT compallm_rls.organization_allowed('organization.onboarding.update')));
CREATE POLICY onboarding_self ON public.organization_member_onboarding TO compallm_restricted
 USING (organization_id = (SELECT compallm_rls.organization_id()) AND user_id = (SELECT compallm_rls.user_id()))
 WITH CHECK (organization_id = (SELECT compallm_rls.organization_id()) AND user_id = (SELECT compallm_rls.user_id()));
CREATE POLICY analytics_self ON public.analytic_users_profile TO compallm_restricted
 USING (user_id = (SELECT compallm_rls.user_id())) WITH CHECK (user_id = (SELECT compallm_rls.user_id()));
