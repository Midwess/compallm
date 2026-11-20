CREATE FUNCTION provider_catalog_info_pointer_valid(value text) RETURNS boolean
LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE AS $$
    SELECT length(value) <= 1024
       AND (value = '' OR left(value, 1) = '/')
       AND value !~ '[[:cntrl:]]'
       AND value !~ '~([^01]|$)'
$$;

CREATE FUNCTION provider_catalog_info_sources_valid(value jsonb) RETURNS boolean
LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE AS $$
DECLARE
    item jsonb;
    request jsonb;
    response jsonb;
    pagination jsonb;
    mapping jsonb;
    field_name text;
    key_name text;
    known_fields text[] := ARRAY[
        'base_url', 'pricing', 'specification', 'upstream_model_id', 'display_name', 'description', 'protocols', 'paths', 'capabilities',
        'max_context_tokens', 'max_input_tokens', 'max_output_tokens', 'effort_mapping',
        'input_price', 'output_price', 'cached_price'
    ];
BEGIN
    IF jsonb_typeof(value) IS DISTINCT FROM 'array' THEN RETURN false; END IF;
    IF jsonb_array_length(value) NOT BETWEEN 1 AND 32 THEN RETURN false; END IF;
    FOR item IN SELECT jsonb_array_elements(value) LOOP
        IF jsonb_typeof(item) IS DISTINCT FROM 'object'
           OR NOT item ?& ARRAY['kind', 'access', 'status', 'fields', 'notes'] THEN RETURN false; END IF;
        FOR key_name IN SELECT jsonb_object_keys(item) LOOP
            IF key_name <> ALL(ARRAY['kind','url','access','status','fields','request','response','pagination','revision','notes']) THEN RETURN false; END IF;
        END LOOP;
        IF coalesce(item->>'kind','') NOT IN ('api','documentation','repository','manual')
           OR coalesce(item->>'access','') NOT IN ('public','provider_credential','manual','unknown','unavailable')
           OR coalesce(item->>'status','') NOT IN ('observed','documented','reference','unavailable')
           OR jsonb_typeof(item->'notes') IS DISTINCT FROM 'string'
           OR length(btrim(item->>'notes')) = 0 OR length(item->>'notes') > 4096
           OR jsonb_typeof(item->'fields') IS DISTINCT FROM 'array' THEN RETURN false; END IF;
        IF EXISTS (SELECT 1 FROM jsonb_array_elements(item->'fields') AS f(v)
                   WHERE jsonb_typeof(v) IS DISTINCT FROM 'string' OR (v #>> '{}') <> ALL(known_fields))
           OR (SELECT count(*) <> count(DISTINCT v) FROM jsonb_array_elements(item->'fields') AS f(v)) THEN RETURN false; END IF;
        IF item->>'kind' = 'manual' THEN
            IF item->>'access' <> 'manual' OR item->>'status' <> 'reference'
               OR coalesce(item->'url', 'null'::jsonb) <> 'null'::jsonb
               OR coalesce(item->'request', 'null'::jsonb) <> 'null'::jsonb
               OR coalesce(item->'response', 'null'::jsonb) <> 'null'::jsonb
               OR coalesce(item->'pagination', 'null'::jsonb) <> 'null'::jsonb
               OR coalesce(item->'revision', 'null'::jsonb) <> 'null'::jsonb THEN RETURN false; END IF;
        ELSE
            IF jsonb_typeof(item->'url') IS DISTINCT FROM 'string'
               OR length(btrim(item->>'url')) = 0
               OR (item->>'kind' = 'api' AND ((item->>'url') !~ '^https?://[^/?#@[:space:]]+([/?][^#[:space:]]*)?$'
               OR (item->>'url') ~* '[?&](key|api_key|apikey|token|access_token|authorization|password|secret)='
               ))
               OR item->>'access' = 'manual' THEN RETURN false; END IF;
        END IF;
        IF coalesce(item->'revision','null'::jsonb) <> 'null'::jsonb AND jsonb_typeof(item->'revision') IS DISTINCT FROM 'string' THEN RETURN false; END IF;
        IF item->>'kind' = 'repository' AND coalesce(length(btrim(item->>'revision')),0) = 0 THEN RETURN false; END IF;
        IF item->>'kind' <> 'api' THEN
            IF coalesce(item->'request','null'::jsonb) <> 'null'::jsonb
               OR coalesce(item->'response','null'::jsonb) <> 'null'::jsonb
               OR coalesce(item->'pagination','null'::jsonb) <> 'null'::jsonb THEN RETURN false; END IF;
            CONTINUE;
        END IF;
        request := item->'request';
        IF jsonb_typeof(request) IS DISTINCT FROM 'object'
           OR NOT request ?& ARRAY['method','authentication']
           OR coalesce(request->>'method','') <> 'GET'
           OR coalesce(request->>'authentication','') NOT IN ('none','provider_api_key','provider_oauth','unknown') THEN RETURN false; END IF;
        FOR key_name IN SELECT jsonb_object_keys(request) LOOP
            IF key_name <> ALL(ARRAY['method','authentication']) THEN RETURN false; END IF;
        END LOOP;
        IF item->>'access' = 'public' AND request->>'authentication' <> 'none' THEN RETURN false; END IF;
        IF item->>'access' = 'provider_credential' AND request->>'authentication' NOT IN ('provider_api_key','provider_oauth') THEN RETURN false; END IF;
        response := coalesce(item->'response','null'::jsonb);
        IF item->>'status' = 'observed' AND response = 'null'::jsonb THEN RETURN false; END IF;
        IF response <> 'null'::jsonb THEN
            IF jsonb_typeof(response) IS DISTINCT FROM 'object'
               OR NOT response ?& ARRAY['format','object_pointer','identity_pointer','fields']
               OR coalesce(response->>'format','') NOT IN ('model_list','model_object')
               OR jsonb_typeof(response->'object_pointer') IS DISTINCT FROM 'string'
               OR jsonb_typeof(response->'identity_pointer') IS DISTINCT FROM 'string'
               OR NOT provider_catalog_info_pointer_valid(response->>'object_pointer')
               OR NOT provider_catalog_info_pointer_valid(response->>'identity_pointer')
               OR length(response->>'identity_pointer') = 0
               OR jsonb_typeof(response->'fields') IS DISTINCT FROM 'object'
               OR response->'fields' = '{}'::jsonb THEN RETURN false; END IF;
            FOR key_name IN SELECT jsonb_object_keys(response) LOOP
                IF key_name <> ALL(ARRAY['format','object_pointer','identity_pointer','identity_value','fields']) THEN RETURN false; END IF;
            END LOOP;
            IF coalesce(response->'identity_value','null'::jsonb) <> 'null'::jsonb AND jsonb_typeof(response->'identity_value') IS DISTINCT FROM 'string' THEN RETURN false; END IF;
            IF (SELECT array_agg(f ORDER BY f) FROM jsonb_array_elements_text(item->'fields') AS f)
                IS DISTINCT FROM (SELECT array_agg(f ORDER BY f) FROM jsonb_object_keys(response->'fields') AS f) THEN RETURN false; END IF;
            FOR field_name, mapping IN SELECT * FROM jsonb_each(response->'fields') LOOP
                IF field_name <> ALL(known_fields) OR jsonb_typeof(mapping) IS DISTINCT FROM 'object'
                   OR NOT mapping ?& ARRAY['pointer','unit']
                   OR jsonb_typeof(mapping->'pointer') IS DISTINCT FROM 'string'
                   OR NOT provider_catalog_info_pointer_valid(mapping->>'pointer')
                   OR coalesce(mapping->>'unit','') NOT IN ('identifier','text','tokens','usd_per_token','usd_per_million_tokens','provider_native') THEN RETURN false; END IF;
                FOR key_name IN SELECT jsonb_object_keys(mapping) LOOP
                    IF key_name <> ALL(ARRAY['pointer','unit']) THEN RETURN false; END IF;
                END LOOP;
                IF field_name IN ('max_context_tokens','max_input_tokens','max_output_tokens') AND mapping->>'unit' <> 'tokens' THEN RETURN false; END IF;
                IF field_name IN ('input_price','output_price','cached_price') AND mapping->>'unit' NOT IN ('usd_per_token','usd_per_million_tokens','provider_native') THEN RETURN false; END IF;
            END LOOP;
        END IF;
        pagination := coalesce(item->'pagination','null'::jsonb);
        IF pagination <> 'null'::jsonb THEN
            IF jsonb_typeof(pagination) IS DISTINCT FROM 'object'
               OR NOT pagination ?& ARRAY['page_parameter','page_size_parameter','page_size','total_pages_pointer']
               OR coalesce(response->>'format','') <> 'model_list'
               OR jsonb_typeof(pagination->'page_parameter') IS DISTINCT FROM 'string'
               OR jsonb_typeof(pagination->'page_size_parameter') IS DISTINCT FROM 'string'
               OR jsonb_typeof(pagination->'total_pages_pointer') IS DISTINCT FROM 'string'
               OR coalesce(length(btrim(pagination->>'page_parameter')),0) = 0
               OR coalesce(length(btrim(pagination->>'page_size_parameter')),0) = 0
               OR jsonb_typeof(pagination->'page_size') IS DISTINCT FROM 'number'
               OR coalesce(pagination->>'page_size','') !~ '^[0-9]+$'
               OR (pagination->>'page_size')::numeric NOT BETWEEN 1 AND 1000
               OR NOT provider_catalog_info_pointer_valid(pagination->>'total_pages_pointer') THEN RETURN false; END IF;
            FOR key_name IN SELECT jsonb_object_keys(pagination) LOOP
                IF key_name <> ALL(ARRAY['page_parameter','page_size_parameter','page_size','total_pages_pointer']) THEN RETURN false; END IF;
            END LOOP;
        END IF;
    END LOOP;
    RETURN true;
END
$$;

ALTER TABLE provider_templates ADD CONSTRAINT provider_templates_model_info_sources_valid
CHECK (provider_catalog_info_sources_valid(model_info_sources));
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_model_info_sources_valid
CHECK (provider_catalog_info_sources_valid(model_info_sources));
