use compallm_core::provider_template::{ModelTemplate, ProviderTemplate};
use compallm_postgres::{Migrator, ProviderModelTemplateSchema, ProviderTemplateSchema};
use sea_orm::{ConnectionTrait, Database, EntityTrait, ModelTrait, TransactionTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};

type Providers = <ProviderTemplateSchema as ModelTrait>::Entity;
type Models = <ProviderModelTemplateSchema as ModelTrait>::Entity;

#[tokio::test]
#[ignore = "requires CATALOG_TYPES_TEST_URL in an isolated database with migrations applied"]
async fn typed_catalog_round_trips_and_preserves_catalog_facts() -> anyhow::Result<()> {
    let db = Database::connect(std::env::var("CATALOG_TYPES_TEST_URL")?).await?;
    let tx = db.begin().await?;
    let migrations = Migrator::migrations();
    let migration = migrations
        .iter()
        .find(|v| v.name().ends_with("catalog_types"))
        .unwrap();
    migration.down(&SchemaManager::new(&tx)).await?;
    let before = snapshot(&tx).await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    let providers = Providers::find().all(&tx).await?;
    let models = Models::find().all(&tx).await?;
    assert_eq!(providers.len(), 145);
    assert_eq!(models.len(), 2348);
    for mut row in providers {
        row.source_refs = serde_json::json!("human-reviewed provenance");
        row.field_metadata = serde_json::json!("human-reviewed notes");
        row.model_info_sources = serde_json::json!("human-reviewed sources");
        let value = ProviderTemplate::try_from(row)?;
        let encoded = serde_json::to_value(&value)?;
        assert_eq!(
            encoded,
            serde_json::to_value(serde_json::from_value::<ProviderTemplate>(encoded.clone())?)?
        );
    }
    for mut row in models {
        row.source_refs = serde_json::json!("human-reviewed provenance");
        row.field_metadata = serde_json::json!("human-reviewed notes");
        row.model_info_sources = serde_json::json!("human-reviewed sources");
        row.pricing_config = serde_json::json!("human-reviewed price evidence");
        row.resolution_plan = serde_json::json!("human-reviewed acquisition plan");
        let value = ModelTemplate::try_from(row)?;
        let encoded = serde_json::to_value(&value)?;
        assert_eq!(
            encoded,
            serde_json::to_value(serde_json::from_value::<ModelTemplate>(encoded.clone())?)?
        );
    }
    migration.down(&SchemaManager::new(&tx)).await?;
    assert_eq!(before, snapshot(&tx).await?);
    tx.rollback().await?;
    Ok(())
}
async fn snapshot<C: ConnectionTrait>(db: &C) -> anyhow::Result<serde_json::Value> {
    use sea_orm::sea_query::{Alias, Asterisk, Expr, Func, Query};
    let mut snapshot = serde_json::Map::new();
    for table in [
        "provider_templates",
        "provider_model_templates",
        "provider_catalog_readiness",
        "provider_catalog_auth_readiness",
    ] {
        let query = Query::select()
            .expr_as(
                Func::cust(Alias::new("to_jsonb")).arg(Expr::col((Alias::new(table), Asterisk))),
                Alias::new("row"),
            )
            .from(Alias::new(table))
            .to_owned();
        let mut rows = db
            .query_all(&query)
            .await?
            .into_iter()
            .map(|r| r.try_get::<serde_json::Value>("", "row"))
            .collect::<Result<Vec<_>, _>>()?;
        rows.sort_by_key(serde_json::Value::to_string);
        snapshot.insert(table.into(), serde_json::Value::Array(rows));
    }
    Ok(serde_json::Value::Object(snapshot))
}
