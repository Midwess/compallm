use compallm_core::{
    organization::OrganizationRole,
    persistent::{DbContext, organization::OrganizationRepository as _},
};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationRoleSchema, OrganizationSchema, OrganizationStatusSchema,
    UserStatusSchema, postgres_connection::PostgresConnection,
    repository::organization::OrganizationRepository,
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
        let organization = OrganizationSchema {
            id: 84,
            name: "Research team".into(),
            status: OrganizationStatusSchema::Active,
        };
        let mut row: BTreeMap<_, _> = (row, organization)
            .into_mock_row()
            .into_column_value_tuples()
            .collect();
        row.insert("A_role".to_string(), "admin".into());
        row.insert("A_status".to_string(), "active".into());
        row.insert("B_status".to_string(), "active".into());
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
    assert_eq!(member.0.role, OrganizationRole::Admin);
    assert_eq!(member.0.organization_id, 84);
    assert_eq!(member.1.name, "Research team");
    drop(context);
    let log = format!(
        "{:?}",
        Arc::try_unwrap(database).unwrap().into_transaction_log()
    );
    assert!(log.contains("LEFT JOIN"));
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
    assert_eq!(member.0.user_id, 1);
    drop(context);
    let log = format!(
        "{:?}",
        Arc::try_unwrap(database).unwrap().into_transaction_log()
    );
    assert!(log.contains("user_id"));
    assert!(log.contains("organization_id"));
    assert!(log.contains("LEFT JOIN"));
    assert!(log.contains("active"));
    assert!(!log.contains(" IN "));
}
