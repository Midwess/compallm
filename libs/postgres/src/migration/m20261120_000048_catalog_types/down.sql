LOCK TABLE provider_templates, provider_model_templates IN ACCESS EXCLUSIVE MODE;

DO $$ BEGIN
 IF EXISTS(SELECT 1 FROM provider_templates p JOIN provider_template_research r ON r.provider_template_id=p.id WHERE EXISTS(SELECT 1 FROM jsonb_object_keys(r.metadata) key WHERE p.field_metadata ? key))
 OR EXISTS(SELECT 1 FROM provider_model_templates m JOIN provider_model_template_research r ON r.provider_template_id=m.provider_template_id AND r.model_template_id=m.id WHERE EXISTS(SELECT 1 FROM jsonb_object_keys(r.metadata) key WHERE m.field_metadata ? key))
 THEN RAISE EXCEPTION 'research metadata conflicts with catalog fields'; END IF;
END $$;

ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_operations_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_references_valid;
ALTER TABLE provider_templates DROP CONSTRAINT IF EXISTS catalog_provider_references_valid;
DROP FUNCTION IF EXISTS catalog_operations_valid(jsonb);
DROP FUNCTION IF EXISTS catalog_references_valid(jsonb);
DROP VIEW provider_catalog_auth_readiness;
DROP VIEW provider_catalog_readiness;
ALTER TABLE provider_templates DROP CONSTRAINT catalog_provider_valid;
ALTER TABLE provider_model_templates DROP CONSTRAINT catalog_model_valid;
ALTER TABLE provider_templates DROP COLUMN readiness;

ALTER TABLE provider_templates RENAME COLUMN authentication TO auth_config;
ALTER TABLE provider_templates ADD COLUMN auth_methods jsonb;
UPDATE provider_templates SET auth_methods=catalog_operation_names(auth_config);
UPDATE provider_templates p SET auth_methods=r.authentication_order,field_metadata=p.field_metadata || r.metadata FROM provider_template_research r WHERE p.id=r.provider_template_id;
ALTER TABLE provider_templates ALTER COLUMN auth_methods SET NOT NULL;
ALTER TABLE provider_model_templates RENAME COLUMN operations TO paths;
ALTER TABLE provider_model_templates ADD COLUMN protocols jsonb;
UPDATE provider_model_templates SET protocols=catalog_operation_names(paths);
UPDATE provider_model_templates m SET protocols=r.operation_order,field_metadata=m.field_metadata || r.metadata FROM provider_model_template_research r WHERE m.provider_template_id=r.provider_template_id AND m.id=r.model_template_id;
ALTER TABLE provider_model_templates ALTER COLUMN protocols SET NOT NULL;
UPDATE provider_templates SET source_refs=(SELECT jsonb_agg((item-'reference') || jsonb_build_object('url',item->'reference') ORDER BY position) FROM jsonb_array_elements(source_refs) WITH ORDINALITY AS t(item,position));
UPDATE provider_model_templates SET source_refs=(SELECT jsonb_agg((item-'reference') || jsonb_build_object('url',item->'reference') ORDER BY position) FROM jsonb_array_elements(source_refs) WITH ORDINALITY AS t(item,position));
DROP TABLE provider_model_template_research;
DROP TABLE provider_template_research;

UPDATE provider_templates SET model_info_sources=(SELECT jsonb_agg(CASE WHEN item ? 'reference' THEN (item-'reference') || jsonb_build_object('url',item->'reference') ELSE item END ORDER BY position) FROM jsonb_array_elements(model_info_sources) WITH ORDINALITY AS t(item,position));

UPDATE provider_model_templates SET model_info_sources=(SELECT jsonb_agg(CASE WHEN item ? 'reference' THEN (item-'reference') || jsonb_build_object('url',item->'reference') ELSE item END ORDER BY position) FROM jsonb_array_elements(model_info_sources) WITH ORDINALITY AS t(item,position));
