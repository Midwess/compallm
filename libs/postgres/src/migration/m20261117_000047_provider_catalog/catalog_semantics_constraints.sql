ALTER TABLE provider_model_templates VALIDATE CONSTRAINT catalog_capabilities_object;
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_valid
CHECK (catalog_provider_valid(to_jsonb(provider_templates)) IS TRUE);
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_valid
CHECK (catalog_model_valid(to_jsonb(provider_model_templates)) IS TRUE);

ALTER TABLE provider_templates ADD COLUMN readiness catalog_readiness
GENERATED ALWAYS AS (catalog_provider_state(endpoint_mode,auth_methods,auth_config)) STORED;
ALTER TABLE provider_model_templates ADD COLUMN readiness catalog_readiness
GENERATED ALWAYS AS (catalog_model_state(specification_mode,pricing_mode,pricing_config,unresolved_fields,resolution_plan)) STORED;

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
    catalog_provider_model_state(to_jsonb(p),m.protocols) AS provider_readiness
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
 CASE WHEN p.field_metadata ? 'request_headers' AND catalog_provider_auth_state(p.endpoint_mode,method,p.auth_config->method,m.protocols) <> 'incomplete'
 THEN 'needs_configuration'::catalog_readiness
 ELSE catalog_provider_auth_state(p.endpoint_mode,method,p.auth_config->method,m.protocols) END AS provider_readiness
 FROM provider_model_templates m JOIN provider_templates p ON p.id = m.provider_template_id
 CROSS JOIN LATERAL jsonb_array_elements_text(p.auth_methods) method
) requirements;
GRANT SELECT ON provider_catalog_auth_readiness TO compallm_restricted;
