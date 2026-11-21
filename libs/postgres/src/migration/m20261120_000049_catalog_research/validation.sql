CREATE FUNCTION catalog_template_limits_valid(value jsonb) RETURNS boolean LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE item record;
BEGIN
 IF jsonb_typeof(value) IS DISTINCT FROM 'object' OR NOT catalog_object_keys_valid(value,ARRAY['max_context_tokens','max_input_tokens','max_output_tokens']) THEN RETURN false; END IF;
 FOR item IN SELECT * FROM jsonb_each(value) LOOP
  IF item.value <> 'null'::jsonb AND (jsonb_typeof(item.value) <> 'number' OR (item.value #>> '{}') !~ '^[0-9]+$' OR (item.value #>> '{}')::numeric NOT BETWEEN 1 AND 4294967295) THEN RETURN false; END IF;
 END LOOP;
 RETURN true;
EXCEPTION WHEN OTHERS THEN RETURN false;
END $$;
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_connection_shape CHECK(jsonb_typeof(connection)='object' AND catalog_object_keys_valid(connection,ARRAY['protocol','base_url','authentication','endpoint_mode','endpoint']) AND connection ?& ARRAY['protocol','base_url','authentication','endpoint_mode','endpoint']);
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_connection_shape CHECK(jsonb_typeof(connection)='object' AND catalog_object_keys_valid(connection,ARRAY['operations']) AND catalog_operations_valid(connection->'operations'));
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_limits_shape CHECK(catalog_template_limits_valid(limits));
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_thinking_shape CHECK(jsonb_typeof(thinking)='object' AND catalog_object_keys_valid(thinking,ARRAY['effort_mapping']) AND jsonb_typeof(thinking->'effort_mapping')='object');
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_pricing_shape CHECK(jsonb_typeof(pricing)='object' AND catalog_object_keys_valid(pricing,ARRAY['billing_model','billing_unit','mode','cache_billing','input_price','output_price','cached_price','configuration']) AND pricing ?& ARRAY['billing_model','billing_unit','mode','cache_billing','input_price','output_price','cached_price','configuration']);
ALTER TABLE provider_templates ADD CONSTRAINT catalog_provider_connection_values CHECK(connection->>'protocol' IN ('openai_compatible','anthropic','provider_specific') AND catalog_configuration_json(connection)=connection);
ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_model_pricing_values CHECK(
 pricing->>'billing_model' IN ('metered','subscription','subscription_with_overage','free','unknown')
 AND pricing->>'mode' IN ('fixed','tiered','api','account_specific','not_applicable','unpublished','scheduled','unknown')
 AND pricing->>'cache_billing' IN ('separate_rate','standard_input_rate','unsupported','included','not_applicable','per_route','unknown')
 AND (pricing->'billing_unit'='null'::jsonb OR pricing->>'billing_unit' IN ('tokens','requests','images','seconds','characters','credits','videos','utf8_bytes'))
 AND (pricing->'input_price'='null'::jsonb OR catalog_json_price_valid(pricing->'input_price'))
 AND (pricing->'output_price'='null'::jsonb OR catalog_json_price_valid(pricing->'output_price'))
 AND (pricing->'cached_price'='null'::jsonb OR catalog_json_price_valid(pricing->'cached_price'))
 AND jsonb_typeof(pricing->'configuration')='object' AND catalog_configuration_json(pricing)=pricing
);
CREATE FUNCTION catalog_linked_template_valid() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE provider_id text; model_id text; document jsonb;
BEGIN
 IF TG_TABLE_NAME IN ('provider_templates','provider_template_research') THEN
  provider_id:=CASE WHEN TG_TABLE_NAME='provider_templates' THEN to_jsonb(NEW)->>'id' ELSE to_jsonb(NEW)->>'provider_template_id' END;
  PERFORM 1 FROM provider_templates WHERE id=provider_id FOR UPDATE;
  SELECT d.document INTO document FROM provider_catalog_documents d WHERE id=provider_id;
  IF document IS NOT NULL AND (catalog_provider_valid(document) IS NOT TRUE OR catalog_references_valid(document->'source_refs') IS NOT TRUE OR provider_catalog_info_sources_valid(document->'model_info_sources') IS NOT TRUE) THEN RAISE EXCEPTION 'invalid linked provider template'; END IF;
 ELSE
  provider_id:=to_jsonb(NEW)->>'provider_template_id';
  model_id:=CASE WHEN TG_TABLE_NAME='provider_model_templates' THEN to_jsonb(NEW)->>'id' ELSE to_jsonb(NEW)->>'model_template_id' END;
  PERFORM 1 FROM provider_model_templates WHERE provider_template_id=provider_id AND id=model_id FOR UPDATE;
  SELECT d.document INTO document FROM model_catalog_documents d WHERE provider_template_id=provider_id AND id=model_id;
  IF document IS NOT NULL AND (catalog_model_valid(document) IS NOT TRUE OR catalog_references_valid(document->'source_refs') IS NOT TRUE OR provider_catalog_info_sources_valid(document->'model_info_sources') IS NOT TRUE OR catalog_operations_valid(document->'operations') IS NOT TRUE) THEN RAISE EXCEPTION 'invalid linked model template'; END IF;
 END IF;
 RETURN NULL;
END $$;
CREATE CONSTRAINT TRIGGER provider_templates_linked_valid AFTER INSERT OR UPDATE ON provider_templates DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION catalog_linked_template_valid();
CREATE CONSTRAINT TRIGGER provider_template_research_linked_valid AFTER INSERT OR UPDATE ON provider_template_research DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION catalog_linked_template_valid();
CREATE CONSTRAINT TRIGGER provider_model_templates_linked_valid AFTER INSERT OR UPDATE ON provider_model_templates DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION catalog_linked_template_valid();
CREATE CONSTRAINT TRIGGER provider_model_template_research_linked_valid AFTER INSERT OR UPDATE ON provider_model_template_research DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION catalog_linked_template_valid();
DO $$ BEGIN
 IF EXISTS(SELECT 1 FROM provider_catalog_documents d WHERE catalog_provider_valid(document) IS NOT TRUE OR catalog_references_valid(document->'source_refs') IS NOT TRUE OR provider_catalog_info_sources_valid(document->'model_info_sources') IS NOT TRUE) OR EXISTS(SELECT 1 FROM model_catalog_documents d WHERE NOT catalog_model_valid(document) OR NOT catalog_operations_valid(document->'operations') OR NOT catalog_references_valid(document->'source_refs') OR NOT provider_catalog_info_sources_valid(document->'model_info_sources')) THEN RAISE EXCEPTION 'invalid catalog research separation'; END IF;
END $$;
