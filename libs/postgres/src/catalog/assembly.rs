use super::{
    prices::{self, PriceRows},
    records,
};
use crate::{CatalogModelRecordSchema, postgres_connection::PostgresConnection};
use compallm_core::model::ModelCatalog;
use sea_orm::{
    DbErr, ExprTrait,
    sea_query::{Alias, Expr},
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct ModelRecord {
    pub row: CatalogModelRecordSchema,
    pub catalog: ModelCatalog,
}
impl std::ops::Deref for ModelRecord {
    type Target = CatalogModelRecordSchema;
    fn deref(&self) -> &Self::Target {
        &self.row
    }
}
impl TryFrom<ModelRecord> for ModelCatalog {
    type Error = DbErr;
    fn try_from(value: ModelRecord) -> Result<Self, DbErr> {
        Ok(value.catalog)
    }
}
fn path(value: &str) -> String {
    value
        .replace("image_generation", "imageGeneration")
        .replace("structured_output", "structuredOutput")
}
fn put_price(model: &mut Value, feature: &str, pricing: &Value) {
    if let Some((_, p)) = prices::FEATURES.iter().find(|(name, _)| *name == feature)
        && let Some(cap) = model.pointer_mut(&path(p))
        && cap["status"] == "supported"
    {
        cap["pricing"] = pricing.clone();
    }
}
fn pointer_set(model: &mut Value, pointer: &str, value: Value) {
    if let Some(p) = model.pointer_mut(pointer) {
        *p = value;
    }
}
fn add_price(model: &mut Value, values: &BTreeMap<String, Value>, billing: &Value) {
    for (key, value) in values {
        put_price(model, key, value);
    }
    if billing != &Value::Null {
        model["billing"] = billing.clone();
    }
}
fn merge_private(model: &mut Value, values: &BTreeMap<String, Value>, flags: &BTreeSet<String>) {
    for (feature, price) in values {
        if feature == "text_input" {
            let target = model.pointer_mut("/text/input/pricing");
            if let Some(target) = target {
                let cached = target
                    .pointer("/configuration/fixed/token/cache")
                    .cloned()
                    .unwrap_or(Value::Null);
                if flags.contains("inputPriceEnabled") {
                    *target = price.clone();
                    if !flags.contains("cachedPriceEnabled")
                        && let Some(v) = target.pointer_mut("/configuration/fixed/token/cache")
                    {
                        *v = cached;
                    }
                } else if flags.contains("cachedPriceEnabled")
                    && let Some(v) = target.pointer_mut("/configuration/fixed/token/cache")
                {
                    *v = price
                        .pointer("/configuration/fixed/token/cache")
                        .cloned()
                        .unwrap_or(Value::Null);
                }
            }
        } else if feature != "text_output" || flags.contains("outputPriceEnabled") {
            put_price(model, feature, price);
        }
    }
}
pub(crate) async fn resolve(
    db: &PostgresConnection,
    rows: Vec<CatalogModelRecordSchema>,
) -> Result<Vec<ModelRecord>, DbErr> {
    if rows.is_empty() {
        return Ok(vec![]);
    }
    let ids = rows
        .iter()
        .map(|r| r.row_id)
        .chain(rows.iter().filter_map(|r| r.base_row_id))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let own = rows.iter().map(|r| r.row_id).collect::<Vec<_>>();
    let efforts = records::rows(
        db,
        "catalog_model_thinking_effort",
        Some(Expr::col(Alias::new("model_id")).is_in(own.clone())),
    )
    .await?;
    let formats = records::rows(
        db,
        "catalog_model_format",
        Some(Expr::col(Alias::new("model_id")).is_in(own)),
    )
    .await?;
    let links = records::rows(
        db,
        "catalog_model_price_book",
        Some(Expr::col(Alias::new("model_id")).is_in(ids)),
    )
    .await?;
    let bids = links
        .iter()
        .filter_map(|r| {
            r["price_book_id"]
                .as_str()
                .and_then(|s| s.parse::<uuid::Uuid>().ok())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let books = records::rows(
        db,
        "catalog_price_book",
        Some(Expr::col(Alias::new("id")).is_in(bids.clone())),
    )
    .await?;
    let prices = PriceRows::load(db, &bids).await?;
    let rules = super::fallback::Rules::load(db).await?;
    let group = |values: Vec<Value>, key: &str| {
        let mut grouped = BTreeMap::<String, Vec<Value>>::new();
        for value in values {
            if let Some(id) = value[key].as_str() {
                grouped.entry(id.to_owned()).or_default().push(value);
            }
        }
        grouped
    };
    let efforts = group(efforts, "model_id");
    let formats = group(formats, "model_id");
    let links = group(links, "model_id");
    let books = books
        .into_iter()
        .map(|b| (b["id"].as_str().unwrap().to_owned(), b))
        .collect::<BTreeMap<_, _>>();
    let mut output = Vec::with_capacity(rows.len());
    for row in rows {
        let mut model = serde_json::to_value(ModelCatalog::try_from(row.clone())?)
            .map_err(|e| DbErr::Custom(e.to_string()))?;
        let row_id = row.row_id.to_string();
        let mapping = efforts
            .get(&row_id)
            .into_iter()
            .flatten()
            .map(|v| {
                (
                    v["level"].as_str().unwrap().to_owned(),
                    v["provider_value"].clone(),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        pointer_set(
            &mut model,
            "/text/output/configuration/thinking/configuration/effortMapping",
            json!(mapping),
        );
        for (feature, pointer) in [
            ("vision", "/vision"),
            ("audio_input", "/audio/input"),
            ("audio_output", "/audio/output"),
            ("video", "/video"),
            ("image_generation", "/imageGeneration"),
        ] {
            let mut list = formats
                .get(&row_id)
                .into_iter()
                .flatten()
                .filter(|v| v["capability"] == feature)
                .collect::<Vec<_>>();
            list.sort_by_key(|v| v["position"].as_i64());
            if !list.is_empty() {
                pointer_set(
                    &mut model,
                    &format!("{pointer}/configuration/formats"),
                    json!(list.iter().map(|v| v["format"].clone()).collect::<Vec<_>>()),
                );
            }
        }
        rules.apply(&row.provider_catalog_id, &row.id, &mut model)?;
        let base = row.base_row_id.unwrap_or(row.row_id).to_string();
        let eligible = |book: &Value| {
            book["region"] == Value::Null
                && book["plan"] == Value::Null
                && book["service_tier"] == Value::Null
                && book["request_mode"] == "standard"
        };
        let selected = |mid: &str, route: &str| -> Result<Option<&Value>, DbErr> {
            let choices = links
                .get(mid)
                .into_iter()
                .flatten()
                .filter(|l| l["route_key"] == route)
                .filter_map(|l| books.get(l["price_book_id"].as_str().unwrap_or_default()))
                .filter(|b| eligible(b))
                .collect::<Vec<_>>();
            if choices.len() > 1 {
                return Ok(None);
            }
            Ok(choices.first().copied())
        };
        if let Some(book) = selected(&base, "")? {
            let (values, billing) = prices.resolve(&book["id"]);
            add_price(&mut model, &values, &billing);
        }
        if row.base_row_id.is_some()
            && let Some(book) = selected(&row.row_id.to_string(), "")?
        {
            let (values, billing) = prices.resolve(&book["id"]);
            merge_private(
                &mut model,
                &values,
                &serde_json::from_value(row.custom_fields.clone())
                    .map_err(|e| DbErr::Custom(e.to_string()))?,
            );
            if billing != Value::Null {
                model["billing"] = billing;
            }
        }
        if let Some(options) = model
            .pointer_mut("/routeSelection/options")
            .and_then(Value::as_array_mut)
        {
            for option in options {
                if let Some(book) = selected(&base, option["id"].as_str().unwrap_or_default())? {
                    let (values, billing) = prices.resolve(&book["id"]);
                    let mut nested = json!({"text":option["text"],"billing":billing});
                    add_price(&mut nested, &values, &Value::Null);
                    option["text"] = nested["text"].clone();
                    option["billing"] = nested["billing"].clone();
                }
            }
        }
        let catalog = serde_json::from_value(model).map_err(|e| {
            DbErr::Custom(format!(
                "catalog {} {}: {e}",
                row.provider_catalog_id, row.id
            ))
        })?;
        output.push(ModelRecord { row, catalog });
    }
    Ok(output)
}
