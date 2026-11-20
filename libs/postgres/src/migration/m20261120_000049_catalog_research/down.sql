LOCK TABLE provider_templates,provider_model_templates,provider_template_research,provider_model_template_research IN ACCESS EXCLUSIVE MODE;
DROP TRIGGER provider_templates_linked_valid ON provider_templates;
DROP TRIGGER provider_template_research_linked_valid ON provider_template_research;
DROP TRIGGER provider_model_templates_linked_valid ON provider_model_templates;
DROP TRIGGER provider_model_template_research_linked_valid ON provider_model_template_research;
DROP FUNCTION catalog_linked_template_valid();
ALTER TABLE provider_templates DROP CONSTRAINT provider_template_research_link;
ALTER TABLE provider_model_templates DROP CONSTRAINT model_template_research_link;
DO $$ BEGIN
 IF EXISTS(SELECT 1 FROM provider_model_templates WHERE headers <> '{}'::jsonb) THEN RAISE EXCEPTION 'model headers cannot be represented by the previous catalog schema'; END IF;
END $$;
ALTER TABLE provider_templates DROP CONSTRAINT IF EXISTS catalog_provider_connection_values;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_pricing_values;
ALTER TABLE provider_templates DROP CONSTRAINT IF EXISTS catalog_provider_connection_shape;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_connection_shape;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_limits_shape;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_thinking_shape;
ALTER TABLE provider_model_templates DROP CONSTRAINT IF EXISTS catalog_model_pricing_shape;
DROP FUNCTION IF EXISTS catalog_template_limits_valid(jsonb);
CREATE TEMP TABLE catalog_provider_restore ON COMMIT DROP AS SELECT * FROM provider_catalog_documents;
CREATE TEMP TABLE catalog_model_restore ON COMMIT DROP AS SELECT * FROM model_catalog_documents;
DROP VIEW provider_catalog_auth_readiness;
DROP VIEW provider_catalog_readiness;
DROP VIEW model_catalog_documents;
DROP VIEW provider_catalog_documents;
ALTER TABLE provider_templates DROP COLUMN readiness;
ALTER TABLE provider_model_templates DROP COLUMN readiness;
ALTER TABLE provider_templates ADD COLUMN protocol text;
ALTER TABLE provider_templates ADD COLUMN base_url varchar(512);
ALTER TABLE provider_templates ADD COLUMN source_refs jsonb;
ALTER TABLE provider_templates ADD COLUMN authentication jsonb;
ALTER TABLE provider_templates ADD COLUMN endpoint_config jsonb;
ALTER TABLE provider_templates ADD COLUMN endpoint_mode catalog_endpoint_mode;
ALTER TABLE provider_templates ADD COLUMN field_metadata jsonb;
ALTER TABLE provider_templates ADD COLUMN model_info_sources jsonb;
UPDATE provider_templates p SET source_refs=r.document->'source_refs',field_metadata=r.document->'field_metadata',model_info_sources=r.document->'model_info_sources',protocol=(r.document->>'protocol')::text,base_url=(r.document->>'base_url')::varchar(512),authentication=r.document->'authentication',endpoint_mode=(r.document->>'endpoint_mode')::catalog_endpoint_mode,endpoint_config=r.document->'endpoint_config' FROM catalog_provider_restore r WHERE p.id=r.id;
ALTER TABLE provider_templates ALTER COLUMN protocol SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN source_refs SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN authentication SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN endpoint_config SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN endpoint_mode SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN field_metadata SET NOT NULL;
ALTER TABLE provider_templates ALTER COLUMN model_info_sources SET NOT NULL;
ALTER TABLE provider_model_templates ADD COLUMN operations jsonb;
ALTER TABLE provider_model_templates ADD COLUMN max_context_tokens bigint;
ALTER TABLE provider_model_templates ADD COLUMN max_input_tokens bigint;
ALTER TABLE provider_model_templates ADD COLUMN max_output_tokens bigint;
ALTER TABLE provider_model_templates ADD COLUMN effort_mapping jsonb;
ALTER TABLE provider_model_templates ADD COLUMN input_price numeric;
ALTER TABLE provider_model_templates ADD COLUMN output_price numeric;
ALTER TABLE provider_model_templates ADD COLUMN cached_price numeric;
ALTER TABLE provider_model_templates ADD COLUMN source_refs jsonb;
ALTER TABLE provider_model_templates ADD COLUMN field_metadata jsonb;
ALTER TABLE provider_model_templates ADD COLUMN model_info_sources jsonb;
ALTER TABLE provider_model_templates ADD COLUMN billing_model catalog_billing_model;
ALTER TABLE provider_model_templates ADD COLUMN billing_unit catalog_billing_unit;
ALTER TABLE provider_model_templates ADD COLUMN pricing_mode catalog_pricing_mode;
ALTER TABLE provider_model_templates ADD COLUMN cache_billing catalog_cache_billing;
ALTER TABLE provider_model_templates ADD COLUMN specification_mode catalog_specification_mode;
ALTER TABLE provider_model_templates ADD COLUMN pricing_config jsonb;
ALTER TABLE provider_model_templates ADD COLUMN resolution_plan jsonb;
ALTER TABLE provider_model_templates ADD COLUMN unresolved_fields jsonb;
UPDATE provider_model_templates p SET source_refs=r.document->'source_refs',field_metadata=r.document->'field_metadata',model_info_sources=r.document->'model_info_sources',operations=r.document->'operations',max_context_tokens=(r.document->>'max_context_tokens')::bigint,max_input_tokens=(r.document->>'max_input_tokens')::bigint,max_output_tokens=(r.document->>'max_output_tokens')::bigint,effort_mapping=r.document->'effort_mapping',input_price=(r.document->>'input_price')::numeric,output_price=(r.document->>'output_price')::numeric,cached_price=(r.document->>'cached_price')::numeric,billing_model=(r.document->>'billing_model')::catalog_billing_model,billing_unit=(r.document->>'billing_unit')::catalog_billing_unit,pricing_mode=(r.document->>'pricing_mode')::catalog_pricing_mode,cache_billing=(r.document->>'cache_billing')::catalog_cache_billing,pricing_config=r.document->'pricing_config',specification_mode=(r.document->>'specification_mode')::catalog_specification_mode,resolution_plan=r.document->'resolution_plan',unresolved_fields=r.document->'unresolved_fields' FROM catalog_model_restore r WHERE p.id=r.id AND p.provider_template_id=r.provider_template_id;
ALTER TABLE provider_model_templates ALTER COLUMN operations SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN effort_mapping SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN source_refs SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN field_metadata SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN model_info_sources SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN billing_model SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN pricing_mode SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN cache_billing SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN specification_mode SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN pricing_config SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN resolution_plan SET NOT NULL;
ALTER TABLE provider_model_templates ALTER COLUMN unresolved_fields SET NOT NULL;
ALTER TABLE provider_templates DROP COLUMN connection;
ALTER TABLE provider_model_templates DROP COLUMN headers;
ALTER TABLE provider_model_templates DROP COLUMN connection;
ALTER TABLE provider_model_templates DROP COLUMN limits;
ALTER TABLE provider_model_templates DROP COLUMN thinking;
ALTER TABLE provider_model_templates DROP COLUMN pricing;
ALTER TABLE provider_template_research DROP COLUMN source_refs;
ALTER TABLE provider_template_research DROP COLUMN field_metadata;
ALTER TABLE provider_template_research DROP COLUMN model_info_sources;
ALTER TABLE provider_template_research DROP COLUMN endpoint_config;
ALTER TABLE provider_template_research DROP COLUMN authentication;
ALTER TABLE provider_template_research DROP COLUMN state;
ALTER TABLE provider_model_template_research DROP COLUMN source_refs;
ALTER TABLE provider_model_template_research DROP COLUMN field_metadata;
ALTER TABLE provider_model_template_research DROP COLUMN model_info_sources;
ALTER TABLE provider_model_template_research DROP COLUMN pricing_config;
ALTER TABLE provider_model_template_research DROP COLUMN specification_mode;
ALTER TABLE provider_model_template_research DROP COLUMN resolution_plan;
ALTER TABLE provider_model_template_research DROP COLUMN unresolved_fields;
ALTER TABLE provider_model_template_research DROP COLUMN state;
ALTER TABLE provider_templates ADD COLUMN readiness catalog_readiness GENERATED ALWAYS AS(catalog_provider_state(endpoint_mode,catalog_operation_names(authentication),authentication)) STORED;
ALTER TABLE provider_model_templates ADD COLUMN readiness catalog_readiness GENERATED ALWAYS AS(catalog_model_state(specification_mode,pricing_mode,pricing_config,unresolved_fields,resolution_plan)) STORED;
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_valid CHECK(catalog_provider_valid(to_jsonb(provider_templates)) IS TRUE);
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_valid CHECK(catalog_model_valid(to_jsonb(provider_model_templates)) IS TRUE);
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_references_valid CHECK(catalog_references_valid(source_refs));
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_references_valid CHECK(catalog_references_valid(source_refs));
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_operations_valid CHECK(catalog_operations_valid(operations));
ALTER TABLE provider_templates ADD CONSTRAINT provider_templates_model_info_sources_valid CHECK(provider_catalog_info_sources_valid(model_info_sources));
ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_model_info_sources_valid CHECK(provider_catalog_info_sources_valid(model_info_sources));
DROP FUNCTION catalog_model_document(jsonb,jsonb);
DROP FUNCTION catalog_provider_document(jsonb,jsonb);
DROP FUNCTION catalog_provider_configuration_state(jsonb);
DROP FUNCTION catalog_model_configuration_state(jsonb,jsonb,jsonb);
DROP FUNCTION catalog_restore_review(jsonb,jsonb);
DROP FUNCTION catalog_configuration_json(jsonb);
DROP TYPE catalog_research_progress;
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

DROP TABLE catalog_provider_restore;
DROP TABLE catalog_model_restore;
