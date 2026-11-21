ALTER TABLE models ADD COLUMN capabilities jsonb;
ALTER TABLE models DROP COLUMN thinking;
ALTER TABLE models ADD COLUMN thinking jsonb;
ALTER TABLE provider_models ADD COLUMN capabilities jsonb;
ALTER TABLE provider_models DROP COLUMN thinking;
ALTER TABLE provider_models ADD COLUMN thinking jsonb;
ALTER TABLE provider_model_templates ADD COLUMN capabilities jsonb;
ALTER TABLE provider_model_templates DROP COLUMN thinking;
ALTER TABLE provider_model_templates ADD COLUMN thinking jsonb;
