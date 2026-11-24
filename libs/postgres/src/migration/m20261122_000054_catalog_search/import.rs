use sea_orm::sea_query::{Alias, Asterisk, Expr, Func, Query};
use sea_orm::{ConnectionTrait, DbErr, ExprTrait};
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};

const JOURNAL: &str = "catalog_import_rollback";
pub const DATA: &str = include_str!("import.json");

async fn rows<C: ConnectionTrait>(db: &C, table: &str) -> Result<Vec<Value>, DbErr> {
    db.query_all(
        &Query::select()
            .expr_as(
                Func::cust(Alias::new("to_jsonb")).arg(Expr::col((Alias::new(table), Asterisk))),
                Alias::new("row"),
            )
            .from(Alias::new(table))
            .to_owned(),
    )
    .await?
    .into_iter()
    .map(|r| r.try_get("", "row"))
    .collect()
}
fn cell(field: &str, value: &Value) -> Result<sea_orm::sea_query::Expr, DbErr> {
    let raw: sea_orm::Value = match field {
        "row_id" | "base_row_id" => match value.as_str() {
            Some(v) => v
                .parse::<uuid::Uuid>()
                .map_err(|e| DbErr::Custom(e.to_string()))?
                .into(),
            None => Option::<uuid::Uuid>::None.into(),
        },
        "organization_id" | "user_id" => value.as_i64().into(),
        "table_name"
        | "id"
        | "key"
        | "name"
        | "provider_template_id"
        | "model_template_id"
        | "research_id"
        | "display_name"
        | "description"
        | "logo_url"
        | "service_kind"
        | "state"
        | "specification_mode" => value.as_str().map(str::to_owned).into(),
        _ => {
            if value.is_null() {
                sea_orm::Value::Json(None)
            } else {
                value.clone().into()
            }
        }
    };
    let expr = Expr::val(raw);
    Ok(match field {
        "state" => expr.cast_as(Alias::new("catalog_research_progress")),
        "specification_mode" => expr.cast_as(Alias::new("catalog_specification_mode")),
        _ => expr,
    })
}
fn identity(table: &str, row: &Value) -> Value {
    if table == "provider_model_templates" {
        row["row_id"].clone()
    } else {
        row["id"].clone()
    }
}
async fn insert<C: ConnectionTrait>(db: &C, table: &str, row: &Value) -> Result<(), DbErr> {
    let object = row
        .as_object()
        .ok_or_else(|| DbErr::Custom("invalid catalog import row".into()))?;
    let mut q = Query::insert();
    q.into_table(Alias::new(table))
        .columns(object.keys().map(Alias::new));
    q.values_panic(
        object
            .iter()
            .map(|(k, v)| {
                if table == JOURNAL && k != "table_name" {
                    Ok(Expr::val(v.clone()))
                } else {
                    cell(k, v)
                }
            })
            .collect::<Result<Vec<_>, _>>()?,
    );
    db.execute(&q.to_owned()).await?;
    Ok(())
}
pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let data: Value = serde_json::from_str(DATA).map_err(|e| DbErr::Custom(e.to_string()))?;
    let db = manager.get_connection();
    db.execute_unprepared("LOCK TABLE provider_templates,provider_model_templates,provider_template_research,provider_model_template_research IN ACCESS EXCLUSIVE MODE").await?;
    manager
        .create_table(
            Table::create()
                .table(Alias::new(JOURNAL))
                .col(ColumnDef::new(Alias::new("table_name")).text().not_null())
                .col(
                    ColumnDef::new(Alias::new("row_key"))
                        .json_binary()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Alias::new("original"))
                        .json_binary()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Alias::new("applied"))
                        .json_binary()
                        .not_null(),
                )
                .primary_key(
                    Index::create()
                        .col(Alias::new("table_name"))
                        .col(Alias::new("row_key")),
                )
                .to_owned(),
        )
        .await?;
    let mut originals = std::collections::BTreeMap::new();
    for table in [
        "provider_template_research",
        "provider_model_template_research",
    ] {
        for row in rows(db, table).await? {
            originals.insert(
                (table.to_owned(), row["id"].as_str().unwrap().to_owned()),
                row,
            );
        }
    }
    for change in data["changes"].as_array().unwrap() {
        let table = change["table"].as_str().unwrap();
        let (original, applied) = if let Some(key) = change.get("key") {
            let original = originals
                .get(&(table.to_owned(), key.as_str().unwrap().to_owned()))
                .cloned()
                .ok_or_else(|| DbErr::Custom("catalog import baseline is missing".into()))?;
            let mut applied = original.clone();
            if applied["metadata"].get("catalog_import").is_some() {
                return Err(DbErr::Custom(
                    "catalog import would overwrite research".into(),
                ));
            }
            applied["metadata"]["catalog_import"] = change["supplement"].clone();
            db.execute(
                &Query::update()
                    .table(Alias::new(table))
                    .value(Alias::new("metadata"), applied["metadata"].clone())
                    .and_where(Expr::col(Alias::new("id")).eq(key.as_str().unwrap()))
                    .to_owned(),
            )
            .await?;
            (original, applied)
        } else {
            let applied = change["row"].clone();
            check_configuration(table, &applied)?;
            insert(db, table, &applied).await?;
            (Value::Null, applied)
        };
        insert(db,JOURNAL,&json!({"table_name":table,"row_key":identity(table,&applied),"original":original,"applied":applied})).await?;
    }
    insert(db, JOURNAL, &json!({"table_name":"manifest","row_key":data["manifest"]["revision"],"original":null,"applied":data["manifest"]})).await?;
    Ok(())
}
pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    if !manager.has_table(JOURNAL).await? {
        return Ok(());
    }
    let db = manager.get_connection();
    db.execute_unprepared("LOCK TABLE provider_templates,provider_model_templates,provider_template_research,provider_model_template_research IN ACCESS EXCLUSIVE MODE").await?;
    let journal = rows(db, JOURNAL).await?;
    for table in [
        "provider_model_templates",
        "provider_model_template_research",
        "provider_templates",
        "provider_template_research",
    ] {
        let actual = rows(db, table)
            .await?
            .into_iter()
            .map(|row| (identity(table, &row).to_string(), row))
            .collect::<std::collections::BTreeMap<_, _>>();
        for item in journal.iter().filter(|v| v["table_name"] == table) {
            if actual.get(&item["row_key"].to_string()) != Some(&item["applied"]) {
                return Err(DbErr::Custom(
                    "rollback would discard imported catalog edits".into(),
                ));
            }
        }
    }
    let removed_providers = journal
        .iter()
        .filter(|item| {
            item["table_name"] == "provider_templates" && item["original"] == Value::Null
        })
        .map(|item| item["applied"]["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let imported_models = journal
        .iter()
        .filter(|item| item["table_name"] == "provider_model_templates")
        .map(|item| item["applied"]["row_id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    if rows(db, "provider_model_templates")
        .await?
        .iter()
        .any(|row| {
            removed_providers.contains(row["provider_template_id"].as_str().unwrap())
                && !imported_models.contains(row["row_id"].as_str().unwrap())
        })
    {
        return Err(DbErr::Custom(
            "rollback would discard models added to an imported provider".into(),
        ));
    }
    for table in [
        "provider_model_templates",
        "provider_model_template_research",
        "provider_templates",
        "provider_template_research",
    ] {
        for item in journal.iter().filter(|v| v["table_name"] == table) {
            let field = if table == "provider_model_templates" {
                "row_id"
            } else {
                "id"
            };
            let key = cell(field, &item["row_key"])?;
            if item["original"] == Value::Null {
                db.execute(
                    &Query::delete()
                        .from_table(Alias::new(table))
                        .and_where(Expr::col(Alias::new(field)).eq(key))
                        .to_owned(),
                )
                .await?;
            } else {
                db.execute(
                    &Query::update()
                        .table(Alias::new(table))
                        .value(Alias::new("metadata"), item["original"]["metadata"].clone())
                        .and_where(Expr::col(Alias::new(field)).eq(key))
                        .to_owned(),
                )
                .await?;
            }
        }
    }
    manager
        .drop_table(Table::drop().table(Alias::new(JOURNAL)).to_owned())
        .await?;
    Ok(())
}

fn check_configuration(table: &str, row: &Value) -> Result<(), DbErr> {
    if matches!(
        table,
        "provider_template_research" | "provider_model_template_research"
    ) {
        crate::catalog::model_info::ModelInfoSources::try_from(row["model_info_sources"].clone())
            .map_err(|error| DbErr::Custom(error.into()))?;
    }
    if !matches!(table, "provider_templates" | "provider_model_templates") {
        return Ok(());
    }
    let mut object = serde_json::Map::new();
    for (key, value) in row.as_object().unwrap() {
        if ["row_id", "organization_id", "user_id", "base_row_id"].contains(&key.as_str()) {
            continue;
        }
        let mut parts = key.split('_');
        let mut name = parts.next().unwrap().to_owned();
        for part in parts {
            let mut chars = part.chars();
            if let Some(c) = chars.next() {
                name.extend(c.to_uppercase());
                name.extend(chars);
            }
        }
        object.insert(
            if name == "providerTemplateId" {
                "providerCatalogId".into()
            } else {
                name
            },
            value.clone(),
        );
    }
    if table == "provider_templates" {
        object.insert("models".into(), json!([]));
        serde_json::from_value::<compallm_core::provider::ProviderCatalog>(Value::Object(object))
            .map(|_| ())
            .map_err(|e| DbErr::Custom(e.to_string()))
    } else {
        serde_json::from_value::<compallm_core::model::ModelCatalog>(Value::Object(object))
            .map(|_| ())
            .map_err(|e| DbErr::Custom(e.to_string()))
    }
}
