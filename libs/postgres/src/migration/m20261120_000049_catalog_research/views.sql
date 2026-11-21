CREATE VIEW provider_catalog_documents AS SELECT p.id,catalog_provider_document(to_jsonb(p),to_jsonb(r)) AS document FROM provider_templates p JOIN provider_template_research r ON r.provider_template_id=p.id;
CREATE VIEW model_catalog_documents AS SELECT m.provider_template_id,m.id,catalog_model_document(to_jsonb(m),to_jsonb(r)) AS document FROM provider_model_templates m JOIN provider_model_template_research r ON r.provider_template_id=m.provider_template_id AND r.model_template_id=m.id;
CREATE VIEW provider_catalog_readiness AS
 SELECT m.provider_template_id,m.id,
 CASE WHEN provider_readiness='incomplete' OR model_readiness='incomplete' THEN 'incomplete'::catalog_readiness
 WHEN provider_readiness='needs_configuration' OR model_readiness='needs_configuration' THEN 'needs_configuration'::catalog_readiness
 WHEN provider_readiness='needs_fetch' OR model_readiness='needs_fetch' THEN 'needs_fetch'::catalog_readiness ELSE 'ready'::catalog_readiness END readiness,provider_readiness,model_readiness
 FROM provider_model_templates m JOIN provider_templates p ON p.id=m.provider_template_id JOIN provider_model_template_research r ON r.provider_template_id=m.provider_template_id AND r.model_template_id=m.id JOIN provider_template_research pr ON pr.provider_template_id=p.id,
 LATERAL (SELECT catalog_provider_model_state(jsonb_build_object('endpoint_mode',p.connection->'endpoint_mode','authentication',p.connection->'authentication','field_metadata',pr.field_metadata),catalog_operation_names(m.connection->'operations')) provider_readiness,
 catalog_model_state(r.specification_mode,(m.pricing->>'mode')::catalog_pricing_mode,m.pricing->'configuration',r.unresolved_fields,r.resolution_plan) model_readiness) state;
CREATE VIEW provider_catalog_auth_readiness AS
 SELECT m.provider_template_id,m.id,method AS auth_method,provider_readiness,model_readiness,
 CASE WHEN provider_readiness='incomplete' OR model_readiness='incomplete' THEN 'incomplete'::catalog_readiness
 WHEN provider_readiness='needs_configuration' OR model_readiness='needs_configuration' THEN 'needs_configuration'::catalog_readiness
 WHEN provider_readiness='needs_fetch' OR model_readiness='needs_fetch' THEN 'needs_fetch'::catalog_readiness ELSE 'ready'::catalog_readiness END readiness
 FROM provider_model_templates m JOIN provider_templates p ON p.id=m.provider_template_id JOIN provider_model_template_research r ON r.provider_template_id=m.provider_template_id AND r.model_template_id=m.id JOIN provider_template_research pr ON pr.provider_template_id=p.id CROSS JOIN LATERAL jsonb_object_keys(p.connection->'authentication') method,
 LATERAL (SELECT CASE WHEN pr.field_metadata ? 'request_headers' AND catalog_provider_auth_state((p.connection->>'endpoint_mode')::catalog_endpoint_mode,method,p.connection->'authentication'->method,catalog_operation_names(m.connection->'operations')) <> 'incomplete' THEN 'needs_configuration'::catalog_readiness ELSE catalog_provider_auth_state((p.connection->>'endpoint_mode')::catalog_endpoint_mode,method,p.connection->'authentication'->method,catalog_operation_names(m.connection->'operations')) END provider_readiness,
 catalog_model_state(r.specification_mode,(m.pricing->>'mode')::catalog_pricing_mode,m.pricing->'configuration',r.unresolved_fields,r.resolution_plan) model_readiness) state;
GRANT SELECT ON provider_catalog_readiness,provider_catalog_auth_readiness TO compallm_restricted;
