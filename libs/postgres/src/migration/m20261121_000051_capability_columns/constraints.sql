CREATE FUNCTION model_capability_valid(value jsonb) RETURNS boolean LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_typeof(value)='object' AND CASE value->>'status'
 WHEN 'unknown' THEN value='{"status":"unknown"}'::jsonb
 WHEN 'unsupported' THEN value='{"status":"unsupported"}'::jsonb
 WHEN 'supported' THEN catalog_object_keys_valid(value,ARRAY['status','configuration']) AND jsonb_typeof(value->'configuration')='object'
 ELSE false END,false)
$$;
ALTER TABLE models ALTER COLUMN text SET NOT NULL;
ALTER TABLE models ALTER COLUMN text SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(text));
ALTER TABLE models ALTER COLUMN vision SET NOT NULL;
ALTER TABLE models ALTER COLUMN vision SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(vision));
ALTER TABLE models ALTER COLUMN pdf SET NOT NULL;
ALTER TABLE models ALTER COLUMN pdf SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(pdf));
ALTER TABLE models ALTER COLUMN video SET NOT NULL;
ALTER TABLE models ALTER COLUMN video SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(video));
ALTER TABLE models ALTER COLUMN image_generation SET NOT NULL;
ALTER TABLE models ALTER COLUMN image_generation SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(image_generation));
ALTER TABLE models ALTER COLUMN thinking SET NOT NULL;
ALTER TABLE models ALTER COLUMN thinking SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(thinking));
ALTER TABLE models ALTER COLUMN structured_output SET NOT NULL;
ALTER TABLE models ALTER COLUMN structured_output SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(structured_output));
ALTER TABLE models ALTER COLUMN search SET NOT NULL;
ALTER TABLE models ALTER COLUMN search SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE models ADD CHECK(model_capability_valid(search));
ALTER TABLE models ALTER COLUMN audio SET NOT NULL;
ALTER TABLE models ALTER COLUMN audio SET DEFAULT '{"input": {"status": "unknown"}, "output": {"status": "unknown"}}'::jsonb;
ALTER TABLE models ADD CHECK(jsonb_typeof(audio)='object' AND audio ?& ARRAY['input','output'] AND model_capability_valid(audio->'input') AND model_capability_valid(audio->'output'));
ALTER TABLE models ALTER COLUMN tools SET NOT NULL;
ALTER TABLE models ALTER COLUMN tools SET DEFAULT '{"calling": {"status": "unknown"}, "forcedChoice": {"status": "unknown"}, "parallelCalls": {"status": "unknown"}}'::jsonb;
ALTER TABLE models ADD CHECK(jsonb_typeof(tools)='object' AND tools ?& ARRAY['calling','forcedChoice','parallelCalls'] AND model_capability_valid(tools->'calling') AND model_capability_valid(tools->'forcedChoice') AND model_capability_valid(tools->'parallelCalls'));
ALTER TABLE provider_models ALTER COLUMN text SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN text SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(text));
ALTER TABLE provider_models ALTER COLUMN vision SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN vision SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(vision));
ALTER TABLE provider_models ALTER COLUMN pdf SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN pdf SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(pdf));
ALTER TABLE provider_models ALTER COLUMN video SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN video SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(video));
ALTER TABLE provider_models ALTER COLUMN image_generation SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN image_generation SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(image_generation));
ALTER TABLE provider_models ALTER COLUMN thinking SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN thinking SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(thinking));
ALTER TABLE provider_models ALTER COLUMN structured_output SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN structured_output SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(structured_output));
ALTER TABLE provider_models ALTER COLUMN search SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN search SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_models ADD CHECK(model_capability_valid(search));
ALTER TABLE provider_models ALTER COLUMN audio SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN audio SET DEFAULT '{"input": {"status": "unknown"}, "output": {"status": "unknown"}}'::jsonb;
ALTER TABLE provider_models ADD CHECK(jsonb_typeof(audio)='object' AND audio ?& ARRAY['input','output'] AND model_capability_valid(audio->'input') AND model_capability_valid(audio->'output'));
ALTER TABLE provider_models ALTER COLUMN tools SET NOT NULL;
ALTER TABLE provider_models ALTER COLUMN tools SET DEFAULT '{"calling": {"status": "unknown"}, "forcedChoice": {"status": "unknown"}, "parallelCalls": {"status": "unknown"}}'::jsonb;
ALTER TABLE provider_models ADD CHECK(jsonb_typeof(tools)='object' AND tools ?& ARRAY['calling','forcedChoice','parallelCalls'] AND model_capability_valid(tools->'calling') AND model_capability_valid(tools->'forcedChoice') AND model_capability_valid(tools->'parallelCalls'));
ALTER TABLE provider_model_templates ALTER COLUMN text SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN text SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(text));
ALTER TABLE provider_model_templates ALTER COLUMN vision SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN vision SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(vision));
ALTER TABLE provider_model_templates ALTER COLUMN pdf SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN pdf SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(pdf));
ALTER TABLE provider_model_templates ALTER COLUMN video SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN video SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(video));
ALTER TABLE provider_model_templates ALTER COLUMN image_generation SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN image_generation SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(image_generation));
ALTER TABLE provider_model_templates ALTER COLUMN thinking SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN thinking SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(thinking));
ALTER TABLE provider_model_templates ALTER COLUMN structured_output SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN structured_output SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(structured_output));
ALTER TABLE provider_model_templates ALTER COLUMN search SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN search SET DEFAULT '{"status":"unknown"}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(model_capability_valid(search));
ALTER TABLE provider_model_templates ALTER COLUMN audio SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN audio SET DEFAULT '{"input": {"status": "unknown"}, "output": {"status": "unknown"}}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(jsonb_typeof(audio)='object' AND audio ?& ARRAY['input','output'] AND model_capability_valid(audio->'input') AND model_capability_valid(audio->'output'));
ALTER TABLE provider_model_templates ALTER COLUMN tools SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN tools SET DEFAULT '{"calling": {"status": "unknown"}, "forcedChoice": {"status": "unknown"}, "parallelCalls": {"status": "unknown"}}'::jsonb;
ALTER TABLE provider_model_templates ADD CHECK(jsonb_typeof(tools)='object' AND tools ?& ARRAY['calling','forcedChoice','parallelCalls'] AND model_capability_valid(tools->'calling') AND model_capability_valid(tools->'forcedChoice') AND model_capability_valid(tools->'parallelCalls'));

CREATE FUNCTION model_capability_configuration_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE item record;
BEGIN
 IF jsonb_typeof(value)<>'object' THEN RETURN false; END IF;
 FOR item IN SELECT * FROM jsonb_each(value) LOOP
  IF item.key IN ('maxImages','maxFiles','maxPages','maxDurationMs','maxBytes','maxFrames','maxTools','maxResults','minTokens','maxTokens','defaultTokens') AND item.value<>'null'::jsonb THEN
   IF jsonb_typeof(item.value)<>'number' OR item.value #>> '{}' !~ '^[0-9]+$' OR (item.value #>> '{}')::numeric NOT BETWEEN 1 AND 4294967295 THEN RETURN false; END IF;
  ELSIF item.key='formats' AND item.value<>'null'::jsonb THEN
   IF jsonb_typeof(item.value)<>'array' OR jsonb_array_length(item.value)=0 OR EXISTS(SELECT 1 FROM jsonb_array_elements(item.value) v WHERE NOT catalog_nonblank(v)) THEN RETURN false; END IF;
  ELSIF item.key IN ('jsonMode','jsonSchema') THEN
   IF NOT model_capability_valid(item.value) THEN RETURN false; END IF;
  ELSIF item.key='budget' AND item.value<>'null'::jsonb THEN
   IF NOT model_capability_configuration_valid(item.value) OR NOT item.value ?& ARRAY['minTokens','maxTokens'] OR (item.value->>'maxTokens')::numeric < (item.value->>'minTokens')::numeric OR ((item.value->>'defaultTokens')::numeric NOT BETWEEN (item.value->>'minTokens')::numeric AND (item.value->>'maxTokens')::numeric) THEN RETURN false; END IF;
  END IF;
 END LOOP;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
ALTER TABLE models ADD CHECK(text->>'status'<>'supported' OR (catalog_object_keys_valid(text->'configuration',ARRAY[]::text[]) AND model_capability_configuration_valid(text->'configuration')));
ALTER TABLE models ADD CHECK(vision->>'status'<>'supported' OR (catalog_object_keys_valid(vision->'configuration',ARRAY['formats','maxImages']::text[]) AND model_capability_configuration_valid(vision->'configuration')));
ALTER TABLE models ADD CHECK(pdf->>'status'<>'supported' OR (catalog_object_keys_valid(pdf->'configuration',ARRAY['maxFiles','maxPages']::text[]) AND model_capability_configuration_valid(pdf->'configuration')));
ALTER TABLE models ADD CHECK(video->>'status'<>'supported' OR (catalog_object_keys_valid(video->'configuration',ARRAY['formats','maxDurationMs','maxFrames']::text[]) AND model_capability_configuration_valid(video->'configuration')));
ALTER TABLE models ADD CHECK(image_generation->>'status'<>'supported' OR (catalog_object_keys_valid(image_generation->'configuration',ARRAY['formats','maxImages']::text[]) AND model_capability_configuration_valid(image_generation->'configuration')));
ALTER TABLE models ADD CHECK(thinking->>'status'<>'supported' OR (catalog_object_keys_valid(thinking->'configuration',ARRAY['effortMapping','defaultEffort','budget']::text[]) AND model_capability_configuration_valid(thinking->'configuration')));
ALTER TABLE models ADD CHECK(structured_output->>'status'<>'supported' OR (catalog_object_keys_valid(structured_output->'configuration',ARRAY['jsonMode','jsonSchema']::text[]) AND model_capability_configuration_valid(structured_output->'configuration')));
ALTER TABLE models ADD CHECK(search->>'status'<>'supported' OR (catalog_object_keys_valid(search->'configuration',ARRAY['maxResults']::text[]) AND model_capability_configuration_valid(search->'configuration')));
ALTER TABLE models ADD CHECK(model_capability_configuration_valid(audio->'input'->'configuration') IS NOT FALSE AND model_capability_configuration_valid(audio->'output'->'configuration') IS NOT FALSE AND model_capability_configuration_valid(tools->'calling'->'configuration') IS NOT FALSE);
ALTER TABLE models ADD CHECK(thinking->>'status'<>'supported' OR (jsonb_typeof(thinking->'configuration'->'effortMapping')='object' AND (thinking->'configuration'->>'defaultEffort' IS NULL OR thinking->'configuration'->'effortMapping' ? (thinking->'configuration'->>'defaultEffort'))));
ALTER TABLE provider_models ADD CHECK(text->>'status'<>'supported' OR (catalog_object_keys_valid(text->'configuration',ARRAY[]::text[]) AND model_capability_configuration_valid(text->'configuration')));
ALTER TABLE provider_models ADD CHECK(vision->>'status'<>'supported' OR (catalog_object_keys_valid(vision->'configuration',ARRAY['formats','maxImages']::text[]) AND model_capability_configuration_valid(vision->'configuration')));
ALTER TABLE provider_models ADD CHECK(pdf->>'status'<>'supported' OR (catalog_object_keys_valid(pdf->'configuration',ARRAY['maxFiles','maxPages']::text[]) AND model_capability_configuration_valid(pdf->'configuration')));
ALTER TABLE provider_models ADD CHECK(video->>'status'<>'supported' OR (catalog_object_keys_valid(video->'configuration',ARRAY['formats','maxDurationMs','maxFrames']::text[]) AND model_capability_configuration_valid(video->'configuration')));
ALTER TABLE provider_models ADD CHECK(image_generation->>'status'<>'supported' OR (catalog_object_keys_valid(image_generation->'configuration',ARRAY['formats','maxImages']::text[]) AND model_capability_configuration_valid(image_generation->'configuration')));
ALTER TABLE provider_models ADD CHECK(thinking->>'status'<>'supported' OR (catalog_object_keys_valid(thinking->'configuration',ARRAY['effortMapping','defaultEffort','budget']::text[]) AND model_capability_configuration_valid(thinking->'configuration')));
ALTER TABLE provider_models ADD CHECK(structured_output->>'status'<>'supported' OR (catalog_object_keys_valid(structured_output->'configuration',ARRAY['jsonMode','jsonSchema']::text[]) AND model_capability_configuration_valid(structured_output->'configuration')));
ALTER TABLE provider_models ADD CHECK(search->>'status'<>'supported' OR (catalog_object_keys_valid(search->'configuration',ARRAY['maxResults']::text[]) AND model_capability_configuration_valid(search->'configuration')));
ALTER TABLE provider_models ADD CHECK(model_capability_configuration_valid(audio->'input'->'configuration') IS NOT FALSE AND model_capability_configuration_valid(audio->'output'->'configuration') IS NOT FALSE AND model_capability_configuration_valid(tools->'calling'->'configuration') IS NOT FALSE);
ALTER TABLE provider_models ADD CHECK(thinking->>'status'<>'supported' OR (jsonb_typeof(thinking->'configuration'->'effortMapping')='object' AND (thinking->'configuration'->>'defaultEffort' IS NULL OR thinking->'configuration'->'effortMapping' ? (thinking->'configuration'->>'defaultEffort'))));
ALTER TABLE provider_model_templates ADD CHECK(text->>'status'<>'supported' OR (catalog_object_keys_valid(text->'configuration',ARRAY[]::text[]) AND model_capability_configuration_valid(text->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(vision->>'status'<>'supported' OR (catalog_object_keys_valid(vision->'configuration',ARRAY['formats','maxImages']::text[]) AND model_capability_configuration_valid(vision->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(pdf->>'status'<>'supported' OR (catalog_object_keys_valid(pdf->'configuration',ARRAY['maxFiles','maxPages']::text[]) AND model_capability_configuration_valid(pdf->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(video->>'status'<>'supported' OR (catalog_object_keys_valid(video->'configuration',ARRAY['formats','maxDurationMs','maxFrames']::text[]) AND model_capability_configuration_valid(video->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(image_generation->>'status'<>'supported' OR (catalog_object_keys_valid(image_generation->'configuration',ARRAY['formats','maxImages']::text[]) AND model_capability_configuration_valid(image_generation->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(thinking->>'status'<>'supported' OR (catalog_object_keys_valid(thinking->'configuration',ARRAY['effortMapping','defaultEffort','budget']::text[]) AND model_capability_configuration_valid(thinking->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(structured_output->>'status'<>'supported' OR (catalog_object_keys_valid(structured_output->'configuration',ARRAY['jsonMode','jsonSchema']::text[]) AND model_capability_configuration_valid(structured_output->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(search->>'status'<>'supported' OR (catalog_object_keys_valid(search->'configuration',ARRAY['maxResults']::text[]) AND model_capability_configuration_valid(search->'configuration')));
ALTER TABLE provider_model_templates ADD CHECK(model_capability_configuration_valid(audio->'input'->'configuration') IS NOT FALSE AND model_capability_configuration_valid(audio->'output'->'configuration') IS NOT FALSE AND model_capability_configuration_valid(tools->'calling'->'configuration') IS NOT FALSE);
ALTER TABLE provider_model_templates ADD CHECK(thinking->>'status'<>'supported' OR (jsonb_typeof(thinking->'configuration'->'effortMapping')='object' AND (thinking->'configuration'->>'defaultEffort' IS NULL OR thinking->'configuration'->'effortMapping' ? (thinking->'configuration'->>'defaultEffort'))));

ALTER TABLE models ADD CHECK(catalog_object_keys_valid(audio,ARRAY['input','output']) AND catalog_object_keys_valid(tools,ARRAY['calling','forcedChoice','parallelCalls']));
ALTER TABLE models ADD CHECK(coalesce(thinking->>'status'<>'supported' OR (jsonb_typeof(thinking->'configuration'->'effortMapping')='object' AND catalog_object_keys_valid(thinking->'configuration'->'effortMapping',ARRAY['low','medium','high','xhigh','max'])),false));
ALTER TABLE models ADD CHECK(structured_output->>'status'<>'supported' OR structured_output->'configuration' ?& ARRAY['jsonMode','jsonSchema']);
ALTER TABLE models ADD CHECK(audio->'input'->>'status'<>'supported' OR catalog_object_keys_valid(audio->'input'->'configuration',ARRAY['formats','maxDurationMs','maxBytes']));
ALTER TABLE models ADD CHECK(audio->'output'->>'status'<>'supported' OR catalog_object_keys_valid(audio->'output'->'configuration',ARRAY['formats','maxDurationMs','maxBytes']));
ALTER TABLE models ADD CHECK(tools->'calling'->>'status'<>'supported' OR catalog_object_keys_valid(tools->'calling'->'configuration',ARRAY['maxTools']));

ALTER TABLE provider_models ADD CHECK(catalog_object_keys_valid(audio,ARRAY['input','output']) AND catalog_object_keys_valid(tools,ARRAY['calling','forcedChoice','parallelCalls']));
ALTER TABLE provider_models ADD CHECK(coalesce(thinking->>'status'<>'supported' OR (jsonb_typeof(thinking->'configuration'->'effortMapping')='object' AND catalog_object_keys_valid(thinking->'configuration'->'effortMapping',ARRAY['low','medium','high','xhigh','max'])),false));
ALTER TABLE provider_models ADD CHECK(structured_output->>'status'<>'supported' OR structured_output->'configuration' ?& ARRAY['jsonMode','jsonSchema']);
ALTER TABLE provider_models ADD CHECK(audio->'input'->>'status'<>'supported' OR catalog_object_keys_valid(audio->'input'->'configuration',ARRAY['formats','maxDurationMs','maxBytes']));
ALTER TABLE provider_models ADD CHECK(audio->'output'->>'status'<>'supported' OR catalog_object_keys_valid(audio->'output'->'configuration',ARRAY['formats','maxDurationMs','maxBytes']));
ALTER TABLE provider_models ADD CHECK(tools->'calling'->>'status'<>'supported' OR catalog_object_keys_valid(tools->'calling'->'configuration',ARRAY['maxTools']));

ALTER TABLE provider_model_templates ADD CHECK(catalog_object_keys_valid(audio,ARRAY['input','output']) AND catalog_object_keys_valid(tools,ARRAY['calling','forcedChoice','parallelCalls']));
ALTER TABLE provider_model_templates ADD CHECK(coalesce(thinking->>'status'<>'supported' OR (jsonb_typeof(thinking->'configuration'->'effortMapping')='object' AND catalog_object_keys_valid(thinking->'configuration'->'effortMapping',ARRAY['low','medium','high','xhigh','max'])),false));
ALTER TABLE provider_model_templates ADD CHECK(structured_output->>'status'<>'supported' OR structured_output->'configuration' ?& ARRAY['jsonMode','jsonSchema']);
ALTER TABLE provider_model_templates ADD CHECK(audio->'input'->>'status'<>'supported' OR catalog_object_keys_valid(audio->'input'->'configuration',ARRAY['formats','maxDurationMs','maxBytes']));
ALTER TABLE provider_model_templates ADD CHECK(audio->'output'->>'status'<>'supported' OR catalog_object_keys_valid(audio->'output'->'configuration',ARRAY['formats','maxDurationMs','maxBytes']));
ALTER TABLE provider_model_templates ADD CHECK(tools->'calling'->>'status'<>'supported' OR catalog_object_keys_valid(tools->'calling'->'configuration',ARRAY['maxTools']));

CREATE FUNCTION model_thinking_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE cfg jsonb; budget jsonb;
BEGIN
 IF value->>'status'<>'supported' THEN RETURN true; END IF;
 cfg:=value->'configuration'; budget:=cfg->'budget';
 IF EXISTS(SELECT 1 FROM jsonb_each(cfg->'effortMapping') e WHERE jsonb_typeof(e.value)<>'string' OR length(e.value #>> '{}') NOT BETWEEN 1 AND 25 OR e.value #>> '{}' !~ '^[A-Za-z0-9_-]+$') THEN RETURN false; END IF;
 IF budget IS NOT NULL AND budget<>'null'::jsonb THEN
  IF NOT catalog_object_keys_valid(budget,ARRAY['minTokens','maxTokens','defaultTokens']) OR jsonb_typeof(budget->'minTokens') IS DISTINCT FROM 'number' OR jsonb_typeof(budget->'maxTokens') IS DISTINCT FROM 'number' THEN RETURN false; END IF;
 END IF;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
ALTER TABLE models ADD CHECK(model_thinking_valid(thinking));
ALTER TABLE provider_models ADD CHECK(model_thinking_valid(thinking));
ALTER TABLE provider_model_templates ADD CHECK(model_thinking_valid(thinking));
