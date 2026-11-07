use compallm_postgres::Migrator;
use compallm_utils::input;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::collections::HashSet;

#[tokio::test]
async fn migration_rows_cover_the_catalog_without_credentials_or_invented_defaults() {
    let migration = Migrator::migrations()
        .into_iter()
        .find(|migration| migration.name() == "m20261106_000018_seed_provider_templates")
        .unwrap();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..2).map(|_| MockExecResult::default()))
        .into_connection();
    migration.up(&SchemaManager::new(&db)).await.unwrap();
    let transactions = db.into_transaction_log();
    let statement = &transactions[1].statements()[0];
    let values = statement.values.as_ref().unwrap();
    let sea_orm::Value::Json(Some(data)) = &values.0[0] else {
        panic!("seed rows must be supplied as a structured statement parameter");
    };
    let providers = data["providers"].as_array().unwrap();
    let models = data["models"].as_array().unwrap();
    assert_eq!(providers.len(), 143);
    assert_eq!(models.len(), 1273);
    let mut ids = HashSet::new();
    for provider in providers {
        assert!(ids.insert(provider["id"].as_str().unwrap()));
        let fields: HashSet<_> = provider
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            fields,
            HashSet::from([
                "id",
                "name",
                "description",
                "icon_key",
                "protocol",
                "base_url",
                "headers"
            ])
        );
        if let Some(url) = provider["base_url"].as_str() {
            assert!(input::base_url(url).is_ok());
        }
        for key in provider["headers"].as_object().unwrap().keys() {
            assert!(matches!(
                key.to_ascii_lowercase().as_str(),
                "anthropic-version" | "anthropic-beta"
            ));
        }
    }
    assert!(providers.iter().find(|p| p["id"] == "azure").unwrap()["base_url"].is_null());
    assert!(providers.iter().find(|p| p["id"] == "bedrock").unwrap()["base_url"].is_null());
    assert_eq!(
        providers.iter().find(|p| p["id"] == "openai").unwrap()["base_url"],
        "https://api.openai.com/v1"
    );
    assert!(
        providers
            .iter()
            .find(|p| p["id"] == "cloudflare-ai")
            .unwrap()["base_url"]
            .is_null()
    );
    let mut model_ids = HashSet::new();
    for model in models {
        let provider = model["provider_template_id"].as_str().unwrap();
        assert!(ids.contains(provider));
        assert!(model_ids.insert((provider, model["id"].as_str().unwrap())));
        let fields: HashSet<_> = model
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            fields,
            HashSet::from([
                "provider_template_id",
                "id",
                "name",
                "description",
                "protocols",
                "paths",
                "capabilities",
                "max_context_tokens",
                "effort_mapping",
                "input_price",
                "output_price",
                "cached_price"
            ])
        );
        for key in ["input_price", "output_price", "cached_price"] {
            assert!(model[key].is_null() || input::price(model[key].as_str().unwrap()).is_ok());
        }
        if let Some(context) = model["max_context_tokens"].as_u64() {
            assert!(context > 0 && context <= u64::from(u32::MAX));
        }
        for (protocol, path) in model["paths"].as_object().unwrap() {
            assert!(
                model["protocols"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|p| p == protocol)
            );
            let path = path.as_str().unwrap();
            assert!(input::endpoint_path(path).is_ok());
            assert!(path.len() <= 40);
        }
        for level in model["effort_mapping"].as_object().unwrap().values() {
            assert!(input::effort_value(level.as_str().unwrap()).is_ok());
            assert!(level.as_str().unwrap().len() <= 25);
        }
    }
    let gemini = models
        .iter()
        .find(|m| m["provider_template_id"] == "gemini" && m["id"] == "gemini-2.5-pro")
        .unwrap();
    assert!(gemini["protocols"].as_array().unwrap().is_empty());
    let whisper = models
        .iter()
        .find(|m| m["provider_template_id"] == "openai" && m["id"] == "whisper-1")
        .unwrap();
    assert!(whisper["protocols"].as_array().unwrap().is_empty());
    assert!(whisper["input_price"].is_null());
    assert!(models.iter().any(|m| m["input_price"] == "0"));
    assert!(models.iter().any(|m| m["input_price"].is_null()));
    assert!(models.iter().any(|m| m["max_context_tokens"].is_null()));
    let multimode = models
        .iter()
        .find(|m| m["provider_template_id"] == "glm" && m["id"] == "glm-5.3")
        .unwrap();
    let provider = providers.iter().find(|p| p["id"] == "glm").unwrap();
    assert_eq!(
        format!(
            "{}{}",
            provider["base_url"].as_str().unwrap(),
            multimode["paths"]["anthropic_messages"].as_str().unwrap()
        ),
        "https://api.z.ai/api/anthropic/v1/messages"
    );
}

#[tokio::test]
async fn migration_seeds_parent_and_child_rows_atomically_and_only_removes_unchanged_data() {
    let migration = Migrator::migrations()
        .into_iter()
        .find(|migration| migration.name() == "m20261106_000018_seed_provider_templates")
        .unwrap();
    assert_eq!(migration.name(), "m20261106_000018_seed_provider_templates");
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..4).map(|_| MockExecResult::default()))
        .into_connection();
    let manager = SchemaManager::new(&db);
    migration.up(&manager).await.unwrap();
    migration.down(&manager).await.unwrap();
    let log = format!("{:?}", db.into_transaction_log());
    assert!(log.contains("ALTER COLUMN base_url DROP NOT NULL"));
    assert!(log.contains("WITH snapshot AS"));
    assert!(log.contains("INSERT INTO provider_templates"));
    assert!(log.contains("INSERT INTO provider_model_templates"));
    assert!(!log.contains("ON CONFLICT"));
    assert!(log.contains("to_jsonb(m) = to_jsonb(seed)"));
    assert!(log.contains("to_jsonb(p) = to_jsonb(seed)"));
    assert!(log.contains("SELECT 1 FROM deleted_models"));
    assert!(log.contains("ALTER COLUMN base_url SET NOT NULL"));
    assert!(!log.contains("DELETE FROM providers "));
}
