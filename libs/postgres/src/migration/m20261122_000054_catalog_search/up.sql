CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX provider_template_search_trgm ON provider_templates USING gin(lower(id) gin_trgm_ops,lower(name) gin_trgm_ops);
CREATE INDEX model_template_search_trgm ON provider_model_templates USING gin(lower(id) gin_trgm_ops,lower(name) gin_trgm_ops,lower(display_name) gin_trgm_ops);
CREATE INDEX provider_template_catalog_order ON provider_templates(name,id);
ANALYZE provider_templates;
ANALYZE provider_model_templates;
