use compallm_postgres::Migrator;
use sea_orm::{
    ConnectionTrait, Database, ExprTrait, TransactionTrait,
    sea_query::{Alias, Asterisk, Expr, Func, Query},
};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::{Value, json};

const DATA: &str =
    include_str!("../../libs/postgres/src/migration/m20261122_000054_catalog_search/import.json");
const TABLES: [&str; 4] = [
    "provider_templates",
    "provider_model_templates",
    "provider_template_research",
    "provider_model_template_research",
];
async fn rows<C: ConnectionTrait>(db: &C, table: &str) -> anyhow::Result<Vec<Value>> {
    let mut result = db
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
        .map(|r| r.try_get::<Value>("", "row"))
        .collect::<Result<Vec<_>, _>>()?;
    result.sort_by_key(Value::to_string);
    Ok(result)
}
async fn snapshot(db: &sea_orm::DatabaseTransaction) -> anyhow::Result<Value> {
    let mut object = serde_json::Map::new();
    let relational = SchemaManager::new(db)
        .has_table("catalog_schema_rollback")
        .await?;
    let archived = if relational {
        rows(db, "catalog_schema_rollback").await?
    } else {
        vec![]
    };
    for table in TABLES {
        let mut values = if relational {
            archived
                .iter()
                .filter(|r| r["table_name"] == table)
                .map(|r| r["original"].clone())
                .collect::<Vec<_>>()
        } else {
            rows(db, table).await?
        };
        values.sort_by_key(Value::to_string);
        object.insert(table.into(), json!(values));
    }
    Ok(Value::Object(object))
}
#[tokio::test]
#[ignore = "requires CATALOG_IMPORT_TEST_URL in an isolated database at migration 55"]
async fn catalog_import_preserves_existing_configuration_covers_source_and_reverses()
-> anyhow::Result<()> {
    let pool = Database::connect(std::env::var("CATALOG_IMPORT_TEST_URL")?).await?;
    let tx = pool.begin().await?;
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name().ends_with("catalog_relational"))
        .unwrap();
    migration.down(&SchemaManager::new(&tx)).await?;
    let before = snapshot(&tx).await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    let after = snapshot(&tx).await?;
    for table in ["provider_templates", "provider_model_templates"] {
        let key = if table == "provider_model_templates" {
            "row_id"
        } else {
            "id"
        };
        let confirmed = after[table]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| (row[key].as_str().unwrap(), row))
            .collect::<std::collections::BTreeMap<_, _>>();
        for row in before[table].as_array().unwrap() {
            assert_eq!(
                confirmed.get(row[key].as_str().unwrap()).copied(),
                Some(row),
                "existing configuration changed: {table} {}",
                row["id"]
            );
        }
    }
    let data: Value = serde_json::from_str(DATA)?;
    let providers = after["provider_templates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    for target in data["manifest"]["provider_bindings"]
        .as_object()
        .unwrap()
        .values()
    {
        assert!(providers.contains(target.as_str().unwrap()));
    }
    for review in after["provider_template_research"].as_array().unwrap() {
        if let Some(fields) = review["metadata"]["catalog_import"]["create_fields"].as_object() {
            for (slug, definition) in fields {
                assert_eq!(definition["litellm_provider"], slug.as_str());
            }
        }
    }
    let models = after["provider_model_templates"].as_array().unwrap();
    let research = after["provider_model_template_research"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| (row["id"].as_str().unwrap(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let identities = models
        .iter()
        .filter(|row| row["user_id"] == Value::Null)
        .map(|row| {
            (
                (
                    row["provider_template_id"].as_str().unwrap(),
                    row["id"].as_str().unwrap(),
                ),
                row,
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    for binding in data["manifest"]["bindings"].as_array().unwrap() {
        let identity = (
            binding["provider_template_id"].as_str().unwrap(),
            binding["model_template_id"].as_str().unwrap(),
        );
        let row = identities
            .get(&identity)
            .expect("source model identity was not imported");
        let review = research[row["research_id"].as_str().unwrap()];
        assert!(
            review["metadata"]["catalog_import"]["entries"]
                .get(binding["key"].as_str().unwrap())
                .is_some()
        );
    }
    migration.down(&SchemaManager::new(&tx)).await?;
    assert_eq!(before, snapshot(&tx).await?);
    migration.up(&SchemaManager::new(&tx)).await?;
    assert_eq!(after, snapshot(&tx).await?);
    let edited = tx.begin().await?;
    let id = data["changes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["table"] == "provider_templates")
        .unwrap()["row"]["id"]
        .as_str()
        .unwrap();
    edited
        .execute(
            &Query::update()
                .table(Alias::new("catalog_provider_book"))
                .value(Alias::new("description"), "changed after import")
                .and_where(Expr::col(Alias::new("id")).eq(id))
                .to_owned(),
        )
        .await?;
    assert!(migration.down(&SchemaManager::new(&edited)).await.is_err());
    edited.rollback().await?;
    tx.rollback().await?;
    Ok(())
}
