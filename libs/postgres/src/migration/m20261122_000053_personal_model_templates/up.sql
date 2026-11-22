ALTER TABLE provider_model_templates ADD COLUMN row_id uuid;
UPDATE provider_model_templates SET row_id=md5(jsonb_build_array(provider_template_id,id)::text)::uuid;
ALTER TABLE provider_model_templates ALTER COLUMN row_id SET NOT NULL;
ALTER TABLE provider_model_templates DROP CONSTRAINT provider_model_templates_pkey;
ALTER TABLE provider_model_templates ADD PRIMARY KEY(row_id);
ALTER TABLE provider_model_templates ADD COLUMN organization_id bigint;
ALTER TABLE provider_model_templates ADD COLUMN user_id bigint;
ALTER TABLE provider_model_templates ADD COLUMN custom_fields jsonb NOT NULL DEFAULT '[]' CHECK(jsonb_typeof(custom_fields)='array' AND custom_fields <@ '["contextLimitEnabled","inputLimitEnabled","outputLimitEnabled","inputPriceEnabled","outputPriceEnabled","cachedPriceEnabled"]'::jsonb);
ALTER TABLE provider_model_templates ADD COLUMN base_row_id uuid REFERENCES provider_model_templates(row_id) ON DELETE RESTRICT;
ALTER TABLE provider_model_templates ALTER COLUMN research_id DROP NOT NULL;
ALTER TABLE provider_model_templates ADD CONSTRAINT personal_template_ownership CHECK (
 (organization_id IS NULL AND user_id IS NULL AND base_row_id IS NULL AND research_id IS NOT NULL)
 OR (organization_id IS NOT NULL AND user_id IS NOT NULL AND research_id IS NULL));
ALTER TABLE provider_model_templates ADD FOREIGN KEY(organization_id,user_id) REFERENCES organization_members(organization_id,user_id) ON DELETE CASCADE;
CREATE UNIQUE INDEX model_template_global_identity ON provider_model_templates(provider_template_id,id) WHERE user_id IS NULL;
CREATE UNIQUE INDEX model_template_personal_identity ON provider_model_templates(organization_id,user_id,provider_template_id,id) WHERE user_id IS NOT NULL;
CREATE FUNCTION personal_model_template_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP='UPDATE' AND (NEW.row_id,NEW.organization_id,NEW.user_id,NEW.base_row_id,NEW.provider_template_id,NEW.id,NEW.research_id)
 IS DISTINCT FROM (OLD.row_id,OLD.organization_id,OLD.user_id,OLD.base_row_id,OLD.provider_template_id,OLD.id,OLD.research_id) THEN
 RAISE EXCEPTION 'model template identity is immutable'; END IF;
 IF NEW.base_row_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM provider_model_templates b WHERE b.row_id=NEW.base_row_id AND b.user_id IS NULL AND b.provider_template_id=NEW.provider_template_id AND b.id=NEW.id) THEN
 RAISE EXCEPTION 'invalid global model template reference'; END IF;
 IF NEW.user_id IS NOT NULL AND NEW.base_row_id IS NULL AND NEW.id NOT LIKE 'custom:%' THEN
 RAISE EXCEPTION 'custom model identity must use the private namespace'; END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER personal_model_template_guard BEFORE INSERT OR UPDATE ON provider_model_templates FOR EACH ROW EXECUTE FUNCTION personal_model_template_guard();
ALTER TABLE provider_model_templates ENABLE ROW LEVEL SECURITY;
CREATE POLICY model_template_read ON provider_model_templates FOR SELECT TO compallm_restricted USING
 (user_id IS NULL OR (user_id=(SELECT compallm_rls.user_id()) AND organization_id=(SELECT compallm_rls.organization_id())));
CREATE POLICY model_template_insert ON provider_model_templates FOR INSERT TO compallm_restricted WITH CHECK
 (user_id=(SELECT compallm_rls.user_id()) AND organization_id=(SELECT compallm_rls.organization_id()));
CREATE POLICY model_template_update ON provider_model_templates FOR UPDATE TO compallm_restricted USING
 (user_id=(SELECT compallm_rls.user_id()) AND organization_id=(SELECT compallm_rls.organization_id())) WITH CHECK
 (user_id=(SELECT compallm_rls.user_id()) AND organization_id=(SELECT compallm_rls.organization_id()));
CREATE POLICY model_template_delete ON provider_model_templates FOR DELETE TO compallm_restricted USING
 (user_id=(SELECT compallm_rls.user_id()) AND organization_id=(SELECT compallm_rls.organization_id()));
GRANT INSERT, UPDATE, DELETE ON provider_model_templates TO compallm_restricted;

ALTER TABLE provider_model_templates ADD CONSTRAINT personal_model_context_limits CHECK (
 user_id IS NULL OR (coalesce((text->'input'->'configuration'->>'maxTokens')::bigint <= (limits->>'contextTokens')::bigint,true)
 AND coalesce((text->'output'->'configuration'->>'maxTokens')::bigint <= (limits->>'contextTokens')::bigint,true)));
