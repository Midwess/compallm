use sea_orm::{
    ConnectionTrait, DbErr, ExprTrait,
    sea_query::{Alias, Asterisk, Expr, Func, Query},
};
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};
#[path = "m20261121_000051_capability_columns/transform.rs"]
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
        let research = rows(db, "provider_model_template_research").await?;
        let mut changes = Vec::new();
        for table in ["models", "provider_models", "provider_model_templates"] {
            for row in rows(db, table).await? {
                let observations = research
                    .iter()
                    .find(|r| r["id"] == row["research_id"])
                    .map(|r| &r["configuration_snapshot"]["capabilities"])
                    .unwrap_or(&Value::Null);
                let after = transform::convert(&row, observations);
                changes.push((table,key(table,&row),json!({"capabilities":row["capabilities"],"thinking":row["thinking"],"limits":row["limits"]}),after));
            }
        }
        db.execute_unprepared(include_str!("m20261121_000051_capability_columns/up.sql"))
            .await?;
        for (table, key, before, after) in changes {
            db.execute(
                &Query::insert()
                    .into_table(Alias::new("capability_columns_rollback"))
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
            update(db, table, &key, &after).await?;
        }
        db.execute_unprepared(include_str!(
            "m20261121_000051_capability_columns/constraints.sql"
        ))
        .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("LOCK TABLE models,provider_models,provider_model_templates,capability_columns_rollback IN ACCESS EXCLUSIVE MODE").await?;
        let backups = rows(db, "capability_columns_rollback").await?;
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
                if transform::COLUMNS
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
            "m20261121_000051_capability_columns/restore.sql"
        ))
        .await?;
        for (table, key, fields) in restores {
            update(db, table, &key, &fields).await?;
        }
        db.execute_unprepared(include_str!("m20261121_000051_capability_columns/down.sql"))
            .await?;
        Ok(())
    }
}
