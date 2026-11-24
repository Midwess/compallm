CREATE FUNCTION catalog_price_intervals_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE ids uuid[]; cid uuid; component catalog_price_component%ROWTYPE;
BEGIN
 IF TG_TABLE_NAME='catalog_price_component' THEN
  ids:=ARRAY[CASE WHEN TG_OP<>'DELETE' THEN NEW.id END,CASE WHEN TG_OP<>'DELETE' THEN NEW.parent_id END,CASE WHEN TG_OP<>'INSERT' THEN OLD.id END,CASE WHEN TG_OP<>'INSERT' THEN OLD.parent_id END];
 ELSE
  ids:=ARRAY[CASE WHEN TG_OP<>'DELETE' THEN NEW.component_id END,CASE WHEN TG_OP<>'INSERT' THEN OLD.component_id END];
 END IF;
 FOR cid IN SELECT DISTINCT unnest(ids) LOOP
  SELECT * INTO component FROM catalog_price_component WHERE id=cid;
  IF NOT FOUND OR component.kind<>'usage' THEN CONTINUE; END IF;
  IF component.usage_kind='scheduled' THEN
   IF NOT EXISTS(SELECT 1 FROM catalog_price_component WHERE parent_id=cid)
   OR (SELECT min(from_hour)<>0 OR max(to_hour)<>24 OR sum(to_hour-from_hour)<>24 FROM catalog_price_component WHERE parent_id=cid)
   OR EXISTS(SELECT 1 FROM catalog_price_component a JOIN catalog_price_component b ON a.parent_id=b.parent_id AND a.id<b.id WHERE a.parent_id=cid AND a.from_hour<b.to_hour AND b.from_hour<a.to_hour)
   THEN RAISE EXCEPTION 'catalog schedule must cover every UTC hour exactly once'; END IF;
  ELSIF component.usage_kind='tiered_token' THEN
   IF (SELECT count(*) FROM catalog_price_rate WHERE component_id=cid AND kind='token')=0
   OR (SELECT count(*) FROM catalog_price_rate WHERE component_id=cid AND kind='token' AND to_tokens IS NULL)<>1
   OR EXISTS(SELECT 1 FROM(SELECT from_tokens,lag(to_tokens,1,0::numeric) OVER(ORDER BY position) prior FROM catalog_price_rate WHERE component_id=cid AND kind='token') r WHERE from_tokens IS DISTINCT FROM prior)
   THEN RAISE EXCEPTION 'catalog token tiers must be contiguous from zero to an unbounded final tier'; END IF;
  ELSIF NOT EXISTS(SELECT 1 FROM catalog_price_rate WHERE component_id=cid AND parent_rate_id IS NULL) THEN
   RAISE EXCEPTION 'catalog fixed pricing requires a rate';
  END IF;
  IF EXISTS(SELECT 1 FROM catalog_price_rate r WHERE r.component_id=cid AND r.cache_mode IN ('separate_rate','estimated_rate') AND NOT EXISTS(SELECT 1 FROM catalog_price_rate cache WHERE cache.parent_rate_id=r.id)) THEN
   RAISE EXCEPTION 'catalog cache pricing requires its separate rate';
  END IF;
 END LOOP;
 RETURN NULL;
END $$;
CREATE CONSTRAINT TRIGGER catalog_price_component_intervals AFTER INSERT OR UPDATE OR DELETE ON catalog_price_component DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION catalog_price_intervals_guard();
CREATE CONSTRAINT TRIGGER catalog_price_rate_intervals AFTER INSERT OR UPDATE OR DELETE ON catalog_price_rate DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION catalog_price_intervals_guard();
