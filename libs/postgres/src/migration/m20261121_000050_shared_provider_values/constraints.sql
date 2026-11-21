CREATE FUNCTION shared_catalog_connection_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value)='object'
 AND catalog_object_keys_valid(value,ARRAY['baseUrl','path','timeoutMs','query','authentication','endpoint'])
 AND value ?& ARRAY['baseUrl','timeoutMs','query','authentication','endpoint']
 AND (value->'baseUrl'='null'::jsonb OR catalog_nonblank(value->'baseUrl'))
 AND jsonb_typeof(value->'query')='object'
 AND jsonb_typeof(value->'endpoint')='object'
 AND value->'endpoint'->>'mode' IN ('fixed','account_specific','discovered','unknown')
 AND jsonb_typeof(value->'endpoint'->'requiredInputs')='array'
 AND (value->'authentication'='null'::jsonb OR (jsonb_typeof(value->'authentication')='object' AND value->'authentication'->>'type' IN ('api_key','o_auth','none','custom'))),false)
$$;
CREATE FUNCTION shared_catalog_connections_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 RETURN jsonb_typeof(value)='object' AND NOT EXISTS(SELECT 1 FROM jsonb_each(value) e WHERE e.key NOT IN ('openai','openai_responses','anthropic','gemini','minimax','openrouter','xai') OR NOT shared_catalog_connection_valid(e.value));
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION shared_catalog_capabilities_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 RETURN jsonb_typeof(value)='array' AND NOT EXISTS(SELECT 1 FROM jsonb_array_elements(value) v WHERE jsonb_typeof(v)<>'string' OR v #>> '{}' NOT IN ('text','vision','pdf','audio','video','image_generation','audio_generation','search','reasoning','tool_call','forced_tool_choice','structured_output')) AND jsonb_array_length(value)=(SELECT count(DISTINCT v) FROM jsonb_array_elements(value) v);
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION shared_catalog_limits_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 RETURN jsonb_typeof(value)='object' AND catalog_object_keys_valid(value,ARRAY['contextTokens','maxInputTokens','maxOutputTokens','maxImagesPerRequest','maxDurationMs','maxAudioBytes']) AND NOT EXISTS(SELECT 1 FROM jsonb_each(value) e WHERE e.value<>'null'::jsonb AND (jsonb_typeof(e.value)<>'number' OR e.value #>> '{}' !~ '^[0-9]+$' OR (e.value #>> '{}')::numeric NOT BETWEEN 1 AND 4294967295));
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
ALTER TABLE provider_templates ADD CONSTRAINT provider_templates_connection_object CHECK(jsonb_typeof(connection)='object');
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_connections_object CHECK(jsonb_typeof(connections)='object');
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_capabilities_array CHECK(jsonb_typeof(capabilities)='array');
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_limits_object CHECK(jsonb_typeof(limits)='object');
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_thinking_object CHECK(thinking IS NULL OR jsonb_typeof(thinking)='object');
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_pricing_object CHECK(pricing IS NULL OR jsonb_typeof(pricing)='object');
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_routes_object CHECK(route_selection IS NULL OR jsonb_typeof(route_selection)='object');

ALTER TABLE provider_templates ADD CONSTRAINT provider_templates_connection_values CHECK(shared_catalog_connection_valid(connection));
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_connection_values CHECK(shared_catalog_connections_valid(connections));
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_capabilities_values CHECK(shared_catalog_capabilities_valid(capabilities));
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_limits_values CHECK(shared_catalog_limits_valid(limits));
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_service_values CHECK(service_kind IS NULL OR service_kind IN ('llm','image','video','embedding','tts','stt','audio','rerank'));
ALTER TABLE provider_template_research ADD CONSTRAINT provider_template_research_snapshot_object CHECK(jsonb_typeof(configuration_snapshot)='object');
ALTER TABLE provider_model_template_research ADD CONSTRAINT provider_model_template_research_snapshot_object CHECK(jsonb_typeof(configuration_snapshot)='object');

CREATE FUNCTION shared_catalog_rate_complete(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 RETURN coalesce(jsonb_typeof(value)='object' AND catalog_json_price_valid(value->'amount')
 AND value->>'currency' ~ '^[A-Z]{3}$'
 AND value->>'quantity' ~ '^[0-9]+$' AND (value->>'quantity')::numeric>0,false);
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION shared_catalog_token_complete(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT shared_catalog_rate_complete(value->'input') AND shared_catalog_rate_complete(value->'output') AND coalesce(CASE value->'cache'->>'type'
 WHEN 'separate_rate' THEN shared_catalog_rate_complete(value->'cache'->'configuration'->'rate')
 WHEN 'standard_input_rate' THEN true WHEN 'included' THEN true WHEN 'unsupported' THEN true ELSE false END,false)
$$;
CREATE FUNCTION shared_catalog_pricing_complete(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 IF value ? 'free' THEN RETURN jsonb_typeof(value->'free')='object'; END IF;
 IF value->'usage'->'fixed' ? 'token' THEN RETURN shared_catalog_token_complete(value->'usage'->'fixed'->'token'); END IF;
 IF value->'usage' ? 'tiered_token' THEN
  RETURN jsonb_array_length(value->'usage'->'tiered_token'->'tiers')>0 AND NOT EXISTS(SELECT 1 FROM jsonb_array_elements(value->'usage'->'tiered_token'->'tiers') v WHERE NOT shared_catalog_token_complete(v->'rates'));
 END IF;
 IF value->'usage' ? 'scheduled' THEN
  RETURN jsonb_array_length(value->'usage'->'scheduled'->'slots')>0 AND NOT EXISTS(SELECT 1 FROM jsonb_array_elements(value->'usage'->'scheduled'->'slots') v WHERE NOT shared_catalog_pricing_complete(jsonb_build_object('usage',v->'pricing')));
 END IF;
 RETURN false;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
