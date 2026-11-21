CREATE FUNCTION capability_usage_complete(value jsonb,input boolean) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE item jsonb; token jsonb;
BEGIN
 IF value ? 'fixed' THEN
  token:=value->'fixed'->'token';
  RETURN shared_catalog_rate_complete(token->'rate') AND (NOT input OR CASE token->'cache'->>'type' WHEN 'separate_rate' THEN shared_catalog_rate_complete(token->'cache'->'configuration'->'rate') WHEN 'standard_input_rate' THEN true WHEN 'included' THEN true WHEN 'unsupported' THEN true ELSE false END);
 ELSIF value ? 'tiered_token' THEN
  IF jsonb_array_length(value->'tiered_token'->'tiers')=0 THEN RETURN false; END IF;
  FOR item IN SELECT * FROM jsonb_array_elements(value->'tiered_token'->'tiers') LOOP
   IF NOT capability_usage_complete(jsonb_build_object('fixed',jsonb_build_object('token',item->'rate')),input) THEN RETURN false; END IF;
  END LOOP;
  RETURN true;
 ELSIF value ? 'scheduled' THEN
  IF jsonb_array_length(value->'scheduled'->'slots')=0 THEN RETURN false; END IF;
  FOR item IN SELECT * FROM jsonb_array_elements(value->'scheduled'->'slots') LOOP IF NOT capability_usage_complete(item->'pricing',input) THEN RETURN false; END IF; END LOOP;
  RETURN true;
 END IF;
 RETURN false;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION capability_text_pricing_complete(value jsonb,billing jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT CASE WHEN billing ? 'free' OR billing ? 'subscription' THEN true ELSE
 coalesce(value->'input'->'pricing'->>'type' IN ('free','included') OR (value->'input'->'pricing'->>'type'='model_billing' AND billing IS NOT NULL) OR capability_usage_complete(value->'input'->'pricing'->'configuration',true),false)
 AND coalesce(value->'output'->'pricing'->>'type' IN ('free','included') OR (value->'output'->'pricing'->>'type'='model_billing' AND billing IS NOT NULL) OR capability_usage_complete(value->'output'->'pricing'->'configuration',false),false) END
$$;
CREATE VIEW provider_catalog_readiness AS
 SELECT m.provider_template_id,m.id,
 CASE WHEN provider_readiness='incomplete' OR model_readiness='incomplete' THEN 'incomplete'::catalog_readiness
 WHEN provider_readiness='needs_configuration' OR model_readiness='needs_configuration' THEN 'needs_configuration'::catalog_readiness
 WHEN provider_readiness='needs_fetch' OR model_readiness='needs_fetch' THEN 'needs_fetch'::catalog_readiness ELSE 'ready'::catalog_readiness END readiness,provider_readiness,model_readiness
 FROM provider_model_templates m JOIN provider_templates p ON p.id=m.provider_template_id
 JOIN provider_model_template_research r ON r.id=m.research_id JOIN provider_template_research pr ON pr.id=p.research_id,
 LATERAL(SELECT catalog_provider_model_state(jsonb_build_object('endpoint_mode',p.connection->'endpoint'->'mode','authentication',pr.authentication,'field_metadata',pr.field_metadata),(SELECT coalesce(jsonb_agg(CASE key WHEN 'openai' THEN 'openai_chat_completions' WHEN 'anthropic' THEN 'anthropic_messages' ELSE key END),'[]') FROM jsonb_object_keys(m.connections) key)) provider_readiness,
 CASE WHEN catalog_model_state(r.specification_mode,(r.configuration_snapshot->'pricing'->>'mode')::catalog_pricing_mode,r.pricing_config,r.unresolved_fields,r.resolution_plan)='ready' AND (m.connections='{}'::jsonb OR (m.service_kind='llm' AND ((m.limits->>'contextTokens' IS NULL AND m.route_selection IS NULL) OR (m.route_selection IS NULL AND NOT capability_text_pricing_complete(m.text,m.billing))))) THEN 'incomplete'::catalog_readiness ELSE catalog_model_state(r.specification_mode,(r.configuration_snapshot->'pricing'->>'mode')::catalog_pricing_mode,r.pricing_config,r.unresolved_fields,r.resolution_plan) END model_readiness) state;
CREATE VIEW provider_catalog_auth_readiness AS
 SELECT m.provider_template_id,m.id,method AS auth_method,provider_readiness,model_readiness,
 CASE WHEN provider_readiness='incomplete' OR model_readiness='incomplete' THEN 'incomplete'::catalog_readiness
 WHEN provider_readiness='needs_configuration' OR model_readiness='needs_configuration' THEN 'needs_configuration'::catalog_readiness
 WHEN provider_readiness='needs_fetch' OR model_readiness='needs_fetch' THEN 'needs_fetch'::catalog_readiness ELSE 'ready'::catalog_readiness END readiness
 FROM provider_model_templates m JOIN provider_templates p ON p.id=m.provider_template_id JOIN provider_model_template_research r ON r.id=m.research_id JOIN provider_template_research pr ON pr.id=p.research_id CROSS JOIN LATERAL jsonb_object_keys(pr.authentication) method,
 LATERAL(SELECT CASE WHEN pr.field_metadata ? 'request_headers' AND catalog_provider_auth_state((p.connection->'endpoint'->>'mode')::catalog_endpoint_mode,method,pr.authentication->method,(SELECT coalesce(jsonb_agg(CASE key WHEN 'openai' THEN 'openai_chat_completions' WHEN 'anthropic' THEN 'anthropic_messages' ELSE key END),'[]') FROM jsonb_object_keys(m.connections) key))<>'incomplete' THEN 'needs_configuration'::catalog_readiness ELSE catalog_provider_auth_state((p.connection->'endpoint'->>'mode')::catalog_endpoint_mode,method,pr.authentication->method,(SELECT coalesce(jsonb_agg(CASE key WHEN 'openai' THEN 'openai_chat_completions' WHEN 'anthropic' THEN 'anthropic_messages' ELSE key END),'[]') FROM jsonb_object_keys(m.connections) key)) END provider_readiness,
 CASE WHEN catalog_model_state(r.specification_mode,(r.configuration_snapshot->'pricing'->>'mode')::catalog_pricing_mode,r.pricing_config,r.unresolved_fields,r.resolution_plan)='ready' AND (m.connections='{}'::jsonb OR (m.service_kind='llm' AND ((m.limits->>'contextTokens' IS NULL AND m.route_selection IS NULL) OR (m.route_selection IS NULL AND NOT capability_text_pricing_complete(m.text,m.billing))))) THEN 'incomplete'::catalog_readiness ELSE catalog_model_state(r.specification_mode,(r.configuration_snapshot->'pricing'->>'mode')::catalog_pricing_mode,r.pricing_config,r.unresolved_fields,r.resolution_plan) END model_readiness) state;
GRANT SELECT ON provider_catalog_readiness,provider_catalog_auth_readiness TO compallm_restricted;
