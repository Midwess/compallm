CREATE FUNCTION catalog_operations_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value)='object' AND NOT EXISTS (
 SELECT 1 FROM jsonb_each(value) e WHERE key NOT IN ('openai_chat_completions','openai_responses','anthropic_messages') OR jsonb_typeof(e.value) <> 'string' OR e.value #>> '{}' !~ '^/([A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' OR length(e.value #>> '{}')>40),false)
$$;
CREATE FUNCTION catalog_references_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value)='array' AND jsonb_array_length(value)>0 AND NOT EXISTS (
 SELECT 1 FROM jsonb_array_elements(value) item WHERE jsonb_typeof(item)<>'object'
 OR NOT item ?& ARRAY['kind','reference','fields','status']
 OR NOT catalog_nonblank(item->'reference')
 OR item->>'kind' NOT IN ('repository','web') OR item->>'status' NOT IN ('imported','estimate','verified')
 OR jsonb_typeof(item->'fields')<>'array'
 OR EXISTS(SELECT 1 FROM jsonb_array_elements(item->'fields') field WHERE jsonb_typeof(field)<>'string')
 OR NOT catalog_object_keys_valid(item,ARRAY['kind','reference','revision','fields','status','note'])
 OR (coalesce(item->'revision','null') <> 'null'::jsonb AND jsonb_typeof(item->'revision')<>'string')
 OR (coalesce(item->'note','null') <> 'null'::jsonb AND jsonb_typeof(item->'note')<>'string')
 ),false)
$$;
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_operations_valid CHECK(catalog_operations_valid(operations));
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_references_valid CHECK(catalog_references_valid(source_refs));
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_references_valid CHECK(catalog_references_valid(source_refs));
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_valid CHECK(catalog_provider_valid(to_jsonb(provider_templates)) IS TRUE);
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_valid CHECK(catalog_model_valid(to_jsonb(provider_model_templates)) IS TRUE);
ALTER TABLE provider_templates ADD COLUMN readiness catalog_readiness GENERATED ALWAYS AS (catalog_provider_state(endpoint_mode,catalog_operation_names(authentication),authentication)) STORED;
GRANT SELECT ON provider_template_research,provider_model_template_research TO compallm_restricted;
CREATE VIEW provider_catalog_readiness AS
SELECT provider_template_id,id,
  CASE
    WHEN provider_readiness = 'incomplete' OR model_readiness = 'incomplete' THEN 'incomplete'::catalog_readiness
    WHEN provider_readiness = 'needs_configuration' OR model_readiness = 'needs_configuration' THEN 'needs_configuration'::catalog_readiness
    WHEN provider_readiness = 'needs_fetch' OR model_readiness = 'needs_fetch' THEN 'needs_fetch'::catalog_readiness
    ELSE 'ready'::catalog_readiness
  END AS readiness, provider_readiness, model_readiness
FROM (
  SELECT m.provider_template_id,m.id,m.readiness AS model_readiness,
    catalog_provider_model_state(to_jsonb(p),catalog_operation_names(m.operations)) AS provider_readiness
  FROM provider_model_templates m JOIN provider_templates p ON p.id = m.provider_template_id
) requirements;
GRANT SELECT ON provider_catalog_readiness TO compallm_restricted;

CREATE VIEW provider_catalog_auth_readiness AS
SELECT provider_template_id,id,auth_method,provider_readiness,model_readiness,
 CASE
   WHEN provider_readiness = 'incomplete' OR model_readiness = 'incomplete' THEN 'incomplete'::catalog_readiness
   WHEN provider_readiness = 'needs_configuration' OR model_readiness = 'needs_configuration' THEN 'needs_configuration'::catalog_readiness
   WHEN provider_readiness = 'needs_fetch' OR model_readiness = 'needs_fetch' THEN 'needs_fetch'::catalog_readiness
   ELSE 'ready'::catalog_readiness
 END AS readiness
FROM (
 SELECT m.provider_template_id,m.id,method AS auth_method,m.readiness AS model_readiness,
 CASE WHEN p.field_metadata ? 'request_headers' AND catalog_provider_auth_state(p.endpoint_mode,method,p.authentication->method,catalog_operation_names(m.operations)) <> 'incomplete'
 THEN 'needs_configuration'::catalog_readiness
 ELSE catalog_provider_auth_state(p.endpoint_mode,method,p.authentication->method,catalog_operation_names(m.operations)) END AS provider_readiness
 FROM provider_model_templates m JOIN provider_templates p ON p.id = m.provider_template_id
 CROSS JOIN LATERAL jsonb_object_keys(p.authentication) method
) requirements;
GRANT SELECT ON provider_catalog_auth_readiness TO compallm_restricted;
