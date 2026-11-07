use compallm_core::{
    organization::{CreationService, OrganizationCreationError, OrganizationStatus},
    persistent::{DbContext, organization::OrganizationRepository},
    user::{User, UserStatus},
};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationSchema, OrganizationStatusSchema, UserStatusSchema,
    postgres_connection::PostgresConnection,
};
use sea_orm::{DbBackend, DbErr, IntoMockRow, MockDatabase, MockExecResult};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture {
    database: Arc<sea_orm::DatabaseConnection>,
    service: CreationService,
}

impl Fixture {
    fn new(fail_owner: bool) -> Self {
        compallm_postgres::init_id_generator("organization-tests").unwrap();

        let model = OrganizationSchema {
            id: 84,
            name: "James's organization".into(),
            status: OrganizationStatusSchema::Active,
        };
        let mut organization: BTreeMap<_, _> =
            model.into_mock_row().into_column_value_tuples().collect();
        organization.insert("status".to_string(), "active".into());
        let member = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 42,
            status: UserStatusSchema::Active,
        };
        let mut membership: BTreeMap<_, _> =
            member.into_mock_row().into_column_value_tuples().collect();
        membership.insert("status".to_string(), "active".into());
        let database = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![organization]])
            .append_exec_results([MockExecResult::default()]);
        let database = if fail_owner {
            database.append_query_errors([DbErr::Custom("owner insert failed".into())])
        } else {
            database.append_query_results([vec![membership]])
        };
        let database = Arc::new(database.into_connection());
        let context = Box::leak(Box::new(DbContext::new(Arc::new(PostgresConnection::new(
            database.clone(),
        )))));
        let repository = Box::leak(Box::new(Box::new(
            compallm_postgres::repository::organization::OrganizationRepository,
        ) as Box<dyn OrganizationRepository>));
        Self {
            database,
            service: CreationService::new(context, repository),
        }
    }

    fn user() -> User {
        User {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            status: UserStatus::Active,
        }
    }
}

#[tokio::test]
async fn default_organization_and_owner_are_inserted_in_one_transaction() {
    let fixture = Fixture::new(false);
    let organization = fixture
        .service
        .create_default(&Fixture::user())
        .await
        .unwrap();
    assert_eq!(organization.id, 84);
    assert_eq!(organization.status, OrganizationStatus::Active);
    let transactions = fixture.database.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    let sql = format!("{transactions:?}");
    assert!(sql.contains("James's organization"));
    assert!(sql.contains("organization_members"));
    assert!(sql.contains("Owner"));
    assert!(sql.contains("Admin"));
    assert!(sql.contains("Member"));
    assert!(sql.contains("organization_member_roles"));
    assert!(sql.contains("BEGIN"));
    assert!(sql.contains("COMMIT"));
    let statements = transactions[0].statements();
    let inserted_roles = statements
        .iter()
        .find(|s| s.sql.contains("INSERT INTO organization_roles"))
        .unwrap();
    let sea_orm::Value::Json(Some(rows)) = &inserted_roles.values.as_ref().unwrap().0[0] else {
        panic!("role rows must be JSON")
    };
    let roles = rows.as_array().unwrap();
    let ids = roles
        .iter()
        .map(|role| role["id"].as_u64().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 3);
    assert!(ids.iter().all(|id| *id > 3));
    for role in roles {
        assert!(
            role["cedar_source"]
                .as_str()
                .unwrap()
                .contains(&format!("84:{}", role["id"].as_u64().unwrap()))
        );
    }
    let assignment = statements
        .iter()
        .find(|s| s.sql.contains("INSERT INTO organization_member_roles"))
        .unwrap();
    let sea_orm::Value::Json(Some(assigned)) = &assignment.values.as_ref().unwrap().0[2] else {
        panic!("role assignment must be JSON")
    };
    assert_eq!(assigned.as_array().unwrap(), &vec![roles[0]["id"].clone()]);
}

#[tokio::test]
async fn owner_insert_failure_rolls_back_organization_creation() {
    let fixture = Fixture::new(true);
    assert!(matches!(
        fixture.service.create("Team", &Fixture::user()).await,
        Err(OrganizationCreationError::Persistence(_))
    ));
    let transactions = fixture.database.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    let sql = format!("{transactions:?}");
    assert!(sql.contains("ROLLBACK"));
    assert!(!sql.contains("COMMIT"));
}

#[tokio::test]
async fn blank_organization_names_fail_before_any_database_operation() {
    let fixture = Fixture::new(false);
    assert!(matches!(
        fixture.service.create("  ", &Fixture::user()).await,
        Err(OrganizationCreationError::InvalidName)
    ));
    assert!(
        fixture
            .database
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[test]
fn member_mapper_retains_scoped_identity_without_inventing_assignments() {
    let row = OrganizationMemberSchema {
        organization_id: 84,
        user_id: 42,
        status: UserStatusSchema::Active,
    };
    let member = compallm_core::organization::OrganizationMember::try_from(row).unwrap();
    assert_eq!(member.organization_id, 84);
    assert!(member.roles.is_empty() && member.groups.is_empty());
    let invalid = OrganizationMemberSchema {
        organization_id: 0,
        user_id: -1,
        status: UserStatusSchema::Active,
    };
    assert!(compallm_core::organization::OrganizationMember::try_from(invalid).is_err());
}
