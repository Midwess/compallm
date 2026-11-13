use compallm_core::{
    organization::{
        CreationService, Organization, OrganizationCreationError, OrganizationMember,
        OrganizationStatus, Role,
    },
    persistent::{
        DbContext,
        organization::{OrganizationRepository, OrganizationRepositoryError},
    },
    user::{User, UserStatus},
};
use compallm_postgres::{
    OrganizationMemberSchema, UserStatusSchema, postgres_connection::PostgresConnection,
};
use sea_orm::{DbBackend, MockDatabase};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct State {
    organization: Option<Organization>,
    roles: Vec<Role>,
    member: Option<OrganizationMember>,
}
struct Repository {
    state: Arc<Mutex<State>>,
    fail_owner: bool,
}
#[async_trait::async_trait]
impl OrganizationRepository for Repository {
    fn supports_role_policy(&self, _: &str) -> bool {
        true
    }
    async fn update_role(
        &self,
        _: u64,
        _: u64,
        _: Option<String>,
        _: Option<String>,
        _: Option<String>,
    ) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_role(&self, _: u64, _: u64) -> Result<Role, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_role_groups(&self, _: u64, _: u64) -> Result<bool, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_group(&self, _: u64, _: u64) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn role_assignees(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Vec<u64>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn delete_role(&self, _: u64, _: u64) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn find_roles_for_assignment(
        &self,
        _: u64,
        _: &[u64],
    ) -> Result<Vec<Role>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn create_group(
        &self,
        _: compallm_core::organization::Group,
    ) -> Result<compallm_core::organization::Group, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn find_role(&self, _: u64, _: u64) -> Result<Option<Role>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn list_role_info(&self, _: u64) -> Result<Vec<Role>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn list_group_info(
        &self,
        _: u64,
    ) -> Result<Vec<compallm_core::organization::group::GroupInfo>, OrganizationRepositoryError>
    {
        unimplemented!()
    }
    async fn find_group_info(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Option<compallm_core::organization::group::GroupInfo>, OrganizationRepositoryError>
    {
        unimplemented!()
    }
    async fn find_group(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Option<compallm_core::organization::Group>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn save_member(
        &self,
        _member: &OrganizationMember,
        _previous: &OrganizationMember,
    ) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn find_membership(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn find_first_membership(
        &self,
        _: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn update_name(
        &self,
        _: u64,
        _: &str,
    ) -> Result<Organization, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError> {
        assert!(DbContext::current()?.postgres().is_transaction());
        self.state.lock().unwrap().organization = Some(organization.clone());
        Ok(organization)
    }
    async fn create_roles(&self, roles: Vec<Role>) -> Result<(), OrganizationRepositoryError> {
        assert!(DbContext::current()?.postgres().is_transaction());
        self.state.lock().unwrap().roles = roles;
        Ok(())
    }
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError> {
        assert!(DbContext::current()?.postgres().is_transaction());
        if self.fail_owner {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "owner insert failed"
            )));
        }
        self.state.lock().unwrap().member = Some(member.clone());
        Ok(member)
    }
}
struct Fixture {
    context: &'static DbContext,
    database: Arc<sea_orm::DatabaseConnection>,
    state: Arc<Mutex<State>>,
    service: CreationService,
}
impl Fixture {
    fn new(fail_owner: bool) -> Self {
        compallm_postgres::init_id_generator("organization-tests").unwrap();
        let database = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
        let context = Box::leak(Box::new(DbContext::new(Arc::new(PostgresConnection::new(
            database.clone(),
        )))));
        let state = Arc::new(Mutex::new(State::default()));
        let repository = Box::leak(Box::new(Box::new(Repository {
            state: state.clone(),
            fail_owner,
        }) as Box<dyn OrganizationRepository>));
        Self {
            context,
            database,
            state,
            service: CreationService::new(repository),
        }
    }
    fn user() -> User {
        User {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            username: "james".into(),
            avatar_path: None,
            status: UserStatus::Active,
        }
    }
}
#[tokio::test]
async fn default_organization_and_owner_are_inserted_in_one_transaction() {
    let fixture = Fixture::new(false);
    let organization = fixture
        .context
        .scope(fixture.service.create_default(&Fixture::user()))
        .await
        .unwrap();
    assert_eq!(organization.status, OrganizationStatus::Active);
    assert_eq!(organization.name, "James's organization");
    let state = fixture.state.lock().unwrap();
    let roles = &state.roles;
    assert_eq!(roles.len(), 3);
    let identities = roles
        .iter()
        .map(|role| role.id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(identities.len(), 3);
    for role in roles {
        assert_eq!(role.organization_id, organization.id);
        let policies = cedar_policy::PolicySet::try_from(role).unwrap();
        let expected: cedar_policy::EntityUid =
            format!("Compallm::Role::\"{}:{}\"", organization.id, role.id)
                .parse()
                .unwrap();
        assert!(policies.policies().all(|policy| matches!(
            policy.principal_constraint(),
            cedar_policy::PrincipalConstraint::In(uid) if uid == expected
        )));
    }
    let member = state.member.as_ref().unwrap();
    assert_eq!(member.roles, vec![roles[0].clone()]);
    assert_eq!(member.organization_id, organization.id);
    assert_eq!(member.user_id, 42);
    let sql = format!(
        "{:?}",
        fixture.database.as_ref().clone().into_transaction_log()
    );
    assert!(sql.contains("BEGIN") && sql.contains("COMMIT"));
}
#[tokio::test]
async fn owner_insert_failure_rolls_back_organization_creation() {
    let fixture = Fixture::new(true);
    assert!(matches!(
        fixture
            .context
            .scope(fixture.service.create("Team", &Fixture::user()))
            .await,
        Err(OrganizationCreationError::Persistence(_))
    ));
    let sql = format!(
        "{:?}",
        fixture.database.as_ref().clone().into_transaction_log()
    );
    assert!(sql.contains("ROLLBACK"));
    assert!(!sql.contains("COMMIT"));
}
#[tokio::test]
async fn blank_organization_names_fail_before_any_database_operation() {
    let fixture = Fixture::new(false);
    assert!(matches!(
        fixture
            .context
            .scope(fixture.service.create("  ", &Fixture::user()))
            .await,
        Err(OrganizationCreationError::InvalidName)
    ));
    assert!(fixture.state.lock().unwrap().organization.is_none());
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
