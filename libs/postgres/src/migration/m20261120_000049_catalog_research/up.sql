LOCK TABLE provider_templates,provider_model_templates,provider_template_research,provider_model_template_research IN ACCESS EXCLUSIVE MODE;
DROP VIEW provider_catalog_auth_readiness;
DROP VIEW provider_catalog_readiness;
ALTER TABLE provider_templates DROP CONSTRAINT catalog_provider_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT catalog_model_valid;
ALTER TABLE provider_templates DROP CONSTRAINT IF EXISTS catalog_provider_references_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_references_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_operations_valid;
ALTER TABLE provider_templates DROP CONSTRAINT provider_templates_model_info_sources_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT provider_model_templates_model_info_sources_valid;
ALTER TABLE provider_templates DROP COLUMN readiness;
ALTER TABLE provider_model_templates DROP COLUMN readiness;
CREATE OR REPLACE FUNCTION catalog_operations_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value)='object' AND NOT EXISTS (
 SELECT 1 FROM jsonb_each(value) e WHERE key NOT IN ('openai_chat_completions','openai_responses','anthropic_messages') OR jsonb_typeof(e.value) <> 'string' OR e.value #>> '{}' !~ '^/([A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' OR length(e.value #>> '{}')>40),false)
$$;
CREATE OR REPLACE FUNCTION catalog_references_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
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

CREATE TYPE catalog_research_progress AS ENUM ('in_progress','complete');
CREATE FUNCTION catalog_configuration_json(value jsonb) RETURNS jsonb LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE result jsonb; item record;
BEGIN
 IF jsonb_typeof(value)='object' THEN
  result:='{}';
  FOR item IN SELECT * FROM jsonb_each(value) LOOP
   IF item.key <> ALL(ARRAY['source_urls','source_url','source_index','source_refs','native_reference','free_evidence','cache_evidence','zero_price_evidence','fallback','cost_proxy']) THEN
    result:=result || jsonb_build_object(item.key,catalog_configuration_json(item.value));
   END IF;
  END LOOP;
  RETURN result;
 ELSIF jsonb_typeof(value)='array' THEN
  RETURN coalesce((SELECT jsonb_agg(catalog_configuration_json(v) ORDER BY n) FROM jsonb_array_elements(value) WITH ORDINALITY x(v,n)),'[]');
 END IF;
 RETURN value;
END $$;
CREATE FUNCTION catalog_restore_review(review jsonb,configuration jsonb) RETURNS jsonb LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE result jsonb; item record;
BEGIN
 IF jsonb_typeof(configuration)='object' THEN
  result:=coalesce((SELECT jsonb_object_agg(key,value) FROM jsonb_each(coalesce(review,'{}')) WHERE key=ANY(ARRAY['source_urls','source_url','source_index','source_refs','native_reference','free_evidence','cache_evidence','zero_price_evidence','fallback','cost_proxy'])),'{}');
  FOR item IN SELECT * FROM jsonb_each(configuration) LOOP
   result:=result || jsonb_build_object(item.key,catalog_restore_review(review->item.key,item.value));
  END LOOP;
  RETURN result;
 ELSIF jsonb_typeof(configuration)='array' THEN
  RETURN coalesce((SELECT jsonb_agg(catalog_restore_review(review->(n::integer-1),v) ORDER BY n) FROM jsonb_array_elements(configuration) WITH ORDINALITY x(v,n)),'[]');
 END IF;
 RETURN configuration;
END $$;
CREATE FUNCTION catalog_provider_document(template jsonb,research jsonb) RETURNS jsonb LANGUAGE sql IMMUTABLE AS $$
 SELECT template || jsonb_build_object(
 'protocol',template->'connection'->'protocol','base_url',template->'connection'->'base_url',
 'authentication',catalog_restore_review(research->'authentication',template->'connection'->'authentication'),'endpoint_mode',template->'connection'->'endpoint_mode',
 'endpoint_config',catalog_restore_review(research->'endpoint_config',template->'connection'->'endpoint'),
 'source_refs',research->'source_refs','field_metadata',research->'field_metadata','model_info_sources',research->'model_info_sources')
$$;
CREATE FUNCTION catalog_model_document(template jsonb,research jsonb) RETURNS jsonb LANGUAGE sql IMMUTABLE AS $$
 SELECT template || jsonb_build_object(
 'operations',template->'connection'->'operations',
 'max_context_tokens',template->'limits'->'max_context_tokens','max_input_tokens',template->'limits'->'max_input_tokens','max_output_tokens',template->'limits'->'max_output_tokens',
 'effort_mapping',template->'thinking'->'effort_mapping',
 'input_price',template->'pricing'->'input_price','output_price',template->'pricing'->'output_price','cached_price',template->'pricing'->'cached_price',
 'billing_model',template->'pricing'->'billing_model','billing_unit',template->'pricing'->'billing_unit',
 'pricing_mode',template->'pricing'->'mode','cache_billing',template->'pricing'->'cache_billing',
 'pricing_config',catalog_restore_review(research->'pricing_config',template->'pricing'->'configuration'),
 'source_refs',research->'source_refs','field_metadata',research->'field_metadata','model_info_sources',research->'model_info_sources',
 'specification_mode',research->'specification_mode','resolution_plan',research->'resolution_plan','unresolved_fields',research->'unresolved_fields')
$$;
CREATE FUNCTION catalog_provider_configuration_state(connection jsonb) RETURNS catalog_readiness LANGUAGE sql IMMUTABLE AS $$
 SELECT catalog_provider_state((connection->>'endpoint_mode')::catalog_endpoint_mode,catalog_operation_names(connection->'authentication'),connection->'authentication')
$$;
CREATE FUNCTION catalog_model_configuration_state(connection jsonb,limits jsonb,pricing jsonb) RETURNS catalog_readiness LANGUAGE sql IMMUTABLE AS $$
 SELECT CASE WHEN connection->'operations'='{}'::jsonb THEN 'incomplete'::catalog_readiness
 WHEN limits->>'max_context_tokens' IS NULL OR pricing->>'input_price' IS NULL OR pricing->>'output_price' IS NULL OR pricing->>'cached_price' IS NULL
 THEN 'needs_configuration'::catalog_readiness ELSE 'ready'::catalog_readiness END
$$;

ALTER TABLE provider_template_research ADD COLUMN source_refs jsonb;
ALTER TABLE provider_template_research ADD COLUMN field_metadata jsonb;
ALTER TABLE provider_template_research ADD COLUMN model_info_sources jsonb;
ALTER TABLE provider_template_research ADD COLUMN endpoint_config jsonb;
ALTER TABLE provider_template_research ADD COLUMN authentication jsonb;
ALTER TABLE provider_template_research ADD COLUMN state catalog_research_progress NOT NULL DEFAULT 'in_progress';
UPDATE provider_template_research r SET source_refs=p.source_refs,field_metadata=p.field_metadata,model_info_sources=p.model_info_sources,endpoint_config=p.endpoint_config,authentication=p.authentication,
 state=CASE WHEN catalog_provider_state(p.endpoint_mode,catalog_operation_names(p.authentication),p.authentication)='ready' THEN 'complete'::catalog_research_progress ELSE 'in_progress'::catalog_research_progress END FROM provider_templates p WHERE p.id=r.provider_template_id;
ALTER TABLE provider_model_template_research ADD COLUMN source_refs jsonb;
ALTER TABLE provider_model_template_research ADD COLUMN field_metadata jsonb;
ALTER TABLE provider_model_template_research ADD COLUMN model_info_sources jsonb;
ALTER TABLE provider_model_template_research ADD COLUMN pricing_config jsonb;
ALTER TABLE provider_model_template_research ADD COLUMN specification_mode catalog_specification_mode;
ALTER TABLE provider_model_template_research ADD COLUMN resolution_plan jsonb;
ALTER TABLE provider_model_template_research ADD COLUMN unresolved_fields jsonb;
ALTER TABLE provider_model_template_research ADD COLUMN state catalog_research_progress NOT NULL DEFAULT 'in_progress';
UPDATE provider_model_template_research r SET source_refs=m.source_refs,field_metadata=m.field_metadata,model_info_sources=m.model_info_sources,pricing_config=m.pricing_config,specification_mode=m.specification_mode,resolution_plan=m.resolution_plan,unresolved_fields=m.unresolved_fields,
 state=CASE WHEN catalog_model_state(m.specification_mode,m.pricing_mode,m.pricing_config,m.unresolved_fields,m.resolution_plan)='ready' THEN 'complete'::catalog_research_progress ELSE 'in_progress'::catalog_research_progress END FROM provider_model_templates m WHERE m.provider_template_id=r.provider_template_id AND m.id=r.model_template_id;
ALTER TABLE provider_templates ADD COLUMN connection jsonb;
UPDATE provider_templates SET connection=jsonb_build_object('protocol',protocol,'base_url',base_url,'authentication',catalog_configuration_json(authentication),'endpoint_mode',endpoint_mode,'endpoint',catalog_configuration_json(endpoint_config));
ALTER TABLE provider_model_templates ADD COLUMN headers jsonb NOT NULL DEFAULT '{}'::jsonb CHECK(jsonb_typeof(headers)='object');
ALTER TABLE provider_model_templates ADD COLUMN connection jsonb;
ALTER TABLE provider_model_templates ADD COLUMN limits jsonb;
ALTER TABLE provider_model_templates ADD COLUMN thinking jsonb;
ALTER TABLE provider_model_templates ADD COLUMN pricing jsonb;
UPDATE provider_model_templates SET connection=jsonb_build_object('operations',operations),
 limits=jsonb_build_object('max_context_tokens',max_context_tokens,'max_input_tokens',max_input_tokens,'max_output_tokens',max_output_tokens),
 thinking=jsonb_build_object('effort_mapping',effort_mapping),
 pricing=jsonb_build_object('billing_model',billing_model,'billing_unit',billing_unit,'mode',pricing_mode,'cache_billing',cache_billing,'input_price',input_price::text,'output_price',output_price::text,'cached_price',cached_price::text,'configuration',catalog_configuration_json(pricing_config));
ALTER TABLE provider_templates DROP COLUMN source_refs;
ALTER TABLE provider_templates DROP COLUMN field_metadata;
ALTER TABLE provider_templates DROP COLUMN model_info_sources;
ALTER TABLE provider_templates DROP COLUMN protocol;
ALTER TABLE provider_templates DROP COLUMN base_url;
ALTER TABLE provider_templates DROP COLUMN authentication;
ALTER TABLE provider_templates DROP COLUMN endpoint_mode;
ALTER TABLE provider_templates DROP COLUMN endpoint_config;
ALTER TABLE provider_model_templates DROP COLUMN source_refs;
ALTER TABLE provider_model_templates DROP COLUMN field_metadata;
ALTER TABLE provider_model_templates DROP COLUMN model_info_sources;
ALTER TABLE provider_model_templates DROP COLUMN operations;
ALTER TABLE provider_model_templates DROP COLUMN max_context_tokens;
ALTER TABLE provider_model_templates DROP COLUMN max_input_tokens;
ALTER TABLE provider_model_templates DROP COLUMN max_output_tokens;
ALTER TABLE provider_model_templates DROP COLUMN effort_mapping;
ALTER TABLE provider_model_templates DROP COLUMN input_price;
ALTER TABLE provider_model_templates DROP COLUMN output_price;
ALTER TABLE provider_model_templates DROP COLUMN cached_price;
ALTER TABLE provider_model_templates DROP COLUMN billing_model;
ALTER TABLE provider_model_templates DROP COLUMN billing_unit;
ALTER TABLE provider_model_templates DROP COLUMN pricing_mode;
ALTER TABLE provider_model_templates DROP COLUMN cache_billing;
ALTER TABLE provider_model_templates DROP COLUMN pricing_config;
ALTER TABLE provider_model_templates DROP COLUMN specification_mode;
ALTER TABLE provider_model_templates DROP COLUMN resolution_plan;
ALTER TABLE provider_model_templates DROP COLUMN unresolved_fields;
ALTER TABLE provider_template_research ALTER COLUMN source_refs SET NOT NULL;
ALTER TABLE provider_template_research ALTER COLUMN field_metadata SET NOT NULL;
ALTER TABLE provider_template_research ALTER COLUMN model_info_sources SET NOT NULL;
ALTER TABLE provider_template_research ALTER COLUMN endpoint_config SET NOT NULL;
ALTER TABLE provider_template_research ALTER COLUMN authentication SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN source_refs SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN field_metadata SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN model_info_sources SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN pricing_config SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN specification_mode SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN resolution_plan SET NOT NULL;
ALTER TABLE provider_model_template_research ALTER COLUMN unresolved_fields SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN connection SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN connection SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN limits SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN thinking SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN pricing SET NOT NULL;
ALTER TABLE provider_templates ADD COLUMN readiness catalog_readiness GENERATED ALWAYS AS(catalog_provider_configuration_state(connection)) STORED;
ALTER TABLE provider_model_templates ADD COLUMN readiness catalog_readiness GENERATED ALWAYS AS(catalog_model_configuration_state(connection,limits,pricing)) STORED;
ALTER TABLE provider_templates ADD CONSTRAINT provider_template_research_link FOREIGN KEY(id) REFERENCES provider_template_research(provider_template_id) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE provider_model_templates ADD CONSTRAINT model_template_research_link FOREIGN KEY(provider_template_id,id) REFERENCES provider_model_template_research(provider_template_id,model_template_id) DEFERRABLE INITIALLY DEFERRED;
