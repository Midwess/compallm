use compallm_core::{model::ModelTemplate, provider::ProviderTemplate};
use compallm_postgres::{Migrator, ProviderModelTemplateSchema, ProviderTemplateSchema};
use sea_orm::{
    ConnectionTrait, Database, DatabaseTransaction, EntityTrait, ExprTrait, ModelTrait,
    TransactionTrait,
    sea_query::{Alias, Asterisk, Expr, Func, Query},
};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};
use serde_json::{Value, json};

type Providers = <ProviderTemplateSchema as ModelTrait>::Entity;
type Models = <ProviderModelTemplateSchema as ModelTrait>::Entity;

async fn setup() -> anyhow::Result<(DatabaseTransaction, CatalogMigrations)> {
    let db = Database::connect(std::env::var("CATALOG_TYPES_TEST_URL")?).await?;
    let tx = db.begin().await?;
    tx.execute_unprepared("LOCK TABLE provider_templates IN ACCESS EXCLUSIVE MODE")
        .await?;
    let mut migrations = Migrator::migrations();
    let pricing = migrations.remove(
        migrations
            .iter()
            .position(|m| m.name().ends_with("capability_pricing"))
            .unwrap(),
    );
    let has_pricing=tx.query_one(&Query::select().expr(Expr::cust("EXISTS(SELECT 1 FROM information_schema.columns WHERE table_name='provider_model_templates' AND column_name='billing')")).to_owned()).await?.unwrap().try_get_by_index::<bool>(0)?;
    if has_pricing {
        pricing.down(&SchemaManager::new(&tx)).await?;
    }
    let capabilities = migrations.remove(
        migrations
            .iter()
            .position(|m| m.name().ends_with("capability_columns"))
            .unwrap(),
    );
    let shared = migrations.remove(
        migrations
            .iter()
            .position(|m| m.name().ends_with("shared_provider_values"))
            .unwrap(),
    );
    let migration = CatalogMigrations {
        shared,
        capabilities,
        pricing,
    };
    let has_capabilities=tx.query_one(&Query::select().expr(Expr::cust("EXISTS(SELECT 1 FROM information_schema.columns WHERE table_name='provider_model_templates' AND column_name='vision')")).to_owned()).await?.unwrap().try_get_by_index::<bool>(0)?;
    if has_capabilities {
        migration
            .capabilities
            .down(&SchemaManager::new(&tx))
            .await?;
    }
    let columns = tx.query_one(&Query::select().expr(Expr::cust("EXISTS(SELECT 1 FROM information_schema.columns WHERE table_name='provider_templates' AND column_name='research_id')")).to_owned()).await?.unwrap();
    if columns.try_get_by_index::<bool>(0)? {
        migration.shared.down(&SchemaManager::new(&tx)).await?;
    }
    Ok((tx, migration))
}

async fn table_rows<C: ConnectionTrait>(db: &C, table: &str) -> anyhow::Result<Vec<Value>> {
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
        .map(|r| r.try_get::<Value>("", "row"))
        .collect::<Result<Vec<_>, _>>()?;
    rows.sort_by_key(Value::to_string);
    Ok(rows)
}

async fn snapshot<C: ConnectionTrait>(db: &C) -> anyhow::Result<Value> {
    let mut result = serde_json::Map::new();
    for table in [
        "models",
        "providers",
        "provider_models",
        "provider_templates",
        "provider_model_templates",
        "provider_template_research",
        "provider_model_template_research",
        "provider_catalog_readiness",
        "provider_catalog_auth_readiness",
    ] {
        result.insert(table.into(), json!(table_rows(db, table).await?));
    }
    Ok(Value::Object(result))
}

#[tokio::test]
#[ignore = "requires CATALOG_TYPES_TEST_URL in an isolated catalog database at migration 49 through 52"]
async fn shared_catalog_migration_preserves_every_row_and_decodes_shared_entities()
-> anyhow::Result<()> {
    let (tx, migration) = setup().await?;
    seed_configured(&tx).await?;
    let before = snapshot(&tx).await?;
    for _ in 0..2 {
        migration.up(&SchemaManager::new(&tx)).await?;
        let journal = table_rows(&tx, "capability_pricing_rollback").await?;
        for archived in &journal {
            let original = &archived["original"];
            let mut before = Vec::new();
            amounts(&original["pricing"], &mut before);
            amounts(&original["route_selection"], &mut before);
            let mut after = Vec::new();
            amounts(&archived["migrated"], &mut after);
            before.sort();
            after.sort();
            assert_eq!(
                before, after,
                "monetary rates changed for {} {}",
                archived["table_name"], archived["row_key"]
            );
        }
        let providers = Providers::find().all(&tx).await?;
        let models = Models::find().all(&tx).await?;
        assert_eq!(
            providers.len(),
            before["provider_templates"].as_array().unwrap().len()
        );
        assert_eq!(
            models.len(),
            before["provider_model_templates"].as_array().unwrap().len()
        );
        for row in providers {
            let template = ProviderTemplate::try_from(row)?;
            let encoded = serde_json::to_value(&template)?;
            assert_eq!(
                template,
                serde_json::from_value::<ProviderTemplate>(encoded)?
            );
        }
        for row in <<compallm_postgres::ProviderModelSchema as ModelTrait>::Entity>::find()
            .all(&tx)
            .await?
        {
            let _ = compallm_core::provider::ProviderModel::try_from(row)?;
        }
        for row in <<compallm_postgres::ModelSchema as ModelTrait>::Entity>::find()
            .all(&tx)
            .await?
        {
            let value = compallm_core::model::Model::try_from(row)?;
            assert_eq!(value.vision.configuration().unwrap().max_images, Some(8));
            assert!(
                value
                    .text
                    .output
                    .configuration()
                    .is_some_and(|v| v.thinking.is_supported())
            );
        }
        for row in models {
            let id = format!("{} {}", row.provider_template_id, row.id);
            let template =
                ModelTemplate::try_from(row).map_err(|e| anyhow::anyhow!("{id}: {e}"))?;
            let encoded = serde_json::to_value(&template)?;
            assert_eq!(template, serde_json::from_value::<ModelTemplate>(encoded)?);
        }
        for table in [
            "provider_catalog_readiness",
            "provider_catalog_auth_readiness",
        ] {
            let after = table_rows(&tx, table).await?;
            let prior = before[table].as_array().unwrap();
            for old in prior {
                let current = after
                    .iter()
                    .find(|new| {
                        new["id"] == old["id"]
                            && new["provider_template_id"] == old["provider_template_id"]
                            && new.get("auth_method") == old.get("auth_method")
                    })
                    .unwrap();
                assert_eq!(
                    old["readiness"], current["readiness"],
                    "{table}: {old:?} -> {current:?}"
                );
            }
        }
        migration.down(&SchemaManager::new(&tx)).await?;
        assert_eq!(before, snapshot(&tx).await?);
    }
    tx.rollback().await?;
    Ok(())
}

async fn update<C: ConnectionTrait>(
    db: &C,
    table: &str,
    id: &str,
    field: &str,
    value: Value,
) -> Result<(), sea_orm::DbErr> {
    db.execute(
        &Query::update()
            .table(Alias::new(table))
            .value(Alias::new(field), value)
            .and_where(Expr::col(Alias::new("id")).eq(id))
            .to_owned(),
    )
    .await?;
    Ok(())
}

#[tokio::test]
#[ignore = "requires CATALOG_TYPES_TEST_URL in an isolated catalog database at migration 49 through 52"]
async fn shared_catalog_research_links_and_rollback_preserve_later_edits() -> anyhow::Result<()> {
    let (tx, migration) = setup().await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    let provider = Providers::find().one(&tx).await?.unwrap();
    let nested = tx.begin().await?;
    let invalid = Query::update()
        .table(Alias::new("provider_templates"))
        .value(Alias::new("research_id"), "missing-research")
        .and_where(Expr::col(Alias::new("id")).eq(&provider.id))
        .to_owned();
    assert!(nested.execute(&invalid).await.is_err());
    nested.rollback().await?;
    let nested = tx.begin().await?;
    let delete = Query::delete()
        .from_table(Alias::new("provider_template_research"))
        .and_where(Expr::col(Alias::new("id")).eq(&provider.research_id))
        .to_owned();
    assert!(nested.execute(&delete).await.is_err());
    nested.rollback().await?;
    let nested = tx.begin().await?;
    let research_count = table_rows(&nested, "provider_template_research")
        .await?
        .len();
    nested
        .execute(
            &Query::delete()
                .from_table(Alias::new("provider_templates"))
                .and_where(Expr::col(Alias::new("id")).eq(&provider.id))
                .to_owned(),
        )
        .await?;
    assert_eq!(
        research_count,
        table_rows(&nested, "provider_template_research")
            .await?
            .len()
    );
    assert!(migration.down(&SchemaManager::new(&nested)).await.is_err());
    nested.rollback().await?;
    let nested = tx.begin().await?;
    let model = Models::find().one(&nested).await?.unwrap();
    update(
        &nested,
        "provider_model_templates",
        &model.id,
        "body_defaults",
        json!({"temperature":0.7}),
    )
    .await?;
    assert!(migration.down(&SchemaManager::new(&nested)).await.is_err());
    nested.rollback().await?;
    migration.down(&SchemaManager::new(&tx)).await?;
    tx.rollback().await?;
    Ok(())
}

#[tokio::test]
#[ignore = "requires CATALOG_TYPES_TEST_URL in an isolated catalog database at migration 49 through 52"]
async fn shared_catalog_rejects_invalid_capabilities_and_reflects_missing_configuration()
-> anyhow::Result<()> {
    let (tx, migration) = setup().await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    let ready = table_rows(&tx, "provider_catalog_readiness")
        .await?
        .into_iter()
        .find(|r| r["readiness"] == "ready")
        .unwrap();
    let model = Models::find_by_id((
        ready["provider_template_id"].as_str().unwrap().to_owned(),
        ready["id"].as_str().unwrap().to_owned(),
    ))
    .one(&tx)
    .await?
    .unwrap();
    let nested = tx.begin().await?;
    assert!(
        update(
            &nested,
            "provider_model_templates",
            &model.id,
            "vision",
            json!({"status":"imaginary"})
        )
        .await
        .is_err()
    );
    nested.rollback().await?;
    let nested = tx.begin().await?;
    let mut invalid = model.text.clone();
    invalid["input"]["pricing"] = json!({"type":"usage","configuration":{"fixed":{"token":{"rate":{"amount":"-1","currency":"USD","quantity":1000000},"cache":{"type":"included"}}}}});
    assert!(
        update(
            &nested,
            "provider_model_templates",
            &model.id,
            "text",
            invalid
        )
        .await
        .is_err()
    );
    nested.rollback().await?;
    let nested = tx.begin().await?;
    update(&nested,"provider_model_templates",&model.id,"text",json!({"input":{"status":"supported","configuration":{"maxTokens":null},"pricing":null},"output":{"status":"supported","configuration":{"maxTokens":null,"thinking":{"status":"unknown"}},"pricing":null}})).await?;
    let states = table_rows(&nested, "provider_catalog_readiness").await?;
    assert!(
        states
            .iter()
            .filter(
                |r| r["id"] == model.id && r["provider_template_id"] == model.provider_template_id
            )
            .all(|r| r["readiness"] == "incomplete")
    );
    nested.rollback().await?;
    update(
        &tx,
        "provider_model_templates",
        &model.id,
        "connections",
        json!({}),
    )
    .await?;
    let states = table_rows(&tx, "provider_catalog_readiness").await?;
    assert!(
        states
            .iter()
            .filter(|r| r["id"] == model.id)
            .all(|r| r["readiness"] == "incomplete")
    );
    tx.rollback().await?;
    Ok(())
}

async fn seed_configured(db: &DatabaseTransaction) -> anyhow::Result<()> {
    let organization_id = 9_000_000_000_000_001_i64;
    db.execute(
        &Query::insert()
            .into_table(Alias::new("organizations"))
            .columns([Alias::new("id"), Alias::new("name")])
            .values_panic([organization_id.into(), "Migration review".into()])
            .to_owned(),
    )
    .await?;
    let connection = json!({"baseUrl":"https://example.com/v1","timeoutMs":30000,"query":{},"authentication":{"apiKeyHeader":{"name":"Authorization","prefix":"Bearer "}}});
    db.execute(
        &Query::insert()
            .into_table(Alias::new("providers"))
            .columns(["id", "organization_id", "name", "connection", "credentials"].map(Alias::new))
            .values_panic([
                organization_id.into(),
                organization_id.into(),
                "Migration provider".into(),
                connection.clone().into(),
                "opaque-test-credential".into(),
            ])
            .to_owned(),
    )
    .await?;
    let prices = [
        json!({"token":{"inputUsdPerMillionTokens":"0.1","outputUsdPerMillionTokens":"0.25","cachedUsdPerMillionTokens":"0"}}),
        json!({"request":{"usdPerRequest":"0.01"}}),
        json!({"audio":{"usdPerSecond":"0.002"}}),
        json!({"audio":{"usdPerMillionCharacters":"5"}}),
        json!({"image":{"rates":[{"usdPerImage":"0.04","conditions":{"resolution":"1024x1024"}}]}}),
        json!({"video":{"rates":[{"usdPerSecond":"0.15","conditions":{"audio":true}}]}}),
    ];
    for (index, pricing) in prices.into_iter().enumerate() {
        db.execute(
            &Query::insert()
                .into_table(Alias::new("provider_models"))
                .columns(
                    [
                        "id",
                        "organization_id",
                        "provider_id",
                        "name",
                        "service_kind",
                        "protocol",
                        "connection",
                        "pricing",
                        "thinking",
                    ]
                    .map(Alias::new),
                )
                .values_panic([
                    (organization_id + index as i64).into(),
                    organization_id.into(),
                    organization_id.into(),
                    format!("model-{index}").into(),
                    (match index {
                        2 => "stt",
                        3 => "tts",
                        4 => "image",
                        5 => "video",
                        _ => "llm",
                    })
                    .into(),
                    "openai".into(),
                    connection.clone().into(),
                    pricing.into(),
                    json!({"effort":["low","medium","high"],"budget":null}).into(),
                ])
                .to_owned(),
        )
        .await?;
    }
    db.execute(&Query::insert().into_table(Alias::new("models"))
        .columns(["id","organization_id","name","service_kind","capabilities","limits","thinking","pricing"].map(Alias::new))
        .values_panic([organization_id.into(),organization_id.into(),"migration-model".into(),"llm".into(),json!(["text","vision","reasoning"]).into(),json!({"contextTokens":128000,"maxImagesPerRequest":8,"maxAudioBytes":1000000}).into(),json!({"effortMapping":{"low":"minimal"},"budget":null}).into(),json!({"token":{"inputUsdPerMillionTokens":"1","outputUsdPerMillionTokens":"2","cachedUsdPerMillionTokens":"0"}}).into()]).to_owned()).await?;
    Ok(())
}

struct CatalogMigrations {
    shared: Box<dyn MigrationTrait>,
    capabilities: Box<dyn MigrationTrait>,
    pricing: Box<dyn MigrationTrait>,
}
impl CatalogMigrations {
    async fn up(&self, manager: &SchemaManager<'_>) -> Result<(), sea_orm::DbErr> {
        self.shared.up(manager).await?;
        self.capabilities.up(manager).await?;
        self.pricing.up(manager).await
    }
    async fn down(&self, manager: &SchemaManager<'_>) -> Result<(), sea_orm::DbErr> {
        self.pricing.down(manager).await?;
        self.capabilities.down(manager).await?;
        self.shared.down(manager).await
    }
}

fn amounts(value: &Value, result: &mut Vec<String>) {
    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                if [
                    "amount",
                    "credits",
                    "amountPerCredit",
                    "inputUsdPerMillionTokens",
                    "outputUsdPerMillionTokens",
                    "cachedUsdPerMillionTokens",
                    "usdPerRequest",
                    "usdPerSecond",
                    "usdPerMillionCharacters",
                    "usdPerImage",
                ]
                .contains(&key.as_str())
                {
                    if let Some(amount) = value.as_str() {
                        result.push(amount.to_owned());
                    }
                } else {
                    amounts(value, result);
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                amounts(value, result);
            }
        }
        _ => {}
    }
}
