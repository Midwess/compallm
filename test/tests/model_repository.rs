use compallm_core::{
    model::Model,
    persistent::{
        DbContext,
        model::{ModelRepository as _, ModelRepositoryError},
    },
};
use compallm_postgres::{
    ModelSchema, postgres_connection::PostgresConnection, repository::model::ModelRepository,
};
use sea_orm::{DbBackend, MockDatabase, prelude::Decimal};
use serde_json::json;
use std::sync::Arc;

struct Fixture;

impl Fixture {
    fn at() -> sea_orm::prelude::DateTimeWithTimeZone {
        "2026-10-27T00:00:00Z".parse().unwrap()
    }

    fn schema(id: i64, name: &str, enabled: bool, hidden: bool) -> ModelSchema {
        ModelSchema {
            id,
            organization_id: 84,
            name: name.into(),
            input_price_per_million_tokens: Decimal::new(125, 2),
            output_price_per_million_tokens: Decimal::new(25, 1),
            cached_price_per_million_tokens: Decimal::new(50, 2),
            created_at: Self::at(),
            updated_at: Self::at(),
            enabled,
            hidden,
            settings: json!({ "contextWindow": 128_000 }),
        }
    }

    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }

    fn model() -> Model {
        Model {
            id: 100,
            organization_id: 84,
            name: "general-chat".into(),
            input_price_per_million_tokens: Decimal::new(125, 2),
            output_price_per_million_tokens: Decimal::new(25, 1),
            cached_price_per_million_tokens: Decimal::new(50, 2),
            enabled: true,
            hidden: false,
            settings: json!({ "contextWindow": 128_000 }),
        }
    }
}

#[tokio::test]
async fn find_all_returns_management_records_and_scopes_to_the_organization() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(100, "internal-model", false, true)]])
            .into_connection(),
    );
    let models = Fixture::context(db.clone())
        .scope(ModelRepository.find_all(84, 12))
        .await
        .unwrap();

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].organization_id, 84);
    assert!(!models[0].enabled);
    assert!(models[0].hidden);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("LIMIT"));
    assert!(log.contains("12"));
    assert!(log.contains("ORDER BY"));
}

#[tokio::test]
async fn insert_writes_prices_lifecycle_flags_and_settings() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::schema(100, "general-chat", true, false)]])
            .into_connection(),
    );
    let model = Fixture::context(db.clone())
        .scope(ModelRepository.insert(Fixture::model()))
        .await
        .unwrap();

    assert_eq!(model.id, 100);
    assert_eq!(model.input_price_per_million_tokens, Decimal::new(125, 2));
    assert_eq!(model.settings, json!({ "contextWindow": 128_000 }));

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO"));
    assert!(log.contains("input_price_per_million_tokens"));
    assert!(log.contains("enabled"));
    assert!(log.contains("hidden"));
    assert!(log.contains("settings"));
}

#[tokio::test]
async fn update_is_organization_scoped_and_updates_every_mutable_field() {
    let mut updated = Fixture::schema(100, "internal-chat", false, true);
    updated.settings = json!({ "contextWindow": 200_000, "reasoning": true });
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::schema(100, "general-chat", true, false)],
                vec![updated],
            ])
            .into_connection(),
    );
    let mut update = Fixture::model();
    update.name = "internal-chat".into();
    update.input_price_per_million_tokens = Decimal::new(2, 0);
    update.output_price_per_million_tokens = Decimal::new(3, 0);
    update.cached_price_per_million_tokens = Decimal::new(1, 0);
    update.enabled = false;
    update.hidden = true;
    update.settings = json!({ "contextWindow": 200_000, "reasoning": true });

    let model = Fixture::context(db.clone())
        .scope(ModelRepository.update(update))
        .await
        .unwrap();
    assert_eq!(model.name, "internal-chat");
    assert!(!model.enabled);
    assert!(model.hidden);
    assert_eq!(model.settings["contextWindow"], 200_000);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("UPDATE"));
    assert!(log.contains("updated_at"));
    assert!(log.contains("settings"));
}

#[tokio::test]
async fn delete_is_organization_scoped_and_reports_missing_rows() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::schema(100, "general-chat", true, false)],
                vec![],
            ])
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
            .append_query_results([vec![Fixture::schema(-1, "corrupt-model", true, false)]])
            .into_connection(),
    );
    let result = Fixture::context(db)
        .scope(ModelRepository.find_all(84, 12))
        .await;

    assert!(matches!(result, Err(ModelRepositoryError::Database(_))));
}

#[tokio::test]
async fn negative_persisted_prices_are_rejected_during_mapping() {
    let mut corrupt = Fixture::schema(100, "corrupt-model", true, false);
    corrupt.input_price_per_million_tokens = Decimal::NEGATIVE_ONE;
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
