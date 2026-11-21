CREATE FUNCTION capability_rate_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
BEGIN
 RETURN coalesce(jsonb_typeof(value)='object' AND value ?& ARRAY['amount','currency','quantity'] AND catalog_object_keys_valid(value,ARRAY['amount','currency','quantity'])
 AND (value->'amount'='null'::jsonb OR catalog_json_price_valid(value->'amount'))
 AND (value->'currency'='null'::jsonb OR (jsonb_typeof(value->'currency')='string' AND value->>'currency' ~ '^[A-Z]{3}$'))
 AND (value->'quantity'='null'::jsonb OR (jsonb_typeof(value->'quantity')='number' AND value->>'quantity' ~ '^[0-9]+$' AND (value->>'quantity')::numeric BETWEEN 1 AND 18446744073709551615)),false);
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION capability_token_rate_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE cache jsonb;
BEGIN
 IF jsonb_typeof(value)<>'object' OR NOT catalog_object_keys_valid(value,ARRAY['rate','cache']) OR NOT capability_rate_valid(value->'rate') THEN RETURN false; END IF;
 cache:=value->'cache';
 IF cache IS NULL OR cache='null'::jsonb THEN RETURN true; END IF;
 RETURN CASE cache->>'type'
 WHEN 'separate_rate' THEN catalog_object_keys_valid(cache,ARRAY['type','configuration']) AND capability_rate_valid(cache->'configuration'->'rate')
 WHEN 'standard_input_rate' THEN cache='{"type":"standard_input_rate"}'::jsonb
 WHEN 'included' THEN cache='{"type":"included"}'::jsonb
 WHEN 'unsupported' THEN cache='{"type":"unsupported"}'::jsonb ELSE false END;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION capability_usage_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE cfg jsonb; rates jsonb; item jsonb; start_value numeric; end_value numeric; position integer; size integer; kind text;
BEGIN
 IF jsonb_typeof(value)<>'object' OR (SELECT count(*) FROM jsonb_object_keys(value))<>1 THEN RETURN false; END IF;
 IF value ? 'fixed' THEN
  rates:=value->'fixed';
  IF jsonb_typeof(rates)<>'object' OR (SELECT count(*) FROM jsonb_object_keys(rates))<>1 THEN RETURN false; END IF;
  SELECT key,val INTO kind,cfg FROM jsonb_each(rates) x(key,val);
  IF kind='token' THEN RETURN capability_token_rate_valid(cfg);
  ELSIF kind='request' THEN RETURN catalog_object_keys_valid(cfg,ARRAY['rate']) AND capability_rate_valid(cfg->'rate');
  ELSIF kind='audio' THEN RETURN cfg->>'type' IN ('duration','character','utf8_bytes') AND catalog_object_keys_valid(cfg,ARRAY['type','configuration']) AND capability_rate_valid(cfg->'configuration'->'rate');
  ELSIF kind IN ('image','video','credit') THEN
   IF NOT catalog_object_keys_valid(cfg,ARRAY['rates']) OR jsonb_typeof(cfg->'rates')<>'array' THEN RETURN false; END IF;
   FOR item IN SELECT * FROM jsonb_array_elements(cfg->'rates') LOOP
    IF kind<>'credit' AND NOT capability_rate_valid(item->'rate') THEN RETURN false; END IF;
    IF kind='video' AND (item->>'unit' IS NULL OR item->>'unit' NOT IN ('seconds','videos')) THEN RETURN false; END IF;
    IF kind='credit' AND (item->>'unit' IS NULL OR item->>'unit' NOT IN ('tokens','requests','images','seconds','characters','utf8_bytes','credits','videos') OR (item->>'direction' IS NOT NULL AND item->>'direction' NOT IN ('input','output','cache_read'))) THEN RETURN false; END IF;
   END LOOP;
   RETURN true;
  END IF;
 ELSIF value ? 'tiered_token' THEN
  cfg:=value->'tiered_token';
  IF NOT catalog_object_keys_valid(cfg,ARRAY['measuredBy','tiers']) OR cfg->>'measuredBy' NOT IN ('input_tokens','total_tokens') OR jsonb_typeof(cfg->'tiers')<>'array' THEN RETURN false; END IF;
  start_value:=0;position:=0;size:=jsonb_array_length(cfg->'tiers');IF size=0 THEN RETURN false; END IF;
  FOR item IN SELECT * FROM jsonb_array_elements(cfg->'tiers') LOOP
   position:=position+1;end_value:=(item->>'toTokens')::numeric;
   IF jsonb_typeof(item->'fromTokens') IS DISTINCT FROM 'number' OR item->>'fromTokens' !~ '^[0-9]+$' OR (end_value IS NOT NULL AND (jsonb_typeof(item->'toTokens') IS DISTINCT FROM 'number' OR item->>'toTokens' !~ '^[0-9]+$' OR end_value>18446744073709551615)) OR NOT catalog_object_keys_valid(item,ARRAY['fromTokens','toTokens','rate']) OR (item->>'fromTokens')::numeric IS DISTINCT FROM start_value OR (end_value IS NULL)<>(position=size) OR end_value<=start_value OR NOT capability_token_rate_valid(item->'rate') THEN RETURN false; END IF;
   start_value:=end_value;
  END LOOP;
  RETURN true;
 ELSIF value ? 'scheduled' THEN
  cfg:=value->'scheduled';
  IF NOT catalog_object_keys_valid(cfg,ARRAY['timezone','slots']) OR cfg->>'timezone' IS DISTINCT FROM 'UTC' OR jsonb_typeof(cfg->'slots')<>'array' THEN RETURN false; END IF;
  start_value:=0;
  FOR item IN SELECT * FROM jsonb_array_elements(cfg->'slots') LOOP
   end_value:=(item->>'toHour')::numeric;
   IF jsonb_typeof(item->'fromHour') IS DISTINCT FROM 'number' OR item->>'fromHour' !~ '^[0-9]+$' OR jsonb_typeof(item->'toHour') IS DISTINCT FROM 'number' OR item->>'toHour' !~ '^[0-9]+$' OR NOT catalog_object_keys_valid(item,ARRAY['fromHour','toHour','pricing']) OR (item->>'fromHour')::numeric IS DISTINCT FROM start_value OR end_value IS NULL OR end_value<=start_value OR end_value>24 OR item->'pricing' ? 'scheduled' OR NOT capability_usage_valid(item->'pricing') THEN RETURN false; END IF;
   start_value:=end_value;
  END LOOP;
  RETURN start_value=24;
 END IF;
 RETURN false;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
CREATE FUNCTION capability_price_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(value IS NULL OR value='null'::jsonb OR (jsonb_typeof(value)='object' AND value->>'type' IN ('usage','free','included','model_billing') AND CASE WHEN value->>'type'='usage' THEN capability_usage_valid(value->'configuration') ELSE NOT value ? 'configuration' END),false)
$$;
CREATE FUNCTION priced_capability_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(model_capability_valid(value-'pricing') AND capability_price_valid(value->'pricing') AND CASE WHEN value->>'status'='supported' THEN value ? 'pricing' ELSE NOT value ? 'pricing' END,false)
$$;
CREATE FUNCTION text_capability_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value)='object' AND catalog_object_keys_valid(value,ARRAY['input','output']) AND value ?& ARRAY['input','output'] AND priced_capability_valid(value->'input') AND priced_capability_valid(value->'output') AND CASE WHEN value->'output'->>'status'='supported' THEN priced_capability_valid(value->'output'->'configuration'->'thinking') ELSE true END,false)
$$;
ALTER TABLE models ADD CHECK(text_capability_valid(text));
ALTER TABLE models ALTER COLUMN text SET DEFAULT '{"input":{"status":"unknown"},"output":{"status":"unknown"}}'::jsonb;
ALTER TABLE models ADD CHECK(priced_capability_valid(vision));
ALTER TABLE models ADD CHECK(priced_capability_valid(pdf));
ALTER TABLE models ADD CHECK(priced_capability_valid(video));
ALTER TABLE models ADD CHECK(priced_capability_valid(image_generation));
ALTER TABLE models ADD CHECK(priced_capability_valid(structured_output));
ALTER TABLE models ADD CHECK(priced_capability_valid(search));
ALTER TABLE models ADD CHECK(priced_capability_valid(audio->'input') AND priced_capability_valid(audio->'output') AND priced_capability_valid(tools->'calling') AND priced_capability_valid(tools->'forcedChoice') AND priced_capability_valid(tools->'parallelCalls'));
ALTER TABLE models ADD CHECK(billing IS NULL OR (jsonb_typeof(billing)='object' AND catalog_object_keys_valid(billing,ARRAY['subscription','free','request','credit'])));
ALTER TABLE provider_models ADD CHECK(text_capability_valid(text));
ALTER TABLE provider_models ALTER COLUMN text SET DEFAULT '{"input":{"status":"unknown"},"output":{"status":"unknown"}}'::jsonb;
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(vision));
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(pdf));
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(video));
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(image_generation));
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(structured_output));
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(search));
ALTER TABLE provider_models ADD CHECK(priced_capability_valid(audio->'input') AND priced_capability_valid(audio->'output') AND priced_capability_valid(tools->'calling') AND priced_capability_valid(tools->'forcedChoice') AND priced_capability_valid(tools->'parallelCalls'));
ALTER TABLE provider_models ADD CHECK(billing IS NULL OR (jsonb_typeof(billing)='object' AND catalog_object_keys_valid(billing,ARRAY['subscription','free','request','credit'])));
ALTER TABLE provider_model_templates ADD CHECK(text_capability_valid(text));
ALTER TABLE provider_model_templates ALTER COLUMN text SET DEFAULT '{"input":{"status":"unknown"},"output":{"status":"unknown"}}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(vision));
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(pdf));
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(video));
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(image_generation));
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(structured_output));
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(search));
ALTER TABLE provider_model_templates ADD CHECK(priced_capability_valid(audio->'input') AND priced_capability_valid(audio->'output') AND priced_capability_valid(tools->'calling') AND priced_capability_valid(tools->'forcedChoice') AND priced_capability_valid(tools->'parallelCalls'));
ALTER TABLE provider_model_templates ADD CHECK(billing IS NULL OR (jsonb_typeof(billing)='object' AND catalog_object_keys_valid(billing,ARRAY['subscription','free','request','credit'])));

CREATE FUNCTION capability_amounts_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE item record; nested jsonb;
BEGIN
 IF jsonb_typeof(value)='object' THEN
  FOR item IN SELECT * FROM jsonb_each(value) LOOP
   IF item.key IN ('amount','credits','amountPerCredit') AND item.value<>'null'::jsonb THEN
    IF NOT catalog_json_price_valid(item.value) THEN RETURN false; END IF;
   ELSIF item.key IN ('quantity','quota') AND item.value<>'null'::jsonb THEN
    IF jsonb_typeof(item.value)<>'number' OR item.value #>> '{}' !~ '^[0-9]+$' OR (item.value #>> '{}')::numeric NOT BETWEEN 1 AND 18446744073709551615 THEN RETURN false; END IF;
   ELSIF NOT capability_amounts_valid(item.value) THEN RETURN false;
   END IF;
  END LOOP;
 ELSIF jsonb_typeof(value)='array' THEN
  FOR nested IN SELECT * FROM jsonb_array_elements(value) LOOP IF NOT capability_amounts_valid(nested) THEN RETURN false; END IF; END LOOP;
 END IF;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
ALTER TABLE models ADD CHECK(capability_amounts_valid(text));
ALTER TABLE models ADD CHECK(capability_amounts_valid(vision));
ALTER TABLE models ADD CHECK(capability_amounts_valid(pdf));
ALTER TABLE models ADD CHECK(capability_amounts_valid(audio));
ALTER TABLE models ADD CHECK(capability_amounts_valid(video));
ALTER TABLE models ADD CHECK(capability_amounts_valid(image_generation));
ALTER TABLE models ADD CHECK(capability_amounts_valid(tools));
ALTER TABLE models ADD CHECK(capability_amounts_valid(structured_output));
ALTER TABLE models ADD CHECK(capability_amounts_valid(search));
ALTER TABLE models ADD CHECK(capability_amounts_valid(billing));
ALTER TABLE models ADD CHECK(text->'input'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'input'->'configuration',ARRAY['maxTokens']) AND model_capability_configuration_valid(text->'input'->'configuration')));
ALTER TABLE models ADD CHECK(text->'output'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'output'->'configuration',ARRAY['maxTokens','thinking']) AND model_capability_configuration_valid(text->'output'->'configuration') AND model_thinking_valid(text->'output'->'configuration'->'thinking') AND (text->'output'->'configuration'->'thinking'->>'status'<>'supported' OR model_capability_configuration_valid(text->'output'->'configuration'->'thinking'->'configuration'))));
ALTER TABLE models ADD CHECK(vision->>'status'<>'supported' OR (catalog_object_keys_valid(vision->'configuration',ARRAY['formats','maxImages']) AND model_capability_configuration_valid(vision->'configuration')));
ALTER TABLE models ADD CHECK(pdf->>'status'<>'supported' OR (catalog_object_keys_valid(pdf->'configuration',ARRAY['maxFiles','maxPages']) AND model_capability_configuration_valid(pdf->'configuration')));
ALTER TABLE models ADD CHECK(video->>'status'<>'supported' OR (catalog_object_keys_valid(video->'configuration',ARRAY['formats','maxDurationMs','maxFrames']) AND model_capability_configuration_valid(video->'configuration')));
ALTER TABLE models ADD CHECK(image_generation->>'status'<>'supported' OR (catalog_object_keys_valid(image_generation->'configuration',ARRAY['formats','maxImages']) AND model_capability_configuration_valid(image_generation->'configuration')));
ALTER TABLE models ADD CHECK(structured_output->>'status'<>'supported' OR (catalog_object_keys_valid(structured_output->'configuration',ARRAY['jsonMode','jsonSchema']) AND model_capability_configuration_valid((structured_output->'configuration') #- '{jsonMode,pricing}' #- '{jsonSchema,pricing}')));
ALTER TABLE models ADD CHECK(search->>'status'<>'supported' OR (catalog_object_keys_valid(search->'configuration',ARRAY['maxResults']) AND model_capability_configuration_valid(search->'configuration')));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(text));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(vision));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(pdf));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(audio));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(video));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(image_generation));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(tools));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(structured_output));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(search));
ALTER TABLE provider_models ADD CHECK(capability_amounts_valid(billing));
ALTER TABLE provider_models ADD CHECK(text->'input'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'input'->'configuration',ARRAY['maxTokens']) AND model_capability_configuration_valid(text->'input'->'configuration')));
ALTER TABLE provider_models ADD CHECK(text->'output'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'output'->'configuration',ARRAY['maxTokens','thinking']) AND model_capability_configuration_valid(text->'output'->'configuration') AND model_thinking_valid(text->'output'->'configuration'->'thinking') AND (text->'output'->'configuration'->'thinking'->>'status'<>'supported' OR model_capability_configuration_valid(text->'output'->'configuration'->'thinking'->'configuration'))));
ALTER TABLE provider_models ADD CHECK(vision->>'status'<>'supported' OR (catalog_object_keys_valid(vision->'configuration',ARRAY['formats','maxImages']) AND model_capability_configuration_valid(vision->'configuration')));
ALTER TABLE provider_models ADD CHECK(pdf->>'status'<>'supported' OR (catalog_object_keys_valid(pdf->'configuration',ARRAY['maxFiles','maxPages']) AND model_capability_configuration_valid(pdf->'configuration')));
ALTER TABLE provider_models ADD CHECK(video->>'status'<>'supported' OR (catalog_object_keys_valid(video->'configuration',ARRAY['formats','maxDurationMs','maxFrames']) AND model_capability_configuration_valid(video->'configuration')));
ALTER TABLE provider_models ADD CHECK(image_generation->>'status'<>'supported' OR (catalog_object_keys_valid(image_generation->'configuration',ARRAY['formats','maxImages']) AND model_capability_configuration_valid(image_generation->'configuration')));
ALTER TABLE provider_models ADD CHECK(structured_output->>'status'<>'supported' OR (catalog_object_keys_valid(structured_output->'configuration',ARRAY['jsonMode','jsonSchema']) AND model_capability_configuration_valid((structured_output->'configuration') #- '{jsonMode,pricing}' #- '{jsonSchema,pricing}')));
ALTER TABLE provider_models ADD CHECK(search->>'status'<>'supported' OR (catalog_object_keys_valid(search->'configuration',ARRAY['maxResults']) AND model_capability_configuration_valid(search->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(text));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(vision));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(pdf));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(audio));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(video));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(image_generation));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(tools));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(structured_output));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(search));
ALTER TABLE provider_model_templates ADD CHECK(capability_amounts_valid(billing));
ALTER TABLE provider_model_templates ADD CHECK(text->'input'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'input'->'configuration',ARRAY['maxTokens']) AND model_capability_configuration_valid(text->'input'->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(text->'output'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'output'->'configuration',ARRAY['maxTokens','thinking']) AND model_capability_configuration_valid(text->'output'->'configuration') AND model_thinking_valid(text->'output'->'configuration'->'thinking') AND (text->'output'->'configuration'->'thinking'->>'status'<>'supported' OR model_capability_configuration_valid(text->'output'->'configuration'->'thinking'->'configuration'))));
ALTER TABLE provider_model_templates ADD CHECK(vision->>'status'<>'supported' OR (catalog_object_keys_valid(vision->'configuration',ARRAY['formats','maxImages']) AND model_capability_configuration_valid(vision->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(pdf->>'status'<>'supported' OR (catalog_object_keys_valid(pdf->'configuration',ARRAY['maxFiles','maxPages']) AND model_capability_configuration_valid(pdf->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(video->>'status'<>'supported' OR (catalog_object_keys_valid(video->'configuration',ARRAY['formats','maxDurationMs','maxFrames']) AND model_capability_configuration_valid(video->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(image_generation->>'status'<>'supported' OR (catalog_object_keys_valid(image_generation->'configuration',ARRAY['formats','maxImages']) AND model_capability_configuration_valid(image_generation->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(structured_output->>'status'<>'supported' OR (catalog_object_keys_valid(structured_output->'configuration',ARRAY['jsonMode','jsonSchema']) AND model_capability_configuration_valid((structured_output->'configuration') #- '{jsonMode,pricing}' #- '{jsonSchema,pricing}')));
ALTER TABLE provider_model_templates ADD CHECK(search->>'status'<>'supported' OR (catalog_object_keys_valid(search->'configuration',ARRAY['maxResults']) AND model_capability_configuration_valid(search->'configuration')));

CREATE FUNCTION capability_token_limit_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(value IS NULL OR value='null'::jsonb OR (jsonb_typeof(value)='number' AND value #>> '{}' ~ '^[0-9]+$' AND (value #>> '{}')::numeric BETWEEN 1 AND 4294967295),false)
$$;
ALTER TABLE models ADD CHECK(capability_token_limit_valid(text->'input'->'configuration'->'maxTokens') AND capability_token_limit_valid(text->'output'->'configuration'->'maxTokens'));
ALTER TABLE models ADD CHECK(catalog_object_keys_valid(limits,ARRAY['contextTokens']));
ALTER TABLE models ADD CHECK(catalog_object_keys_valid(audio,ARRAY['input','output']) AND catalog_object_keys_valid(tools,ARRAY['calling','forcedChoice','parallelCalls']));
ALTER TABLE models ADD CHECK(structured_output->>'status'<>'supported' OR (priced_capability_valid(structured_output->'configuration'->'jsonMode') AND priced_capability_valid(structured_output->'configuration'->'jsonSchema')));
ALTER TABLE models ADD CHECK(text->'output'->'configuration'->'thinking'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'output'->'configuration'->'thinking'->'configuration',ARRAY['effortMapping','defaultEffort','budget']) AND jsonb_typeof(text->'output'->'configuration'->'thinking'->'configuration'->'effortMapping')='object'));
ALTER TABLE provider_models ADD CHECK(capability_token_limit_valid(text->'input'->'configuration'->'maxTokens') AND capability_token_limit_valid(text->'output'->'configuration'->'maxTokens'));
ALTER TABLE provider_models ADD CHECK(catalog_object_keys_valid(limits,ARRAY['contextTokens']));
ALTER TABLE provider_models ADD CHECK(catalog_object_keys_valid(audio,ARRAY['input','output']) AND catalog_object_keys_valid(tools,ARRAY['calling','forcedChoice','parallelCalls']));
ALTER TABLE provider_models ADD CHECK(structured_output->>'status'<>'supported' OR (priced_capability_valid(structured_output->'configuration'->'jsonMode') AND priced_capability_valid(structured_output->'configuration'->'jsonSchema')));
ALTER TABLE provider_models ADD CHECK(text->'output'->'configuration'->'thinking'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'output'->'configuration'->'thinking'->'configuration',ARRAY['effortMapping','defaultEffort','budget']) AND jsonb_typeof(text->'output'->'configuration'->'thinking'->'configuration'->'effortMapping')='object'));
ALTER TABLE provider_model_templates ADD CHECK(capability_token_limit_valid(text->'input'->'configuration'->'maxTokens') AND capability_token_limit_valid(text->'output'->'configuration'->'maxTokens'));
ALTER TABLE provider_model_templates ADD CHECK(catalog_object_keys_valid(limits,ARRAY['contextTokens']));
ALTER TABLE provider_model_templates ADD CHECK(catalog_object_keys_valid(audio,ARRAY['input','output']) AND catalog_object_keys_valid(tools,ARRAY['calling','forcedChoice','parallelCalls']));
ALTER TABLE provider_model_templates ADD CHECK(structured_output->>'status'<>'supported' OR (priced_capability_valid(structured_output->'configuration'->'jsonMode') AND priced_capability_valid(structured_output->'configuration'->'jsonSchema')));
ALTER TABLE provider_model_templates ADD CHECK(text->'output'->'configuration'->'thinking'->>'status'<>'supported' OR (catalog_object_keys_valid(text->'output'->'configuration'->'thinking'->'configuration',ARRAY['effortMapping','defaultEffort','budget']) AND jsonb_typeof(text->'output'->'configuration'->'thinking'->'configuration'->'effortMapping')='object'));
