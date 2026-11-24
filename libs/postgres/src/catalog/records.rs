use sea_orm::{
    ConnectionTrait, DbErr, ExprTrait,
    sea_query::{Alias, Asterisk, Expr, Func, Query},
};
use serde_json::Value;

pub(crate) async fn rows<C: ConnectionTrait>(
    db: &C,
    table: &str,
    filter: Option<Expr>,
) -> Result<Vec<Value>, DbErr> {
    let mut query = Query::select();
    let mut record: Expr = Func::cust(Alias::new("to_jsonb"))
        .arg(Expr::col((Alias::new(table), Asterisk)))
        .into();
    let decimals: &[&str] = match table {
        "catalog_price_rate" => &["amount", "credits", "quantity", "from_tokens", "to_tokens"],
        "catalog_price_billing" => &["fee_amount", "amount_per_credit"],
        "catalog_price_allowance" => &["quota"],
        _ => &[],
    };
    for field in decimals {
        record = record.binary(
            sea_orm::sea_query::extension::postgres::PgBinOper::Concatenate,
            Func::cust(Alias::new("jsonb_build_object"))
                .arg(Expr::val(*field))
                .arg(
                    Expr::col((Alias::new(table), Alias::new(*field))).cast_as(Alias::new("text")),
                ),
        );
    }
    query
        .expr_as(record, Alias::new("row"))
        .from(Alias::new(table));
    if let Some(filter) = filter {
        query.and_where(filter);
    }
    db.query_all(&query.to_owned())
        .await?
        .into_iter()
        .map(|row| row.try_get("", "row"))
        .collect()
}
pub(crate) fn cell(field: &str, value: &Value) -> Result<Expr, DbErr> {
    if [
        "connections",
        "headers",
        "body_defaults",
        "route_selection",
        "custom_fields",
        "source_refs",
        "source_records",
        "acquisition_recipes",
        "observations",
        "original",
        "applied",
    ]
    .contains(&field)
    {
        return Ok(Expr::val(if value.is_null() {
            None::<Value>
        } else {
            Some(value.clone())
        }));
    }
    if field.ends_with("_tokens") && !["from_tokens", "to_tokens"].contains(&field)
        || field.starts_with("max_")
        || field.starts_with("thinking_budget_")
        || [
            "context_tokens",
            "from_hour",
            "to_hour",
            "position",
            "priority",
            "integer_value",
        ]
        .contains(&field)
    {
        return Ok(Expr::val(value.as_i64()).cast_as(Alias::new("bigint")));
    }
    if ["organization_id", "user_id"].contains(&field) {
        return Ok(Expr::val(value.as_i64()));
    }
    if field == "audio" || field.ends_with("_formats_known") {
        return Ok(Expr::val(value.as_bool()));
    }
    let v: sea_orm::Value = if field == "row_id"
        || field == "model_id"
        || field == "rule_id"
        || field == "rate_id"
        || field == "price_book_id"
        || field == "component_id"
        || field == "parent_id"
        || field == "parent_rate_id"
        || field == "base_row_id"
        || field == "id"
            && value
                .as_str()
                .is_some_and(|s| uuid::Uuid::parse_str(s).is_ok())
    {
        value
            .as_str()
            .map(|v| {
                v.parse::<uuid::Uuid>()
                    .map_err(|e| DbErr::Custom(e.to_string()))
            })
            .transpose()?
            .into()
    } else if [
        "amount",
        "credits",
        "quantity",
        "quota",
        "fee_amount",
        "amount_per_credit",
        "from_tokens",
        "to_tokens",
    ]
    .contains(&field)
    {
        return Ok(Expr::val(
            value
                .as_str()
                .map(str::to_owned)
                .or_else(|| value.as_u64().map(|v| v.to_string())),
        )
        .cast_as(Alias::new("numeric")));
    } else {
        match value {
            Value::Null => sea_orm::Value::String(None),
            Value::Bool(v) => (*v).into(),
            Value::Number(v) => v.as_i64().into(),
            Value::String(v) => v.clone().into(),
            _ => value.clone().into(),
        }
    };
    Ok(Expr::val(v))
}
pub(crate) async fn insert<C: ConnectionTrait>(
    db: &C,
    table: &str,
    row: &Value,
) -> Result<(), DbErr> {
    let object = row
        .as_object()
        .ok_or_else(|| DbErr::Custom("invalid catalog record".into()))?;
    let mut q = Query::insert();
    q.into_table(Alias::new(table))
        .columns(object.keys().map(Alias::new));
    q.values_panic(
        object
            .iter()
            .map(|(key, value)| {
                if key == "id"
                    && [
                        "catalog_model_book",
                        "catalog_provider_book",
                        "catalog_source",
                    ]
                    .contains(&table)
                {
                    Ok(Expr::val(value.as_str().map(str::to_owned)))
                } else {
                    cell(key, value)
                }
            })
            .collect::<Result<Vec<_>, _>>()?,
    );
    db.execute(&q.to_owned()).await?;
    Ok(())
}
pub(crate) async fn delete<C: ConnectionTrait>(
    db: &C,
    table: &str,
    filter: Expr,
) -> Result<(), DbErr> {
    db.execute(
        &Query::delete()
            .from_table(Alias::new(table))
            .and_where(filter)
            .to_owned(),
    )
    .await?;
    Ok(())
}
