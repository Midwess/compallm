use compallm_core::{
    organization::OrganizationRole,
    persistent::{DbContext, organization::OrganizationRepository as _},
};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationRoleSchema, UserStatusSchema,
    postgres_connection::PostgresConnection, repository::organization::OrganizationRepository,
};
use sea_orm::{DbBackend, IntoMockRow, MockDatabase};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture;

impl Fixture {
    fn database(role: OrganizationRoleSchema) -> Arc<sea_orm::DatabaseConnection> {
        let row = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 1,
            role,
            status: UserStatusSchema::Active,
        };
        let mut row: BTreeMap<_, _> = row.into_mock_row().into_column_value_tuples().collect();
        row.insert("role".to_string(), "admin".into());
        row.insert("status".to_string(), "active".into());
        Arc::new(
            MockDatabase::new(DbBackend::Postgres)
                .append_query_results([vec![row]])
                .into_connection(),
        )
    }
}

#[tokio::test]
async fn first_admin_membership_query_filters_active_organizations_and_admin_roles() {
    let database = Fixture::database(OrganizationRoleSchema::Admin);
    let context = DbContext::new(Arc::new(PostgresConnection::new(database.clone())));
    let member = context
        .scope(OrganizationRepository.find_first_admin_membership(1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(member.role, OrganizationRole::Admin);
    assert_eq!(member.organization_id, 84);
    drop(context);
    let log = format!(
        "{:?}",
        Arc::try_unwrap(database).unwrap().into_transaction_log()
    );
    assert!(log.contains("INNER JOIN"));
    assert!(log.contains("organizations"));
    assert!(log.contains("active"));
    assert!(log.contains("owner"));
    assert!(log.contains("admin"));
    assert!(log.contains("ORDER BY"));
    assert!(log.contains("organization_id"));
}

#[tokio::test]
async fn selected_membership_query_checks_both_user_and_organization() {
    let database = Fixture::database(OrganizationRoleSchema::Admin);
    let context = DbContext::new(Arc::new(PostgresConnection::new(database.clone())));
    let member = context
        .scope(OrganizationRepository.find_membership(84, 1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(member.user_id, 1);
    drop(context);
    let log = format!(
        "{:?}",
        Arc::try_unwrap(database).unwrap().into_transaction_log()
    );
    assert!(log.contains("user_id"));
    assert!(log.contains("organization_id"));
    assert!(log.contains("INNER JOIN"));
    assert!(log.contains("active"));
    assert!(!log.contains(" IN "));
}
