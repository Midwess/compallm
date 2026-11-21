DROP VIEW provider_catalog_auth_readiness;
DROP VIEW provider_catalog_readiness;
CREATE TABLE capability_pricing_rollback (table_name text NOT NULL,row_key jsonb NOT NULL,original jsonb NOT NULL,migrated jsonb NOT NULL,PRIMARY KEY(table_name,row_key));
DO $$ DECLARE item record; BEGIN
 FOR item IN SELECT conrelid::regclass table_name,conname FROM pg_constraint WHERE contype='c' AND conrelid IN ('models'::regclass,'provider_models'::regclass,'provider_model_templates'::regclass) AND pg_get_constraintdef(oid) ~ '(model_capability|model_thinking|catalog_object_keys_valid\((audio|tools)|thinking.*configuration|structured_output.*configuration|audio.*configuration|tools.*configuration)' LOOP
  EXECUTE format('ALTER TABLE %s DROP CONSTRAINT %I',item.table_name,item.conname);
 END LOOP;
END $$;
ALTER TABLE models DROP COLUMN thinking;
ALTER TABLE models DROP COLUMN pricing;
ALTER TABLE models ADD COLUMN billing jsonb;
ALTER TABLE models ALTER COLUMN text DROP DEFAULT;
ALTER TABLE provider_models DROP COLUMN thinking;
ALTER TABLE provider_models DROP COLUMN pricing;
ALTER TABLE provider_models ADD COLUMN billing jsonb;
ALTER TABLE provider_models ALTER COLUMN text DROP DEFAULT;
ALTER TABLE provider_model_templates DROP COLUMN thinking;
ALTER TABLE provider_model_templates DROP COLUMN pricing;
ALTER TABLE provider_model_templates ADD COLUMN billing jsonb;
ALTER TABLE provider_model_templates ALTER COLUMN text DROP DEFAULT;
