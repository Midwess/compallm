use super::records;
use sea_orm::{
    ConnectionTrait, DbErr, ExprTrait,
    sea_query::{Alias, Expr},
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub(crate) const FEATURES: [(&str, &str); 16] = [
    ("text_input", "/text/input"),
    ("text_output", "/text/output"),
    ("thinking", "/text/output/configuration/thinking"),
    ("vision", "/vision"),
    ("pdf", "/pdf"),
    ("audio_input", "/audio/input"),
    ("audio_output", "/audio/output"),
    ("video", "/video"),
    ("image_generation", "/image_generation"),
    ("tool_calling", "/tools/calling"),
    ("forced_tool_choice", "/tools/forcedChoice"),
    ("parallel_tool_calls", "/tools/parallelCalls"),
    ("structured_output", "/structured_output"),
    ("json_mode", "/structured_output/configuration/jsonMode"),
    ("json_schema", "/structured_output/configuration/jsonSchema"),
    ("search", "/search"),
];
pub(crate) fn clear_route_prices(route: &mut Value) {
    if let Some(options) = route.get_mut("options").and_then(Value::as_array_mut) {
        for option in options {
            for pointer in [
                "/text/input/pricing",
                "/text/output/pricing",
                "/text/output/configuration/thinking/pricing",
            ] {
                if let Some(price) = option.pointer_mut(pointer) {
                    *price = Value::Null;
                }
            }
            option["billing"] = Value::Null;
        }
    }
}
fn id() -> Value {
    json!(uuid::Uuid::new_v4().to_string())
}
fn rate_row(component: &Value, position: usize, kind: &str, rate: &Value) -> Value {
    json!({"id":id(),"component_id":component,"position":position,"kind":kind,"amount":rate["amount"],"currency":rate["currency"],"quantity":rate["quantity"]})
}
async fn token<C: ConnectionTrait>(
    db: &C,
    component: &Value,
    position: usize,
    value: &Value,
    tier: Option<&Value>,
) -> Result<(), DbErr> {
    let mut row = rate_row(component, position, "token", &value["rate"]);
    row["cache_mode"] = value["cache"]["type"].clone();
    if let Some(tier) = tier {
        row["from_tokens"] = tier["fromTokens"].clone();
        row["to_tokens"] = tier["toTokens"].clone();
    }
    let parent = row["id"].clone();
    records::insert(db, "catalog_price_rate", &row).await?;
    if value["cache"]["configuration"]["rate"].is_object() {
        let mut cache = rate_row(
            component,
            position,
            "cache",
            &value["cache"]["configuration"]["rate"],
        );
        cache["parent_rate_id"] = parent;
        records::insert(db, "catalog_price_rate", &cache).await?;
    }
    Ok(())
}
fn conditions(row: &mut Value, value: &Value) {
    for (a, b) in [
        ("resolution", "resolution"),
        ("quality", "quality"),
        ("audio", "audio"),
        ("service_tier", "serviceTier"),
    ] {
        row[a] = value[b].clone();
    }
}
async fn parameter<C: ConnectionTrait>(db: &C, id: &Value, value: &Value) -> Result<(), DbErr> {
    if let Some(params) = value.as_object() {
        for (key, value) in params {
            records::insert(
                db,
                "catalog_price_parameter",
                &json!({"rate_id":id,"name":key,"value":value}),
            )
            .await?;
        }
    }
    Ok(())
}
pub(crate) async fn write_component<C: ConnectionTrait>(
    db: &C,
    book: &Value,
    feature: &str,
    pricing: &Value,
    parent: Option<&Value>,
    slot: Option<&Value>,
) -> Result<(), DbErr> {
    if pricing.is_null() {
        return Ok(());
    }
    let cid = id();
    let mut component =
        json!({"id":cid,"price_book_id":book,"feature":feature,"kind":pricing["type"]});
    if let Some(parent) = parent {
        component["parent_id"] = parent.clone();
    }
    if let Some(slot) = slot {
        component["from_hour"] = slot["fromHour"].clone();
        component["to_hour"] = slot["toHour"].clone();
    }
    let usage = &pricing["configuration"];
    let (usage_kind, config) = if let Some(v) = usage.get("fixed") {
        ("fixed", v)
    } else if let Some(v) = usage.get("tiered_token") {
        ("tiered_token", v)
    } else if let Some(v) = usage.get("scheduled") {
        ("scheduled", v)
    } else {
        ("", &Value::Null)
    };
    if !usage_kind.is_empty() {
        component["usage_kind"] = json!(usage_kind);
    }
    if usage_kind == "scheduled" {
        component["timezone"] = config["timezone"].clone();
    }
    if usage_kind == "tiered_token" {
        component["measured_by"] = config["measuredBy"].clone();
    }
    let rates = config
        .as_object()
        .filter(|_| usage_kind == "fixed")
        .and_then(|v| v.iter().next());
    if let Some((kind, value)) = rates {
        component["rate_kind"] = json!(kind);
        if kind == "audio" {
            component["audio_kind"] = value["type"].clone();
        }
    }
    records::insert(db, "catalog_price_component", &component).await?;
    if usage_kind == "scheduled" {
        for slot in config["slots"].as_array().into_iter().flatten() {
            Box::pin(write_component(
                db,
                book,
                feature,
                &json!({"type":"usage","configuration":slot["pricing"]}),
                Some(&cid),
                Some(slot),
            ))
            .await?;
        }
    } else if usage_kind == "tiered_token" {
        for (position, tier) in config["tiers"].as_array().into_iter().flatten().enumerate() {
            token(db, &cid, position, &tier["rate"], Some(tier)).await?;
        }
    } else if let Some((kind, value)) = rates {
        match kind.as_str() {
            "token" => token(db, &cid, 0, value, None).await?,
            "request" | "audio" => {
                let r = if kind == "audio" {
                    &value["configuration"]["rate"]
                } else {
                    &value["rate"]
                };
                records::insert(db, "catalog_price_rate", &rate_row(&cid, 0, kind, r)).await?;
            }
            "image" | "video" | "credit" => {
                for (position, value) in value["rates"].as_array().into_iter().flatten().enumerate()
                {
                    let mut row = rate_row(&cid, position, kind, &value["rate"]);
                    conditions(&mut row, &value["conditions"]);
                    for field in ["unit", "direction", "credits", "quantity"] {
                        if let Some(v) = value.get(field) {
                            row[field] = v.clone();
                        }
                    }
                    let rid = row["id"].clone();
                    records::insert(db, "catalog_price_rate", &row).await?;
                    parameter(db, &rid, &value["conditions"]["parameters"]).await?;
                }
            }
            _ => return Err(DbErr::Custom("unsupported catalog pricing unit".into())),
        }
    }
    Ok(())
}
pub(crate) async fn write_billing<C: ConnectionTrait>(
    db: &C,
    book: &Value,
    billing: &Value,
) -> Result<(), DbErr> {
    let Some((kind, value)) = billing.as_object().and_then(|v| v.iter().next()) else {
        return Ok(());
    };
    let mut row = json!({"price_book_id":book,"kind":kind});
    for (key, path) in [
        ("product", "/product"),
        ("plan", "/plan"),
        ("description", "/description"),
        ("fee_amount", "/fee/amount"),
        ("fee_currency", "/fee/currency"),
        ("fee_period", "/fee/period"),
        ("amount_per_credit", "/amountPerCredit"),
        ("currency", "/currency"),
    ] {
        row[key] = value.pointer(path).cloned().unwrap_or(Value::Null);
    }
    records::insert(db, "catalog_price_billing", &row).await?;
    let allowances = value
        .get("entitlements")
        .or_else(|| value.get("allowances"));
    for (position, v) in allowances
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        records::insert(db,"catalog_price_allowance",&json!({"price_book_id":book,"position":position,"unit":v["unit"],"quota":v["quota"],"reset_period":v["resetPeriod"]})).await?;
    }
    if kind == "request" {
        write_component(
            db,
            book,
            "billing_request",
            &json!({"type":"usage","configuration":{"fixed":{"request":value}}}),
            None,
            None,
        )
        .await?;
    }
    Ok(())
}
#[derive(Default)]
pub(crate) struct PriceRows {
    pub components: Vec<Value>,
    pub rates: Vec<Value>,
    pub parameters: Vec<Value>,
    pub billing: Vec<Value>,
    pub allowances: Vec<Value>,
    components_by_book: BTreeMap<String, Vec<usize>>,
    children: BTreeMap<String, Vec<usize>>,
    rates_by_component: BTreeMap<String, Vec<usize>>,
    caches: BTreeMap<String, Vec<usize>>,
    parameters_by_rate: BTreeMap<String, Vec<usize>>,
    billing_by_book: BTreeMap<String, Vec<usize>>,
    allowances_by_book: BTreeMap<String, Vec<usize>>,
}
fn index(rows: &[Value], key: &str) -> BTreeMap<String, Vec<usize>> {
    let mut result = BTreeMap::<String, Vec<usize>>::new();
    for (position, row) in rows.iter().enumerate() {
        if let Some(key) = row[key].as_str() {
            result.entry(key.into()).or_default().push(position);
        }
    }
    result
}
fn indexed<'a>(
    rows: &'a [Value],
    index: &'a BTreeMap<String, Vec<usize>>,
    id: &Value,
) -> impl Iterator<Item = &'a Value> {
    index
        .get(id.as_str().unwrap_or_default())
        .into_iter()
        .flatten()
        .map(|i| &rows[*i])
}
fn amount(v: &Value) -> Value {
    if v.is_null() {
        Value::Null
    } else {
        json!(
            v.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| v.to_string())
        )
    }
}
fn number(v: &Value) -> Value {
    v.as_u64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .map_or(Value::Null, |v| json!(v))
}
fn rate(v: &Value) -> Value {
    json!({"amount":amount(&v["amount"]),"currency":v["currency"],"quantity":number(&v["quantity"])})
}
impl PriceRows {
    pub(crate) async fn load<C: ConnectionTrait>(
        db: &C,
        books: &[uuid::Uuid],
    ) -> Result<Self, DbErr> {
        if books.is_empty() {
            return Ok(Self::default());
        }
        let expr = Expr::col(Alias::new("price_book_id")).is_in(books.iter().copied());
        let components = records::rows(db, "catalog_price_component", Some(expr.clone())).await?;
        let ids = components
            .iter()
            .filter_map(|v| v["id"].as_str().and_then(|s| s.parse::<uuid::Uuid>().ok()))
            .collect::<Vec<_>>();
        let rates = records::rows(
            db,
            "catalog_price_rate",
            Some(Expr::col(Alias::new("component_id")).is_in(ids)),
        )
        .await?;
        let rids = rates
            .iter()
            .filter_map(|v| v["id"].as_str().and_then(|s| s.parse::<uuid::Uuid>().ok()))
            .collect::<Vec<_>>();
        let mut result = Self {
            components,
            rates,
            parameters: records::rows(
                db,
                "catalog_price_parameter",
                Some(Expr::col(Alias::new("rate_id")).is_in(rids)),
            )
            .await?,
            billing: records::rows(db, "catalog_price_billing", Some(expr.clone())).await?,
            allowances: records::rows(db, "catalog_price_allowance", Some(expr)).await?,
            ..Self::default()
        };
        result.components_by_book = index(&result.components, "price_book_id");
        result.children = index(&result.components, "parent_id");
        result.rates_by_component = index(&result.rates, "component_id");
        result.caches = index(&result.rates, "parent_rate_id");
        result.parameters_by_rate = index(&result.parameters, "rate_id");
        result.billing_by_book = index(&result.billing, "price_book_id");
        result.allowances_by_book = index(&result.allowances, "price_book_id");
        Ok(result)
    }
    fn token(&self, v: &Value) -> Value {
        let cache = match v["cache_mode"].as_str() {
            Some("separate_rate" | "estimated_rate") => {
                json!({"type":v["cache_mode"],"configuration":{"rate":indexed(&self.rates, &self.caches, &v["id"]).next().map(rate)}})
            }
            Some(mode) => json!({"type":mode}),
            None => Value::Null,
        };
        json!({"rate":rate(v),"cache":cache})
    }
    fn condition(&self, v: &Value) -> Value {
        let parameters = indexed(&self.parameters, &self.parameters_by_rate, &v["id"])
            .map(|p| (p["name"].as_str().unwrap().to_owned(), p["value"].clone()))
            .collect::<serde_json::Map<_, _>>();
        json!({"resolution":v["resolution"],"quality":v["quality"],"audio":v["audio"],"serviceTier":v["service_tier"],"parameters":parameters})
    }
    fn component(&self, c: &Value) -> Value {
        if c["kind"] != "usage" {
            return json!({"type":c["kind"]});
        }
        let mut rates = indexed(&self.rates, &self.rates_by_component, &c["id"])
            .filter(|r| r["parent_rate_id"] == Value::Null)
            .collect::<Vec<_>>();
        rates.sort_by_key(|r| r["position"].as_i64());
        let value = match c["usage_kind"].as_str() {
            Some("scheduled") => {
                let mut children =
                    indexed(&self.components, &self.children, &c["id"]).collect::<Vec<_>>();
                children.sort_by_key(|v| v["from_hour"].as_i64());
                json!({"scheduled":{"timezone":c["timezone"],"slots":children.iter().map(|v|json!({"fromHour":v["from_hour"],"toHour":v["to_hour"],"pricing":self.component(v)["configuration"]})).collect::<Vec<_>>()}})
            }
            Some("tiered_token") => {
                json!({"tiered_token":{"measuredBy":c["measured_by"],"tiers":rates.iter().map(|v|json!({"fromTokens":number(&v["from_tokens"]),"toTokens":number(&v["to_tokens"]),"rate":self.token(v)})).collect::<Vec<_>>()}})
            }
            _ => {
                let r = rates.first().copied().unwrap_or(&Value::Null);
                match c["rate_kind"].as_str() {
                    Some("token") => json!({"fixed":{"token":self.token(r)}}),
                    Some("request") => json!({"fixed":{"request":{"rate":rate(r)}}}),
                    Some("audio") => {
                        json!({"fixed":{"audio":{"type":c["audio_kind"],"configuration":{"rate":rate(r)}}}})
                    }
                    Some(kind @ ("image" | "video" | "credit")) => {
                        json!({"fixed":{kind:{"rates":rates.iter().map(|r|{let mut value=if kind=="credit"{json!({"unit":r["unit"],"direction":r["direction"],"quantity":number(&r["quantity"]),"credits":amount(&r["credits"])})}else{json!({"rate":rate(r)})};value["conditions"]=self.condition(r);if kind=="video"{value["unit"]=r["unit"].clone();}value}).collect::<Vec<_>>()}}})
                    }
                    _ => Value::Null,
                }
            }
        };
        json!({"type":"usage","configuration":value})
    }
    pub(crate) fn resolve(&self, book: &Value) -> (BTreeMap<String, Value>, Value) {
        let prices = indexed(&self.components, &self.components_by_book, book)
            .filter(|v| v["parent_id"] == Value::Null)
            .map(|v| (v["feature"].as_str().unwrap().to_owned(), self.component(v)))
            .collect::<BTreeMap<_, _>>();
        let billing=indexed(&self.billing, &self.billing_by_book, book).next().map(|v|{let allowances=indexed(&self.allowances, &self.allowances_by_book, book).map(|r|json!({"unit":r["unit"],"quota":number(&r["quota"]),"resetPeriod":r["reset_period"]})).collect::<Vec<_>>();match v["kind"].as_str(){Some("free")=>json!({"free":{"description":v["description"],"allowances":allowances}}),Some("subscription")=>json!({"subscription":{"product":v["product"],"plan":v["plan"],"fee":if v["fee_amount"]==Value::Null && v["fee_currency"]==Value::Null && v["fee_period"]==Value::Null{Value::Null}else{json!({"amount":amount(&v["fee_amount"]),"currency":v["fee_currency"],"period":v["fee_period"]})},"entitlements":allowances}}),Some("credit")=>json!({"credit":{"amountPerCredit":amount(&v["amount_per_credit"]),"currency":v["currency"]}}),Some("request")=>json!({"request":prices.get("billing_request").map(|v|v["configuration"]["fixed"]["request"].clone())}),_=>Value::Null}}).unwrap_or(Value::Null);
        (prices, billing)
    }
}
