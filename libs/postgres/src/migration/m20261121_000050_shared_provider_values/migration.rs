use sea_orm::{ConnectionTrait, DbErr};
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};
use std::collections::BTreeMap;
#[path = "transform.rs"]
mod transform;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261121_000050_shared_provider_values"
    }
}

async fn rows<C: ConnectionTrait>(db: &C, table: &str) -> Result<Vec<Value>, DbErr> {
    use sea_orm::sea_query::{Alias, Asterisk, Expr, Func, Query};
    let query = Query::select()
        .expr_as(
            Func::cust(Alias::new("to_jsonb")).arg(Expr::col((Alias::new(table), Asterisk))),
            Alias::new("row"),
        )
        .from(Alias::new(table))
        .to_owned();
    db.query_all(&query)
        .await?
        .into_iter()
        .map(|r| r.try_get("", "row"))
        .collect()
}
async fn update<C: ConnectionTrait>(
    db: &C,
    table: &str,
    key: &Value,
    fields: Value,
) -> Result<(), DbErr> {
    use sea_orm::sea_query::{Alias, Expr, Query, Value as SqlValue};
    let mut query = Query::update();
    query.table(Alias::new(table));
    for (field, value) in fields
        .as_object()
        .ok_or_else(|| DbErr::Custom("invalid migration update".into()))?
    {
        let value: SqlValue = match (field.as_str(), value) {
            ("service_kind", Value::Null) => Option::<String>::None.into(),
            ("service_kind", Value::String(s)) => s.clone().into(),
            (_, Value::Null) => Option::<serde_json::Value>::None.into(),
            _ => value.clone().into(),
        };
        query.value(Alias::new(field), value);
    }
    for (field, value) in key.as_object().unwrap() {
        query.and_where(Expr::col(Alias::new(field)).eq(value.as_str().unwrap()));
    }
    db.execute(&query.to_owned()).await?;
    Ok(())
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("LOCK TABLE providers,provider_models,provider_templates,provider_model_templates,provider_template_research,provider_model_template_research IN ACCESS EXCLUSIVE MODE").await?;
        let providers = rows(db, "provider_templates").await?;
        let models = rows(db, "provider_model_templates").await?;
        let configured_providers = rows(db, "providers").await?;
        let configured_models = rows(db, "provider_models").await?;
        db.execute_unprepared(include_str!("up.sql")).await?;
        let provider_map: BTreeMap<_, _> = providers
            .iter()
            .map(|row| (row["id"].as_str().unwrap(), row))
            .collect();
        for row in &providers {
            update(
                db,
                "provider_templates",
                &json!({"id":row["id"]}),
                transform::provider(row),
            )
            .await?;
        }
        for row in &models {
            let parent = provider_map
                .get(row["provider_template_id"].as_str().unwrap())
                .ok_or_else(|| DbErr::Custom("catalog parent missing".into()))?;
            update(
                db,
                "provider_model_templates",
                &json!({"provider_template_id":row["provider_template_id"],"id":row["id"]}),
                transform::model(row, parent),
            )
            .await?;
        }
        for row in configured_providers {
            update_configured(
                db,
                "providers",
                &row,
                json!({"connection":transform::configured_connection(&row["connection"])}),
            )
            .await?;
        }
        for row in configured_models {
            update_configured(db,"provider_models",&row,json!({"connection":transform::configured_connection(&row["connection"]),"thinking":transform::configured_thinking(&row["thinking"]),"pricing":transform::configured_pricing(&row["pricing"])})).await?;
        }
        db.execute_unprepared(include_str!("constraints.sql"))
            .await?;
        db.execute_unprepared(include_str!("views.sql")).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("LOCK TABLE providers,provider_models,provider_templates,provider_model_templates,provider_template_research,provider_model_template_research IN ACCESS EXCLUSIVE MODE").await?;
        let providers = rows(db, "provider_templates").await?;
        let models = rows(db, "provider_model_templates").await?;
        let provider_research = rows(db, "provider_template_research").await?;
        let model_research = rows(db, "provider_model_template_research").await?;
        let snapshots: BTreeMap<_, _> = provider_research
            .iter()
            .map(|r| (r["id"].as_str().unwrap(), &r["configuration_snapshot"]))
            .collect();
        let model_snapshots: BTreeMap<_, _> = model_research
            .iter()
            .map(|r| (r["id"].as_str().unwrap(), &r["configuration_snapshot"]))
            .collect();
        for row in &providers {
            let original = snapshots
                .get(row["research_id"].as_str().unwrap())
                .ok_or_else(rollback_error)?;
            ensure_fields(row, &transform::provider(original))?;
            if row["logo_url"] == Value::Null {
                return Err(rollback_error());
            }
        }
        for row in &models {
            let original = model_snapshots
                .get(row["research_id"].as_str().unwrap())
                .ok_or_else(rollback_error)?;
            let parent = providers
                .iter()
                .find(|p| p["id"] == row["provider_template_id"])
                .and_then(|p| snapshots.get(p["research_id"].as_str().unwrap()))
                .ok_or_else(rollback_error)?;
            ensure_fields(row, &transform::model(original, parent))?;
            if row["body_defaults"] != json!({}) {
                return Err(rollback_error());
            }
        }
        if provider_research.len() != providers.len() || model_research.len() != models.len() {
            return Err(rollback_error());
        }
        let configured_providers = rows(db, "providers").await?;
        let configured_models = rows(db, "provider_models").await?;
        let mut writes = Vec::new();
        for row in &configured_providers {
            for field in ["description", "logo_url", "template_id"] {
                if row[field] != Value::Null {
                    return Err(rollback_error());
                }
            }
            let connection = transform::legacy_connection(&row["connection"]);
            if transform::configured_connection(&connection) != row["connection"] {
                return Err(rollback_error());
            }
            writes.push(("providers", row, json!({"connection":connection})));
        }
        for row in &configured_models {
            for field in ["display_name", "description", "template", "route_selection"] {
                if row[field] != Value::Null {
                    return Err(rollback_error());
                }
            }
            if row["body_defaults"] != json!({}) {
                return Err(rollback_error());
            }
            let connection = transform::legacy_connection(&row["connection"]);
            let thinking = transform::legacy_thinking(&row["thinking"]);
            let pricing = transform::legacy_pricing(&row["pricing"]);
            if transform::configured_connection(&connection) != row["connection"]
                || transform::configured_thinking(&thinking) != row["thinking"]
                || transform::configured_pricing(&pricing) != row["pricing"]
            {
                return Err(rollback_error());
            }
            writes.push((
                "provider_models",
                row,
                json!({"connection":connection,"thinking":thinking,"pricing":pricing}),
            ));
        }
        for (table, row, fields) in writes {
            update_configured(db, table, row, fields).await?;
        }
        db.execute_unprepared(include_str!("down.sql")).await?;
        db.execute_unprepared(include_str!(
            "../m20261120_000049_catalog_research/views.sql"
        ))
        .await?;
        let validation = include_str!("../m20261120_000049_catalog_research/validation.sql")
            .replace("CREATE FUNCTION", "CREATE OR REPLACE FUNCTION");
        db.execute_unprepared(&validation).await?;
        Ok(())
    }
}
async fn update_configured<C: ConnectionTrait>(
    db: &C,
    table: &str,
    row: &Value,
    fields: Value,
) -> Result<(), DbErr> {
    use sea_orm::sea_query::{Alias, Expr, Query, Value as SqlValue};
    let mut query = Query::update();
    query.table(Alias::new(table));
    for (field, value) in fields.as_object().unwrap() {
        let value: SqlValue = if value.is_null() {
            Option::<Value>::None.into()
        } else {
            value.clone().into()
        };
        query.value(Alias::new(field), value);
    }
    query.and_where(Expr::col(Alias::new("id")).eq(row["id"].as_i64().unwrap()));
    db.execute(&query.to_owned()).await?;
    Ok(())
}

fn rollback_error() -> DbErr {
    DbErr::Custom("rollback would discard configuration or research; restore migration-compatible values first".into())
}
fn ensure_fields(row: &Value, expected: &Value) -> Result<(), DbErr> {
    if expected
        .as_object()
        .unwrap()
        .iter()
        .any(|(field, value)| row[field] != *value)
    {
        return Err(rollback_error());
    }
    Ok(())
}
