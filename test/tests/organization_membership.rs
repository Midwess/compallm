#[path = "support/roles.rs"]
mod role_fixture;
use role_fixture::default_roles;

use compallm_core::persistent::{DbContext, organization::OrganizationRepository as _};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationRoleSchema, OrganizationSchema, OrganizationStatusSchema,
    UserStatusSchema, postgres_connection::PostgresConnection,
    repository::organization::OrganizationRepository,
};
use sea_orm::{DbBackend, IntoMockRow, MockDatabase};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture;

impl Fixture {
    async fn database() -> Arc<sea_orm::DatabaseConnection> {
        let row = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 1,
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
        row.insert("A_status".to_string(), "active".into());
        row.insert("B_status".to_string(), "active".into());
        let role = default_roles(84, [1, 2, 3]).await.remove(1);
        let role = OrganizationRoleSchema {
            organization_id: 84,
            id: role.id as i64,
            name: role.name,
            description: role.description,
            cedar_source: role.cedar_source,
            org_owner: role.org_owner,
        };
        let role_row = role
            .into_mock_row()
            .into_column_value_tuples()
            .collect::<BTreeMap<_, _>>();
        Arc::new(
            MockDatabase::new(DbBackend::Postgres)
                .append_query_results([vec![row], vec![role_row], vec![]])
                .into_connection(),
        )
    }
}

#[tokio::test]
async fn first_membership_query_filters_active_organizations_without_role_identity_checks() {
    let database = Fixture::database().await;
    let context = DbContext::new(Arc::new(PostgresConnection::new(database.clone())));
    let member = context
        .scope(OrganizationRepository.find_first_membership(1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(member.0.roles[0].id, 2);
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
    assert!(!log.contains("r.id IN (1,2)"));
    assert!(log.contains("ORDER BY"));
    assert!(log.contains("organization_id"));
}

#[tokio::test]
async fn selected_membership_query_checks_both_user_and_organization() {
    let database = Fixture::database().await;
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
    assert!(log.contains("organization_member_roles"));
}

#[tokio::test]
async fn membership_hydrates_direct_and_inherited_roles_with_four_scoped_queries() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    use compallm_postgres::{OrganizationGroupSchema, OrganizationRoleSchema};
    let member = OrganizationMemberSchema {
        organization_id: 84,
        user_id: 1,
        status: UserStatusSchema::Active,
    };
    let org = OrganizationSchema {
        id: 84,
        name: "Research".into(),
        status: OrganizationStatusSchema::Active,
    };
    let mut membership = (member, org)
        .into_mock_row()
        .into_column_value_tuples()
        .collect::<BTreeMap<_, _>>();
    membership.insert("A_status".into(), "active".into());
    membership.insert("B_status".into(), "active".into());
    let direct = OrganizationRoleSchema {
        organization_id: 84,
        id: 9,
        name: "Reports".into(),
        description: String::new(),
        cedar_source: String::new(),
        org_owner: false,
    }
    .into_mock_row()
    .into_column_value_tuples()
    .collect::<BTreeMap<_, _>>();
    let groups = [20, 21]
        .into_iter()
        .map(|id| {
            OrganizationGroupSchema {
                organization_id: 84,
                id,
                name: format!("Team {id}"),
                description: String::new(),
            }
            .into_mock_row()
            .into_column_value_tuples()
            .collect::<BTreeMap<_, _>>()
        })
        .collect::<Vec<_>>();
    let inherited = [20, 21]
        .into_iter()
        .map(|group_id| {
            let role = fixture_roles_0.clone().remove(1);
            let mut row = OrganizationRoleSchema {
                organization_id: 84,
                id: 2,
                name: role.name,
                description: role.description,
                cedar_source: role.cedar_source,
                org_owner: role.org_owner,
            }
            .into_mock_row()
            .into_column_value_tuples()
            .collect::<BTreeMap<_, _>>();
            row.insert("group_id".into(), sea_orm::Value::BigInt(Some(group_id)));
            row
        })
        .collect::<Vec<_>>();
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![membership], vec![direct], groups, inherited])
            .into_connection(),
    );
    let ctx = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let (member, _) = ctx
        .scope(OrganizationRepository.find_membership(84, 1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        member.roles.iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![9]
    );
    assert_eq!(member.groups.len(), 2);
    assert!(
        member
            .groups
            .iter()
            .all(|g| g.roles.len() == 1 && g.roles[0].id == 2)
    );
    drop(ctx);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(log.iter().map(|t| t.statements().len()).sum::<usize>(), 4);
    for statement in log.iter().flat_map(|t| t.statements()) {
        assert!(statement.sql.contains("organization_id"));
    }
}
