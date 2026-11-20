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
    let research = migrations
        .iter()
        .find(|v| v.name().ends_with("catalog_research"))
        .unwrap();
    research.down(&SchemaManager::new(&tx)).await?;
    migration.down(&SchemaManager::new(&tx)).await?;
    let before = snapshot(&tx).await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    research.up(&SchemaManager::new(&tx)).await?;
    let providers = Providers::find().all(&tx).await?;
    let models = Models::find().all(&tx).await?;
    assert_eq!(providers.len(), 145);
    assert_eq!(models.len(), 2348);
    for row in providers {
        let value = ProviderTemplate::try_from(row)?;
        let encoded = serde_json::to_value(&value)?;
        assert_eq!(
            encoded,
            serde_json::to_value(serde_json::from_value::<ProviderTemplate>(encoded.clone())?)?
        );
    }
    for row in models {
        let value = ModelTemplate::try_from(row)?;
        let encoded = serde_json::to_value(&value)?;
        assert_eq!(
            encoded,
            serde_json::to_value(serde_json::from_value::<ModelTemplate>(encoded.clone())?)?
        );
    }
    research.down(&SchemaManager::new(&tx)).await?;
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

#[tokio::test]
#[ignore = "requires CATALOG_TYPES_TEST_URL in an isolated database with migrations applied"]
async fn linked_research_updates_eligibility_and_cannot_be_removed_independently()
-> anyhow::Result<()> {
    use sea_orm::{ColumnTrait, QueryFilter, sea_query::Expr};
    type Research = <compallm_postgres::ProviderModelTemplateResearchSchema as ModelTrait>::Entity;
    type Column = <Research as EntityTrait>::Column;
    type Eligibility =
        <compallm_postgres::ProviderCatalogAuthReadinessSchema as ModelTrait>::Entity;
    type EligibilityColumn = <Eligibility as EntityTrait>::Column;
    let db = Database::connect(std::env::var("CATALOG_TYPES_TEST_URL")?).await?;
    let candidate = Eligibility::find()
        .filter(EligibilityColumn::Readiness.eq(compallm_postgres::CatalogReadinessSchema::Ready))
        .filter(EligibilityColumn::AuthMethod.eq("api_key"))
        .one(&db)
        .await?
        .unwrap();
    let tx = db.begin().await?;
    Research::update_many().col_expr(Column::UnresolvedFields,Expr::value(serde_json::json!({"execution":{"status":"unknown","reason":"Research requires further review"}}))).filter(Column::ProviderTemplateId.eq(&candidate.provider_template_id)).filter(Column::ModelTemplateId.eq(&candidate.id)).exec(&tx).await?;
    tx.execute_unprepared("SET CONSTRAINTS ALL IMMEDIATE")
        .await?;
    let changed = Eligibility::find_by_id((
        candidate.provider_template_id.clone(),
        candidate.id.clone(),
        "api_key".into(),
    ))
    .one(&tx)
    .await?
    .unwrap();
    assert_eq!(
        changed.readiness,
        compallm_postgres::CatalogReadinessSchema::Incomplete
    );
    tx.rollback().await?;
    let tx = db.begin().await?;
    Research::update_many()
        .col_expr(
            Column::State,
            Expr::value(compallm_postgres::CatalogResearchProgressSchema::InProgress),
        )
        .filter(Column::ProviderTemplateId.eq(&candidate.provider_template_id))
        .filter(Column::ModelTemplateId.eq(&candidate.id))
        .exec(&tx)
        .await?;
    tx.execute_unprepared("SET CONSTRAINTS ALL IMMEDIATE")
        .await?;
    let unchanged = Eligibility::find_by_id((
        candidate.provider_template_id.clone(),
        candidate.id.clone(),
        "api_key".into(),
    ))
    .one(&tx)
    .await?
    .unwrap();
    assert_eq!(
        unchanged.readiness,
        compallm_postgres::CatalogReadinessSchema::Ready
    );
    tx.rollback().await?;
    let tx = db.begin().await?;
    Research::delete_by_id((candidate.provider_template_id.clone(), candidate.id.clone()))
        .exec(&tx)
        .await?;
    assert!(
        tx.execute_unprepared("SET CONSTRAINTS ALL IMMEDIATE")
            .await
            .is_err()
    );
    tx.rollback().await?;
    Ok(())
}
