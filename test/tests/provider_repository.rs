use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository as _, ProviderRepositoryError},
    },
    provider::{ApiProtocol, Provider, ProviderApiKey},
};
use compallm_postgres::{
    ApiProtocolSchema, ProviderSchema, postgres_connection::PostgresConnection,
    repository::provider::ProviderRepository,
};
use sea_orm::{ActiveEnum, DbBackend, IntoMockRow, MockDatabase};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture;

impl Fixture {
    fn at() -> sea_orm::prelude::DateTimeWithTimeZone {
        "2026-10-26T00:00:00Z".parse().unwrap()
    }

    fn provider(id: i64, name: &str, api_key: &str, enabled: bool) -> ProviderSchema {
        ProviderSchema {
            id,
            organization_id: 84,
            name: name.into(),
            url: "https://provider.example/v1".into(),
            api_key: api_key.into(),
            api_protocol: ApiProtocolSchema::OpenaiCompatible,
            updated_at: Self::at(),
            enabled,
        }
    }

    fn provider_row(model: ProviderSchema) -> BTreeMap<String, sea_orm::Value> {
        let protocol = model.api_protocol.to_value().value.to_string();
        let mut row: BTreeMap<_, _> = model.into_mock_row().into_column_value_tuples().collect();
        row.insert("api_protocol".into(), protocol.into());
        row
    }

    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }

    fn new_provider() -> Provider {
        Provider {
            id: 100,
            organization_id: 84,
            name: "Primary".into(),
            url: "https://provider.example/v1".into(),
            api_protocol: ApiProtocol::OpenaiCompatible,
            enabled: true,
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
                false,
            ))]])
            .into_connection(),
    );
    let providers = Fixture::context(db.clone())
        .scope(ProviderRepository.find_all(84, 12))
        .await
        .unwrap();
    assert_eq!(providers.len(), 1);
    assert_eq!(providers[0].organization_id, 84);
    assert_eq!(providers[0].api_protocol, ApiProtocol::OpenaiCompatible);
    assert!(!providers[0].enabled);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("LIMIT"));
    assert!(log.contains("12"));
    assert!(!format!("{providers:?}").contains("must-not-leak"));
}

#[tokio::test]
async fn insert_writes_availability_and_secret_without_business_validation_queries() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::provider_row(Fixture::provider(
                100,
                "Primary",
                "encrypted-key",
                true,
            ))]])
            .into_connection(),
    );
    let provider = Fixture::context(db.clone())
        .scope(ProviderRepository.insert(
            Fixture::new_provider(),
            ProviderApiKey::new("encrypted-key").unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(provider.id, 100);
    assert!(provider.enabled);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO"));
    assert!(log.contains("encrypted-key"));
    assert!(log.contains("enabled"));
    assert!(!log.contains("FOR UPDATE"));
    assert!(!log.contains("COUNT(*)"));
}

#[tokio::test]
async fn update_preserves_the_secret_when_no_replacement_is_supplied() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::provider_row(Fixture::provider(
                    100,
                    "Primary",
                    "existing-secret",
                    true,
                ))],
                vec![Fixture::provider_row(Fixture::provider(
                    100,
                    "Renamed",
                    "existing-secret",
                    false,
                ))],
            ])
            .into_connection(),
    );
    let provider = Fixture::context(db.clone())
        .scope(ProviderRepository.update(
            Provider {
                id: 100,
                organization_id: 84,
                name: "Renamed".into(),
                url: "https://provider.example/v1".into(),
                api_protocol: ApiProtocol::Anthropic,
                enabled: false,
            },
            None,
        ))
        .await
        .unwrap();
    assert_eq!(provider.name, "Renamed");
    assert!(!provider.enabled);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("UPDATE"));
    assert!(!log.contains("existing-secret"));
    assert!(log.contains("updated_at"));
    assert!(log.contains("enabled"));
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
                    false,
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
