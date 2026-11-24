use crate::catalog::{prices, records};
use sea_orm::{
    ConnectionTrait, DbErr, ExprTrait,
    sea_query::{Alias, Expr, Query},
};
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};

async fn update<C: ConnectionTrait>(
    db: &C,
    id: &Value,
    values: &serde_json::Map<String, Value>,
) -> Result<(), DbErr> {
    let mut q = Query::update();
    q.table(Alias::new("catalog_model_book"));
    for (key, value) in values {
        q.value(Alias::new(key), records::cell(key, value)?);
    }
    q.and_where(Expr::col(Alias::new("row_id")).eq(records::cell("row_id", id)?));
    db.execute(&q.to_owned()).await?;
    Ok(())
}
fn pointer(path: &str) -> String {
    format!("/{}", path.replace(',', "/"))
}
pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let models = records::rows(db, "provider_model_templates", None).await?;
    db.execute_unprepared(include_str!("relational/up.sql"))
        .await?;
    let shape: Value = serde_json::from_str(include_str!("relational/shape.json"))
        .map_err(|e| DbErr::Custom(e.to_string()))?;
    for row in &models {
        let id = &row["row_id"];
        let mut values = serde_json::Map::new();
        for (feature, path) in shape["features"].as_object().unwrap() {
            let path = pointer(path.as_str().unwrap());
            let support = row
                .pointer(&(path + "/status"))
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            values.insert(format!("{feature}_support"), json!(support));
        }
        for (column, path) in shape["limits"].as_object().unwrap() {
            values.insert(
                column.clone(),
                row.pointer(&pointer(path.as_str().unwrap()))
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        }
        values.insert(
            "thinking_default_effort".into(),
            row.pointer("/text/output/configuration/thinking/configuration/defaultEffort")
                .cloned()
                .unwrap_or(Value::Null),
        );
        if let Some(levels) = row
            .pointer("/text/output/configuration/thinking/configuration/effortMapping")
            .and_then(Value::as_object)
        {
            for (level, value) in levels {
                records::insert(
                    db,
                    "catalog_model_thinking_effort",
                    &json!({"model_id":id,"level":level,"provider_value":value}),
                )
                .await?;
            }
        }
        for (feature, path) in shape["formats"].as_object().unwrap() {
            let formats = row
                .pointer(&pointer(path.as_str().unwrap()))
                .and_then(Value::as_array);
            values.insert(format!("{feature}_formats_known"), json!(formats.is_some()));
            for (position, format) in formats.into_iter().flatten().enumerate() {
                records::insert(db,"catalog_model_format",&json!({"model_id":id,"capability":feature,"format":format,"position":position})).await?;
            }
        }
        let mut routes = row["route_selection"].clone();
        prices::clear_route_prices(&mut routes);
        values.insert("route_selection".into(), routes);
        update(db, id, &values).await?;
        let mut selected = row.clone();
        if row["user_id"] != Value::Null {
            for (feature, path) in prices::FEATURES {
                let flag = match feature {
                    "text_input"
                        if !row["custom_fields"]
                            .as_array()
                            .is_some_and(|v| v.iter().any(|s| s == "cachedPriceEnabled")) =>
                    {
                        Some("inputPriceEnabled")
                    }
                    "text_input" => None,
                    "text_output" => Some("outputPriceEnabled"),
                    _ => None,
                };
                if let Some(flag) = flag
                    && !row["custom_fields"]
                        .as_array()
                        .is_some_and(|v| v.iter().any(|s| s == flag))
                    && let Some(v) = selected.pointer_mut(path)
                {
                    v["pricing"] = Value::Null;
                }
            }
        }
        write_book(db, row, &selected, "").await?;
        if let Some(options) = row["route_selection"]["options"].as_array() {
            for option in options {
                let mut priced = json!({"text":option["text"],"billing":option["billing"]});
                priced["provider_catalog_id"] = row["provider_catalog_id"].clone();
                write_book(db, row, &priced, option["id"].as_str().unwrap()).await?;
            }
        }
    }
    promote_prices(db, &models).await?;
    let provider_reviews = records::rows(db, "provider_template_research", None).await?;
    let manifest: Value =
        serde_json::from_str(super::import::DATA).map_err(|e| DbErr::Custom(e.to_string()))?;
    for review in provider_reviews {
        let metadata = &review["metadata"]["catalog_import"];
        let credential_forms = metadata["create_field_variants"]
            .as_object()
            .into_iter()
            .flat_map(|v| v.values())
            .filter_map(Value::as_array)
            .flatten();
        let declares_key = credential_forms
            .into_iter()
            .filter_map(|v| v["credential_fields"].as_array())
            .flatten()
            .any(|f| f["key"] == "api_key");
        let confirmed = declares_key
            || review["authentication"]["api_key"]["status"] == "configured"
            || metadata["configurations"]
                .as_object()
                .is_some_and(|v| !v.is_empty())
            || metadata["configuration"]["api_key_env"].as_str().is_some()
            || review["provider_template_id"] == "bedrock"
            || review["provider_template_id"] == "bedrock-mantle";
        db.execute(
            &Query::update()
                .table(Alias::new("catalog_provider_book"))
                .value(Alias::new("api_key_supported"), confirmed)
                .and_where(
                    Expr::col(Alias::new("id"))
                        .eq(review["provider_template_id"].as_str().unwrap()),
                )
                .to_owned(),
        )
        .await?;
    }
    let source = json!({"id":"catalog:fallback-rules","source_refs":[{"kind":"repository","reference":"opensource(l)","revision":manifest["manifest"]["revision"]}],"source_records":serde_json::from_str::<Value>(include_str!("relational/fallbacks.json")).map_err(|e|DbErr::Custom(e.to_string()))?,"acquisition_recipes":[],"observations":{},"state":"complete"});
    records::insert(db, "catalog_source", &source).await?;
    let fallback = &source["source_records"];
    for (position, rule) in fallback.as_array().unwrap().iter().enumerate() {
        let id = json!(uuid::Uuid::new_v4().to_string());
        let routing = rule["model_info"].get("litellm_provider").is_some();
        records::insert(db,"catalog_fallback_book",&json!({"id":id,"key":rule["name"],"priority":position,"rule_kind":if routing{"provider_inference"}else{"capability"},"model_pattern":rule["pattern"],"fill_missing":rule["fill_missing_for_providers"].as_array().is_some(),"source_id":"catalog:fallback-rules"})).await?;
        let targets = rule["fill_missing_for_providers"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|slug| {
                manifest["manifest"]["provider_bindings"]
                    .get(slug.as_str().unwrap())
                    .and_then(Value::as_str)
            })
            .collect::<std::collections::BTreeSet<_>>();
        for target in targets {
            records::insert(
                db,
                "catalog_fallback_provider",
                &json!({"rule_id":id,"provider_catalog_id":target}),
            )
            .await?;
        }
        for (key, value) in rule["model_info"].as_object().unwrap() {
            let mut fact = json!({"rule_id":id,"field":key});
            fact[if value.is_boolean() {
                "boolean_value"
            } else if value.is_number() {
                "integer_value"
            } else {
                "text_value"
            }] = value.clone();
            records::insert(db, "catalog_fallback_fact", &fact).await?;
        }
    }
    db.execute_unprepared(include_str!("relational/finish.sql"))
        .await?;
    db.execute_unprepared(include_str!("relational/integrity.sql"))
        .await?;
    db.execute_unprepared(include_str!("relational/view.sql"))
        .await?;
    db.execute_unprepared(include_str!("relational/security.sql"))
        .await?;
    for table in TABLES {
        for row in records::rows(db, table, None).await? {
            records::insert(
                db,
                "catalog_relational_applied",
                &json!({"table_name":table,"row_key":key(table,&row),"applied":row}),
            )
            .await?;
        }
    }
    Ok(())
}
async fn write_book<C: ConnectionTrait>(
    db: &C,
    row: &Value,
    selected: &Value,
    route: &str,
) -> Result<(), DbErr> {
    let present = selected["billing"] != Value::Null
        || prices::FEATURES.iter().any(|(_, path)| {
            selected
                .pointer(&(format!("{path}/pricing")))
                .is_some_and(|v| v != &Value::Null)
        });
    if !present {
        return Ok(());
    }
    let book = json!(uuid::Uuid::new_v4().to_string());
    records::insert(db,"catalog_price_book",&json!({"id":book,"provider_catalog_id":row["provider_template_id"],"key":format!("{}:{route}",row["row_id"].as_str().unwrap()),"organization_id":row["organization_id"],"user_id":row["user_id"],"source_id":row["research_id"],"evidence_status":if row["user_id"]==Value::Null{"imported"}else{"user_defined"}})).await?;
    records::insert(
        db,
        "catalog_model_price_book",
        &json!({"model_id":row["row_id"],"price_book_id":book,"route_key":route}),
    )
    .await?;
    for (feature, path) in prices::FEATURES {
        if let Some(value) = selected.pointer(&(format!("{path}/pricing"))) {
            prices::write_component(db, &book, feature, value, None, None).await?;
        }
    }
    prices::write_billing(db, &book, &selected["billing"]).await
}
async fn promote_prices<C: ConnectionTrait>(db: &C, models: &[Value]) -> Result<(), DbErr> {
    use std::collections::{BTreeMap, BTreeSet};
    let data: Value = serde_json::from_str(include_str!("relational/price-book.json"))
        .map_err(|e| DbErr::Custom(e.to_string()))?;
    let models = models
        .iter()
        .filter(|r| r["user_id"] == Value::Null)
        .map(|r| {
            (
                (
                    r["provider_template_id"].as_str().unwrap(),
                    r["id"].as_str().unwrap(),
                ),
                r,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let links = records::rows(db, "catalog_model_price_book", None).await?;
    let defaults = links
        .iter()
        .filter(|r| r["route_key"] == "")
        .map(|r| (r["model_id"].as_str().unwrap(), &r["price_book_id"]))
        .collect::<BTreeMap<_, _>>();
    let components = records::rows(db, "catalog_price_component", None).await?;
    let mut features = BTreeMap::<String, BTreeSet<String>>::new();
    for c in components.iter().filter(|c| c["parent_id"] == Value::Null) {
        features
            .entry(c["price_book_id"].as_str().unwrap().to_owned())
            .or_default()
            .insert(c["feature"].as_str().unwrap().to_owned());
    }
    for entry in data["books"].as_array().unwrap() {
        let Some(model) = models.get(&(
            entry["provider"].as_str().unwrap(),
            entry["model"].as_str().unwrap(),
        )) else {
            continue;
        };
        let standard = entry["service_tier"] == Value::Null && entry["request_mode"] == "standard";
        let existing = standard
            .then(|| defaults.get(model["row_id"].as_str().unwrap()).copied())
            .flatten();
        let book = existing
            .cloned()
            .unwrap_or_else(|| json!(uuid::Uuid::new_v4().to_string()));
        let missing = entry["prices"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(feature, _)| {
                !features
                    .get(book.as_str().unwrap())
                    .is_some_and(|f| f.contains(*feature))
            })
            .collect::<Vec<_>>();
        if missing.is_empty() {
            continue;
        }
        if existing.is_none() {
            records::insert(db,"catalog_price_book",&json!({"id":book,"provider_catalog_id":entry["provider"],"key":format!("opensource(l):{}:{}:{}",entry["source_key"].as_str().unwrap(),entry["service_tier"],entry["request_mode"]),"service_tier":entry["service_tier"],"request_mode":entry["request_mode"],"source_id":model["research_id"],"evidence_status":"imported"})).await?;
            records::insert(
                db,
                "catalog_model_price_book",
                &json!({"model_id":model["row_id"],"price_book_id":book}),
            )
            .await?;
        }
        for (feature, price) in missing {
            prices::write_component(db, &book, feature, price, None, None).await?;
        }
    }
    Ok(())
}
const TABLES: [&str; 15] = [
    "catalog_provider_book",
    "catalog_model_book",
    "catalog_source",
    "catalog_model_thinking_effort",
    "catalog_model_format",
    "catalog_price_book",
    "catalog_model_price_book",
    "catalog_price_component",
    "catalog_price_rate",
    "catalog_price_parameter",
    "catalog_price_billing",
    "catalog_price_allowance",
    "catalog_fallback_book",
    "catalog_fallback_provider",
    "catalog_fallback_fact",
];
fn key(table: &str, row: &Value) -> String {
    match table {
        "catalog_model_book" => row["row_id"].to_string(),
        "catalog_model_price_book" => {
            json!([row["model_id"], row["price_book_id"], row["route_key"]]).to_string()
        }
        "catalog_model_thinking_effort" => json!([row["model_id"], row["level"]]).to_string(),
        "catalog_model_format" => {
            json!([row["model_id"], row["capability"], row["position"]]).to_string()
        }
        "catalog_price_parameter" => json!([row["rate_id"], row["name"]]).to_string(),
        "catalog_price_allowance" => json!([row["price_book_id"], row["position"]]).to_string(),
        "catalog_price_billing" => row["price_book_id"].to_string(),
        "catalog_fallback_provider" => {
            json!([row["rule_id"], row["provider_catalog_id"]]).to_string()
        }
        "catalog_fallback_fact" => json!([row["rule_id"], row["field"]]).to_string(),
        _ => row["id"].to_string(),
    }
}
pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    if !manager.has_table("catalog_model_book").await? {
        return Ok(());
    }
    let db = manager.get_connection();
    let applied = records::rows(db, "catalog_relational_applied", None).await?;
    for table in TABLES {
        let current = records::rows(db, table, None).await?;
        let expected = applied
            .iter()
            .filter(|v| v["table_name"] == table)
            .map(|v| (v["row_key"].as_str().unwrap(), &v["applied"]))
            .collect::<std::collections::BTreeMap<_, _>>();
        if current.len() != expected.len()
            || current
                .iter()
                .any(|v| expected.get(key(table, v).as_str()).copied() != Some(v))
        {
            return Err(DbErr::Custom(
                "rollback would discard relational catalog changes".into(),
            ));
        }
    }
    db.execute_unprepared(include_str!("relational/down.sql"))
        .await?;
    Ok(())
}
