DO $$ DECLARE item record; BEGIN
 FOR item IN SELECT conname FROM pg_constraint WHERE conrelid='catalog_model_book'::regclass AND contype='c' AND pg_get_constraintdef(oid) ~ '(text|vision|pdf|audio|video|image_generation|tools|structured_output|search|limits|billing|route_selection)' LOOP
  IF item.conname NOT LIKE '%support%' AND item.conname NOT LIKE '%tokens%' AND item.conname NOT LIKE '%budget%' AND item.conname NOT LIKE '%max_%' AND item.conname NOT LIKE '%default_effort%' THEN EXECUTE format('ALTER TABLE catalog_model_book DROP CONSTRAINT %I',item.conname); END IF;
 END LOOP;
END $$;
ALTER TABLE catalog_model_book DROP COLUMN text CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN vision CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN pdf CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN audio CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN video CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN image_generation CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN tools CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN structured_output CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN search CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN limits CASCADE;
ALTER TABLE catalog_model_book DROP COLUMN billing CASCADE;
DROP TABLE provider_model_template_research;
DROP TABLE provider_template_research;
CREATE TABLE catalog_relational_applied(table_name text NOT NULL,row_key text NOT NULL,applied jsonb NOT NULL,PRIMARY KEY(table_name,row_key));
CREATE INDEX catalog_model_provider_name ON catalog_model_book(provider_catalog_id,name,id);
CREATE INDEX catalog_model_private_base ON catalog_model_book(base_row_id) WHERE user_id IS NOT NULL;
CREATE INDEX catalog_price_component_book ON catalog_price_component(price_book_id);
CREATE INDEX catalog_price_rate_component ON catalog_price_rate(component_id);
CREATE INDEX catalog_model_price_lookup ON catalog_model_price_book(model_id);
ALTER TABLE catalog_model_book ADD CHECK(coalesce(max_input_tokens<=context_tokens,true) AND coalesce(max_output_tokens<=context_tokens,true) OR user_id IS NULL);
CREATE FUNCTION catalog_model_identity_guard() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN
 IF TG_OP='UPDATE' AND (NEW.row_id,NEW.organization_id,NEW.user_id,NEW.base_row_id,NEW.provider_catalog_id,NEW.id,NEW.source_id) IS DISTINCT FROM (OLD.row_id,OLD.organization_id,OLD.user_id,OLD.base_row_id,OLD.provider_catalog_id,OLD.id,OLD.source_id) THEN RAISE EXCEPTION 'catalog model identity is immutable'; END IF;
 IF NEW.base_row_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM catalog_model_book b WHERE b.row_id=NEW.base_row_id AND b.user_id IS NULL AND b.provider_catalog_id=NEW.provider_catalog_id AND b.id=NEW.id) THEN RAISE EXCEPTION 'invalid catalog model base'; END IF;
 IF NEW.user_id IS NOT NULL AND NEW.base_row_id IS NULL AND NEW.id NOT LIKE 'custom:%' THEN RAISE EXCEPTION 'custom model requires private namespace'; END IF;
 RETURN NEW; END $$;
CREATE TRIGGER catalog_model_identity_guard BEFORE INSERT OR UPDATE ON catalog_model_book FOR EACH ROW EXECUTE FUNCTION catalog_model_identity_guard();
CREATE FUNCTION catalog_price_link_guard() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN
 IF NOT EXISTS(SELECT 1 FROM catalog_model_book m JOIN catalog_price_book p ON p.id=NEW.price_book_id WHERE m.row_id=NEW.model_id AND m.provider_catalog_id=p.provider_catalog_id AND (p.user_id IS NULL OR(m.user_id=p.user_id AND m.organization_id=p.organization_id))) THEN RAISE EXCEPTION 'invalid catalog price scope'; END IF;
 RETURN NEW; END $$;
CREATE TRIGGER catalog_price_link_guard BEFORE INSERT OR UPDATE ON catalog_model_price_book FOR EACH ROW EXECUTE FUNCTION catalog_price_link_guard();

ALTER TABLE catalog_price_rate ADD CHECK(amount IS NULL OR(scale(amount)<=18 AND length(replace(trim(leading '-' from amount::text),'.',''))<=28));
CREATE FUNCTION catalog_price_header_guard() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN
 IF TG_OP='UPDATE' AND (NEW.id,NEW.provider_catalog_id,NEW.organization_id,NEW.user_id) IS DISTINCT FROM(OLD.id,OLD.provider_catalog_id,OLD.organization_id,OLD.user_id) THEN RAISE EXCEPTION 'catalog tariff ownership is immutable'; END IF;
 RETURN NEW; END $$;
CREATE TRIGGER catalog_price_header_guard BEFORE UPDATE ON catalog_price_book FOR EACH ROW EXECUTE FUNCTION catalog_price_header_guard();
CREATE FUNCTION catalog_price_parent_guard() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN
 IF NEW.parent_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM catalog_price_component p WHERE p.id=NEW.parent_id AND p.price_book_id=NEW.price_book_id AND p.usage_kind='scheduled' AND p.parent_id IS NULL) THEN RAISE EXCEPTION 'invalid pricing component parent'; END IF;
 RETURN NEW; END $$;
CREATE TRIGGER catalog_price_parent_guard BEFORE INSERT OR UPDATE ON catalog_price_component FOR EACH ROW EXECUTE FUNCTION catalog_price_parent_guard();

ALTER TABLE catalog_model_format ADD CHECK(capability IN ('vision','audio_input','audio_output','video','image_generation'));
ALTER TABLE catalog_price_component ADD CHECK(feature IN ('text_input','text_output','thinking','vision','pdf','audio_input','audio_output','video','image_generation','tool_calling','forced_tool_choice','parallel_tool_calls','structured_output','json_mode','json_schema','search','billing_request'));
ALTER TABLE catalog_price_component ADD CHECK((kind='usage')=(usage_kind IS NOT NULL));
ALTER TABLE catalog_price_component ADD CHECK(coalesce(usage_kind='fixed',false)=(rate_kind IS NOT NULL));
ALTER TABLE catalog_price_rate ADD CHECK(kind IN ('token','cache','request','audio','image','video','credit'));
ALTER TABLE catalog_price_rate ADD CHECK((kind='cache')=(parent_rate_id IS NOT NULL));
ALTER TABLE catalog_price_rate ADD CHECK((kind='credit') OR(currency IS NOT NULL AND quantity IS NOT NULL));
ALTER TABLE catalog_price_rate ADD CHECK(credits IS NULL OR(scale(credits)<=18 AND length(replace(trim(leading '-' from credits::text),'.',''))<=28));
CREATE FUNCTION catalog_price_cache_parent_guard() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN
 IF NEW.parent_rate_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM catalog_price_rate p WHERE p.id=NEW.parent_rate_id AND p.component_id=NEW.component_id AND p.kind='token' AND p.parent_rate_id IS NULL) THEN RAISE EXCEPTION 'invalid cache rate parent'; END IF;
 RETURN NEW; END $$;
CREATE TRIGGER catalog_price_cache_parent_guard BEFORE INSERT OR UPDATE ON catalog_price_rate FOR EACH ROW EXECUTE FUNCTION catalog_price_cache_parent_guard();
