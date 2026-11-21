LOCK TABLE provider_templates, provider_model_templates IN ACCESS EXCLUSIVE MODE;
DROP VIEW provider_catalog_auth_readiness;
DROP VIEW provider_catalog_readiness;
ALTER TABLE provider_templates DROP CONSTRAINT catalog_provider_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT catalog_model_valid;
ALTER TABLE provider_templates DROP COLUMN readiness;
CREATE FUNCTION catalog_operation_names(value jsonb) RETURNS jsonb LANGUAGE sql IMMUTABLE AS $$
 SELECT coalesce(jsonb_agg(key ORDER BY key),'[]'::jsonb) FROM jsonb_object_keys(value) key
$$;

CREATE TABLE provider_template_research (
 provider_template_id text PRIMARY KEY REFERENCES provider_templates(id) ON DELETE CASCADE,
 metadata jsonb NOT NULL CHECK(jsonb_typeof(metadata)='object'),
 authentication_order jsonb NOT NULL CHECK(jsonb_typeof(authentication_order)='array')
);
CREATE TABLE provider_model_template_research (
 provider_template_id text NOT NULL,
 model_template_id text NOT NULL,
 metadata jsonb NOT NULL CHECK(jsonb_typeof(metadata)='object'),
 operation_order jsonb NOT NULL CHECK(jsonb_typeof(operation_order)='array'),
 PRIMARY KEY(provider_template_id,model_template_id),
 FOREIGN KEY(provider_template_id,model_template_id) REFERENCES provider_model_templates(provider_template_id,id) ON DELETE CASCADE
);
INSERT INTO provider_template_research SELECT id,
 coalesce((SELECT jsonb_object_agg(key,value) FROM jsonb_each(field_metadata) WHERE key LIKE 'gemini\_%' ESCAPE '\' OR key LIKE 'openrouter\_%' ESCAPE '\'),'{}'),auth_methods FROM provider_templates;
INSERT INTO provider_model_template_research SELECT provider_template_id,id,
 coalesce((SELECT jsonb_object_agg(key,value) FROM jsonb_each(field_metadata) WHERE key LIKE 'gemini\_%' ESCAPE '\' OR key LIKE 'openrouter\_%' ESCAPE '\'),'{}'),protocols FROM provider_model_templates;
ALTER TABLE provider_templates RENAME COLUMN auth_config TO authentication;
ALTER TABLE provider_templates DROP COLUMN auth_methods;
ALTER TABLE provider_model_templates RENAME COLUMN paths TO operations;
ALTER TABLE provider_model_templates DROP COLUMN protocols;
UPDATE provider_templates p SET field_metadata=p.field_metadata-r.metadata_keys FROM
 (SELECT provider_template_id,ARRAY(SELECT jsonb_object_keys(metadata)) metadata_keys FROM provider_template_research) r WHERE p.id=r.provider_template_id;
UPDATE provider_model_templates m SET field_metadata=m.field_metadata-r.metadata_keys FROM
 (SELECT provider_template_id,model_template_id,ARRAY(SELECT jsonb_object_keys(metadata)) metadata_keys FROM provider_model_template_research) r WHERE m.provider_template_id=r.provider_template_id AND m.id=r.model_template_id;
UPDATE provider_templates SET source_refs=(SELECT jsonb_agg((item-'url') || jsonb_build_object('reference',item->'url') ORDER BY position) FROM jsonb_array_elements(source_refs) WITH ORDINALITY AS t(item,position));
UPDATE provider_model_templates SET source_refs=(SELECT jsonb_agg((item-'url') || jsonb_build_object('reference',item->'url') ORDER BY position) FROM jsonb_array_elements(source_refs) WITH ORDINALITY AS t(item,position));

UPDATE provider_templates SET model_info_sources=(SELECT jsonb_agg(CASE WHEN item->>'kind' IN ('repository','documentation') THEN (item-'url') || jsonb_build_object('reference',item->'url') ELSE item END ORDER BY position) FROM jsonb_array_elements(model_info_sources) WITH ORDINALITY AS t(item,position));

UPDATE provider_model_templates SET model_info_sources=(SELECT jsonb_agg(CASE WHEN item->>'kind' IN ('repository','documentation') THEN (item-'url') || jsonb_build_object('reference',item->'url') ELSE item END ORDER BY position) FROM jsonb_array_elements(model_info_sources) WITH ORDINALITY AS t(item,position));
