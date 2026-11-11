CREATE ROLE compallm_restricted NOLOGIN NOSUPERUSER NOBYPASSRLS;
DO $$ BEGIN EXECUTE format('GRANT compallm_restricted TO %I WITH INHERIT FALSE', current_user); END $$;
CREATE SCHEMA compallm_rls;
REVOKE ALL ON SCHEMA compallm_rls FROM PUBLIC;
GRANT USAGE ON SCHEMA public, compallm_rls TO compallm_restricted;

CREATE FUNCTION compallm_rls.organization_id() RETURNS bigint
LANGUAGE plpgsql STABLE SET search_path = pg_catalog AS $$
DECLARE value bigint;
BEGIN
  IF current_setting('app.rls_version', true) IS DISTINCT FROM '1' THEN RETURN NULL; END IF;
  value := nullif(current_setting('app.organization_id', true), '')::bigint;
  IF value > 0 THEN RETURN value; END IF;
  RETURN NULL;
EXCEPTION WHEN invalid_text_representation OR numeric_value_out_of_range THEN RETURN NULL;
END $$;

CREATE FUNCTION compallm_rls.entity(kind text, id bigint, organization_id bigint,
  enabled boolean DEFAULT NULL, provider_id bigint DEFAULT NULL, user_id bigint DEFAULT NULL)
RETURNS jsonb LANGUAGE sql IMMUTABLE SET search_path = pg_catalog AS $$
  SELECT jsonb_strip_nulls(jsonb_build_object(
    'kind', kind,
    'id', CASE WHEN kind IN ('Organization', 'User') THEN id::text ELSE organization_id::text || ':' || id::text END,
    'organization', jsonb_build_object('kind', 'Organization', 'id', organization_id::text),
    'enabled', enabled,
    'provider', CASE WHEN provider_id IS NOT NULL THEN jsonb_build_object('kind', 'Provider', 'id', organization_id::text || ':' || provider_id::text) END,
    'user', CASE WHEN user_id IS NOT NULL THEN jsonb_build_object('kind', 'User', 'id', user_id::text) END
  ));
$$;

CREATE FUNCTION compallm_rls.load_entity(kind text, organization_id bigint, id bigint) RETURNS jsonb
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
DECLARE result jsonb;
BEGIN
  IF organization_id IS DISTINCT FROM compallm_rls.organization_id() THEN RETURN NULL; END IF;
  CASE kind
    WHEN 'Organization' THEN SELECT compallm_rls.entity(kind, t.id, t.id) INTO result FROM public.organizations t WHERE t.id = load_entity.organization_id AND t.id = load_entity.id;
    WHEN 'Role' THEN SELECT compallm_rls.entity(kind, t.id, t.organization_id) INTO result FROM public.organization_roles t WHERE t.organization_id = load_entity.organization_id AND t.id = load_entity.id;
    WHEN 'Group' THEN SELECT compallm_rls.entity(kind, t.id, t.organization_id) INTO result FROM public.organization_groups t WHERE t.organization_id = load_entity.organization_id AND t.id = load_entity.id;
    WHEN 'Member' THEN SELECT compallm_rls.entity(kind, t.user_id, t.organization_id, NULL, NULL, t.user_id) INTO result FROM public.organization_members t WHERE t.organization_id = load_entity.organization_id AND t.user_id = load_entity.id;
    WHEN 'Provider' THEN SELECT compallm_rls.entity(kind, t.id, t.organization_id) INTO result FROM public.providers t WHERE t.organization_id = load_entity.organization_id AND t.id = load_entity.id;
    WHEN 'ProviderModel' THEN SELECT compallm_rls.entity(kind, t.id, t.organization_id, t.enabled, t.provider_id) INTO result FROM public.provider_models t WHERE t.organization_id = load_entity.organization_id AND t.id = load_entity.id;
    WHEN 'Model' THEN SELECT compallm_rls.entity(kind, t.id, t.organization_id, t.enabled) INTO result FROM public.models t WHERE t.organization_id = load_entity.organization_id AND t.id = load_entity.id;
    ELSE RETURN NULL;
  END CASE;
  RETURN result;
END $$;

CREATE FUNCTION compallm_rls.equal_entity(a jsonb, b jsonb) RETURNS boolean
LANGUAGE sql IMMUTABLE SET search_path = pg_catalog AS $$
  SELECT CASE WHEN a->>'kind' IS NOT NULL AND a->>'id' IS NOT NULL AND b->>'kind' IS NOT NULL AND b->>'id' IS NOT NULL
    THEN a->>'kind' = b->>'kind' AND a->>'id' = b->>'id' END;
$$;

CREATE FUNCTION compallm_rls.entity_in(child jsonb, parent jsonb) RETURNS boolean
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
DECLARE org bigint; child_id bigint; parent_id bigint;
BEGIN
  IF child IS NULL OR parent IS NULL THEN RETURN NULL; END IF;
  IF compallm_rls.equal_entity(child, parent) THEN RETURN true; END IF;
  IF parent->>'kind' = 'Organization' THEN RETURN coalesce(compallm_rls.equal_entity(child->'organization', parent), false); END IF;
  IF child->>'kind' = 'ProviderModel' AND parent->>'kind' = 'Provider' THEN RETURN coalesce(compallm_rls.equal_entity(child->'provider', parent), false); END IF;
  org := compallm_rls.organization_id();
  IF org IS NULL OR split_part(parent->>'id', ':', 1) <> org::text THEN RETURN false; END IF;
  parent_id := split_part(parent->>'id', ':', 2)::bigint;
  IF child->>'kind' = 'Group' AND parent->>'kind' = 'Role' THEN
    IF split_part(child->>'id', ':', 1) <> org::text THEN RETURN false; END IF;
    child_id := split_part(child->>'id', ':', 2)::bigint;
    RETURN EXISTS (SELECT 1 FROM public.organization_group_roles g WHERE g.organization_id = org AND g.group_id = child_id AND g.role_id = parent_id);
  END IF;
  IF child->>'kind' = 'User' THEN
    child_id := (child->>'id')::bigint;
    IF parent->>'kind' = 'Group' THEN
      RETURN EXISTS (SELECT 1 FROM public.organization_group_members g WHERE g.organization_id = org AND g.user_id = child_id AND g.group_id = parent_id);
    ELSIF parent->>'kind' = 'Role' THEN
      RETURN EXISTS (SELECT 1 FROM public.organization_member_roles r WHERE r.organization_id = org AND r.user_id = child_id AND r.role_id = parent_id)
        OR EXISTS (SELECT 1 FROM public.organization_group_members m JOIN public.organization_group_roles r ON r.organization_id = m.organization_id AND r.group_id = m.group_id WHERE m.organization_id = org AND m.user_id = child_id AND r.role_id = parent_id);
    END IF;
  END IF;
  RETURN false;
EXCEPTION WHEN invalid_text_representation OR numeric_value_out_of_range THEN RETURN NULL;
END $$;

CREATE FUNCTION compallm_rls.field(name text, resource jsonb, context jsonb) RETURNS jsonb
LANGUAGE sql IMMUTABLE SET search_path = pg_catalog AS $$
  SELECT CASE name
    WHEN 'resource' THEN resource
    WHEN 'resource_organization' THEN resource->'organization'
    WHEN 'resource_provider' THEN resource->'provider'
    WHEN 'resource_user' THEN resource->'user'
    WHEN 'resource_enabled' THEN resource->'enabled'
    WHEN 'context' THEN context
    WHEN 'context_organization' THEN context->'organization'
    WHEN 'context_provider' THEN context->'provider'
    WHEN 'context_user' THEN context->'user'
    WHEN 'context_enabled' THEN context->'enabled'
  END;
$$;

CREATE FUNCTION compallm_rls.operand(value jsonb, resource jsonb, context jsonb) RETURNS jsonb
LANGUAGE sql IMMUTABLE SET search_path = pg_catalog AS $$
  SELECT CASE WHEN jsonb_typeof(value) = 'object' AND (SELECT count(*) FROM jsonb_object_keys(value)) = 1 THEN
    CASE WHEN value ? 'entity' THEN value->'entity'
         WHEN value ? 'field' THEN compallm_rls.field(value->>'field', resource, context) END END;
$$;

CREATE FUNCTION compallm_rls.condition(value jsonb, resource jsonb, context jsonb) RETURNS boolean
LANGUAGE plpgsql STABLE SET search_path = pg_catalog AS $$
DECLARE op text; args jsonb; item jsonb; result boolean; child boolean; left_value jsonb; right_value jsonb;
BEGIN
  IF jsonb_typeof(value) IS DISTINCT FROM 'object' THEN RETURN NULL; END IF;
  IF (SELECT count(*) FROM jsonb_object_keys(value)) <> 1 THEN RETURN NULL; END IF;
  SELECT key, val INTO op, args FROM jsonb_each(value) AS e(key, val);
  CASE op
    WHEN 'constant' THEN
      IF jsonb_typeof(args) = 'boolean' THEN RETURN args::boolean; END IF;
    WHEN 'all', 'any' THEN
      IF jsonb_typeof(args) IS DISTINCT FROM 'array' THEN RETURN NULL; END IF;
      result := op = 'all';
      FOR item IN SELECT jsonb_array_elements(args) LOOP
        child := compallm_rls.condition(item, resource, context);
        IF child IS NULL THEN RETURN NULL; END IF;
        IF op = 'all' THEN result := result AND child; ELSE result := result OR child; END IF;
      END LOOP;
      RETURN result;
    WHEN 'not' THEN RETURN NOT compallm_rls.condition(args, resource, context);
    WHEN 'enabled' THEN
      left_value := compallm_rls.field(args #>> '{}', resource, context);
      IF jsonb_typeof(left_value) = 'boolean' THEN RETURN left_value::boolean; END IF;
    WHEN 'equal', 'in', 'is' THEN
      IF jsonb_typeof(args) IS DISTINCT FROM 'array' OR jsonb_array_length(args) <> 2 THEN RETURN NULL; END IF;
      left_value := compallm_rls.operand(args->0, resource, context);
      IF op = 'equal' THEN
        right_value := compallm_rls.operand(args->1, resource, context);
        IF jsonb_typeof(left_value) = 'boolean' AND jsonb_typeof(right_value) = 'boolean' THEN RETURN left_value = right_value; END IF;
        RETURN compallm_rls.equal_entity(left_value, right_value);
      ELSIF op = 'in' THEN RETURN compallm_rls.entity_in(left_value, args->1);
      ELSE RETURN left_value->>'kind' = args->>1;
      END IF;
    ELSE RETURN NULL;
  END CASE;
  RETURN NULL;
END $$;

CREATE FUNCTION compallm_rls.rules(action text) RETURNS jsonb
LANGUAGE plpgsql STABLE SET search_path = pg_catalog AS $$
DECLARE rules jsonb;
BEGIN
  IF compallm_rls.organization_id() IS NULL THEN RETURN NULL; END IF;
  rules := nullif(current_setting('app.' || action, true), '')::jsonb;
  IF jsonb_typeof(rules) IS DISTINCT FROM 'object' OR jsonb_typeof(rules->'permits') IS DISTINCT FROM 'array' OR jsonb_typeof(rules->'forbids') IS DISTINCT FROM 'array' THEN RETURN NULL; END IF;
  RETURN rules;
EXCEPTION WHEN invalid_text_representation THEN RETURN NULL;
END $$;

CREATE FUNCTION compallm_rls.allowed(action text, resource jsonb, context jsonb DEFAULT NULL) RETURNS boolean
LANGUAGE plpgsql STABLE SET search_path = pg_catalog AS $$
DECLARE rules jsonb; rule jsonb; result boolean; permitted boolean := false;
BEGIN
  IF resource IS NULL OR resource->'organization'->>'id' IS DISTINCT FROM compallm_rls.organization_id()::text THEN RETURN false; END IF;
  rules := compallm_rls.rules(action);
  IF rules IS NULL THEN RETURN false; END IF;
  FOR rule IN SELECT jsonb_array_elements(rules->'permits') LOOP
    result := compallm_rls.condition(rule, resource, context);
    IF result IS NULL THEN RETURN false; END IF;
    permitted := permitted OR result;
  END LOOP;
  FOR rule IN SELECT jsonb_array_elements(rules->'forbids') LOOP
    result := compallm_rls.condition(rule, resource, context);
    IF result IS NULL OR result THEN RETURN false; END IF;
  END LOOP;
  RETURN permitted;
END $$;

CREATE FUNCTION compallm_rls.organization_allowed(action text) RETURNS boolean
LANGUAGE sql STABLE SET search_path = pg_catalog AS $$
  SELECT compallm_rls.allowed(action, compallm_rls.entity('Organization', compallm_rls.organization_id(), compallm_rls.organization_id()));
$$;

CREATE FUNCTION compallm_rls.role_policy(organization_id bigint, role_id bigint) RETURNS text
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
DECLARE result text;
BEGIN
  IF NOT compallm_rls.allowed('role.policy.read', compallm_rls.load_entity('Role', organization_id, role_id)) THEN RAISE EXCEPTION 'role policy access denied' USING ERRCODE = '42501'; END IF;
  SELECT r.cedar_source INTO result FROM public.organization_roles r WHERE r.organization_id = role_policy.organization_id AND r.id = role_id;
  RETURN result;
END $$;

CREATE FUNCTION compallm_rls.model_usable(organization_id bigint, model_id bigint) RETURNS boolean
LANGUAGE sql STABLE SET search_path = pg_catalog AS $$
  SELECT compallm_rls.allowed('model.use', compallm_rls.load_entity('Model', organization_id, model_id));
$$;
