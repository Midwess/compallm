#[path = "support/permission.rs"]
mod permission_fixture;
use compallm_core::authorization::{MemberPermission, policy::Action};

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
    permission: MemberPermission,
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
                        let model = OrganizationSchema {
                            id: 84,
                            name: name.into(),
                            status: OrganizationStatusSchema::Active,
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
            permission: permission_fixture::permission(84, &[Action::OrganizationSettingsUpdate]),
            context,
            database,
            service: ConfigService::new(organizations),
        }
    }
}

#[tokio::test]
async fn rename_updates_only_name_in_one_transaction() {
    let fixture = Fixture::new(Ok(Some("New-Team")));
    let organization = fixture
        .context
        .transaction(
            fixture
                .service
                .update_name(&fixture.permission, " New-Team "),
        )
        .await
        .unwrap();
    assert_eq!(organization.id, 84);
    assert_eq!(organization.name, "New-Team");
    let transactions = fixture.database.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    let log = format!("{transactions:?}");
    assert!(log.contains("UPDATE"));
    assert!(log.contains("New-Team"));
    assert!(log.contains("RETURNING"));
    assert!(log.contains("COMMIT"));
    assert!(!log.contains("organization_members"));
}

#[tokio::test]
async fn missing_organization_or_query_failure_rolls_back() {
    for result in [Ok(None), Err(DbErr::Custom("update failed".into()))] {
        let missing = result.is_ok();
        let fixture = Fixture::new(result);
        let error = fixture
            .context
            .transaction(fixture.service.update_name(&fixture.permission, "New-Team"))
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

#[tokio::test]
async fn settings_permission_is_required_before_repository_access() {
    use compallm_core::authorization::MemberPermissionError;
    let fixture = Fixture::new(Err(DbErr::Custom("unexpected repository access".into())));
    let permission = permission_fixture::permission(84, &[]);
    let result = fixture
        .context
        .scope(fixture.service.update_name(&permission, "New-Team"))
        .await;
    assert!(matches!(
        result,
        Err(OrganizationConfigError::Permission(
            MemberPermissionError::Denied
        ))
    ));
}
