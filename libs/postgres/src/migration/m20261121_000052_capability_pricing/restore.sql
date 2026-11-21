DROP VIEW provider_catalog_auth_readiness;
DROP VIEW provider_catalog_readiness;
DO $$ DECLARE item record; BEGIN
 FOR item IN SELECT conrelid::regclass table_name,conname FROM pg_constraint WHERE contype='c' AND conrelid IN ('models'::regclass,'provider_models'::regclass,'provider_model_templates'::regclass) AND pg_get_constraintdef(oid) ~ '(capability_price|priced_capability|text_capability|capability_token|capability_amounts|model_capability_configuration|model_thinking|catalog_object_keys_valid|billing)' LOOP EXECUTE format('ALTER TABLE %s DROP CONSTRAINT %I',item.table_name,item.conname); END LOOP;
END $$;
ALTER TABLE models ADD COLUMN thinking jsonb;
ALTER TABLE models ADD COLUMN pricing jsonb;
ALTER TABLE provider_models ADD COLUMN thinking jsonb;
ALTER TABLE provider_models ADD COLUMN pricing jsonb;
ALTER TABLE provider_model_templates ADD COLUMN thinking jsonb;
ALTER TABLE provider_model_templates ADD COLUMN pricing jsonb;
