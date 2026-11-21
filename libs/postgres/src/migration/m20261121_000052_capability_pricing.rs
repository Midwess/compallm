use sea_orm::{
    ConnectionTrait, DbErr, ExprTrait,
    sea_query::{Alias, Asterisk, Expr, Func, Query},
};
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};
#[path = "m20261121_000052_capability_pricing/transform.rs"]
mod transform;

#[derive(DeriveMigrationName)]
pub struct Migration;

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
fn key(table: &str, row: &Value) -> Value {
    if table == "provider_model_templates" {
        json!({"provider_template_id":row["provider_template_id"],"id":row["id"]})
    } else {
        json!({"id":row["id"]})
    }
}
async fn update<C: ConnectionTrait>(
    db: &C,
    table: &str,
    key: &Value,
    fields: &Value,
) -> Result<(), DbErr> {
    let mut q = Query::update();
    q.table(Alias::new(table));
    for (name, value) in fields.as_object().unwrap() {
        q.value(
            Alias::new(name),
            if value.is_null() {
                sea_orm::Value::Json(None)
            } else {
                value.clone().into()
            },
        );
    }
    for (name, value) in key.as_object().unwrap() {
        q.and_where(
            Expr::col(Alias::new(name)).eq(if let Some(s) = value.as_str() {
                sea_orm::Value::from(s)
            } else {
                sea_orm::Value::from(value.as_i64().unwrap())
            }),
        );
    }
    db.execute(&q.to_owned()).await?;
    Ok(())
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("LOCK TABLE models,provider_models,provider_model_templates,provider_model_template_research IN ACCESS EXCLUSIVE MODE").await?;

        let mut changes = Vec::new();
        for table in ["models", "provider_models", "provider_model_templates"] {
            for row in rows(db, table).await? {
                let after = transform::convert(&row);
                if let Some(error) = after.get("allocation_error") {
                    return Err(DbErr::Custom(format!(
                        "Cannot allocate pricing for {table} {}: {error}",
                        row["id"]
                    )));
                }
                let mut original = serde_json::Map::new();
                for field in transform::FIELDS {
                    if field != "billing" && (field != "route_selection" || table != "models") {
                        original.insert(field.into(), row[field].clone());
                    }
                }
                original.insert("pricing".into(), row["pricing"].clone());
                original.insert("thinking".into(), row["thinking"].clone());
                changes.push((table, key(table, &row), Value::Object(original), after));
            }
        }
        db.execute_unprepared(include_str!("m20261121_000052_capability_pricing/up.sql"))
            .await?;
        for (table, key, before, after) in changes {
            db.execute(
                &Query::insert()
                    .into_table(Alias::new("capability_pricing_rollback"))
                    .columns(["table_name", "row_key", "original", "migrated"].map(Alias::new))
                    .values_panic([
                        table.into(),
                        key.clone().into(),
                        before.into(),
                        after.clone().into(),
                    ])
                    .to_owned(),
            )
            .await?;
            let mut fields = after.clone();
            if table == "models" {
                fields.as_object_mut().unwrap().remove("route_selection");
            }
            update(db, table, &key, &fields).await?;
        }
        db.execute_unprepared(include_str!(
            "m20261121_000052_capability_pricing/constraints.sql"
        ))
        .await?;
        db.execute_unprepared(include_str!(
            "m20261121_000052_capability_pricing/views.sql"
        ))
        .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("LOCK TABLE models,provider_models,provider_model_templates,capability_pricing_rollback IN ACCESS EXCLUSIVE MODE").await?;
        let backups = rows(db, "capability_pricing_rollback").await?;
        let mut restores = Vec::new();
        for table in ["models", "provider_models", "provider_model_templates"] {
            for row in rows(db, table).await? {
                let row_key = key(table, &row);
                let backup = backups
                    .iter()
                    .find(|b| b["table_name"] == table && b["row_key"] == row_key)
                    .ok_or_else(|| {
                        DbErr::Custom("rollback cannot preserve new capability records".into())
                    })?;
                if transform::FIELDS
                    .iter()
                    .any(|name| row[*name] != backup["migrated"][*name])
                {
                    return Err(DbErr::Custom(
                        "rollback would discard capability changes".into(),
                    ));
                }
                restores.push((table, row_key, backup["original"].clone()));
            }
        }
        db.execute_unprepared(include_str!(
            "m20261121_000052_capability_pricing/restore.sql"
        ))
        .await?;
        for (table, key, fields) in restores {
            update(db, table, &key, &fields).await?;
        }
        db.execute_unprepared(include_str!("m20261121_000052_capability_pricing/down.sql"))
            .await?;
        db.execute_unprepared("DROP FUNCTION capability_text_pricing_complete(jsonb,jsonb); DROP FUNCTION capability_usage_complete(jsonb,boolean)")
            .await?;

        db.execute_unprepared(
            include_str!("m20261121_000051_capability_columns/constraints.sql")
                .replace("CREATE FUNCTION", "CREATE OR REPLACE FUNCTION")
                .as_str(),
        )
        .await?;
        db.execute_unprepared(include_str!(
            "m20261121_000050_shared_provider_values/views.sql"
        ))
        .await?;
        Ok(())
    }
}
