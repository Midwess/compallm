use compallm_core::{
    organization::{
        CreationService, OrganizationCreationError, OrganizationRole, OrganizationStatus,
    },
    persistent::{DbContext, organization::OrganizationRepository},
    user::{User, UserStatus},
};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationRoleSchema, OrganizationSchema, OrganizationStatusSchema,
    postgres_connection::PostgresConnection,
};
use sea_orm::{DbBackend, DbErr, IntoMockRow, MockDatabase};
use std::{collections::BTreeMap, sync::Arc};

struct Fixture {
    database: Arc<sea_orm::DatabaseConnection>,
    service: CreationService,
}

impl Fixture {
    fn new(fail_owner: bool) -> Self {
        compallm_postgres::init_id_generator("organization-tests").unwrap();
        let now = "2026-10-25T00:00:00Z"
            .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
            .unwrap();
        let model = OrganizationSchema {
            id: 84,
            name: "james-workspace".into(),
            status: OrganizationStatusSchema::Active,
            created_by: 42,
            created_at: now,
            updated_at: now,
        };
        let mut organization: BTreeMap<_, _> =
            model.into_mock_row().into_column_value_tuples().collect();
        organization.insert("status".to_string(), "active".into());
        let member = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 42,
            role: OrganizationRoleSchema::Owner,
            created_at: now,
        };
        let mut membership: BTreeMap<_, _> =
            member.into_mock_row().into_column_value_tuples().collect();
        membership.insert("role".to_string(), "owner".into());
        let database =
            MockDatabase::new(DbBackend::Postgres).append_query_results([vec![organization]]);
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
    assert_eq!(organization.created_by, 42);
    assert_eq!(organization.status, OrganizationStatus::Active);
    let transactions = fixture.database.as_ref().clone().into_transaction_log();
    assert_eq!(transactions.len(), 1);
    let sql = format!("{transactions:?}");
    assert!(sql.contains("james-workspace"));
    assert!(sql.contains("organization_members"));
    assert!(sql.contains("owner"));
    assert!(sql.contains("BEGIN"));
    assert!(sql.contains("COMMIT"));
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
fn member_mapper_preserves_roles_and_rejects_invalid_ids() {
    let now = "2026-10-25T00:00:00Z"
        .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
        .unwrap();
    for (schema_role, expected) in [
        (OrganizationRoleSchema::Owner, OrganizationRole::Owner),
        (OrganizationRoleSchema::Admin, OrganizationRole::Admin),
        (OrganizationRoleSchema::Member, OrganizationRole::Member),
    ] {
        let schema = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 42,
            role: schema_role,
            created_at: now,
        };
        let member = compallm_core::organization::OrganizationMember::try_from(schema).unwrap();
        assert_eq!(member.role, expected);
    }
    let invalid = OrganizationMemberSchema {
        organization_id: 0,
        user_id: -1,
        role: OrganizationRoleSchema::Owner,
        created_at: now,
    };
    assert!(compallm_core::organization::OrganizationMember::try_from(invalid).is_err());
}
