use compallm_core::{
    model::Model,
    persistent::{
        DbContext,
        model::{ModelRepository as _, ModelRepositoryError},
    },
    provider::Pricing,
};
use compallm_postgres::{
    ModelSchema, postgres_connection::PostgresConnection, repository::model::ModelRepository,
};
use sea_orm::{DbBackend, MockDatabase, prelude::Decimal};
use serde_json::json;
use std::sync::Arc;

struct Fixture;

impl Fixture {
    fn pricing() -> serde_json::Value {
        json!({"token": {"inputUsdPerMillionTokens": "1.25", "outputUsdPerMillionTokens": "2.5", "cachedUsdPerMillionTokens": "0.50"}})
    }

    fn schema(id: i64, name: &str) -> ModelSchema {
        ModelSchema {
            id,
            organization_id: 84,
            name: name.into(),
            service_kind: "llm".into(),
            enabled: true,
            capabilities: json!(["text"]),
            limits: json!({"contextTokens": 128_000}),
            thinking: None,
            pricing: Self::pricing(),
        }
    }

    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }

    fn model() -> Model {
        Model::try_from(Self::schema(100, "general-chat")).unwrap()
    }
}

#[tokio::test]
async fn find_all_returns_management_records_and_scopes_to_the_organization() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(100, "internal-model")]])
            .into_connection(),
    );
    let models = Fixture::context(db.clone())
        .scope(ModelRepository.find_all(84, 12))
        .await
        .unwrap();

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].organization_id, 84);
    assert_eq!(models[0].name, "internal-model");

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("LIMIT"));
    assert!(log.contains("12"));
    assert!(log.contains("ORDER BY"));
}

#[tokio::test]
async fn insert_writes_typed_prices_and_configuration() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(100, "general-chat")]])
            .into_connection(),
    );
    let model = Fixture::context(db.clone())
        .scope(ModelRepository.insert(Fixture::model()))
        .await
        .unwrap();

    assert_eq!(model.id, 100);
    let Pricing::Token(pricing) = model.pricing else {
        panic!("expected token prices")
    };
    assert_eq!(pricing.input_usd_per_million_tokens, Decimal::new(125, 2));
    assert_eq!(model.limits.context_tokens, Some(128_000));

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO"));
    assert!(log.contains("pricing"));
    assert!(log.contains("limits"));
    assert!(log.contains("enabled"));
    assert!(!log.contains("hidden"));
}

#[tokio::test]
async fn update_is_organization_scoped_and_updates_every_mutable_field() {
    let mut updated = Fixture::schema(100, "internal-chat");
    updated.limits = json!({ "contextTokens": 200_000 });
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(100, "general-chat")], vec![updated]])
            .into_connection(),
    );
    let mut update = Fixture::model();
    update.name = "internal-chat".into();
    update.limits.context_tokens = Some(200_000);
    update.enabled = false;

    let model = Fixture::context(db.clone())
        .scope(ModelRepository.update(update))
        .await
        .unwrap();
    assert_eq!(model.name, "internal-chat");
    assert_eq!(model.limits.context_tokens, Some(200_000));

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("UPDATE"));
    assert!(log.contains("limits"));
    assert!(log.contains("enabled"));
    assert!(!log.contains("updated_at"));
    assert!(!log.contains("hidden"));
}

#[tokio::test]
async fn delete_is_organization_scoped_and_reports_missing_rows() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(100, "general-chat")], vec![]])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let deleted = context
        .scope(ModelRepository.delete(84, 100))
        .await
        .unwrap();
    assert_eq!(deleted.id, 100);
    assert!(matches!(
        context.scope(ModelRepository.delete(84, 101)).await,
        Err(ModelRepositoryError::NotFound)
    ));

    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("DELETE FROM"));
    assert!(log.contains("organization_id"));
}

#[tokio::test]
async fn invalid_application_ids_fail_before_querying_the_database() {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let result = Fixture::context(db.clone())
        .scope(ModelRepository.find_all(0, 12))
        .await;

    assert!(matches!(result, Err(ModelRepositoryError::Database(_))));
    assert!(
        Arc::try_unwrap(db)
            .unwrap()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn invalid_persisted_ids_are_rejected_during_mapping() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(-1, "corrupt-model")]])
            .into_connection(),
    );
    let result = Fixture::context(db)
        .scope(ModelRepository.find_all(84, 12))
        .await;

    assert!(matches!(result, Err(ModelRepositoryError::Database(_))));
}

#[tokio::test]
async fn negative_persisted_prices_are_rejected_during_mapping() {
    let mut corrupt = Fixture::schema(100, "corrupt-model");
    corrupt.pricing["token"]["inputUsdPerMillionTokens"] = json!("-1");
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![corrupt]])
            .into_connection(),
    );
    let result = Fixture::context(db)
        .scope(ModelRepository.find_all(84, 12))
        .await;

    assert!(matches!(result, Err(ModelRepositoryError::Database(_))));
}
