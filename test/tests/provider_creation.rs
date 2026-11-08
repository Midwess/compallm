use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository, ProviderRepositoryError},
    },
    provider::{
        Capability, CreationService, Protocol, Provider, ProviderApiKey, ProviderCreationError,
        ProviderCredentialCipher, ProviderModel, ServiceKind, Thinking, ThinkingBudget,
    },
};
use compallm_postgres::postgres_connection::PostgresConnection;
use sea_orm::{DbBackend, DbErr, IntoMockRow, MockDatabase, MockExecResult};
use serde_json::json;

fn provider(name: &str) -> Provider {
    let connection = serde_json::from_value(json!({
        "baseUrl": "https://provider.example/v1", "timeoutMs": 30000,
        "authentication": {"apiKeyHeader": {"name": "Authorization", "prefix": "Bearer "}}
    }))
    .unwrap();
    Provider {
        id: 7,
        organization_id: 84,
        name: name.into(),
        connection,
        headers: Default::default(),
        api_key: credential(),
        models: vec![ProviderModel {
            id: 8,
            organization_id: 999,
            provider_id: 777,
            name: " vendor/model.v1 ".into(),
            enabled: true,
            service_kind: ServiceKind::Llm,
            protocol: Protocol::Openai,
            connection: serde_json::from_value(json!({
                "baseUrl": "https://model.example/v2", "timeoutMs": 10000,
                "authentication": {"apiKeyQuery": "key"}
            }))
            .unwrap(),
            headers: Default::default(),
            capabilities: vec![Capability::Text],
            limits: Default::default(),
            thinking: None,
            pricing: serde_json::from_value(json!({"token": {
                "inputUsdPerMillionTokens": "1.12345678",
                "outputUsdPerMillionTokens": "2.00", "cachedUsdPerMillionTokens": "0"
            }}))
            .unwrap(),
        }],
    }
}

fn credential() -> ProviderApiKey {
    static KEY: std::sync::OnceLock<ProviderApiKey> = std::sync::OnceLock::new();
    KEY.get_or_init(|| ProviderApiKey::new("upstream-api-key", &cipher()).unwrap())
        .clone()
}

fn cipher() -> ProviderCredentialCipher {
    ProviderCredentialCipher::new("test-provider-credential-secret-at-least-32-bytes").unwrap()
}

#[derive(Default)]
struct RecordingRepository(Mutex<Vec<Provider>>, Option<usize>);

#[async_trait::async_trait]
impl ProviderRepository for RecordingRepository {
    async fn insert(&self, provider: Provider) -> Result<Provider, ProviderRepositoryError> {
        DbContext::current().unwrap();
        let mut inserted = self.0.lock().unwrap();
        if self.1 == Some(inserted.len()) {
            return Err(ProviderRepositoryError::NameAlreadyExists);
        }
        inserted.push(provider.clone());
        Ok(provider)
    }
    async fn find_all(&self, _: u64, _: u64) -> Result<Vec<Provider>, ProviderRepositoryError> {
        panic!("creation must not list providers")
    }
    async fn update(&self, _: Provider) -> Result<Provider, ProviderRepositoryError> {
        panic!("creation must not update providers")
    }
    async fn delete(&self, _: u64, _: u64) -> Result<Provider, ProviderRepositoryError> {
        panic!("creation must not delete providers")
    }
}

fn context() -> (Arc<sea_orm::DatabaseConnection>, DbContext) {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let context = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    (db, context)
}

#[tokio::test]
async fn stored_name_conflict_rolls_back_the_batch_and_keeps_the_domain_error() {
    compallm_postgres::init_id_generator("provider-creation-tests").unwrap();
    let (db, context) = context();
    let repository = RecordingRepository(Mutex::new(Vec::new()), Some(1));
    let service = CreationService::new(&context, &repository);
    let result = service
        .create_many(vec![provider("First"), provider("Conflicting")])
        .await;
    assert!(matches!(
        result,
        Err(ProviderCreationError::NameAlreadyExists)
    ));
    let sql = format!("{:?}", db.as_ref().clone().into_transaction_log());
    assert!(sql.contains("ROLLBACK"));
    assert!(!sql.contains("COMMIT"));
}

#[tokio::test]
async fn creates_batch_in_order_with_fresh_ids_and_parent_ownership() {
    compallm_postgres::init_id_generator("provider-creation-tests").unwrap();
    let (db, context) = context();
    let repository = RecordingRepository::default();
    let service = CreationService::new(&context, &repository);
    let mut first = provider(" Primary ");
    let mut disabled = first.models[0].clone();
    disabled.name = "disabled.model".into();
    disabled.enabled = false;
    first.models.push(disabled);
    let expected_model = first.models[0].clone();
    let created = service
        .create_many(vec![first, provider("primary")])
        .await
        .unwrap();
    assert_eq!(
        created.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(),
        ["Primary", "primary"]
    );
    let mut ids = HashSet::new();
    for provider in &created {
        assert!(ids.insert(provider.id));
        assert_ne!(provider.id, 7);
        assert_eq!(provider.organization_id, 84);
        for model in &provider.models {
            assert!(ids.insert(model.id));
            assert_ne!(model.id, 8);
            assert_eq!(model.organization_id, 84);
            assert_eq!(model.provider_id, provider.id);
        }
    }
    assert_eq!(created[0].models[0].name, "vendor/model.v1");
    assert_eq!(created[0].models[0].connection, expected_model.connection);
    assert_eq!(created[0].models[0].pricing, expected_model.pricing);
    assert!(!created[0].models[1].enabled);
    assert!(!format!("{created:?}").contains("upstream-api-key"));
    assert!(!format!("{created:?}").contains(&String::from(credential())));
    assert_eq!(
        created[0].api_key.read(&cipher()).unwrap(),
        "upstream-api-key"
    );
    let transactions = db.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    assert!(format!("{transactions:?}").contains("COMMIT"));
}

#[tokio::test]
async fn rejects_invalid_batch_before_any_writes_or_transaction() {
    let (db, context) = context();
    let repository = RecordingRepository::default();
    let service = CreationService::new(&context, &repository);
    assert!(matches!(
        service.create_many(vec![]).await,
        Err(ProviderCreationError::InvalidInput(_))
    ));
    assert!(matches!(
        service
            .create_many(vec![provider("Same"), provider(" Same "),])
            .await,
        Err(ProviderCreationError::NameAlreadyExists)
    ));
    let mut other = provider("Other");
    other.organization_id = 999;
    assert!(matches!(
        service.create_many(vec![provider("First"), other,]).await,
        Err(ProviderCreationError::InvalidInput(_))
    ));
    let mut invalid = provider("Invalid");
    invalid.models.clear();
    assert!(matches!(
        service.create_many(vec![provider("First"), invalid,]).await,
        Err(ProviderCreationError::InvalidInput(_))
    ));
    assert!(repository.0.lock().unwrap().is_empty());
    assert!(db.as_ref().clone().into_transaction_log().is_empty());
}

#[tokio::test]
async fn rejects_invalid_entities_without_exposing_submitted_values() {
    let (db, context) = context();
    let repository = RecordingRepository::default();
    let service = CreationService::new(&context, &repository);
    let mutations: Vec<fn(&mut Provider)> = vec![
        |p| p.organization_id = 0,
        |p| p.name = "  ".into(),
        |p| p.connection.base_url = "https://user:secret@example.test".into(),
        |p| p.connection.timeout_ms = 0,
        |p| {
            p.headers
                .insert("bad header".into(), "submitted-secret".into());
        },
        |p| {
            p.headers.insert("X-Name".into(), "a".into());
            p.headers.insert("x-name".into(), "b".into());
        },
        |p| p.models[0].enabled = false,
        |p| {
            let duplicate = p.models[0].clone();
            p.models.push(duplicate);
        },
        |p| p.models[0].protocol = Protocol::Xai,
        |p| p.models[0].connection.base_url = "".into(),
        |p| p.models[0].capabilities.push(Capability::Text),
        |p| p.models[0].limits.context_tokens = Some(0),
        |p| {
            p.models[0].thinking = Some(Thinking {
                effort: None,
                budget: None,
            })
        },
        |p| {
            p.models[0].thinking = Some(Thinking {
                effort: Some(vec![]),
                budget: None,
            })
        },
        |p| {
            p.models[0].thinking = Some(Thinking {
                effort: None,
                budget: Some(ThinkingBudget {
                    min_tokens: 100,
                    max_tokens: 200,
                    default_tokens: Some(300),
                }),
            })
        },
        |p| {
            p.models[0].pricing = serde_json::from_value(json!({"token": {
            "inputUsdPerMillionTokens": "-1", "outputUsdPerMillionTokens": "1", "cachedUsdPerMillionTokens": "0"
        }})).unwrap()
        },
    ];
    for (index, mutate) in mutations.iter().enumerate() {
        let mut input = provider("Primary");
        mutate(&mut input);
        let error = service.create(input).await.unwrap_err();
        assert!(
            matches!(error, ProviderCreationError::InvalidInput(_)),
            "case {index}: {error}"
        );
        assert!(!format!("{error:?}").contains("submitted-secret"));
        assert!(!format!("{error:?}").contains("user:secret"));
    }
    assert!(repository.0.lock().unwrap().is_empty());
    assert!(db.as_ref().clone().into_transaction_log().is_empty());
}

fn provider_row(name: &str) -> std::collections::BTreeMap<String, sea_orm::Value> {
    let provider = provider(name);
    compallm_postgres::ProviderSchema {
        id: 100,
        organization_id: 84,
        name: name.into(),
        connection: serde_json::to_value(provider.connection).unwrap(),
        headers: json!({}),
        credentials: String::from(credential()),
    }
    .into_mock_row()
    .into_column_value_tuples()
    .collect()
}

#[tokio::test]
async fn persists_models_with_the_connection_and_returns_typed_configuration() {
    compallm_postgres::init_id_generator("provider-creation-tests").unwrap();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![provider_row("Primary")]])
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let context = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let repository = compallm_postgres::repository::provider::ProviderRepository;
    let service = CreationService::new(&context, &repository);
    let input = provider("Primary");
    let pricing = input.models[0].pricing.clone();
    let created = service.create(input).await.unwrap();
    assert_eq!(created.models.len(), 1);
    assert_eq!(created.models[0].pricing, pricing);
    assert_eq!(
        created.models[0].connection.base_url,
        "https://model.example/v2"
    );
    let transactions = db.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    assert_eq!(created.api_key.read(&cipher()).unwrap(), "upstream-api-key");
}

#[tokio::test]
async fn later_model_failure_rolls_back_every_provider_in_the_batch() {
    compallm_postgres::init_id_generator("provider-creation-tests").unwrap();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![provider_row("Primary")], vec![provider_row("Other")]])
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .append_exec_errors([DbErr::Custom("model insert failed".into())])
            .into_connection(),
    );
    let context = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let repository = compallm_postgres::repository::provider::ProviderRepository;
    let service = CreationService::new(&context, &repository);
    let result = service
        .create_many(vec![provider("Primary"), provider("Other")])
        .await;
    assert!(matches!(result, Err(ProviderCreationError::Persistence(_))));
    let transactions = db.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
}

#[tokio::test]
async fn create_many_with_one_provider_uses_a_scope_without_starting_a_transaction() {
    compallm_postgres::init_id_generator("provider-creation-tests").unwrap();
    let (db, context) = context();
    let repository = RecordingRepository::default();
    let service = CreationService::new(&context, &repository);
    let created = service.create_many(vec![provider("Single")]).await.unwrap();
    assert_eq!(created.len(), 1);
    assert_eq!(repository.0.lock().unwrap().len(), 1);
    assert!(db.as_ref().clone().into_transaction_log().is_empty());
}

#[tokio::test]
async fn single_provider_insert_failure_rejects_creation() {
    compallm_postgres::init_id_generator("provider-creation-tests").unwrap();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_errors([DbErr::Custom("model insert failed".into())])
            .into_connection(),
    );
    let context = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let repository = compallm_postgres::repository::provider::ProviderRepository;
    let result = CreationService::new(&context, &repository)
        .create(provider("Single"))
        .await;
    assert!(matches!(result, Err(ProviderCreationError::Persistence(_))));
}
