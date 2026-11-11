CREATE FUNCTION compallm_rls.guard_columns() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
DECLARE before_row jsonb; after_row jsonb;
BEGIN
  IF current_user <> 'compallm_restricted' AND (NOT pg_has_role(current_user, 'compallm_restricted', 'USAGE') OR EXISTS (SELECT 1 FROM pg_roles WHERE rolname = current_user AND rolsuper)) THEN
    IF TG_OP = 'DELETE' THEN RETURN OLD; ELSE RETURN NEW; END IF;
  END IF;
  IF TG_TABLE_NAME = 'organization_roles' THEN
    IF TG_OP = 'INSERT' THEN
      IF NEW.org_owner THEN RAISE EXCEPTION 'owner role creation denied' USING ERRCODE = '42501'; END IF;
      RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
      IF OLD.org_owner THEN RAISE EXCEPTION 'owner role deletion denied' USING ERRCODE = '42501'; END IF;
      RETURN OLD;
    END IF;
  END IF;
  before_row := to_jsonb(OLD); after_row := to_jsonb(NEW);
  CASE TG_TABLE_NAME
    WHEN 'organizations' THEN
      IF (before_row - ARRAY['name','status']::text[]) IS DISTINCT FROM (after_row - ARRAY['name','status']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'name' IS DISTINCT FROM after_row->'name' OR before_row->'status' IS DISTINCT FROM after_row->'status') AND NOT (compallm_rls.allowed('organization.settings.update', compallm_rls.entity('Organization', OLD.id, OLD.id)) AND compallm_rls.allowed('organization.settings.update', compallm_rls.entity('Organization', NEW.id, NEW.id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    WHEN 'organization_roles' THEN
      IF (before_row - ARRAY['name','description','cedar_source']::text[]) IS DISTINCT FROM (after_row - ARRAY['name','description','cedar_source']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'name' IS DISTINCT FROM after_row->'name' OR before_row->'description' IS DISTINCT FROM after_row->'description') AND NOT (compallm_rls.allowed('role.metadata.update', compallm_rls.entity('Role', OLD.id, OLD.organization_id)) AND compallm_rls.allowed('role.metadata.update', compallm_rls.entity('Role', NEW.id, NEW.organization_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
      IF (before_row->'cedar_source' IS DISTINCT FROM after_row->'cedar_source') AND NOT (compallm_rls.allowed('role.policy.update', compallm_rls.entity('Role', OLD.id, OLD.organization_id)) AND compallm_rls.allowed('role.policy.update', compallm_rls.entity('Role', NEW.id, NEW.organization_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    WHEN 'organization_groups' THEN
      IF (before_row - ARRAY['name','description']::text[]) IS DISTINCT FROM (after_row - ARRAY['name','description']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'name' IS DISTINCT FROM after_row->'name' OR before_row->'description' IS DISTINCT FROM after_row->'description') AND NOT (compallm_rls.allowed('group.metadata.update', compallm_rls.entity('Group', OLD.id, OLD.organization_id)) AND compallm_rls.allowed('group.metadata.update', compallm_rls.entity('Group', NEW.id, NEW.organization_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    WHEN 'providers' THEN
      IF (before_row - ARRAY['name','connection','headers','credentials']::text[]) IS DISTINCT FROM (after_row - ARRAY['name','connection','headers','credentials']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'name' IS DISTINCT FROM after_row->'name' OR before_row->'connection' IS DISTINCT FROM after_row->'connection' OR before_row->'headers' IS DISTINCT FROM after_row->'headers') AND NOT (compallm_rls.allowed('provider.configuration.update', compallm_rls.entity('Provider', OLD.id, OLD.organization_id)) AND compallm_rls.allowed('provider.configuration.update', compallm_rls.entity('Provider', NEW.id, NEW.organization_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
      IF (before_row->'credentials' IS DISTINCT FROM after_row->'credentials') AND NOT (compallm_rls.allowed('provider.credential.replace', compallm_rls.entity('Provider', OLD.id, OLD.organization_id)) AND compallm_rls.allowed('provider.credential.replace', compallm_rls.entity('Provider', NEW.id, NEW.organization_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    WHEN 'provider_models' THEN
      IF (before_row - ARRAY['name','enabled','service_kind','protocol','connection','headers','capabilities','limits','thinking','pricing']::text[]) IS DISTINCT FROM (after_row - ARRAY['name','enabled','service_kind','protocol','connection','headers','capabilities','limits','thinking','pricing']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'name' IS DISTINCT FROM after_row->'name' OR before_row->'enabled' IS DISTINCT FROM after_row->'enabled' OR before_row->'service_kind' IS DISTINCT FROM after_row->'service_kind' OR before_row->'protocol' IS DISTINCT FROM after_row->'protocol' OR before_row->'connection' IS DISTINCT FROM after_row->'connection' OR before_row->'headers' IS DISTINCT FROM after_row->'headers' OR before_row->'capabilities' IS DISTINCT FROM after_row->'capabilities' OR before_row->'limits' IS DISTINCT FROM after_row->'limits' OR before_row->'thinking' IS DISTINCT FROM after_row->'thinking') AND NOT (compallm_rls.allowed('provider.model.configuration.update', compallm_rls.entity('ProviderModel', OLD.id, OLD.organization_id, OLD.enabled, OLD.provider_id)) AND compallm_rls.allowed('provider.model.configuration.update', compallm_rls.entity('ProviderModel', NEW.id, NEW.organization_id, NEW.enabled, NEW.provider_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
      IF (before_row->'pricing' IS DISTINCT FROM after_row->'pricing') AND NOT (compallm_rls.allowed('provider.model.pricing.update', compallm_rls.entity('ProviderModel', OLD.id, OLD.organization_id, OLD.enabled, OLD.provider_id)) AND compallm_rls.allowed('provider.model.pricing.update', compallm_rls.entity('ProviderModel', NEW.id, NEW.organization_id, NEW.enabled, NEW.provider_id))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    WHEN 'models' THEN
      IF (before_row - ARRAY['name','enabled','service_kind','capabilities','limits','thinking','pricing']::text[]) IS DISTINCT FROM (after_row - ARRAY['name','enabled','service_kind','capabilities','limits','thinking','pricing']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'name' IS DISTINCT FROM after_row->'name' OR before_row->'enabled' IS DISTINCT FROM after_row->'enabled' OR before_row->'service_kind' IS DISTINCT FROM after_row->'service_kind' OR before_row->'capabilities' IS DISTINCT FROM after_row->'capabilities' OR before_row->'limits' IS DISTINCT FROM after_row->'limits' OR before_row->'thinking' IS DISTINCT FROM after_row->'thinking') AND NOT (compallm_rls.allowed('model.configuration.update', compallm_rls.entity('Model', OLD.id, OLD.organization_id, OLD.enabled)) AND compallm_rls.allowed('model.configuration.update', compallm_rls.entity('Model', NEW.id, NEW.organization_id, NEW.enabled))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
      IF (before_row->'pricing' IS DISTINCT FROM after_row->'pricing') AND NOT (compallm_rls.allowed('model.pricing.update', compallm_rls.entity('Model', OLD.id, OLD.organization_id, OLD.enabled)) AND compallm_rls.allowed('model.pricing.update', compallm_rls.entity('Model', NEW.id, NEW.organization_id, NEW.enabled))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    WHEN 'model_router' THEN
      IF (before_row - ARRAY['priority','enabled']::text[]) IS DISTINCT FROM (after_row - ARRAY['priority','enabled']::text[]) THEN RAISE EXCEPTION 'immutable columns changed' USING ERRCODE = '42501'; END IF;
      IF (before_row->'priority' IS DISTINCT FROM after_row->'priority' OR before_row->'enabled' IS DISTINCT FROM after_row->'enabled') AND NOT ((compallm_rls.allowed('model.route.update', compallm_rls.load_entity('Model', OLD.organization_id, OLD.model_id), compallm_rls.load_entity('ProviderModel', OLD.organization_id, OLD.provider_model_id))) AND (compallm_rls.allowed('model.route.update', compallm_rls.load_entity('Model', NEW.organization_id, NEW.model_id), compallm_rls.load_entity('ProviderModel', NEW.organization_id, NEW.provider_model_id)))) THEN RAISE EXCEPTION 'column permission denied' USING ERRCODE = '42501'; END IF;
    ELSE RAISE EXCEPTION 'unsupported protected update' USING ERRCODE = '42501';
  END CASE;
  RETURN NEW;
END $$;
CREATE TRIGGER compallm_columns BEFORE UPDATE ON public.organizations FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
CREATE TRIGGER compallm_columns BEFORE INSERT OR UPDATE OR DELETE ON public.organization_roles FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
CREATE TRIGGER compallm_columns BEFORE UPDATE ON public.organization_groups FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
CREATE TRIGGER compallm_columns BEFORE UPDATE ON public.providers FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
CREATE TRIGGER compallm_columns BEFORE UPDATE ON public.provider_models FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
CREATE TRIGGER compallm_columns BEFORE UPDATE ON public.models FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
CREATE TRIGGER compallm_columns BEFORE UPDATE ON public.model_router FOR EACH ROW EXECUTE FUNCTION compallm_rls.guard_columns();
REVOKE ALL ON ALL FUNCTIONS IN SCHEMA compallm_rls FROM PUBLIC;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA compallm_rls TO compallm_restricted;
