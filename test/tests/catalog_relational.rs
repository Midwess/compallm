use compallm_core::persistent::DbContext;
use compallm_postgres::{
    Migrator, catalog::repository::CatalogRepository, postgres_connection::PostgresConnection,
};
use sea_orm::{
    ConnectionTrait, Database, EntityTrait, ModelTrait, TransactionTrait,
    sea_query::{Alias, Asterisk, Expr, Func, Query},
};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::Value;
use std::sync::Arc;
async fn rows<C: ConnectionTrait>(db: &C, table: &str) -> anyhow::Result<Vec<Value>> {
    let mut rows = db
        .query_all(
            &Query::select()
                .expr_as(
                    Func::cust(Alias::new("to_jsonb"))
                        .arg(Expr::col((Alias::new(table), Asterisk))),
                    Alias::new("row"),
                )
                .from(Alias::new(table))
                .to_owned(),
        )
        .await?
        .into_iter()
        .map(|v| v.try_get::<Value>("", "row"))
        .collect::<Result<Vec<_>, _>>()?;
    rows.sort_by_key(Value::to_string);
    Ok(rows)
}
fn normalize(value: &mut Value) {
    match value {
        Value::Object(v) => {
            for (key, value) in v.iter_mut() {
                if ["amount", "credits", "amountPerCredit"].contains(&key.as_str()) {
                    if let Some(v) = value.as_str()
                        && v.contains('.')
                    {
                        *value =
                            Value::String(v.trim_end_matches('0').trim_end_matches('.').to_owned());
                    }
                } else {
                    normalize(value);
                }
            }
        }
        Value::Array(v) => {
            for value in v {
                normalize(value);
            }
        }
        _ => {}
    }
}
#[tokio::test]
#[ignore = "requires CATALOG_RELATIONAL_TEST_URL with the relational catalog applied"]
async fn catalog_books_reassemble_prices_and_keep_missing_values_readable() -> anyhow::Result<()> {
    let pool = Arc::new(Database::connect(std::env::var("CATALOG_RELATIONAL_TEST_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    root.scope(async {
        let archived = rows(pool.as_ref(), "catalog_schema_rollback")
            .await?
            .into_iter()
            .filter(|r| {
                r["table_name"] == "provider_model_templates"
                    && r["original"]["user_id"] == Value::Null
            })
            .map(|r| {
                (
                    (
                        r["original"]["provider_template_id"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        r["original"]["id"].as_str().unwrap().to_owned(),
                    ),
                    r["original"].clone(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();
        let catalog = CatalogRepository;
        for provider in [
            "openai",
            "openrouter",
            "siliconflow",
            "nvidia",
            "minimax",
            "synthetic",
            "anthropic",
            "deepinfra",
        ] {
            let models = catalog
                .models(provider, Box::new(compallm_query::Query::default()), None)
                .await?;
            assert!(!models.is_empty(), "{provider}");
            for model in models {
                let mut encoded = serde_json::to_value(&model)?;
                normalize(&mut encoded);
                let original = &archived[&(model.provider_catalog_id.clone(), model.id.clone())];
                for (actual, prior) in [
                    ("/text/input/pricing", "/text/input/pricing"),
                    ("/text/output/pricing", "/text/output/pricing"),
                    ("/vision/pricing", "/vision/pricing"),
                    ("/audio/input/pricing", "/audio/input/pricing"),
                    ("/audio/output/pricing", "/audio/output/pricing"),
                    ("/imageGeneration/pricing", "/image_generation/pricing"),
                    ("/video/pricing", "/video/pricing"),
                ] {
                    let mut expected = original.pointer(prior).cloned().unwrap_or(Value::Null);
                    normalize(&mut expected);
                    if expected == Value::Null {
                        continue;
                    }
                    assert_eq!(
                        encoded.pointer(actual).cloned().unwrap_or(Value::Null),
                        expected,
                        "{} {} {actual}",
                        model.provider_catalog_id,
                        model.id
                    );
                }
            }
        }
        let source = catalog
            .model("deepinfra", "meta-llama/Meta-Llama-3.1-8B-Instruct", None)
            .await?;
        if let Some(model) = source {
            assert!(model.limits.context_tokens.is_none());
        }
        let known = catalog.model("openai", "gpt-4o", None).await?.unwrap();
        assert!(known.text.input.pricing().is_some());
        Ok::<_, anyhow::Error>(())
    })
    .await?;
    type Models = <compallm_postgres::CatalogModelRecordSchema as ModelTrait>::Entity;
    for row in Models::find().all(pool.as_ref()).await? {
        let _ = compallm_core::model::ModelCatalog::try_from(row)?;
    }
    Ok(())
}
#[tokio::test]
#[ignore = "requires CATALOG_RELATIONAL_TEST_URL in an isolated database"]
async fn relational_catalog_rolls_back_without_losing_imported_rows() -> anyhow::Result<()> {
    let pool = Database::connect(std::env::var("CATALOG_RELATIONAL_TEST_URL")?).await?;
    let tx = pool.begin().await?;
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name().ends_with("catalog_relational"))
        .unwrap();
    let before = rows(&tx, "catalog_model_book").await?;
    migration.down(&SchemaManager::new(&tx)).await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    assert_eq!(before, rows(&tx, "catalog_model_book").await?);
    tx.rollback().await?;
    Ok(())
}
