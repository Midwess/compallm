use compallm_core::{
    organization::{ConfigService, OrganizationConfigError},
    persistent::{
        DbContext,
        organization::{OrganizationRepository, OrganizationRepositoryError},
    },
};
use compallm_postgres::{
    OrganizationSchema, OrganizationStatusSchema, postgres_connection::PostgresConnection,
};
use sea_orm::{DbBackend, DbErr, IntoMockRow, MockDatabase};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture {
    context: &'static DbContext,
    database: Arc<sea_orm::DatabaseConnection>,
    service: ConfigService,
}

impl Fixture {
    fn new(result: Result<Option<&str>, DbErr>) -> Self {
        let db = MockDatabase::new(DbBackend::Postgres);
        let db = match result {
            Ok(name) => {
                let rows = name
                    .into_iter()
                    .map(|name| {
                        let at = "2026-10-25T00:00:00Z"
                            .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
                            .unwrap();
                        let model = OrganizationSchema {
                            id: 84,
                            name: name.into(),
                            status: OrganizationStatusSchema::Active,
                            created_by: 42,
                            created_at: at,
                            updated_at: at,
                        };
                        let mut row: BTreeMap<_, _> =
                            model.into_mock_row().into_column_value_tuples().collect();
                        row.insert("status".to_string(), "active".into());
                        row
                    })
                    .collect::<Vec<_>>();
                db.append_query_results([rows])
            }
            Err(error) => db.append_query_errors([error]),
        };
        let database = Arc::new(db.into_connection());
        let context = Box::leak(Box::new(DbContext::new(Arc::new(PostgresConnection::new(
            database.clone(),
        )))));
        let organizations = Box::leak(Box::new(Box::new(
            compallm_postgres::repository::organization::OrganizationRepository,
        ) as Box<dyn OrganizationRepository>));
        Self {
            context,
            database,
            service: ConfigService::new(context, organizations),
        }
    }
}

#[tokio::test]
async fn rename_updates_only_name_and_timestamp_in_one_transaction() {
    let fixture = Fixture::new(Ok(Some("New-Team")));
    let organization = fixture
        .context
        .transaction(fixture.service.update_name(84, " New-Team "))
        .await
        .unwrap();
    assert_eq!(organization.id, 84);
    assert_eq!(organization.created_by, 42);
    assert_eq!(organization.name, "New-Team");
    let transactions = fixture.database.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    let log = format!("{transactions:?}");
    assert!(log.contains("UPDATE"));
    assert!(log.contains("updated_at"));
    assert!(log.contains("CURRENT_TIMESTAMP"));
    assert!(log.contains("New-Team"));
    assert!(log.contains("RETURNING"));
    assert!(log.contains("COMMIT"));
    assert!(!log.contains("organization_members"));
}

#[tokio::test]
async fn invalid_names_do_not_touch_the_database() {
    let fixture = Fixture::new(Ok(None));
    for name in ["", "bad name", "bad_name", "bad.name", &"a".repeat(64)] {
        assert!(matches!(
            fixture.service.update_name(84, name).await,
            Err(OrganizationConfigError::InvalidName)
        ));
    }
    assert!(
        fixture
            .database
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn missing_organization_or_query_failure_rolls_back() {
    for result in [Ok(None), Err(DbErr::Custom("update failed".into()))] {
        let missing = result.is_ok();
        let fixture = Fixture::new(result);
        let error = fixture
            .context
            .transaction(fixture.service.update_name(84, "New-Team"))
            .await
            .unwrap_err();
        if missing {
            assert!(matches!(
                error,
                OrganizationConfigError::Persistence(OrganizationRepositoryError::NotFound)
            ));
        } else {
            assert!(matches!(
                error,
                OrganizationConfigError::Persistence(OrganizationRepositoryError::Database(_))
            ));
        }
        let log = format!(
            "{:?}",
            fixture.database.as_ref().clone().into_transaction_log()
        );
        assert!(log.contains("ROLLBACK"));
        assert!(!log.contains("COMMIT"));
    }
}
