CREATE OR REPLACE FUNCTION catalog_object_keys_valid(value jsonb, allowed text[]) RETURNS boolean
LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value) = 'object' AND NOT EXISTS (
   SELECT 1 FROM jsonb_object_keys(value) k WHERE k <> ALL(allowed)
 ),false)
$$;
CREATE OR REPLACE FUNCTION catalog_nonblank(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value) = 'string' AND length(btrim(value #>> '{}')) BETWEEN 1 AND 4096,false)
$$;
CREATE OR REPLACE FUNCTION catalog_url_valid(value text) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(value ~ '^https?://[^/?#@[:space:]]+([/?][^#[:space:]]*)?$'
    AND value !~* '[?&](key|api_key|apikey|token|access_token|authorization|password|secret)=',false)
$$;
CREATE OR REPLACE FUNCTION catalog_price_valid(value numeric) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT value >= 0 AND value < 1000000000000 AND value::text NOT IN ('NaN','Infinity','-Infinity')
    AND scale(value) <= 18 AND length(trunc(value)::text) + scale(value) <= 28
$$;
CREATE OR REPLACE FUNCTION catalog_json_price_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 RETURN coalesce(jsonb_typeof(value) = 'string' AND (value #>> '{}') ~ '^[0-9]+([.][0-9]+)?$'
    AND catalog_price_valid((value #>> '{}')::numeric),false);
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_inputs_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE item jsonb; names text[] := ARRAY[]::text[];
BEGIN
 IF jsonb_typeof(value) IS DISTINCT FROM 'array' OR jsonb_array_length(value) NOT BETWEEN 1 AND 32 THEN RETURN false; END IF;
 FOR item IN SELECT jsonb_array_elements(value) LOOP
   IF NOT catalog_object_keys_valid(item,ARRAY['name','type','required','allowed_values'])
     OR NOT catalog_nonblank(item->'name') OR item->>'name' = ANY(names)
     OR coalesce(item->>'type','') NOT IN ('string','url','object','positive_integer','price')
     OR item->'required' IS DISTINCT FROM 'true'::jsonb THEN RETURN false; END IF;
   IF item ? 'allowed_values' AND (
     item->>'type' <> 'string' OR jsonb_typeof(item->'allowed_values') IS DISTINCT FROM 'array'
     OR jsonb_array_length(item->'allowed_values') NOT BETWEEN 1 AND 128
     OR EXISTS(SELECT 1 FROM jsonb_array_elements(item->'allowed_values') v WHERE NOT catalog_nonblank(v))
     OR (SELECT count(*) <> count(DISTINCT v) FROM jsonb_array_elements(item->'allowed_values') v)
   ) THEN RETURN false; END IF;
   names := array_append(names,item->>'name');
 END LOOP;
 RETURN true;
END
$$;
CREATE OR REPLACE FUNCTION catalog_provider_valid(row_data jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE method text; config jsonb; placement jsonb; methods text[]; mode text := row_data->>'endpoint_mode';
BEGIN
 IF jsonb_typeof(row_data->'source_refs') IS DISTINCT FROM 'array' OR jsonb_array_length(row_data->'source_refs') = 0 THEN RETURN false; END IF;
 IF jsonb_typeof(row_data->'authentication') IS DISTINCT FROM 'object'
    OR jsonb_array_length(catalog_operation_names(row_data->'authentication')) NOT BETWEEN 1 AND 4 THEN RETURN false; END IF;
 SELECT array_agg(x) INTO methods FROM jsonb_object_keys(row_data->'authentication') x;
 IF cardinality(methods) <> (SELECT count(DISTINCT x) FROM unnest(methods) x)
   OR NOT catalog_object_keys_valid(row_data->'authentication',methods) THEN RETURN false; END IF;
 FOREACH method IN ARRAY methods LOOP
   IF method NOT IN ('api_key','oauth','none','custom') THEN RETURN false; END IF;
   config := row_data->'authentication'->method;
   IF NOT catalog_object_keys_valid(config,ARRAY['status','placements','flow','authorization_url','token_url','device_authorization_url','scopes','required_inputs','reason','source_urls','adapter'])
      OR coalesce(config->>'status','') NOT IN ('configured','unknown') THEN RETURN false; END IF;
   IF config->>'status' = 'unknown' THEN
     IF NOT catalog_nonblank(config->'reason') THEN RETURN false; END IF;
     CONTINUE;
   END IF;
   IF method = 'none' AND NOT catalog_object_keys_valid(config,ARRAY['status','placements']) THEN RETURN false; END IF;
   IF method = 'api_key' AND NOT catalog_object_keys_valid(config,ARRAY['status','placements']) THEN RETURN false; END IF;
   IF method = 'api_key' THEN
     IF jsonb_typeof(config->'placements') IS DISTINCT FROM 'array' OR jsonb_array_length(config->'placements') = 0 THEN RETURN false; END IF;
     FOR placement IN SELECT jsonb_array_elements(config->'placements') LOOP
       IF NOT catalog_object_keys_valid(placement,ARRAY['operations','location','name','prefix','adapter_requirements'])
          OR coalesce(placement->>'location','') NOT IN ('header','query')
          OR NOT catalog_nonblank(placement->'name') OR jsonb_typeof(placement->'prefix') IS DISTINCT FROM 'string'
          OR jsonb_typeof(placement->'operations') IS DISTINCT FROM 'array' OR jsonb_array_length(placement->'operations') = 0
          OR NOT placement->'operations' <@ '["openai_chat_completions","openai_responses","anthropic_messages"]'::jsonb
          OR jsonb_typeof(placement->'adapter_requirements') IS DISTINCT FROM 'array' THEN RETURN false; END IF;
     END LOOP;
   ELSIF method = 'oauth' THEN
     IF coalesce(config->>'flow','') NOT IN ('authorization_code','device_code','client_credentials')
       OR NOT catalog_url_valid(config->>'token_url')
       OR jsonb_typeof(config->'scopes') IS DISTINCT FROM 'array'
       OR NOT catalog_inputs_valid(config->'required_inputs') THEN RETURN false; END IF;
     IF config->>'flow' = 'authorization_code' AND NOT catalog_url_valid(config->>'authorization_url') THEN RETURN false; END IF;
     IF config->>'flow' = 'device_code' AND NOT catalog_url_valid(config->>'device_authorization_url') THEN RETURN false; END IF;
   ELSIF method = 'custom' AND (NOT catalog_inputs_valid(config->'required_inputs') OR NOT catalog_nonblank(config->'adapter')) THEN RETURN false;
   END IF;
 END LOOP;
 IF NOT catalog_object_keys_valid(row_data->'endpoint_config',ARRAY['required_inputs','source_urls','reason','source_index']) THEN RETURN false; END IF;
 IF mode = 'fixed' THEN
   IF NOT catalog_url_valid(row_data->>'base_url') THEN RETURN false; END IF;
 ELSIF mode = 'account_specific' THEN
   IF row_data->>'base_url' IS NOT NULL OR NOT catalog_inputs_valid(row_data->'endpoint_config'->'required_inputs') THEN RETURN false; END IF;
 ELSIF mode = 'discovered' THEN
   config := row_data->'model_info_sources'->((row_data->'endpoint_config'->>'source_index')::integer);
   IF config->>'kind' IS DISTINCT FROM 'api' OR config->>'status' IS DISTINCT FROM 'observed'
      OR NOT config->'fields' @> '["base_url"]'::jsonb THEN RETURN false; END IF;
 ELSIF mode = 'unknown' THEN
   IF NOT catalog_nonblank(row_data->'endpoint_config'->'reason') THEN RETURN false; END IF;
 ELSE RETURN false;
 END IF;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_provider_auth_state(mode catalog_endpoint_mode, method text, config jsonb, operations jsonb) RETURNS catalog_readiness
LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE operation text;
BEGIN
 IF mode = 'unknown' OR config->>'status' IS DISTINCT FROM 'configured' THEN RETURN 'incomplete'; END IF;
 IF method = 'api_key' THEN
   FOR operation IN SELECT jsonb_array_elements_text(operations) LOOP
     IF NOT EXISTS(SELECT 1 FROM jsonb_array_elements(config->'placements') p
       WHERE p->'operations' @> to_jsonb(ARRAY[operation])
       AND p->'adapter_requirements' = '[]'::jsonb) THEN RETURN 'incomplete'; END IF;
   END LOOP;
   IF operations = '[]'::jsonb AND NOT EXISTS(SELECT 1 FROM jsonb_array_elements(config->'placements') p
     WHERE p->'adapter_requirements' = '[]'::jsonb) THEN RETURN 'incomplete'; END IF;
 ELSIF method = 'custom' THEN RETURN 'incomplete';
 ELSIF method NOT IN ('none','oauth') THEN RETURN 'incomplete';
 END IF;
 IF mode = 'account_specific' OR method = 'oauth' THEN RETURN 'needs_configuration'; END IF;
 IF mode = 'discovered' THEN RETURN 'needs_fetch'; END IF;
 RETURN 'ready';
END
$$;
CREATE OR REPLACE FUNCTION catalog_provider_state(mode catalog_endpoint_mode, methods jsonb, config jsonb) RETURNS catalog_readiness
LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce((SELECT state FROM (
   SELECT catalog_provider_auth_state(mode,method,config->method,'[]'::jsonb) AS state
   FROM jsonb_array_elements_text(methods) method
 ) states ORDER BY CASE state WHEN 'ready' THEN 0 WHEN 'needs_configuration' THEN 1 WHEN 'needs_fetch' THEN 2 ELSE 3 END LIMIT 1),'incomplete'::catalog_readiness)
$$;

CREATE OR REPLACE FUNCTION catalog_resolution_valid(plan jsonb, sources jsonb, identity text) RETURNS boolean
LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE field text; entry jsonb; source jsonb;
BEGIN
 IF NOT catalog_object_keys_valid(plan,ARRAY['max_context_tokens','max_input_tokens','max_output_tokens','input_price','output_price','cached_price','pricing','specification']) THEN RETURN false; END IF;
 FOR field,entry IN SELECT * FROM jsonb_each(plan) LOOP
   IF entry->>'method' = 'api' THEN
     IF NOT catalog_object_keys_valid(entry,ARRAY['method','source_index'])
        OR jsonb_typeof(entry->'source_index') IS DISTINCT FROM 'number'
        OR entry->>'source_index' !~ '^[0-9]+$' THEN RETURN false; END IF;
     source := sources->((entry->>'source_index')::integer);
     IF source->>'kind' IS DISTINCT FROM 'api' OR source->>'status' IS DISTINCT FROM 'observed'
       OR source->'response'->>'identity_value' IS DISTINCT FROM identity
       OR source->'response'->'fields'->field IS NULL THEN RETURN false; END IF;
     IF field LIKE 'max_%' AND source->'response'->'fields'->field->>'unit' IS DISTINCT FROM 'tokens' THEN RETURN false; END IF;
     IF field IN ('input_price','output_price','cached_price') AND coalesce(source->'response'->'fields'->field->>'unit','') NOT IN ('usd_per_token','usd_per_million_tokens') THEN RETURN false; END IF;
   ELSIF entry->>'method' = 'account' THEN
     IF NOT catalog_object_keys_valid(entry,ARRAY['method','required_inputs']) OR NOT catalog_inputs_valid(entry->'required_inputs') THEN RETURN false; END IF;
   ELSE RETURN false;
   END IF;
 END LOOP;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_rates_valid(rates jsonb, cache text) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 IF NOT catalog_object_keys_valid(rates,ARRAY['input_price','output_price','cached_price'])
   OR NOT catalog_json_price_valid(rates->'input_price') OR NOT catalog_json_price_valid(rates->'output_price') THEN RETURN false; END IF;
 IF cache = 'separate_rate' THEN RETURN catalog_json_price_valid(rates->'cached_price'); END IF;
 RETURN NOT rates ? 'cached_price';
END
$$;
CREATE OR REPLACE FUNCTION catalog_route_tiers_valid(tiers jsonb) RETURNS boolean
LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE estimate_kind text; tier jsonb; lower_bound bigint := 0; upper_bound bigint; last_seen boolean := false;
BEGIN
 IF jsonb_typeof(tiers) IS DISTINCT FROM 'array' THEN RETURN false; END IF;
 IF tiers = '[]'::jsonb THEN RETURN true; END IF;
 FOR tier IN SELECT jsonb_array_elements(tiers) LOOP
   IF last_seen OR NOT catalog_object_keys_valid(tier,ARRAY['from_tokens','to_tokens','rates'])
     OR jsonb_typeof(tier->'from_tokens') IS DISTINCT FROM 'number'
     OR tier->>'from_tokens' !~ '^[0-9]+$' OR (tier->>'from_tokens')::bigint <> lower_bound
     OR NOT catalog_rates_valid(tier->'rates','separate_rate') THEN RETURN false; END IF;
   IF tier->'to_tokens' IS DISTINCT FROM 'null'::jsonb AND (
     jsonb_typeof(tier->'to_tokens') IS DISTINCT FROM 'number' OR tier->>'to_tokens' !~ '^[0-9]+$'
   ) THEN RETURN false; END IF;
   upper_bound := (tier->>'to_tokens')::bigint;
   IF upper_bound IS NULL THEN last_seen := true;
   ELSIF upper_bound <= lower_bound THEN RETURN false; END IF;
   lower_bound := upper_bound;
 END LOOP;
 RETURN last_seen;
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_route_options_valid(row_data jsonb) RETURNS boolean
LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE cfg jsonb := row_data->'pricing_config'; option jsonb; input jsonb; field text; source jsonb; tags text[] := ARRAY[]::text[]; route jsonb;
BEGIN
 IF row_data->>'billing_model' IS DISTINCT FROM 'metered' OR row_data->>'billing_unit' IS DISTINCT FROM 'tokens'
   OR row_data->>'pricing_mode' IS DISTINCT FROM 'account_specific'
   OR row_data->>'specification_mode' IS DISTINCT FROM 'account_specific'
   OR cfg->>'currency' IS DISTINCT FROM 'USD' OR cfg->'unit_quantity' IS DISTINCT FROM '1000000'::jsonb
   OR jsonb_typeof(cfg->'route_options') IS DISTINCT FROM 'array'
   OR jsonb_array_length(cfg->'route_options') NOT BETWEEN 1 AND 128
   OR cfg->'route_policy' IS DISTINCT FROM '{"allow_fallbacks":false,"exact_provider_selection":true}'::jsonb
   OR row_data->>'input_price' IS NOT NULL OR row_data->>'output_price' IS NOT NULL OR row_data->>'cached_price' IS NOT NULL
   OR row_data->>'max_context_tokens' IS NOT NULL
   OR NOT catalog_inputs_valid(cfg->'required_inputs') OR jsonb_array_length(cfg->'required_inputs') <> 1 THEN RETURN false; END IF;
 input := cfg->'required_inputs'->0;
 IF input->>'name' IS DISTINCT FROM 'provider_route' OR input->>'type' IS DISTINCT FROM 'string'
   OR row_data->'resolution_plan'->'max_context_tokens'->>'method' IS DISTINCT FROM 'account'
   OR row_data->'resolution_plan'->'max_context_tokens'->'required_inputs' IS DISTINCT FROM cfg->'required_inputs' THEN RETURN false; END IF;
 FOR option IN SELECT jsonb_array_elements(cfg->'route_options') LOOP
   IF NOT catalog_object_keys_valid(option,ARRAY['model_id','tag','provider','context_tokens','max_input_tokens','max_output_tokens','implicit_caching','rates','tiers','source_index'])
     OR option->>'model_id' IS DISTINCT FROM row_data->>'upstream_model_id'
     OR NOT catalog_nonblank(option->'tag') OR option->>'tag' = ANY(tags)
     OR NOT catalog_nonblank(option->'provider')
     OR jsonb_typeof(option->'source_index') IS DISTINCT FROM 'number' OR option->>'source_index' !~ '^[0-9]+$'
     OR NOT catalog_rates_valid(option->'rates','separate_rate') OR NOT catalog_route_tiers_valid(option->'tiers')
     OR jsonb_typeof(option->'implicit_caching') NOT IN ('boolean','null') THEN RETURN false; END IF;
   FOREACH field IN ARRAY ARRAY['context_tokens','max_input_tokens','max_output_tokens'] LOOP
     IF option->>field IS NULL THEN
       IF field = 'context_tokens' THEN RETURN false; END IF;
     ELSIF jsonb_typeof(option->field) IS DISTINCT FROM 'number' OR option->>field !~ '^[1-9][0-9]*$'
       OR (option->>field)::bigint NOT BETWEEN 1 AND 4294967295 THEN RETURN false;
     END IF;
   END LOOP;
   source := row_data->'source_refs'->((option->>'source_index')::integer);
   IF source IS NULL OR NOT catalog_nonblank(to_jsonb(source->>'reference'))
     OR source->>'status' IS DISTINCT FROM 'verified'
     OR NOT source->'fields' @> '["cached_price","field_metadata"]'::jsonb THEN RETURN false; END IF;
   SELECT value INTO route FROM jsonb_array_elements(row_data->'field_metadata'->'routing'->'conditions')
     WHERE value->>'tag' = option->>'tag' AND value->>'provider_name' = option->>'provider';
   IF route IS NULL OR option->'context_tokens' IS DISTINCT FROM route->'context_length'
     OR option->'implicit_caching' IS DISTINCT FROM route->'supports_implicit_caching'
     OR (option->'rates'->>'input_price')::numeric IS DISTINCT FROM (route->'pricing'->>'prompt')::numeric * 1000000
     OR (option->'rates'->>'output_price')::numeric IS DISTINCT FROM (route->'pricing'->>'completion')::numeric * 1000000
     OR (option->'rates'->>'cached_price')::numeric IS DISTINCT FROM (route->'pricing'->>'input_cache_read')::numeric * 1000000
     THEN RETURN false; END IF;
   IF option->'tiers' <> '[]'::jsonb AND option->'tiers'->0->'rates' IS DISTINCT FROM option->'rates' THEN RETURN false; END IF;
   tags := array_append(tags,option->>'tag');
 END LOOP;
 IF input->'allowed_values' IS DISTINCT FROM to_jsonb(tags) THEN RETURN false; END IF;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_schedule_valid(schedule jsonb, cache text) RETURNS boolean
LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE slot jsonb; lower_bound integer := 0; upper_bound integer;
BEGIN
 IF NOT catalog_object_keys_valid(schedule,ARRAY['timezone','slots'])
   OR schedule->>'timezone' IS DISTINCT FROM 'UTC'
   OR jsonb_typeof(schedule->'slots') IS DISTINCT FROM 'array'
   OR jsonb_array_length(schedule->'slots') NOT BETWEEN 1 AND 24 THEN RETURN false; END IF;
 FOR slot IN SELECT jsonb_array_elements(schedule->'slots') LOOP
   IF NOT catalog_object_keys_valid(slot,ARRAY['from_hour','to_hour','rates'])
     OR jsonb_typeof(slot->'from_hour') IS DISTINCT FROM 'number' OR slot->>'from_hour' !~ '^[0-9]+$'
     OR jsonb_typeof(slot->'to_hour') IS DISTINCT FROM 'number' OR slot->>'to_hour' !~ '^[0-9]+$'
     OR (slot->>'from_hour')::integer <> lower_bound
     OR NOT catalog_rates_valid(slot->'rates',cache) THEN RETURN false; END IF;
   upper_bound := (slot->>'to_hour')::integer;
   IF upper_bound <= lower_bound OR upper_bound > 24 THEN RETURN false; END IF;
   lower_bound := upper_bound;
 END LOOP;
 RETURN lower_bound = 24;
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_model_valid(row_data jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
 kind text := row_data->>'service_kind'; billing text := row_data->>'billing_model';
 pricing text := row_data->>'pricing_mode'; unit text := row_data->>'billing_unit'; cache text := row_data->>'cache_billing';
 spec text := row_data->>'specification_mode'; cfg jsonb := row_data->'pricing_config'; plan jsonb := row_data->'resolution_plan';
 unresolved jsonb := row_data->'unresolved_fields'; field text; value jsonb; amount numeric;
 estimate_kind text; tier jsonb; lower_bound bigint := 0; upper_bound bigint; last_seen boolean := false; sub jsonb;
BEGIN
 IF NOT catalog_resolution_valid(plan,row_data->'model_info_sources',row_data->>'upstream_model_id')
   OR NOT catalog_object_keys_valid(cfg,ARRAY['currency','unit_quantity','tiers','subscription','required_inputs','reason','native_reference','rates','conversion','source_index','free_evidence','cache_evidence','route_options','route_policy','schedule','zero_price_evidence','fallback','cost_proxy'])
   OR NOT catalog_object_keys_valid(unresolved,ARRAY['service_kind','execution','operations','capabilities','max_context_tokens','billing_model','pricing','cache_billing','input_price','output_price','cached_price'])
   OR NOT catalog_object_keys_valid(row_data->'capabilities',ARRAY['text','vision','pdf','audio','video','image_generation','audio_generation','search','reasoning','tool_call','forced_tool_choice','structured_output']) THEN RETURN false; END IF;
 FOR field,value IN SELECT * FROM jsonb_each(row_data->'capabilities') LOOP
   IF jsonb_typeof(value) NOT IN ('boolean','null') THEN RETURN false; END IF;
 END LOOP;
 FOR field,value IN SELECT * FROM jsonb_each(unresolved) LOOP
   IF NOT catalog_object_keys_valid(value,ARRAY['status','reason']) OR coalesce(value->>'status','') NOT IN ('unknown','unsupported','unpublished') OR NOT catalog_nonblank(value->'reason') THEN RETURN false; END IF;
 END LOOP;
 FOREACH field IN ARRAY ARRAY['input_price','output_price','cached_price'] LOOP
   IF row_data->>field IS NOT NULL AND NOT catalog_price_valid((row_data->>field)::numeric) THEN RETURN false; END IF;
 END LOOP;
 FOREACH estimate_kind IN ARRAY ARRAY['fallback','cost_proxy'] LOOP
 IF cfg ? estimate_kind THEN
   sub := cfg->estimate_kind;
   IF kind <> 'text'
     OR (estimate_kind = 'fallback' AND (billing IN ('free','subscription') OR unit IS DISTINCT FROM 'tokens'
       OR pricing IN ('tiered','scheduled','account_specific','not_applicable')))
     OR (estimate_kind = 'cost_proxy' AND billing NOT IN ('free','subscription','subscription_with_overage'))
     OR NOT catalog_object_keys_valid(sub,ARRAY['currency','unit_quantity','input_price','output_price','cached_price','basis','reference_provider','reference_model','source_urls','confidence','notes'])
     OR sub->>'currency' IS DISTINCT FROM 'USD' OR sub->'unit_quantity' IS DISTINCT FROM '1000000'::jsonb
     OR coalesce(sub->>'basis','') NOT IN ('same_model_other_provider','related_model','target_provider_estimate')
     OR coalesce(sub->>'confidence','') NOT IN ('high','medium','low')
     OR NOT catalog_nonblank(sub->'reference_provider') OR NOT catalog_nonblank(sub->'reference_model')
     OR NOT catalog_nonblank(sub->'notes')
     OR jsonb_typeof(sub->'source_urls') IS DISTINCT FROM 'array'
     OR jsonb_array_length(sub->'source_urls') NOT BETWEEN 1 AND 16
     OR EXISTS(SELECT 1 FROM jsonb_array_elements_text(sub->'source_urls') u WHERE NOT catalog_nonblank(to_jsonb(u)))
     OR (sub->>'input_price' IS NULL AND sub->>'output_price' IS NULL AND sub->>'cached_price' IS NULL) THEN RETURN false; END IF;
   FOREACH field IN ARRAY ARRAY['input_price','output_price','cached_price'] LOOP
     IF sub->>field IS NOT NULL AND NOT catalog_json_price_valid(sub->field) THEN RETURN false; END IF;
   END LOOP;
 END IF;
 END LOOP;
 IF cfg ? 'zero_price_evidence' THEN
   IF NOT catalog_object_keys_valid(cfg->'zero_price_evidence',ARRAY['input_price','output_price','cached_price']) THEN RETURN false; END IF;
   FOR field,value IN SELECT * FROM jsonb_each(cfg->'zero_price_evidence') LOOP
     IF row_data->>field IS NULL OR (row_data->>field)::numeric <> 0
       OR NOT catalog_object_keys_valid(value,ARRAY['source_index','reason'])
       OR NOT catalog_nonblank(value->'reason')
       OR jsonb_typeof(value->'source_index') IS DISTINCT FROM 'number'
       OR value->>'source_index' !~ '^[0-9]+$' THEN RETURN false; END IF;
     sub := row_data->'source_refs'->((value->>'source_index')::integer);
     IF sub IS NULL OR NOT catalog_nonblank(to_jsonb(sub->>'reference')) OR NOT sub->'fields' @> to_jsonb(ARRAY[field]) THEN RETURN false; END IF;
     IF sub->>'status' IS DISTINCT FROM 'verified' AND NOT coalesce((
       sub->>'status' = 'estimate'
       AND cfg->'fallback'->>field IS NOT NULL AND (cfg->'fallback'->>field)::numeric = 0
       AND row_data->'field_metadata'->field->>'status' = 'conditional'
     ),false) THEN RETURN false; END IF;
   END LOOP;
 END IF;
 FOREACH field IN ARRAY ARRAY['input_price','output_price','cached_price'] LOOP
   IF (row_data->>field)::numeric = 0 AND NOT coalesce(cfg->'zero_price_evidence' ? field,false) THEN RETURN false; END IF;
 END LOOP;
 IF length(row_data->>'upstream_model_id') NOT BETWEEN 1 AND 512 OR row_data->>'upstream_model_id' !~ '^[!-~]+$' THEN RETURN false; END IF;
 IF NOT catalog_object_keys_valid(row_data->'effort_mapping',ARRAY['low','medium','high','xhigh','max']) THEN RETURN false; END IF;
 FOR field,value IN SELECT * FROM jsonb_each(row_data->'effort_mapping') LOOP
   IF jsonb_typeof(value) IS DISTINCT FROM 'string' OR length(value #>> '{}') NOT BETWEEN 1 AND 25 OR (value #>> '{}') !~ '^[A-Za-z0-9_-]+$' THEN RETURN false; END IF;
 END LOOP;
 IF row_data->'field_metadata'->'client_access'->>'status' = 'client_restricted' AND NOT unresolved ? 'execution' THEN RETURN false; END IF;
 IF row_data->'field_metadata'->'catalog_presence'->>'status' = 'unpublished' AND NOT unresolved ? 'operations' THEN RETURN false; END IF;
 IF jsonb_typeof(row_data->'source_refs') IS DISTINCT FROM 'array' OR jsonb_array_length(row_data->'source_refs') = 0 THEN RETURN false; END IF;
 IF kind = 'unknown' AND NOT unresolved ? 'service_kind' THEN RETURN false; END IF;
 IF kind <> 'text' AND NOT unresolved ? 'execution' THEN RETURN false; END IF;
 IF kind = 'text' THEN
   IF jsonb_array_length(catalog_operation_names(row_data->'operations')) = 0 AND NOT unresolved ? 'operations' THEN RETURN false; END IF;
   IF row_data->'capabilities'->'text' IS DISTINCT FROM 'true'::jsonb AND NOT unresolved ? 'capabilities' THEN RETURN false; END IF;
   FOR field IN SELECT jsonb_array_elements_text(catalog_operation_names(row_data->'operations')) LOOP
     IF field NOT IN ('openai_chat_completions','openai_responses','anthropic_messages')
       OR coalesce(row_data->'operations'->>field,'') !~ '^/([A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' OR length(row_data->'operations'->>field)>40 THEN RETURN false; END IF;
   END LOOP;
 END IF;
 IF spec = 'stored' THEN
   IF kind = 'text' AND row_data->>'max_context_tokens' IS NULL THEN RETURN false; END IF;
   IF kind = 'embedding' AND row_data->>'max_input_tokens' IS NULL THEN RETURN false; END IF;
 ELSIF spec = 'api' THEN
   IF NOT plan ? 'max_context_tokens' OR plan->'max_context_tokens'->>'method' IS DISTINCT FROM 'api' THEN RETURN false; END IF;
 ELSIF spec = 'account_specific' THEN
   IF NOT plan ? 'max_context_tokens' OR plan->'max_context_tokens'->>'method' IS DISTINCT FROM 'account' THEN RETURN false; END IF;
 ELSIF spec = 'unknown' AND kind = 'text' AND NOT unresolved ? 'max_context_tokens' THEN RETURN false;
 ELSIF spec = 'not_applicable' AND kind = 'text' THEN RETURN false;
 END IF;
 IF billing = 'unknown' AND NOT unresolved ? 'billing_model' THEN RETURN false; END IF;
 IF billing IN ('subscription','subscription_with_overage') THEN
   sub := cfg->'subscription';
   IF NOT catalog_object_keys_valid(sub,ARRAY['product','plan_mode','required_inputs','source_urls','plan','entitlement','fee'])
      OR NOT catalog_nonblank(sub->'product') OR jsonb_typeof(sub->'source_urls') IS DISTINCT FROM 'array' OR jsonb_array_length(sub->'source_urls') = 0 THEN RETURN false; END IF;
   IF sub->>'plan_mode' = 'account_specific' THEN
     IF NOT catalog_inputs_valid(sub->'required_inputs') THEN RETURN false; END IF;
   ELSIF sub->>'plan_mode' = 'stored' THEN
     IF NOT catalog_nonblank(sub->'plan')
       OR NOT catalog_object_keys_valid(sub->'entitlement',ARRAY['unit','quota','period','overage_policy'])
       OR NOT catalog_nonblank(sub->'entitlement'->'unit')
       OR jsonb_typeof(sub->'entitlement'->'quota') IS DISTINCT FROM 'number'
       OR sub->'entitlement'->>'quota' !~ '^[1-9][0-9]*$'
       OR coalesce(sub->'entitlement'->>'period','') NOT IN ('hour','day','week','month','year')
       OR coalesce(sub->'entitlement'->>'overage_policy','') NOT IN ('blocked','metered')
       OR NOT catalog_object_keys_valid(sub->'fee',ARRAY['amount','currency','period'])
       OR NOT catalog_json_price_valid(sub->'fee'->'amount')
       OR NOT catalog_nonblank(sub->'fee'->'currency')
       OR coalesce(sub->'fee'->>'period','') NOT IN ('hour','day','week','month','year')
       THEN RETURN false; END IF;
       IF (billing = 'subscription' AND sub->'entitlement'->>'overage_policy' <> 'blocked')
         OR (billing = 'subscription_with_overage' AND sub->'entitlement'->>'overage_policy' <> 'metered') THEN RETURN false; END IF;
   ELSE RETURN false;
   END IF;
   IF sub->>'plan_mode' = 'account_specific' AND (sub ? 'plan' OR sub ? 'entitlement' OR sub ? 'fee') THEN RETURN false; END IF;
   IF billing = 'subscription' AND (pricing <> 'not_applicable' OR unit IS NOT NULL OR cache <> 'included') THEN RETURN false; END IF;
   IF billing = 'subscription_with_overage' AND pricing NOT IN ('fixed','tiered','api','account_specific') THEN RETURN false; END IF;
 END IF;
 IF billing = 'free' THEN
   IF NOT catalog_object_keys_valid(cfg->'free_evidence',ARRAY['source_index','reason','restrictions'])
     OR NOT catalog_nonblank(cfg->'free_evidence'->'reason')
     OR jsonb_typeof(cfg->'free_evidence'->'restrictions') IS DISTINCT FROM 'object'
     OR jsonb_typeof(cfg->'free_evidence'->'source_index') IS DISTINCT FROM 'number'
     OR cfg->'free_evidence'->>'source_index' !~ '^[0-9]+$' THEN RETURN false; END IF;
   value := row_data->'source_refs'->((cfg->'free_evidence'->>'source_index')::integer);
   IF value IS NULL OR NOT value->'fields' @> '["input_price","output_price"]'::jsonb
     OR NOT catalog_nonblank(to_jsonb(value->>'reference')) OR pricing <> 'not_applicable' OR unit IS NOT NULL OR cache NOT IN ('included','not_applicable') THEN RETURN false; END IF;
 END IF;
 IF billing = 'metered' AND unit IS NULL THEN RETURN false; END IF;
 IF pricing <> 'tiered' AND cfg ? 'tiers' THEN RETURN false; END IF;
 IF pricing <> 'scheduled' AND cfg ? 'schedule' THEN RETURN false; END IF;
 IF billing NOT IN ('subscription','subscription_with_overage') AND cfg ? 'subscription' THEN RETURN false; END IF;
 IF pricing = 'fixed' THEN
   IF unit = 'tokens' THEN
     IF row_data->>'input_price' IS NULL OR cfg->>'currency' IS DISTINCT FROM 'USD' OR cfg->'unit_quantity' IS DISTINCT FROM '1000000'::jsonb THEN RETURN false; END IF;
     IF kind IN ('embedding','rerank') THEN
       IF row_data->>'output_price' IS NOT NULL OR cache <> 'not_applicable' THEN RETURN false; END IF;
     ELSIF row_data->>'output_price' IS NULL THEN RETURN false;
     END IF;
   ELSE
     IF jsonb_typeof(cfg->'rates') IS DISTINCT FROM 'array' OR jsonb_array_length(cfg->'rates') = 0 OR NOT catalog_nonblank(cfg->'currency') THEN RETURN false; END IF;
     FOR value IN SELECT jsonb_array_elements(cfg->'rates') LOOP
       IF NOT catalog_object_keys_valid(value,ARRAY['amount','quantity','conditions']) OR NOT catalog_json_price_valid(value->'amount') OR jsonb_typeof(value->'quantity') IS DISTINCT FROM 'number' OR value->>'quantity' !~ '^[1-9][0-9]*$' OR (value->>'quantity')::numeric <= 0 OR jsonb_typeof(value->'conditions') IS DISTINCT FROM 'object' THEN RETURN false; END IF;
     END LOOP;
     IF unit = 'credits' AND (NOT catalog_json_price_valid(cfg->'conversion'->'usd_per_credit') OR cfg->'conversion'->>'currency' IS DISTINCT FROM 'USD') THEN RETURN false; END IF;
   END IF;
 ELSIF pricing = 'tiered' THEN
   IF unit <> 'tokens' OR cfg->>'currency' IS DISTINCT FROM 'USD' OR cfg->'unit_quantity' IS DISTINCT FROM '1000000'::jsonb OR jsonb_typeof(cfg->'tiers') IS DISTINCT FROM 'array' OR jsonb_array_length(cfg->'tiers') = 0 THEN RETURN false; END IF;
   FOR tier IN SELECT jsonb_array_elements(cfg->'tiers') LOOP
     IF last_seen OR NOT catalog_object_keys_valid(tier,ARRAY['from_tokens','to_tokens','rates'])
       OR jsonb_typeof(tier->'from_tokens') IS DISTINCT FROM 'number' OR tier->>'from_tokens' !~ '^[0-9]+$' OR (tier->>'from_tokens')::bigint <> lower_bound
       OR NOT catalog_rates_valid(tier->'rates',cache) THEN RETURN false; END IF;
     IF tier->'to_tokens' IS DISTINCT FROM 'null'::jsonb AND (jsonb_typeof(tier->'to_tokens') IS DISTINCT FROM 'number' OR tier->>'to_tokens' !~ '^[0-9]+$') THEN RETURN false; END IF;
     upper_bound := (tier->>'to_tokens')::bigint;
     IF upper_bound IS NULL THEN last_seen := true;
     ELSIF upper_bound <= lower_bound THEN RETURN false;
     END IF;
     lower_bound := upper_bound;
   END LOOP;
   IF NOT last_seen THEN RETURN false; END IF;
   IF row_data->>'input_price' IS NOT NULL OR row_data->>'output_price' IS NOT NULL OR row_data->>'cached_price' IS NOT NULL THEN RETURN false; END IF;
 ELSIF pricing = 'scheduled' THEN
   IF kind <> 'text' OR unit <> 'tokens' OR cfg->>'currency' IS DISTINCT FROM 'USD'
      OR cfg->'unit_quantity' IS DISTINCT FROM '1000000'::jsonb
      OR NOT catalog_schedule_valid(cfg->'schedule',cache)
      OR row_data->>'input_price' IS NOT NULL OR row_data->>'output_price' IS NOT NULL OR row_data->>'cached_price' IS NOT NULL THEN RETURN false; END IF;
 ELSIF pricing = 'api' THEN
   FOREACH field IN ARRAY ARRAY['input_price','output_price'] LOOP
     IF NOT plan ? field OR plan->field->>'method' IS DISTINCT FROM 'api' THEN RETURN false; END IF;
   END LOOP;
   IF cache = 'separate_rate' AND (NOT plan ? 'cached_price' OR plan->'cached_price'->>'method' IS DISTINCT FROM 'api') THEN RETURN false; END IF;
 ELSIF pricing = 'account_specific' THEN
   IF NOT catalog_inputs_valid(cfg->'required_inputs') THEN RETURN false; END IF;
 ELSIF pricing IN ('unknown','unpublished') THEN
   IF NOT unresolved ? 'pricing' THEN RETURN false; END IF;
 ELSIF pricing = 'not_applicable' THEN
   IF billing NOT IN ('subscription','free') THEN RETURN false; END IF;
 END IF;
 IF row_data->'field_metadata' ? 'siliconflow_specification' THEN
   value := row_data->'field_metadata'->'siliconflow_specification'->'conditions';
   IF value->>'website_status' = 'deprecated' AND (jsonb_array_length(catalog_operation_names(row_data->'operations')) <> 0 OR NOT unresolved ? 'operations') THEN RETURN false; END IF;
   IF kind = 'text' AND cache = 'unsupported' AND value->'kv_cache' IS DISTINCT FROM 'false'::jsonb THEN RETURN false; END IF;
   IF kind = 'text' AND cache = 'separate_rate' AND value->'kv_cache' IS DISTINCT FROM 'true'::jsonb THEN RETURN false; END IF;
   IF kind = 'text' AND row_data->>'max_context_tokens' IS NOT NULL AND row_data->>'max_output_tokens' IS NOT NULL
     AND (row_data->>'max_output_tokens')::bigint > (row_data->>'max_context_tokens')::bigint THEN RETURN false; END IF;
   IF kind = 'audio' AND value->'native_pricing'->>'unit' = '/ M UTF-8 bytes' THEN
     IF unit <> 'utf8_bytes' OR cfg->'rates'->0->'quantity' IS DISTINCT FROM '1000000'::jsonb THEN RETURN false; END IF;
   END IF;
 END IF;
 IF cache IN ('standard_input_rate','unsupported') THEN
   IF NOT catalog_object_keys_valid(cfg->'cache_evidence',ARRAY['source_index','reason'])
     OR NOT catalog_nonblank(cfg->'cache_evidence'->'reason')
     OR jsonb_typeof(cfg->'cache_evidence'->'source_index') IS DISTINCT FROM 'number'
     OR cfg->'cache_evidence'->>'source_index' !~ '^[0-9]+$' THEN RETURN false; END IF;
   value := row_data->'source_refs'->((cfg->'cache_evidence'->>'source_index')::integer);
   IF value IS NULL OR NOT value->'fields' @> '["cached_price"]'::jsonb OR NOT catalog_nonblank(to_jsonb(value->>'reference')) THEN RETURN false; END IF;
 END IF;
 IF cache = 'per_route' THEN
   IF NOT catalog_route_options_valid(row_data) THEN RETURN false; END IF;
 ELSIF cfg ? 'route_options' OR cfg ? 'route_policy' THEN RETURN false;
 END IF;
 IF cache = 'unknown' AND NOT unresolved ? 'cache_billing' THEN RETURN false; END IF;
 IF cache = 'standard_input_rate' AND unit IS DISTINCT FROM 'tokens' THEN RETURN false; END IF;
 IF cache = 'separate_rate' AND pricing = 'fixed' AND row_data->>'cached_price' IS NULL THEN RETURN false; END IF;
 IF cache <> 'separate_rate' AND row_data->>'cached_price' IS NOT NULL AND NOT coalesce((
   cache = 'unknown' AND cfg->'fallback'->>'cached_price' IS NOT NULL
   AND (cfg->'fallback'->>'cached_price')::numeric = (row_data->>'cached_price')::numeric
   AND row_data->'field_metadata'->'cached_price'->>'status' = 'conditional'
 ),false) THEN RETURN false; END IF;
 IF cache = 'not_applicable' AND kind = 'text' AND billing <> 'free' THEN RETURN false; END IF;
 IF cache = 'included' AND billing NOT IN ('free','subscription','subscription_with_overage') THEN RETURN false; END IF;
 IF pricing = 'not_applicable' AND (row_data->>'input_price' IS NOT NULL OR row_data->>'output_price' IS NOT NULL OR row_data->>'cached_price' IS NOT NULL) THEN RETURN false; END IF;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END
$$;
CREATE OR REPLACE FUNCTION catalog_model_state(spec catalog_specification_mode, pricing catalog_pricing_mode, config jsonb, unresolved jsonb, plan jsonb) RETURNS catalog_readiness
LANGUAGE sql IMMUTABLE AS $$
 SELECT CASE
   WHEN unresolved <> '{}'::jsonb THEN 'incomplete'::catalog_readiness
   WHEN spec = 'account_specific' OR pricing = 'account_specific' OR config->'subscription'->>'plan_mode' = 'account_specific'
     OR EXISTS(SELECT 1 FROM jsonb_each(plan) e WHERE e.value->>'method' = 'account') THEN 'needs_configuration'::catalog_readiness
   WHEN spec = 'api' OR pricing = 'api' OR EXISTS(SELECT 1 FROM jsonb_each(plan) e WHERE e.value->>'method' = 'api') THEN 'needs_fetch'::catalog_readiness
   ELSE 'ready'::catalog_readiness END
$$;

CREATE OR REPLACE FUNCTION catalog_provider_model_state(provider jsonb, operations jsonb) RETURNS catalog_readiness
LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE state catalog_readiness;
BEGIN
 SELECT candidate INTO state FROM (
   SELECT catalog_provider_auth_state((provider->>'endpoint_mode')::catalog_endpoint_mode,method,provider->'authentication'->method,operations) AS candidate
   FROM jsonb_object_keys(provider->'authentication') method
 ) states ORDER BY CASE candidate WHEN 'ready' THEN 0 WHEN 'needs_configuration' THEN 1 WHEN 'needs_fetch' THEN 2 ELSE 3 END LIMIT 1;
 IF state <> 'incomplete' AND provider->'field_metadata' ? 'request_headers' THEN RETURN 'needs_configuration'; END IF;
 RETURN coalesce(state,'incomplete'::catalog_readiness);
END
$$;
