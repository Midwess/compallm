use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository as _, ProviderRepositoryError},
    },
    provider::{Provider, ProviderApiKey, ProviderCredentialCipher},
};
use compallm_postgres::{
    ProviderSchema, postgres_connection::PostgresConnection,
    repository::provider::ProviderRepository,
};
use sea_orm::{DbBackend, IntoMockRow, MockDatabase};
use serde_json::json;
use std::{collections::BTreeMap, sync::Arc};

struct Fixture;

impl Fixture {
    fn cipher() -> ProviderCredentialCipher {
        ProviderCredentialCipher::new("test-provider-credential-secret-at-least-32-bytes").unwrap()
    }

    fn key(value: &str) -> ProviderApiKey {
        ProviderApiKey::new(value, &Self::cipher()).unwrap()
    }

    fn connection() -> serde_json::Value {
        json!({"baseUrl": "https://provider.example/v1", "timeoutMs": 30000,
            "authentication": {"apiKeyHeader": {"name": "Authorization", "prefix": "Bearer "}}})
    }

    fn provider(id: i64, name: &str, api_key: &str) -> ProviderSchema {
        ProviderSchema {
            id,
            organization_id: 84,
            name: name.into(),
            connection: Self::connection(),
            headers: json!({}),
            credentials: String::from(Self::key(api_key)),
        }
    }

    fn provider_row(model: ProviderSchema) -> BTreeMap<String, sea_orm::Value> {
        model.into_mock_row().into_column_value_tuples().collect()
    }

    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }

    fn new_provider() -> Provider {
        Provider {
            id: 100,
            organization_id: 84,
            name: "Primary".into(),
            connection: serde_json::from_value(Self::connection()).unwrap(),
            headers: Default::default(),
            api_key: Self::key("upstream-api-key"),
            models: Vec::new(),
        }
    }
}

#[tokio::test]
async fn find_all_applies_the_requested_limit_and_scopes_to_the_organization() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::provider_row(Fixture::provider(
                100,
                "Primary",
                "must-not-leak",
            ))]])
            .into_connection(),
    );
    let providers = Fixture::context(db.clone())
        .scope(ProviderRepository.find_all(84, 12))
        .await
        .unwrap();
    assert_eq!(providers.len(), 1);
    assert_eq!(
        providers[0].api_key.read(&Fixture::cipher()).unwrap(),
        "must-not-leak"
    );
    let encrypted = String::from(providers[0].api_key.clone());
    assert_eq!(providers[0].organization_id, 84);
    assert_eq!(
        providers[0].connection.base_url,
        "https://provider.example/v1"
    );

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("LIMIT"));
    assert!(log.contains("12"));
    assert!(!format!("{providers:?}").contains("must-not-leak"));
    assert!(!format!("{providers:?}").contains(&encrypted));
}

#[tokio::test]
async fn insert_writes_connection_and_secret_without_business_validation_queries() {
    let input = Fixture::new_provider();
    let encrypted = String::from(input.api_key.clone());
    let mut stored = Fixture::provider(100, "Primary", "upstream-api-key");
    stored.credentials = encrypted.clone();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::provider_row(stored)]])
            .into_connection(),
    );
    let provider = Fixture::context(db.clone())
        .scope(ProviderRepository.insert(input))
        .await
        .unwrap();
    assert_eq!(provider.id, 100);
    assert_eq!(provider.connection.timeout_ms, 30000);
    assert_eq!(
        provider.api_key.read(&Fixture::cipher()).unwrap(),
        "upstream-api-key"
    );
    assert_eq!(String::from(provider.api_key), encrypted);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO"));
    assert!(log.contains(&encrypted));
    assert!(!log.contains("upstream-api-key"));
    assert!(log.contains("credentials"));
    assert!(log.contains("connection"));
    assert!(!log.contains("FOR UPDATE"));
    assert!(!log.contains("COUNT(*)"));
}

#[tokio::test]
async fn update_preserves_the_entity_ciphertext_without_decrypting() {
    let stored = Fixture::provider(100, "Primary", "existing-secret");
    let encrypted = stored.credentials.clone();
    let mut changed = stored.clone();
    changed.name = "Renamed".into();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::provider_row(stored.clone())],
                vec![Fixture::provider_row(changed)],
            ])
            .into_connection(),
    );
    let mut input = Provider::try_from(stored).unwrap();
    input.name = "Renamed".into();
    let provider = Fixture::context(db.clone())
        .scope(ProviderRepository.update(input))
        .await
        .unwrap();
    assert_eq!(provider.name, "Renamed");
    assert_eq!(String::from(provider.api_key.clone()), encrypted);
    assert_eq!(
        provider.api_key.read(&Fixture::cipher()).unwrap(),
        "existing-secret"
    );

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("UPDATE"));
    assert!(log.contains(&encrypted));
    assert!(!log.contains("existing-secret"));
    assert!(log.contains("connection"));
    assert!(log.contains("headers"));
}

#[tokio::test]
async fn update_persists_a_replacement_key_from_the_entity() {
    let stored = Fixture::provider(100, "Primary", "existing-secret");
    let replacement = Fixture::key("replacement-secret");
    let encrypted = String::from(replacement.clone());
    let mut changed = stored.clone();
    changed.credentials = encrypted.clone();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::provider_row(stored.clone())],
                vec![Fixture::provider_row(changed)],
            ])
            .into_connection(),
    );
    let mut input = Provider::try_from(stored).unwrap();
    input.api_key = replacement;
    let updated = Fixture::context(db.clone())
        .scope(ProviderRepository.update(input))
        .await
        .unwrap();
    assert_eq!(
        updated.api_key.read(&Fixture::cipher()).unwrap(),
        "replacement-secret"
    );
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains(&encrypted));
    assert!(!log.contains("replacement-secret"));
}

#[test]
fn mapper_rejects_plaintext_credentials_in_storage() {
    let mut stored = Fixture::provider(100, "Primary", "test-secret");
    stored.credentials = "unprotected-secret".into();
    let error = Provider::try_from(stored).unwrap_err();
    assert!(!format!("{error:?}").contains("unprotected-secret"));
}

#[tokio::test]
async fn delete_is_organization_scoped_and_reports_missing_rows() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::provider_row(Fixture::provider(
                    100,
                    "Primary",
                    "existing-secret",
                ))],
                vec![],
            ])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let deleted = context
        .scope(ProviderRepository.delete(84, 100))
        .await
        .unwrap();
    assert_eq!(deleted.id, 100);
    assert!(matches!(
        context.scope(ProviderRepository.delete(84, 101)).await,
        Err(ProviderRepositoryError::NotFound)
    ));

    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("DELETE FROM"));
    assert!(log.contains("organization_id"));
}
