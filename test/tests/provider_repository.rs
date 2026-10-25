use compallm_core::{
    persistent::{
        DbContext,
        provider::{MAX_PROVIDER_FIND_LIMIT, ProviderRepository as _, ProviderRepositoryError},
    },
    provider::{ApiProtocol, NewProvider, ProviderApiKey, ProviderUpdate},
};
use compallm_postgres::{
    ApiProtocolSchema, OrganizationSchema, OrganizationStatusSchema, ProviderSchema,
    postgres_connection::PostgresConnection, repository::provider::ProviderRepository,
};
use sea_orm::{ActiveEnum, DbBackend, IntoMockRow, MockDatabase, MockExecResult};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture;

impl Fixture {
    fn at() -> sea_orm::prelude::DateTimeWithTimeZone {
        "2026-10-26T00:00:00Z".parse().unwrap()
    }

    fn organization_row() -> BTreeMap<String, sea_orm::Value> {
        let model = OrganizationSchema {
            id: 84,
            name: "team".into(),
            status: OrganizationStatusSchema::Active,
            created_by: 42,
            created_at: Self::at(),
            updated_at: Self::at(),
        };
        let mut row: BTreeMap<_, _> = model.into_mock_row().into_column_value_tuples().collect();
        row.insert("status".into(), "active".into());
        row
    }

    fn provider(id: i64, name: &str, api_key: &str) -> ProviderSchema {
        ProviderSchema {
            id,
            organization_id: 84,
            name: name.into(),
            url: "https://provider.example/v1".into(),
            api_key: api_key.into(),
            api_protocol: ApiProtocolSchema::OpenaiCompatible,
            updated_at: Self::at(),
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

    fn new_provider() -> NewProvider {
        NewProvider {
            id: 100,
            organization_id: 84,
            name: "Primary".into(),
            url: "https://provider.example/v1".into(),
            api_key: ProviderApiKey::new("encrypted-key").unwrap(),
            api_protocol: ApiProtocol::OpenaiCompatible,
        }
    }
}

#[tokio::test]
async fn find_all_requires_a_bounded_limit_and_scopes_to_the_organization() {
    for limit in [0, MAX_PROVIDER_FIND_LIMIT + 1] {
        assert!(matches!(
            ProviderRepository.find_all(84, limit).await,
            Err(ProviderRepositoryError::InvalidLimit)
        ));
    }

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
    assert_eq!(providers[0].organization_id, 84);
    assert_eq!(providers[0].api_protocol, ApiProtocol::OpenaiCompatible);

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("LIMIT"));
    assert!(log.contains("12"));
    assert!(!format!("{providers:?}").contains("must-not-leak"));
}

#[tokio::test]
async fn insert_locks_the_organization_checks_the_cap_and_writes_the_secret() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::organization_row()],
                vec![BTreeMap::from([("num_items".into(), 0_i64.into())])],
                vec![Fixture::provider_row(Fixture::provider(
                    100,
                    "Primary",
                    "encrypted-key",
                ))],
            ])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let provider = context
        .transaction(async { ProviderRepository.insert(Fixture::new_provider()).await })
        .await
        .unwrap();
    assert_eq!(provider.id, 100);

    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("BEGIN"));
    assert!(log.contains("FOR UPDATE"));
    assert!(log.contains("COUNT(*)"));
    assert!(log.contains("INSERT INTO"));
    assert!(log.contains("encrypted-key"));
    assert!(log.contains("COMMIT"));
}

#[tokio::test]
async fn insert_rejects_the_129th_provider_without_an_insert() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![Fixture::organization_row()],
                vec![BTreeMap::from([("num_items".into(), 128_i64.into())])],
            ])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let error = context
        .transaction(async { ProviderRepository.insert(Fixture::new_provider()).await })
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ProviderRepositoryError::MaximumProvidersReached
    ));

    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("FOR UPDATE"));
    assert!(!log.contains("INSERT INTO"));
    assert!(log.contains("ROLLBACK"));
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
                ))],
                vec![Fixture::provider_row(Fixture::provider(
                    100,
                    "Renamed",
                    "existing-secret",
                ))],
            ])
            .into_connection(),
    );
    let provider = Fixture::context(db.clone())
        .scope(ProviderRepository.update(
            84,
            100,
            ProviderUpdate {
                name: "Renamed".into(),
                url: "https://provider.example/v1".into(),
                api_key: None,
                api_protocol: ApiProtocol::Anthropic,
            },
        ))
        .await
        .unwrap();
    assert_eq!(provider.name, "Renamed");

    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("organization_id"));
    assert!(log.contains("UPDATE"));
    assert!(!log.contains("existing-secret"));
    assert!(log.contains("updated_at"));
}

#[tokio::test]
async fn delete_is_organization_scoped_and_reports_missing_rows() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 0,
                },
            ])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    context
        .scope(ProviderRepository.delete(84, 100))
        .await
        .unwrap();
    assert!(matches!(
        context.scope(ProviderRepository.delete(84, 101)).await,
        Err(ProviderRepositoryError::NotFound)
    ));

    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("DELETE FROM"));
    assert!(log.contains("organization_id"));
}
