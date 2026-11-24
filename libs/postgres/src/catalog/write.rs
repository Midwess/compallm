use super::{
    prices, records,
    repository::{CatalogRepository, CatalogScope},
};
use crate::postgres_connection::PostgresConnection;
use compallm_core::{
    model::ModelCatalog,
    persistent::{DbContext, db_context},
};
use sea_orm::{
    ConnectionTrait, DbErr, ExprTrait,
    sea_query::{Alias, Expr, Query},
};
use serde_json::{Value, json};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) enum WriteError {
    Database(#[from] DbErr),
    Connection(#[from] compallm_core::persistent::ConnectionError),
}
fn encoded(value: &ModelCatalog) -> Result<Value, DbErr> {
    let value = serde_json::to_value(value).map_err(|e| DbErr::Custom(e.to_string()))?;
    let mut row = value.clone();
    for (camel, snake) in [
        ("imageGeneration", "image_generation"),
        ("structuredOutput", "structured_output"),
        ("bodyDefaults", "body_defaults"),
        ("routeSelection", "route_selection"),
        ("customFields", "custom_fields"),
    ] {
        row[snake] = value[camel].clone();
    }
    Ok(row)
}
async fn model_write(
    db: &PostgresConnection,
    value: &ModelCatalog,
    id: uuid::Uuid,
    scope: CatalogScope,
    base: Option<uuid::Uuid>,
    new: bool,
    changed: &[String],
) -> Result<(), DbErr> {
    let mut row = encoded(value)?;
    prices::clear_route_prices(&mut row["route_selection"]);
    let shape: Value = serde_json::from_str(include_str!(
        "../migration/m20261122_000054_catalog_search/relational/shape.json"
    ))
    .map_err(|e| DbErr::Custom(e.to_string()))?;
    let has = |field: &str| new || changed.iter().any(|v| v == field);
    let mut fields = serde_json::Map::new();
    if new {
        fields.extend(json!({"row_id":id.to_string(),"organization_id":scope.organization_id,"user_id":scope.user_id,"base_row_id":base.map(|v|v.to_string()),"provider_catalog_id":value.provider_catalog_id,"id":value.id,"name":value.name,"display_name":value.display_name,"description":value.description,"service_kind":value.service_kind.map(String::from),"connections":value.connections,"headers":value.headers,"body_defaults":value.body_defaults,"source_id":null,"custom_fields":[],"route_selection":row["route_selection"]}).as_object().unwrap().clone());
    }
    for (flag, column) in [
        ("name", "name"),
        ("path", "connections"),
        ("headers", "headers"),
        ("body_defaults", "body_defaults"),
        ("limits", "context_tokens"),
    ] {
        if has(flag) {
            fields.insert(
                column.into(),
                if column == "context_tokens" {
                    row["limits"]["contextTokens"].clone()
                } else {
                    row[column].clone()
                },
            );
        }
    }
    for (feature, path) in shape["features"].as_object().unwrap() {
        if has("capabilities") || feature == "thinking" && has("effort_mapping") {
            fields.insert(
                format!("{feature}_support"),
                row.pointer(&format!(
                    "/{}/status",
                    path.as_str().unwrap().replace(',', "/")
                ))
                .cloned()
                .unwrap_or(json!("unknown")),
            );
        }
    }
    for (column, path) in shape["limits"].as_object().unwrap() {
        if has("limits")
            || column.starts_with("thinking_") && (has("effort_mapping") || has("capabilities"))
        {
            fields.insert(
                column.clone(),
                row.pointer(&format!("/{}", path.as_str().unwrap().replace(',', "/")))
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        }
    }
    if has("capabilities") {
        for (feature, path) in shape["formats"].as_object().unwrap() {
            let formats = row
                .pointer(&format!("/{}", path.as_str().unwrap().replace(',', "/")))
                .and_then(Value::as_array);
            fields.insert(format!("{feature}_formats_known"), json!(formats.is_some()));
        }
    }
    if new {
        records::insert(db, "catalog_model_book", &Value::Object(fields)).await?;
    } else if !fields.is_empty() {
        let mut q = Query::update();
        q.table(Alias::new("catalog_model_book"));
        for (key, value) in fields {
            q.value(Alias::new(&key), records::cell(&key, &value)?);
        }
        q.and_where(Expr::col(Alias::new("row_id")).eq(id));
        db.execute(&q.to_owned()).await?;
    }
    if has("capabilities") {
        records::delete(
            db,
            "catalog_model_format",
            Expr::col(Alias::new("model_id")).eq(id),
        )
        .await?;
        for (feature, path) in shape["formats"].as_object().unwrap() {
            if let Some(formats) = row
                .pointer(&format!("/{}", path.as_str().unwrap().replace(',', "/")))
                .and_then(Value::as_array)
            {
                for (position, format) in formats.iter().enumerate() {
                    records::insert(db, "catalog_model_format", &json!({"model_id":id.to_string(),"capability":feature,"format":format,"position":position})).await?;
                }
            }
        }
    }
    if has("capabilities") || has("effort_mapping") {
        records::delete(
            db,
            "catalog_model_thinking_effort",
            Expr::col(Alias::new("model_id")).eq(id),
        )
        .await?;
        if let Some(mapping) = row
            .pointer("/text/output/configuration/thinking/configuration/effortMapping")
            .and_then(Value::as_object)
        {
            for (level, value) in mapping {
                records::insert(
                    db,
                    "catalog_model_thinking_effort",
                    &json!({"model_id":id.to_string(),"level":level,"provider_value":value}),
                )
                .await?;
            }
        }
        db.execute(
            &Query::update()
                .table(Alias::new("catalog_model_book"))
                .value(
                    Alias::new("thinking_default_effort"),
                    value
                        .text
                        .output
                        .configuration()
                        .and_then(|v| v.thinking.configuration())
                        .and_then(|v| v.default_effort)
                        .map(|v| {
                            serde_json::to_value(v)
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .to_owned()
                        }),
                )
                .and_where(Expr::col(Alias::new("row_id")).eq(id))
                .to_owned(),
        )
        .await?;
    }
    Ok(())
}
#[db_context::current]
pub(crate) async fn save(
    value: ModelCatalog,
    scope: CatalogScope,
    base: Option<uuid::Uuid>,
    changed: &[String],
) -> Result<ModelCatalog, WriteError> {
    let changed = changed.to_vec();
    let provider = value.provider_catalog_id.clone();
    let logical = value.id.clone();
    compallm_core::transaction!(db, {
        let context = DbContext::current().map_err(|e| DbErr::Custom(e.to_string()))?;
        let connection =
            PostgresConnection::from_context(&context).map_err(|e| DbErr::Custom(e.to_string()))?;
        use sha2::{Digest, Sha256};
        let digest = Sha256::digest(
            format!(
                "{}:{}:{provider}:{logical}",
                scope.organization_id, scope.user_id
            )
            .as_bytes(),
        );
        let lock = i64::from_be_bytes(digest[..8].try_into().unwrap());
        connection
            .query_one(
                &Query::select()
                    .expr(
                        sea_orm::sea_query::Func::cust(Alias::new("pg_advisory_xact_lock"))
                            .arg(Expr::val(lock)),
                    )
                    .to_owned(),
            )
            .await?;
        let existing = CatalogRepository
            .personal_row(&provider, &logical, scope)
            .await?;
        let id = existing
            .as_ref()
            .map(|r| r.row_id)
            .unwrap_or_else(uuid::Uuid::new_v4);
        model_write(
            connection,
            &value,
            id,
            scope,
            base,
            existing.is_none(),
            &changed,
        )
        .await?;
        let mut flags = existing
            .as_ref()
            .map(|v| v.catalog.custom_fields.clone())
            .unwrap_or_default();
        for flag in [
            "contextLimitEnabled",
            "inputLimitEnabled",
            "outputLimitEnabled",
            "inputPriceEnabled",
            "outputPriceEnabled",
            "cachedPriceEnabled",
        ] {
            let field = match flag {
                "inputPriceEnabled" => "input_price",
                "outputPriceEnabled" => "output_price",
                "cachedPriceEnabled" => "cached_price",
                _ => "limits",
            };
            if existing.is_none() || changed.iter().any(|v| v == field) {
                if value.custom_fields.contains(flag) {
                    flags.insert(flag.into());
                } else {
                    flags.remove(flag);
                }
            }
        }
        connection
            .execute(
                &Query::update()
                    .table(Alias::new("catalog_model_book"))
                    .value(Alias::new("custom_fields"), json!(flags))
                    .and_where(Expr::col(Alias::new("row_id")).eq(id))
                    .to_owned(),
            )
            .await?;
        if existing.is_none()
            || changed
                .iter()
                .any(|v| ["input_price", "output_price", "cached_price"].contains(&v.as_str()))
        {
            let mut priced = existing
                .as_ref()
                .map(|v| v.catalog.clone())
                .unwrap_or_else(|| value.clone());
            for (field, feature) in [
                ("input_price", "text_input"),
                ("output_price", "text_output"),
                ("cached_price", "cached"),
            ] {
                if existing.is_none() || changed.iter().any(|v| v == field) {
                    match feature {
                        "text_input" => {
                            if let compallm_core::provider::CapabilitySupport::Supported {
                                pricing,
                                ..
                            } = &mut priced.text.input
                            {
                                *pricing = value.text.input.pricing().cloned();
                            }
                        }
                        "text_output" => {
                            if let compallm_core::provider::CapabilitySupport::Supported {
                                pricing,
                                ..
                            } = &mut priced.text.output
                            {
                                *pricing = value.text.output.pricing().cloned();
                            }
                        }
                        _ => {
                            if let compallm_core::provider::CapabilitySupport::Supported {
                                pricing:
                                    Some(compallm_core::provider::CapabilityPricing::Usage(
                                        compallm_core::provider::CapabilityUsagePricing::Fixed(
                                            compallm_core::provider::CapabilityRates::Token(v),
                                        ),
                                    )),
                                ..
                            } = &mut priced.text.input
                            {
                                v.cache = match value.text.input.pricing() {
                                    Some(compallm_core::provider::CapabilityPricing::Usage(
                                        compallm_core::provider::CapabilityUsagePricing::Fixed(
                                            compallm_core::provider::CapabilityRates::Token(v),
                                        ),
                                    )) => v.cache.clone(),
                                    _ => None,
                                };
                            }
                        }
                    }
                }
            }
            let prior = records::rows(
                connection,
                "catalog_model_price_book",
                Some(Expr::col(Alias::new("model_id")).eq(id)),
            )
            .await?;
            for link in prior {
                let bid = link["price_book_id"]
                    .as_str()
                    .unwrap()
                    .parse::<uuid::Uuid>()
                    .map_err(|e| DbErr::Custom(e.to_string()))?;
                records::delete(
                    connection,
                    "catalog_model_price_book",
                    Expr::col(Alias::new("model_id"))
                        .eq(id)
                        .and(Expr::col(Alias::new("price_book_id")).eq(bid)),
                )
                .await?;
                let remaining = records::rows(
                    connection,
                    "catalog_model_price_book",
                    Some(Expr::col(Alias::new("price_book_id")).eq(bid)),
                )
                .await?;
                if remaining.is_empty() {
                    records::delete(
                        connection,
                        "catalog_price_book",
                        Expr::col(Alias::new("id")).eq(bid),
                    )
                    .await?;
                }
            }
            let mut encoded = encoded(&priced)?;
            if !flags.contains("inputPriceEnabled")
                && !flags.contains("cachedPriceEnabled")
                && let Some(v) = encoded.pointer_mut("/text/input/pricing")
            {
                *v = Value::Null;
            }
            if !flags.contains("outputPriceEnabled")
                && let Some(v) = encoded.pointer_mut("/text/output/pricing")
            {
                *v = Value::Null;
            }
            if [
                "inputPriceEnabled",
                "outputPriceEnabled",
                "cachedPriceEnabled",
            ]
            .iter()
            .any(|f| flags.contains(*f))
            {
                let book = uuid::Uuid::new_v4();
                records::insert(connection,"catalog_price_book",&json!({"id":book.to_string(),"provider_catalog_id":provider,"key":format!("private:{id}"),"organization_id":scope.organization_id,"user_id":scope.user_id,"evidence_status":"user_defined"})).await?;
                records::insert(
                    connection,
                    "catalog_model_price_book",
                    &json!({"model_id":id.to_string(),"price_book_id":book.to_string()}),
                )
                .await?;
                for (feature, path) in prices::FEATURES {
                    if !["text_input", "text_output"].contains(&feature) {
                        continue;
                    }
                    if let Some(price) = encoded.pointer(&(path.to_string() + "/pricing")) {
                        prices::write_component(
                            connection,
                            &json!(book.to_string()),
                            feature,
                            price,
                            None,
                            None,
                        )
                        .await?;
                    }
                }
            }
        }
        Ok::<_, WriteError>(
            CatalogRepository
                .personal_row(&provider, &logical, scope)
                .await?
                .map(|v| v.catalog)
                .ok_or_else(|| DbErr::Custom("catalog save unavailable".into()))?,
        )
    })
}
#[db_context::current]
pub(crate) async fn delete(
    provider: &str,
    id: &str,
    scope: CatalogScope,
    custom: bool,
) -> Result<bool, WriteError> {
    let Some(row) = CatalogRepository.personal_row(provider, id, scope).await? else {
        return Ok(false);
    };
    if custom != row.base_row_id.is_none() {
        return Ok(false);
    }
    compallm_core::transaction!(db, {
        let context = DbContext::current().map_err(|e| DbErr::Custom(e.to_string()))?;
        let connection =
            PostgresConnection::from_context(&context).map_err(|e| DbErr::Custom(e.to_string()))?;
        let links = records::rows(
            connection,
            "catalog_model_price_book",
            Some(Expr::col(Alias::new("model_id")).eq(row.row_id)),
        )
        .await?;
        records::delete(
            connection,
            "catalog_model_book",
            Expr::col(Alias::new("row_id")).eq(row.row_id),
        )
        .await?;
        for link in links {
            let bid = link["price_book_id"]
                .as_str()
                .unwrap()
                .parse::<uuid::Uuid>()
                .map_err(|e| DbErr::Custom(e.to_string()))?;
            if records::rows(
                connection,
                "catalog_model_price_book",
                Some(Expr::col(Alias::new("price_book_id")).eq(bid)),
            )
            .await?
            .is_empty()
            {
                records::delete(
                    connection,
                    "catalog_price_book",
                    Expr::col(Alias::new("id")).eq(bid),
                )
                .await?;
            }
        }
        Ok::<_, WriteError>(true)
    })
}
