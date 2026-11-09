#[path = "support/roles.rs"]
mod role_fixture;
#[path = "support/token_codec.rs"]
mod test_codec;
#[path = "support/token_pair.rs"]
mod token_pair;
use role_fixture::default_roles;

#[path = "support/session.rs"]
mod session_fixture;
use session_fixture::stored_session;

// Exercise actual API middleware with mock persistence, without application startup.
#[path = "../../api/src/controllers/auth.rs"]
pub mod auth_controller;
#[path = "../../api/src/middleware/authorization.rs"]
pub mod authorization;
#[path = "../../api/src/error.rs"]
mod error;
mod controllers {
    pub use crate::auth_controller as auth;
}
mod middleware {
    pub use crate::authorization;
}

use compallm_postgres::postgres_connection::PostgresConnection;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use compallm_core::{
    auth::{
        AccessToken, AuthError, AuthService, EmailOtpAccessToken, EmailOtpChallenge, RefreshToken,
        Session, TokenLifetimes, UserAccessToken, UserTokens,
    },
    organization::{CreationService, Organization, OrganizationMember, Role},
    persistent::{
        DbContext,
        organization::{OrganizationRepository, OrganizationRepositoryError},
        session::{SessionRepository, SessionRepositoryError},
        user::{UserRepository, UserRepositoryError},
    },
    user::{RegistrationService, User, UserStatus},
};
use sea_orm::{DatabaseConnection, DbBackend, DbErr, MockDatabase, Statement, Transaction};

const SECRET: &str = "test-signing-secret-at-least-32-bytes";
const REFRESH_LIFETIME: Duration = Duration::from_secs(30 * 24 * 60 * 60);

struct TestDictionary {
    dictionary: compallm_core::compression::DCompression,
    newer: Option<compallm_core::compression::DCompression>,
}

struct CountingDictionary {
    inner: TestDictionary,
    reads: Arc<std::sync::atomic::AtomicUsize>,
}

#[async_trait::async_trait]
impl compallm_core::persistent::dictionary::DictionaryRepository for CountingDictionary {
    async fn latest(
        &self,
        kind: compallm_core::compression::DictionaryType,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        compallm_core::persistent::dictionary::DictionaryRepository::latest(&self.inner, kind).await
    }
    async fn get(
        &self,
        kind: compallm_core::compression::DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        compallm_core::persistent::dictionary::DictionaryRepository::get(&self.inner, kind, version)
            .await
    }
}

struct TestEmail;

#[async_trait::async_trait]
impl compallm_core::mail::EmailService for TestEmail {
    async fn send(
        &self,
        _: compallm_core::mail::EmailRequest,
    ) -> Result<(), compallm_core::mail::EmailError> {
        Ok(())
    }
}

trait TestTokenEncoding {
    fn encode_compressed(&self, secret: &str, lifetime: Duration) -> Result<String, AuthError>;
}

fn sign_unchecked(token: &AccessToken, lifetime: Duration) -> String {
    test_codec::sign_unchecked(token, SECRET, lifetime)
}

fn make_oversized(role: &mut Role) {
    let mut state = 0x123456789abcdefu64;
    let mut random = String::new();
    for _ in 0..500 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        random.push_str(&format!("{state:016x}"));
    }
    role.cedar_source = format!("@note(\"{random}\")\n{}", role.cedar_source);
}

impl TestTokenEncoding for AccessToken {
    fn encode_compressed(&self, secret: &str, lifetime: Duration) -> Result<String, AuthError> {
        test_codec::encode(self, secret, lifetime)
    }
}

#[async_trait::async_trait]
impl compallm_core::persistent::dictionary::DictionaryRepository for TestDictionary {
    async fn latest(
        &self,
        _: compallm_core::compression::DictionaryType,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        Ok(Some(self.newer.as_ref().unwrap_or(&self.dictionary)))
    }
    async fn get(
        &self,
        _: compallm_core::compression::DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        Ok(self
            .newer
            .as_ref()
            .filter(|dictionary| dictionary.version == version)
            .or_else(|| (version == self.dictionary.version).then_some(&self.dictionary)))
    }
}

fn decode_token(encoded: &str, secret: &str) -> Result<AccessToken, AuthError> {
    test_codec::decode(encoded, secret)
}

fn test_challenge(email: &str, secret: &str) -> Result<EmailOtpChallenge, AuthError> {
    test_codec::challenge(email, secret)
}

#[derive(Default)]
struct State {
    user: Option<User>,
    contexts: Vec<usize>,
    created: Option<(String, String)>,
    fail_create: bool,
    fail_session: bool,
    change_after_authority_read: bool,
    full_default_policies: bool,
    fail_authority_read: bool,
    freshness_reads: usize,
    authority_reads: usize,
    handled_requests: usize,
    fail_rotation: bool,
    competing_rotation: bool,
    logins: Vec<u64>,
    sessions: std::collections::BTreeMap<u64, Session>,
    organization: Option<Organization>,
    owner: Option<OrganizationMember>,
    fail_organization: bool,
    fail_membership: bool,
    memberships: Vec<OrganizationMember>,
    disabled_organizations: Vec<u64>,
}

#[derive(Clone)]
struct Repository(Arc<Mutex<State>>, DbContext);

struct OrganizationRepositoryStub(Arc<Mutex<State>>);

#[async_trait::async_trait]
impl OrganizationRepository for OrganizationRepositoryStub {
    async fn lock_role(&self, _: u64, _: u64) -> Result<Role, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_role_groups(&self, _: u64, _: u64) -> Result<(), OrganizationRepositoryError> {
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
    async fn find_roles(
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
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        DbContext::current()?;
        let state = self.0.lock().unwrap();
        Ok(state
            .memberships
            .iter()
            .find(|member| {
                member.user_id == user_id
                    && member.organization_id == organization_id
                    && !state
                        .disabled_organizations
                        .contains(&member.organization_id)
            })
            .cloned()
            .map(|mut member| {
                if !state.full_default_policies {
                    // Synthetic bounded policies for transport/lifecycle tests; full defaults have dedicated rejection tests.
                    for role in &mut member.roles {
                        role.cedar_source = format!(r#"permit(principal in Compallm::Role::"{}:{}", action == Compallm::Action::"model.list", resource == Compallm::Organization::"{}");"#, role.organization_id, role.id, role.organization_id);
                    }
                }
                let organization = state
                    .organization
                    .clone()
                    .filter(|o| o.id == member.organization_id)
                    .unwrap_or(Organization {
                        id: member.organization_id,
                        name: if member.organization_id == 84 {
                            "Research team".into()
                        } else {
                            format!("Organization {}", member.organization_id)
                        },
                        status: compallm_core::organization::OrganizationStatus::Active,
                    });
                (member, organization)
            }))
    }

    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        DbContext::current()?;
        let state = self.0.lock().unwrap();
        Ok(state
            .memberships
            .iter()
            .find(|member| {
                member.user_id == user_id
                    && !state
                        .disabled_organizations
                        .contains(&member.organization_id)
            })
            .cloned()
            .map(|mut member| {
                if !state.full_default_policies {
                    // Synthetic bounded policies for transport/lifecycle tests; full defaults have dedicated rejection tests.
                    for role in &mut member.roles {
                        role.cedar_source = format!(r#"permit(principal in Compallm::Role::"{}:{}", action == Compallm::Action::"model.list", resource == Compallm::Organization::"{}");"#, role.organization_id, role.id, role.organization_id);
                    }
                }
                let organization = state
                    .organization
                    .clone()
                    .filter(|o| o.id == member.organization_id)
                    .unwrap_or(Organization {
                        id: member.organization_id,
                        name: if member.organization_id == 84 {
                            "Research team".into()
                        } else {
                            format!("Organization {}", member.organization_id)
                        },
                        status: compallm_core::organization::OrganizationStatus::Active,
                    });
                (member, organization)
            }))
    }

    async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationRepositoryError> {
        let mut state = self.0.lock().unwrap();
        let organization = state
            .organization
            .as_mut()
            .filter(|organization| organization.id == organization_id)
            .ok_or(OrganizationRepositoryError::NotFound)?;
        organization.name = name.into();
        Ok(organization.clone())
    }
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_organization {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "organization insert failed"
            )));
        }
        state.organization = Some(organization.clone());
        Ok(organization)
    }

    async fn create_roles(&self, _roles: Vec<Role>) -> Result<(), OrganizationRepositoryError> {
        Ok(())
    }
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_membership {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "owner insert failed"
            )));
        }
        state.owner = Some(member.clone());
        state.memberships.push(member.clone());
        Ok(member)
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        Ok(state.user.clone().filter(|u| u.email == email))
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        Ok(state.user.clone().filter(|u| u.id == id))
    }

    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        if state.fail_create {
            return Err(UserRepositoryError::database(DbErr::Custom(
                "test storage failure".into(),
            )));
        }
        state.created = Some((user.email.clone(), user.display_name.clone()));
        Ok(user)
    }
}

#[async_trait::async_trait]
impl SessionRepository for Repository {
    async fn authorization_version(
        &self,
        presented: &Session,
    ) -> Result<Option<u32>, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state.freshness_reads += 1;
        if state.fail_authority_read {
            return Err(anyhow::anyhow!("freshness query failed").into());
        }
        Ok(state
            .sessions
            .get(&presented.id)
            .filter(|s| s.is_active(presented))
            .map(|s| s.authorization_version))
    }

    async fn find_policy_membership(
        &self,
        presented: &Session,
    ) -> Result<Option<(Session, OrganizationMember)>, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state.authority_reads += 1;
        if state.fail_authority_read {
            return Err(anyhow::anyhow!("authority query failed").into());
        }
        let Some(mut session) = state
            .sessions
            .get(&presented.id)
            .filter(|s| s.is_active(presented))
            .cloned()
        else {
            return Ok(None);
        };
        if state
            .user
            .as_ref()
            .is_none_or(|u| u.id != session.user_id || u.status != UserStatus::Active)
            || state
                .disabled_organizations
                .contains(&session.organization_id)
        {
            return Ok(None);
        }
        let Some(member) = state
            .memberships
            .iter()
            .find(|m| m.organization_id == session.organization_id && m.user_id == session.user_id)
            .cloned()
        else {
            return Ok(None);
        };
        if let Some(organization) = state
            .organization
            .as_ref()
            .filter(|o| o.id == session.organization_id)
        {
            session.organization_name = organization.name.clone();
        }
        if state.change_after_authority_read {
            state
                .sessions
                .get_mut(&presented.id)
                .unwrap()
                .authorization_version += 1;
            state.memberships.clear();
            state.change_after_authority_read = false;
        }
        Ok(Some((session, member)))
    }

    async fn invalidate_permissions(
        &self,
        organization_id: u64,
        user_ids: &[u64],
    ) -> Result<u64, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_rotation {
            return Err(anyhow::anyhow!("invalidation failed").into());
        }
        let mut count = 0;
        for session in state.sessions.values_mut() {
            if session.organization_id == organization_id
                && user_ids.contains(&session.user_id)
                && session.revoked_at.is_none()
            {
                session.authorization_version += 1;
                count += 1;
            }
        }
        Ok(count)
    }

    async fn create(&self, session: Session) -> Result<(), SessionRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        if state.fail_session {
            return Err(anyhow::anyhow!("session insert failed").into());
        }
        state.logins.push(session.user_id);
        state.sessions.insert(session.id, session);
        Ok(())
    }

    async fn find(&self, session: &Session) -> Result<Option<Session>, SessionRepositoryError> {
        DbContext::current()?;
        Ok(self
            .0
            .lock()
            .unwrap()
            .sessions
            .get(&session.id)
            .filter(|stored| stored.has_same_identity(session))
            .cloned())
    }

    async fn update(
        &self,
        session: &Session,
        previous: &Session,
    ) -> Result<bool, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_rotation {
            return Ok(false);
        }
        if state.competing_rotation {
            state.sessions.get_mut(&session.id).unwrap().organization_id = 85;
            state.competing_rotation = false;
            return Ok(false);
        }
        let Some(current) = state.sessions.get_mut(&session.id) else {
            return Ok(false);
        };
        if !current.has_same_identity(previous) || current.revoked_at != previous.revoked_at {
            return Ok(false);
        }
        if session.organization_id != previous.organization_id {
            current.organization_id = session.organization_id;
        }
        current.revoked_at = session.revoked_at;
        Ok(true)
    }
}

struct Fixture {
    db: Arc<DatabaseConnection>,
    repo: Repository,
}

impl Fixture {
    fn otp(challenge: &EmailOtpChallenge) -> compallm_core::auth::EmailOtpAccessToken {
        compallm_core::auth::EmailOtpAccessToken::try_from(
            decode_token(&challenge.access_token, SECRET).unwrap(),
        )
        .unwrap()
    }

    fn new(user: Option<User>) -> Self {
        match hero::distributed_id::init_scoped_id_generator("auth-tests") {
            Ok(())
            | Err(hero::distributed_id::generator::DistributedIdError::AlreadyInitialized) => {}
            Err(error) => panic!("{error}"),
        }
        let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
        let memberships = user
            .as_ref()
            .map(|user| {
                vec![OrganizationMember {
                    organization_id: 84,
                    user_id: user.id,
                    roles: vec![Fixture::role(84, 0)],
                    groups: Vec::new(),
                }]
            })
            .unwrap_or_default();
        let repo = Repository(
            Arc::new(Mutex::new(State {
                user,
                memberships,
                ..State::default()
            })),
            DbContext::new(Arc::new(PostgresConnection::new(db.clone()))),
        );
        Self { db, repo }
    }

    fn service(&self) -> AuthService {
        self.service_with_dictionaries(Box::new(TestDictionary {
            dictionary: compallm_core::compression::DCompression::new(
                compallm_core::compression::DictionaryType::AccessToken,
                1,
                "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
            ).unwrap(),
            newer: None,
        }))
    }

    fn service_with_dictionaries(
        &self,
        dictionary: Box<dyn compallm_core::persistent::dictionary::DictionaryRepository>,
    ) -> AuthService {
        self.service_with_dependencies(dictionary, None)
    }

    fn service_with_dependencies(
        &self,
        dictionary: Box<dyn compallm_core::persistent::dictionary::DictionaryRepository>,
        email: Option<&'static Box<dyn compallm_core::mail::EmailService>>,
    ) -> AuthService {
        // Static dependencies mirror the application container. Test-only
        // allocations live for this test process and never touch a real DB.
        let db: &'static DbContext = Box::leak(Box::new(self.repo.1.clone()));
        let users = Box::leak(Box::new(
            Box::new(self.repo.clone()) as Box<dyn UserRepository>
        ));
        let organizations = Box::leak(Box::new(Box::new(OrganizationRepositoryStub(
            self.repo.0.clone(),
        )) as Box<dyn OrganizationRepository>));
        let creation = Box::leak(Box::new(CreationService::new(db, organizations)));
        let registration = Box::leak(Box::new(RegistrationService::new(db, users, creation)));
        let dictionaries = Box::leak(Box::new(dictionary));
        AuthService::new(
            db,
            users,
            organizations,
            Box::leak(Box::new(
                Box::new(self.repo.clone()) as Box<dyn SessionRepository>
            )),
            dictionaries,
            registration,
            SECRET.into(),
            TokenLifetimes {
                user: Duration::from_secs(60),
                refresh: REFRESH_LIFETIME,
            },
            email,
        )
        .unwrap()
    }

    async fn refresh(&self, token: &str) -> Result<UserTokens, AuthError> {
        self.repo.1.scope(self.service().refresh(token)).await
    }

    async fn signin(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        self.repo.1.scope(self.service().signin(claims, code)).await
    }

    async fn signup_in_transaction(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        // The caller keeps account creation and the first session atomic.
        self.repo
            .1
            .transaction(self.service().signup(claims, code))
            .await
    }

    async fn register(&self, email: &str) -> Result<User, compallm_core::user::RegistrationError> {
        self.repo.1.scope(self.registration().register(email)).await
    }

    async fn switch_organization(
        &self,
        token: &str,
        organization_id: u64,
    ) -> Result<UserTokens, AuthError> {
        self.repo
            .1
            .scope(self.service().switch_organization(token, organization_id))
            .await
    }

    fn remember(&self, token: &str) {
        let refresh = RefreshToken::try_from(decode_token(token, SECRET).unwrap()).unwrap();
        self.repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(refresh.session.id, refresh.session.clone());
    }

    fn registration(&self) -> RegistrationService {
        let db = Box::leak(Box::new(self.repo.1.clone()));
        let users = Box::leak(Box::new(
            Box::new(self.repo.clone()) as Box<dyn UserRepository>
        ));
        let organizations = Box::leak(Box::new(Box::new(OrganizationRepositoryStub(
            self.repo.0.clone(),
        )) as Box<dyn OrganizationRepository>));
        let creation = Box::leak(Box::new(CreationService::new(db, organizations)));
        RegistrationService::new(db, users, creation)
    }

    fn role(organization_id: u64, index: usize) -> Role {
        let mut role = Role {
            id: (index + 1) as u64,
            organization_id,
            name: ["Owner", "Admin", "Member"][index].into(),
            description: String::new(),
            cedar_source: String::new(),
            org_owner: index == 0,
        };
        role.cedar_source = format!(
            r#"permit(principal in Compallm::Role::"{}:{}", action == Compallm::Action::"model.list", resource == Compallm::Organization::"{}");"#,
            organization_id, role.id, organization_id
        );
        role
    }

    fn user(status: UserStatus) -> User {
        User {
            id: 1,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            status,
        }
    }

    fn assert_transaction(self, finish: &str) {
        drop(self.repo);
        let mut statements = vec![Statement::from_string(DbBackend::Postgres, "BEGIN")];
        statements.push(Statement::from_string(DbBackend::Postgres, finish));
        assert_eq!(
            self.db.as_ref().clone().into_transaction_log(),
            vec![Transaction::many(statements)]
        );
    }

    fn assert_no_transaction(&self) {
        assert!(self.db.as_ref().clone().into_transaction_log().is_empty());
    }
}

#[tokio::test]
async fn direct_registration_commits_user_organization_and_owner_together() {
    let fixture = Fixture::new(None);
    let user = fixture.register("james@midwess.com").await.unwrap();
    {
        let state = fixture.repo.0.lock().unwrap();
        assert!(state.created.is_some());
        let organization = state.organization.as_ref().unwrap();
        let owner = state.owner.as_ref().unwrap();
        assert_eq!(owner.user_id, user.id);
        assert_eq!(owner.organization_id, organization.id);
    }
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn direct_registration_rolls_back_when_default_organization_or_owner_fails() {
    for fail_membership in [false, true] {
        let fixture = Fixture::new(None);
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.fail_organization = !fail_membership;
            state.fail_membership = fail_membership;
        }
        assert!(matches!(
            fixture.register("james@midwess.com").await,
            Err(compallm_core::user::RegistrationError::Organization(_))
        ));
        assert!(fixture.repo.0.lock().unwrap().created.is_some());
        assert!(fixture.repo.0.lock().unwrap().owner.is_none());
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn signin_checks_user_and_records_session_without_starting_a_transaction() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let issued = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    assert_eq!(
        decode_token(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .session
            .user_id,
        1
    );
    let contexts = fixture.repo.0.lock().unwrap().contexts.clone();
    let access =
        UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(access.session.organization_id, 84);
    assert_eq!(contexts.len(), 2);
    assert_eq!(fixture.repo.0.lock().unwrap().logins, [1]);
    assert!(contexts.iter().all(|context| *context == contexts[0]));
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn issued_tokens_use_signed_versioned_compression() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let service = fixture.service();
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    for (encoded, expected) in [
        (&challenge.access_token, "otp"),
        (&tokens.access_token, "user"),
        (&tokens.refresh_token, "refresh"),
    ] {
        use base64::Engine;
        use coset::{
            CborSerializable, CoseMac0,
            cwt::{ClaimName, ClaimsSet, Timestamp},
            iana,
        };
        use hmac::Mac;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded)
            .unwrap();
        let cose = CoseMac0::from_slice(&bytes).unwrap();
        assert_eq!(
            cose.protected.header.alg,
            Some(iana::Algorithm::HMAC_256_256.into())
        );
        cose.verify_payload_tag(
            b"",
            || (),
            |tag, message| {
                let mut mac =
                    <hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(SECRET.as_bytes()).unwrap();
                mac.update(message);
                mac.verify_slice(tag).map_err(|_| ())
            },
        )
        .unwrap();
        let claims = ClaimsSet::from_slice(cose.payload.as_deref().unwrap()).unwrap();
        assert!(matches!(
            claims.expiration_time,
            Some(Timestamp::WholeSeconds(_))
        ));
        assert!(matches!(claims.issued_at, Some(Timestamp::WholeSeconds(_))));
        assert!(matches!(
            &claims.rest[..],
            [(
                ClaimName::PrivateUse(-65537),
                ciborium::value::Value::Bytes(_)
            )]
        ));
        let token = fixture
            .repo
            .1
            .scope(service.authenticate_token(encoded))
            .await
            .unwrap();
        assert!(matches!(
            (expected, token),
            ("otp", AccessToken::EmailOtp(_))
                | ("user", AccessToken::User(_))
                | ("refresh", AccessToken::Refresh(_))
        ));
        let mut tampered = encoded.as_bytes().to_vec();
        let index = tampered.len() - 4;
        tampered[index] = if tampered[index] == b'a' { b'b' } else { b'a' };
        assert!(
            fixture
                .repo
                .1
                .scope(service.authenticate_token(std::str::from_utf8(&tampered).unwrap()))
                .await
                .is_err()
        );
    }
    let missing = AccessToken::Compressed(compallm_core::auth::token::CompressedToken {
        dictionary_version: 999,
        data: vec![1],
    });
    let encoded = test_codec::sign_raw(&missing, SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&encoded))
            .await,
        Err(AuthError::DictionaryUnavailable)
    ));
    let legacy = test_codec::sign_raw(
        &AccessToken::Refresh(
            RefreshToken::from_session(&stored_session(1, 7, 84, "Team"), REFRESH_LIFETIME)
                .unwrap(),
        ),
        SECRET,
        REFRESH_LIFETIME,
    );
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&legacy))
            .await,
        Ok(AccessToken::Refresh(_))
    ));
    let wrong = compallm_core::compression::DCompression::new(
        compallm_core::compression::DictionaryType::AccessToken,
        1,
        "a different dictionary",
    )
    .unwrap();
    let payload = AccessToken::Compressed(compallm_core::auth::token::CompressedToken {
        dictionary_version: 1,
        data: wrong.compress_bytes(b"{}").await.unwrap(),
    });
    let encoded = test_codec::sign_raw(&payload, SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&encoded))
            .await,
        Err(AuthError::InvalidToken)
    ));
}

#[tokio::test]
async fn email_challenge_uses_compressed_wrapper_and_keeps_otp_encrypted() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let dictionary = Box::new(TestDictionary {
        dictionary: compallm_core::compression::DCompression::new(
            compallm_core::compression::DictionaryType::AccessToken, 1,
            "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
        ).unwrap(),
        newer: None,
    });
    let email = Box::leak(Box::new(
        Box::new(TestEmail) as Box<dyn compallm_core::mail::EmailService>
    ));
    let service = fixture.service_with_dependencies(dictionary, Some(email));
    let issued = fixture
        .repo
        .1
        .scope(service.signin_email("james@midwess.com"))
        .await
        .unwrap();
    let token = fixture
        .repo
        .1
        .scope(service.authenticate_token(&issued.token))
        .await
        .unwrap();
    assert!(matches!(token, AccessToken::EmailOtp(_)));
    let envelope = test_codec::envelope(&issued.token);
    let AccessToken::Compressed(envelope) = envelope else {
        panic!("expected compressed token")
    };
    let dictionary = compallm_core::compression::DCompression::new(
        compallm_core::compression::DictionaryType::AccessToken,
        1,
        "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
    )
    .unwrap();
    let source = dictionary.decompress_bytes(&envelope.data).await.unwrap();
    assert!(
        !source
            .windows("james@midwess.com".len())
            .any(|part| part == b"james@midwess.com")
    );
    assert!(matches!(
        ciborium::de::from_reader::<AccessToken, _>(source.as_slice()).unwrap(),
        AccessToken::Encrypted(_)
    ));
}

#[tokio::test]
async fn refresh_reads_old_dictionary_and_issues_with_latest() {
    use compallm_core::compression::{DCompression, DictionaryType};
    let original =
        "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source";
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let rotated = fixture.service_with_dictionaries(Box::new(TestDictionary {
        dictionary: DCompression::new(DictionaryType::AccessToken, 1, original).unwrap(),
        newer: Some(
            DCompression::new(
                DictionaryType::AccessToken,
                2,
                &format!("{original} more vocabulary"),
            )
            .unwrap(),
        ),
    }));
    let renewed = fixture
        .repo
        .1
        .scope(rotated.refresh(&tokens.refresh_token))
        .await
        .unwrap();
    for token in [&renewed.access_token, &renewed.refresh_token] {
        let payload = test_codec::envelope(token);
        let AccessToken::Compressed(payload) = payload else {
            panic!("expected compressed token")
        };
        assert_eq!(payload.dictionary_version, 2);
        assert!(
            fixture
                .repo
                .1
                .scope(rotated.authenticate_token(token))
                .await
                .is_ok()
        );
    }
    assert!(
        fixture
            .repo
            .1
            .scope(rotated.authenticate_token(&tokens.access_token))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn invalid_signature_is_rejected_before_dictionary_lookup() {
    use base64::Engine;
    use coset::{
        CborSerializable, CoseMac0Builder, HeaderBuilder,
        cwt::{ClaimsSetBuilder, Timestamp},
        iana,
    };
    use hmac::Mac;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let reads = Arc::new(AtomicUsize::new(0));
    let service = fixture.service_with_dictionaries(Box::new(CountingDictionary {
        inner: TestDictionary { dictionary: compallm_core::compression::DCompression::new(
            compallm_core::compression::DictionaryType::AccessToken, 1,
            "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
        ).unwrap(), newer: None },
        reads: reads.clone(),
    }));
    let mut tampered = challenge.access_token.as_bytes().to_vec();
    let index = tampered.len() - 4;
    tampered[index] = if tampered[index] == b'a' { b'b' } else { b'a' };
    assert!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(std::str::from_utf8(&tampered).unwrap()))
            .await
            .is_err()
    );
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    let now = compallm_utils::time::unix_timestamp() as i64;
    let claims = ClaimsSetBuilder::new()
        .issued_at(Timestamp::WholeSeconds(now - 2))
        .expiration_time(Timestamp::WholeSeconds(now - 1))
        .private_claim(-65537, ciborium::value::Value::Bytes(vec![0]))
        .build()
        .to_vec()
        .unwrap();
    let expired = CoseMac0Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::HMAC_256_256)
                .build(),
        )
        .payload(claims)
        .create_tag(b"", |message| {
            let mut mac =
                <hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(SECRET.as_bytes()).unwrap();
            mac.update(message);
            mac.finalize().into_bytes().to_vec()
        })
        .build()
        .to_vec()
        .unwrap();
    let expired = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(expired);
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&expired))
            .await,
        Err(AuthError::ExpiredToken)
    ));
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    let user = fixture.access_for_session(stored_session(1, 7, 84, "Team"));
    let raw = test_codec::sign_raw(&AccessToken::User(user), SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture.repo.1.scope(service.authenticate_token(&raw)).await,
        Ok(AccessToken::User(_))
    ));
    let otp = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    let encrypted = AccessToken::EmailOtp(otp)
        .encrypt(&compallm_core::auth::EncryptedAccessToken::new(SECRET).unwrap())
        .await
        .unwrap();
    let raw = test_codec::sign_raw(&encrypted, SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture.repo.1.scope(service.authenticate_token(&raw)).await,
        Ok(AccessToken::EmailOtp(_))
    ));
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    assert!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&challenge.access_token))
            .await
            .is_ok()
    );
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn signup_requires_verified_email_and_uses_one_caller_transaction() {
    let fixture = Fixture::new(None);
    let challenge = test_challenge("James@MIDWESS.COM", SECRET).unwrap();
    let issued = fixture
        .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    assert!(
        decode_token(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .session
            .user_id
            > 0
    );
    {
        let state = fixture.repo.0.lock().unwrap();
        assert_eq!(
            state.created,
            Some(("james@midwess.com".into(), "james".into()))
        );
        assert_eq!(state.contexts.len(), 3);
        assert_eq!(state.logins.len(), 1);
        let organization = state.organization.as_ref().unwrap();
        let owner = state.owner.as_ref().unwrap();
        assert_eq!(organization.name, "james's organization");
        assert!(organization.id > 0);
        assert_eq!(organization.id, owner.organization_id);
        let access =
            UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
        assert_eq!(access.session.organization_id, organization.id);
        assert!(
            state
                .contexts
                .iter()
                .all(|context| *context == state.contexts[0])
        );
    }
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn organization_or_owner_failure_rolls_back_the_entire_signup() {
    for fail_membership in [false, true] {
        let fixture = Fixture::new(None);
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.fail_organization = !fail_membership;
            state.fail_membership = fail_membership;
        }
        let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
        assert!(matches!(
            fixture
                .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
                .await,
            Err(AuthError::Organization(_))
        ));
        assert!(fixture.repo.0.lock().unwrap().owner.is_none());
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn duplicate_signup_and_disabled_signin_are_rejected() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::Persistence(
            UserRepositoryError::EmailAlreadyExists
        ))
    ));
    assert!(fixture.repo.0.lock().unwrap().created.is_none());
    fixture.assert_transaction("ROLLBACK");

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Disabled)));
    assert!(matches!(
        fixture
            .signin(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::UserDisabled)
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn failed_session_insert_rejects_signin_without_returning_tokens() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().fail_session = true;
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signin(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::SessionPersistence(
            SessionRepositoryError::Database(_)
        ))
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn failed_repository_operation_rolls_back_and_returns_no_tokens() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_create = true;
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::Persistence(UserRepositoryError::Database(_)))
    ));
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn invalid_otp_does_not_reach_persistence() {
    let fixture = Fixture::new(None);
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture.signin(&Fixture::otp(&challenge), "wrong").await,
        Err(AuthError::IncorrectCode)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn signup_derives_user_name_and_default_organization_from_the_email() {
    for local_part in [
        "James.Dang+dev".to_string(),
        "!+~".to_string(),
        "a".repeat(64),
    ] {
        let fixture = Fixture::new(None);
        let email = format!("{local_part}@midwess.com");
        let challenge = test_challenge(&email, SECRET).unwrap();
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await
            .unwrap();
        {
            let state = fixture.repo.0.lock().unwrap();
            let name = local_part.to_ascii_lowercase();
            assert_eq!(
                state.created.as_ref().unwrap().0,
                email.to_ascii_lowercase()
            );
            assert_eq!(state.created.as_ref().unwrap().1, name);
            let prefix: String = name
                .chars()
                .take(Organization::MAX_NAME_LENGTH - "'s organization".len())
                .collect();
            let organization = state.organization.as_ref().unwrap();
            assert_eq!(organization.name, format!("{prefix}'s organization"));
            assert!(organization.name.chars().count() <= Organization::MAX_NAME_LENGTH);
        }
        fixture.assert_transaction("COMMIT");
    }
}

#[tokio::test]
async fn refresh_requires_a_context_from_the_application_entry_point() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let service = fixture.service();
    let tokens = token_pair::compressed_new(
        &stored_session(1, 7, 84, "Research team"),
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();

    assert!(matches!(
        service.refresh(&tokens.refresh_token).await,
        Err(AuthError::SessionPersistence(_))
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());

    fixture.remember(&tokens.refresh_token);
    fixture
        .repo
        .1
        .scope(service.refresh(&tokens.refresh_token))
        .await
        .unwrap();
    assert_eq!(fixture.repo.0.lock().unwrap().contexts.len(), 0);
    assert!(DbContext::current().is_err());
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn refresh_checks_current_user_and_renews_both_tokens_for_another_month() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = token_pair::compressed_new(
        &stored_session(1, 7, 84, "Research team"),
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let mut previous =
        RefreshToken::try_from(decode_token(&original.refresh_token, SECRET).unwrap()).unwrap();
    // Simulate a credential issued 29 days ago without waiting or replacing the clock.
    let elapsed = 29 * 24 * 60 * 60;
    previous.validity.issued_at -= elapsed;
    previous.validity.expires_at -= elapsed;
    let original_session = previous.session.clone();
    let previous_expiry = previous.validity.expires_at;
    let old_refresh = AccessToken::Refresh(previous)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&old_refresh);
    let before_refresh = compallm_utils::time::unix_timestamp();
    let issued = fixture.refresh(&old_refresh).await.unwrap();
    let after_refresh = compallm_utils::time::unix_timestamp();
    let renewed =
        RefreshToken::try_from(decode_token(&issued.refresh_token, SECRET).unwrap()).unwrap();
    assert_ne!(issued.refresh_token, old_refresh);
    assert_eq!(renewed.session, original_session);
    assert_eq!(renewed.session.user_id, 1);
    assert!(renewed.validity.issued_at >= before_refresh);
    assert!(renewed.validity.issued_at <= after_refresh);
    assert_eq!(
        renewed.validity.expires_at - renewed.validity.issued_at,
        REFRESH_LIFETIME.as_secs()
    );
    assert!(renewed.validity.expires_at > previous_expiry);
    assert_eq!(renewed.session.id, 7);
    {
        let state = fixture.repo.0.lock().unwrap();
        let session = state.sessions.get(&7).unwrap();
        assert_eq!(session, &renewed.session);
        assert_eq!(state.sessions.len(), 1);
        assert!(state.logins.is_empty(), "refresh is not a new login");
    }
    assert_eq!(
        decode_token(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .session
            .user_id,
        1
    );
    fixture.repo.0.lock().unwrap().user.as_mut().unwrap().status = UserStatus::Disabled;
    assert!(matches!(
        fixture.refresh(&issued.refresh_token).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(matches!(
        fixture.refresh(&original.access_token).await,
        Err(AuthError::WrongTokenVariant)
    ));
}

#[tokio::test]
async fn renewal_refreshes_organization_name_in_both_tokens_without_a_session_write() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().organization = Some(Organization {
        id: 84,
        name: "Original team".into(),
        status: compallm_core::organization::OrganizationStatus::Active,
    });
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let original = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let before = fixture
        .service()
        .authenticate_access(&original.access_token)
        .await
        .unwrap();
    assert_eq!(before.session.organization_name, "Original team");
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .organization
        .as_mut()
        .unwrap()
        .name = "Renamed team".into();
    let tokens = fixture.refresh(&original.refresh_token).await.unwrap();
    let access = fixture
        .service()
        .authenticate_access(&tokens.access_token)
        .await
        .unwrap();
    let refresh =
        RefreshToken::try_from(decode_token(&tokens.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(access.session.organization_name, "Renamed team");
    assert_eq!(refresh.session.organization_name, "Renamed team");
    assert!(access.session.has_same_identity(&before.session));
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&before.session.id].organization_name,
        "Original team"
    );
    assert_eq!(
        fixture
            .service()
            .authenticate_access(&original.access_token)
            .await
            .unwrap()
            .session
            .organization_name,
        "Original team"
    );
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn expired_refresh_requires_signin_before_reading_the_user() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut expired =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let elapsed = REFRESH_LIFETIME.as_secs() + 1;
    expired.validity.issued_at -= elapsed;
    expired.validity.expires_at -= elapsed;
    // A valid JWT envelope must not bypass the expired inner claims.
    let encoded = sign_unchecked(&AccessToken::Refresh(expired), REFRESH_LIFETIME);
    assert!(matches!(
        fixture.refresh(&encoded).await,
        Err(AuthError::ExpiredToken)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
}

#[tokio::test]
async fn signin_selects_first_active_membership_independent_of_roles() {
    let fixture_roles_0 = default_roles(80, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().memberships = vec![
        OrganizationMember {
            organization_id: 80,
            user_id: 1,
            roles: vec![fixture_roles_0.clone().remove(2)],
            groups: Vec::new(),
        },
        OrganizationMember {
            organization_id: 85,
            user_id: 1,
            roles: vec![Fixture::role(85, 1)],
            groups: Vec::new(),
        },
    ];
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 80);
    assert_eq!(claims.policy_snapshot.role_ids, vec![3]);
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn organization_switch_is_separate_and_uses_current_membership_roles() {
    let fixture_roles_0 = default_roles(85, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .memberships
        .push(OrganizationMember {
            organization_id: 85,
            user_id: 1,
            roles: vec![fixture_roles_0.clone().remove(2)],
            groups: Vec::new(),
        });
    let refresh = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&refresh);
    let tokens = fixture.switch_organization(&refresh, 85).await.unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 85);
    let refreshed =
        RefreshToken::try_from(decode_token(&tokens.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session, refreshed.session);
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].organization_id,
        85
    );
    assert!(matches!(
        fixture.switch_organization(&refresh, 84).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    // Membership changes are checked again instead of copying role from an old JWT.
    fixture.repo.0.lock().unwrap().memberships[1].roles = vec![Fixture::role(85, 1)];
    let tokens = fixture
        .switch_organization(&tokens.refresh_token, 85)
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.policy_snapshot.role_ids, vec![2]);
    assert!(matches!(
        fixture
            .switch_organization(&tokens.refresh_token, 999)
            .await,
        Err(AuthError::OrganizationAccessDenied)
    ));
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .disabled_organizations
        .push(85);
    assert!(matches!(
        fixture.switch_organization(&tokens.refresh_token, 85).await,
        Err(AuthError::OrganizationAccessDenied)
    ));
}

#[tokio::test]
async fn signin_rejects_missing_or_disabled_memberships() {
    for disabled in [false, true] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        {
            let mut state = fixture.repo.0.lock().unwrap();
            if disabled {
                state.disabled_organizations.push(84);
            } else {
                state.memberships.clear();
            }
        }
        let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
        assert!(matches!(
            fixture
                .signin(&Fixture::otp(&challenge), &challenge.code)
                .await,
            Err(AuthError::OrganizationAccessDenied)
        ));
        assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
        fixture.assert_no_transaction();
    }
}

#[tokio::test]
async fn refresh_without_selection_keeps_the_session_organization() {
    let fixture_roles_0 = default_roles(80, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().memberships.insert(
        0,
        OrganizationMember {
            organization_id: 80,
            user_id: 1,
            roles: vec![fixture_roles_0.clone().remove(2)],
            groups: Vec::new(),
        },
    );
    let refresh = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&refresh);
    let tokens = fixture.refresh(&refresh).await.unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 84);
}

#[tokio::test]
async fn original_and_renewed_refresh_tokens_remain_usable_without_database_writes() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&original);
    let before = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    let replacement = fixture.refresh(&original).await.unwrap();
    assert!(fixture.refresh(&original).await.is_ok());
    assert!(fixture.refresh(&replacement.refresh_token).await.is_ok());
    let stored = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert_eq!(stored, before);
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn session_insert_failure_rolls_back_signup_and_returns_no_credentials() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_session = true;
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::SessionPersistence(_))
    ));
    assert!(fixture.repo.0.lock().unwrap().sessions.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn each_successful_signin_creates_an_independent_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let one = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let two = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let one = RefreshToken::try_from(decode_token(&one.refresh_token, SECRET).unwrap()).unwrap();
    let two = RefreshToken::try_from(decode_token(&two.refresh_token, SECRET).unwrap()).unwrap();
    assert_ne!(one.session.id, two.session.id);
    assert_eq!(fixture.repo.0.lock().unwrap().sessions.len(), 2);
    fixture
        .repo
        .1
        .scope(async {
            let previous = SessionRepository::find(&fixture.repo, &one.session)
                .await
                .unwrap()
                .unwrap();
            let mut revoked = previous.clone();
            revoked.revoke();
            assert!(
                SessionRepository::update(&fixture.repo, &revoked, &previous)
                    .await
                    .unwrap()
            );
        })
        .await;
    let one = AccessToken::Refresh(one)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    let two = AccessToken::Refresh(two)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&one).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(fixture.refresh(&two).await.is_ok());
}

#[tokio::test]
async fn missing_revoked_or_foreign_sessions_cannot_refresh() {
    for state in ["missing", "revoked", "foreign"] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let refresh = AccessToken::Refresh(
            RefreshToken::from_session(
                &stored_session(1, 7, 84, "Research team"),
                REFRESH_LIFETIME,
            )
            .unwrap(),
        )
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
        fixture.remember(&refresh);
        {
            let mut data = fixture.repo.0.lock().unwrap();
            match state {
                "missing" => {
                    data.sessions.clear();
                }
                "revoked" => {
                    data.sessions.get_mut(&7).unwrap().revoked_at =
                        Some(compallm_utils::time::unix_timestamp());
                }
                "foreign" => {
                    data.sessions.get_mut(&7).unwrap().user_id = 2;
                }
                _ => unreachable!(),
            }
        }
        assert!(
            matches!(
                fixture.refresh(&refresh).await,
                Err(AuthError::InvalidSession)
            ),
            "accepted {state} session"
        );
        assert!(
            fixture.repo.0.lock().unwrap().contexts.is_empty(),
            "invalid session must fail before user lookup"
        );
        assert!(
            fixture
                .db
                .as_ref()
                .clone()
                .into_transaction_log()
                .is_empty()
        );
    }
}

#[tokio::test]
async fn invalid_signature_cannot_revoke_a_valid_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let valid = AccessToken::Refresh(claims.clone())
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&valid);
    let forged = AccessToken::Refresh(claims)
        .encode_compressed("wrong-key-at-least-32-bytes-long", REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&forged).await,
        Err(AuthError::InvalidToken)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn read_only_refresh_runs_inside_an_enclosing_transaction() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let token = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&token);
    fixture
        .repo
        .1
        .transaction(async {
            fixture.service().refresh(&token).await?;
            Ok::<_, AuthError>(())
        })
        .await
        .unwrap();
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn failed_organization_switch_returns_no_credentials_and_preserves_the_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let token = AccessToken::Refresh(claims.clone())
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&token);
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    assert!(matches!(
        fixture.switch_organization(&token, 84).await,
        Err(AuthError::InvalidSession)
    ));
    assert_eq!(fixture.repo.0.lock().unwrap().sessions[&7], claims.session);
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn competing_organization_switch_rejects_the_loser_without_revoking_the_winner() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let token = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&token);
    fixture.repo.0.lock().unwrap().competing_rotation = true;
    assert!(matches!(
        fixture.switch_organization(&token, 84).await,
        Err(AuthError::InvalidSession)
    ));
    let stored = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert!(stored.revoked_at.is_none());
    assert_eq!(stored.organization_id, 85);
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn token_for_another_organization_cannot_read_or_revoke_an_owned_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let valid = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&valid);
    let foreign = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 85, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    assert!(matches!(
        fixture.refresh(&foreign).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(fixture.refresh(&valid).await.is_ok());
}

#[tokio::test]
async fn organization_switch_joins_an_enclosing_transaction() {
    let fixture_roles_0 = default_roles(85, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&original);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .memberships
        .push(OrganizationMember {
            organization_id: 85,
            user_id: 1,
            roles: vec![fixture_roles_0.clone().remove(2)],
            groups: Vec::new(),
        });
    let service = fixture.service();
    let tokens = fixture
        .repo
        .1
        .transaction(service.switch_organization(&original, 85))
        .await
        .unwrap();
    let claims =
        RefreshToken::try_from(decode_token(&tokens.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 85);
    assert!(matches!(
        fixture.refresh(&original).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn expired_refresh_never_writes_or_revokes_a_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut token =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let at = token.validity.issued_at;
    let stored = token.session.clone();
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .insert(7, stored.clone());
    token.validity.issued_at = at - REFRESH_LIFETIME.as_secs() - 1;
    token.validity.expires_at = at - 1;
    let expired = sign_unchecked(&AccessToken::Refresh(token), REFRESH_LIFETIME);
    assert!(matches!(
        fixture.refresh(&expired).await,
        Err(AuthError::ExpiredToken)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn signout_revokes_only_the_selected_session_and_is_repeatable() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let selected = stored_session(1, 7, 84, "Research team");
    let other = stored_session(1, 8, 84, "Research team");
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.sessions.insert(7, selected.clone());
        state.sessions.insert(8, other);
    }
    let claims =
        UserAccessToken::new(selected.clone(), empty_snapshot(), Duration::from_secs(60)).unwrap();
    let service = fixture.service();
    fixture
        .repo
        .1
        .scope(service.signout(&claims))
        .await
        .unwrap();
    fixture
        .repo
        .1
        .scope(service.signout(&claims))
        .await
        .unwrap();
    {
        let state = fixture.repo.0.lock().unwrap();
        assert!(state.sessions[&7].revoked_at.is_some());
        assert!(state.sessions[&8].revoked_at.is_none());
    }
    let refresh =
        AccessToken::Refresh(RefreshToken::from_session(&selected, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    assert!(matches!(
        fixture.refresh(&refresh).await,
        Err(AuthError::InvalidSession)
    ));
}

#[tokio::test]
async fn signout_cannot_revoke_a_foreign_user_or_organization_session() {
    for presented in [
        stored_session(2, 7, 84, "Research team"),
        stored_session(1, 7, 85, "Research team"),
    ] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(7, stored_session(1, 7, 84, "Research team"));
        let claims =
            UserAccessToken::new(presented, empty_snapshot(), Duration::from_secs(60)).unwrap();
        assert!(matches!(
            fixture
                .repo
                .1
                .scope(fixture.service().signout(&claims))
                .await,
            Err(AuthError::InvalidSession)
        ));
        assert!(
            fixture.repo.0.lock().unwrap().sessions[&7]
                .revoked_at
                .is_none()
        );
    }
}

#[tokio::test]
async fn signin_email_rejects_unknown_accounts_before_mail_admission() {
    let fixture = Fixture::new(None);
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(fixture.service().signin_email("new@example.com"))
            .await,
        Err(AuthError::UserNotFound)
    ));
}

#[tokio::test]
async fn signin_email_rejects_disabled_accounts_before_mail_admission() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Disabled)));
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(fixture.service().signin_email("james@midwess.com"))
            .await,
        Err(AuthError::UserDisabled)
    ));
}

#[tokio::test]
async fn signin_email_normalizes_existing_accounts_before_mail_admission() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    // This fixture has no mail service: only an accepted existing user reaches it.
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(fixture.service().signin_email(" James@MIDWESS.COM "))
            .await,
        Err(AuthError::Email(
            compallm_core::mail::EmailError::Unavailable
        ))
    ));
}

#[tokio::test]
async fn refresh_preserves_coherent_session_and_authority_without_restamping() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let presented = stored_session(1, 7, 84, "Research team");
    {
        let mut state = fixture.repo.0.lock().unwrap();
        let mut stored = presented.clone();
        stored.authorization_version = 4;
        state.sessions.insert(7, stored);
        state.memberships[0].roles.clear();
        state.memberships[0]
            .groups
            .push(compallm_core::organization::Group {
                id: 9,
                organization_id: 84,
                name: "Team".into(),
                description: String::new(),
                roles: vec![fixture_roles_0.clone().remove(2)],
            });
        state.change_after_authority_read = true;
    }
    let service = fixture.service();
    let refresh =
        AccessToken::Refresh(RefreshToken::from_session(&presented, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    let issued = fixture
        .repo
        .1
        .scope(service.refresh(&refresh))
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
    let session = claims.session;
    let snapshot = claims.policy_snapshot;
    assert_eq!(session.authorization_version, 4);
    assert!(snapshot.role_ids.is_empty());
    assert_eq!(snapshot.group_roles[&9], vec![3]);
    assert!(!snapshot.cedar_source.is_empty());
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].authorization_version,
        5
    );
    assert!(fixture.repo.0.lock().unwrap().memberships.is_empty());
    assert!(matches!(
        fixture.repo.1.scope(service.refresh(&refresh)).await,
        Err(AuthError::InvalidSession)
    ));
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn refresh_rejects_unavailable_invalid_or_oversized_authority() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    for case in [
        "missing user",
        "disabled user",
        "missing member",
        "disabled organization",
        "missing session",
        "revoked session",
        "foreign organization",
        "foreign user",
        "future version",
        "oversized",
        "database failure",
    ] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let mut presented = stored_session(1, 7, 84, "Research team");
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.sessions.insert(7, presented.clone());
            match case {
                "missing user" => state.user = None,
                "disabled user" => state.user.as_mut().unwrap().status = UserStatus::Disabled,
                "missing member" => state.memberships.clear(),
                "disabled organization" => state.disabled_organizations.push(84),
                "missing session" => state.sessions.clear(),
                "revoked session" => state.sessions.get_mut(&7).unwrap().revoke(),
                "foreign organization" => presented.organization_id = 99,
                "foreign user" => presented.user_id = 99,
                "future version" => presented.authorization_version = 2,
                "oversized" => {
                    state.memberships[0].roles = vec![fixture_roles_0.clone().remove(0)];
                    make_oversized(&mut state.memberships[0].roles[0]);
                    state.full_default_policies = true;
                }
                "database failure" => state.fail_authority_read = true,
                _ => unreachable!(),
            }
        }
        let refresh =
            AccessToken::Refresh(RefreshToken::from_session(&presented, REFRESH_LIFETIME).unwrap())
                .encode_compressed(SECRET, REFRESH_LIFETIME)
                .unwrap();
        let result = fixture.refresh(&refresh).await;
        match case {
            "oversized" => assert!(matches!(result, Err(AuthError::TokenTooLarge { .. }))),
            "database failure" => assert!(matches!(result, Err(AuthError::SessionPersistence(_)))),
            _ => assert!(matches!(result, Err(AuthError::InvalidSession)), "{case}"),
        }
        fixture.assert_no_transaction();
    }
}

#[tokio::test]
async fn snapshot_creation_uses_only_loaded_membership_without_session_or_context() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    let fixture = Fixture::new(None);
    let membership = OrganizationMember {
        organization_id: 84,
        user_id: 1,
        roles: fixture_roles_0.clone(),
        groups: vec![],
    };
    let snapshot = compallm_core::authorization::PolicySnapshot::new(&membership);
    assert_eq!(snapshot.role_ids, vec![1, 2, 3]);
    assert!(!snapshot.cedar_source.is_empty());
    assert!(DbContext::current().is_err());
    assert!(fixture.repo.0.lock().unwrap().sessions.is_empty());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn renewal_copies_current_session_version_and_keeps_stale_refresh_usable() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let session = stored_session(1, 7, 84, "Research team");
    assert_eq!(session.authorization_version, 1);
    let token =
        AccessToken::Refresh(RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    fixture.remember(&token);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .get_mut(&7)
        .unwrap()
        .authorization_version = 42;
    let renewed = fixture.refresh(&token).await.unwrap();
    let access =
        UserAccessToken::try_from(decode_token(&renewed.access_token, SECRET).unwrap()).unwrap();
    let refresh =
        RefreshToken::try_from(decode_token(&renewed.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(access.session.authorization_version, 42);
    assert_eq!(refresh.session.authorization_version, 42);
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].authorization_version,
        42
    );
    assert!(fixture.refresh(&token).await.is_ok());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn explicit_permission_invalidation_scopes_users_and_sessions() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        for session in [
            stored_session(1, 7, 84, "Team"),
            stored_session(1, 8, 84, "Team"),
            stored_session(2, 9, 84, "Team"),
            stored_session(1, 10, 85, "Other"),
        ] {
            state.sessions.insert(session.id, session);
        }
        let mut revoked = stored_session(1, 11, 84, "Team");
        revoked.revoke();
        state.sessions.insert(11, revoked);
    }
    let service = fixture.service();
    for expected in [2, 3] {
        assert_eq!(
            fixture
                .repo
                .1
                .scope(service.invalidate_permissions(84, &[1, 1]))
                .await
                .unwrap(),
            2
        );
        let state = fixture.repo.0.lock().unwrap();
        for id in [7, 8] {
            assert_eq!(state.sessions[&id].authorization_version, expected);
        }
        for id in [9, 10, 11] {
            assert_eq!(state.sessions[&id].authorization_version, 1);
        }
        assert!(state.sessions[&7].revoked_at.is_none());
    }
    assert_eq!(
        fixture
            .repo
            .1
            .scope(service.invalidate_permissions(84, &[]))
            .await
            .unwrap(),
        0
    );
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.invalidate_permissions(84, &[1]))
            .await,
        Err(AuthError::SessionPersistence(_))
    ));
    fixture.assert_no_transaction();
}

fn empty_snapshot() -> compallm_core::authorization::PolicySnapshot {
    compallm_core::authorization::PolicySnapshot {
        cedar_source: String::new(),
        role_ids: vec![],
        group_roles: std::collections::BTreeMap::new(),
    }
}

#[tokio::test]
async fn access_issuance_includes_direct_and_group_authority_and_refresh_rebuilds_it() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    use compallm_core::{authorization::PolicySnapshot, organization::Group};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.memberships[0].groups.push(Group {
            id: 9,
            organization_id: 84,
            name: "Team".into(),
            description: String::new(),
            roles: vec![fixture_roles_0.clone().remove(2)],
        });
    }
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let issued = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let access =
        UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
    let expected = PolicySnapshot::new(&fixture.repo.0.lock().unwrap().memberships[0]);
    assert_eq!(access.policy_snapshot, expected);
    assert_eq!(access.policy_snapshot.role_ids, vec![1]);
    assert_eq!(access.policy_snapshot.group_roles[&9], vec![3]);
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.memberships[0].roles.clear();
        state.memberships[0].groups.clear();
        state
            .sessions
            .get_mut(&access.session.id)
            .unwrap()
            .authorization_version = 2;
    }

    let renewed = fixture.refresh(&issued.refresh_token).await.unwrap();
    let new_access =
        UserAccessToken::try_from(decode_token(&renewed.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(new_access.session.authorization_version, 2);
    assert!(new_access.policy_snapshot.cedar_source.is_empty());
    assert!(new_access.policy_snapshot.role_ids.is_empty());
    assert!(new_access.policy_snapshot.group_roles.is_empty());
    let refresh = decode_token(&renewed.refresh_token, SECRET).unwrap();
    assert!(
        serde_json::to_value(refresh).unwrap()["r"]
            .get("policy_snapshot")
            .is_none()
    );
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn default_policies_fit_with_compression() {
    let roles = default_roles(84, [1, 2, 3]).await;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.full_default_policies = true;
        state.memberships[0].roles = vec![roles[0].clone()];
    }
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    assert!(tokens.access_token.len() <= AccessToken::MAX_ENCODED_BYTES);
    assert!(fixture.refresh(&tokens.refresh_token).await.is_ok());
}

impl Fixture {
    fn access_for_session(&self, session: Session) -> UserAccessToken {
        UserAccessToken::new(session, empty_snapshot(), Duration::from_secs(60)).unwrap()
    }

    async fn request(
        &self,
        claims: &UserAccessToken,
        cookie: bool,
        refresh: Option<&str>,
        origin: bool,
    ) -> axum::response::Response {
        use axum::{body::Body, http::Request};
        let token = AccessToken::User(claims.clone())
            .encode_compressed(SECRET, Duration::from_secs(60))
            .unwrap();
        let mut request = Request::builder().uri("/check");
        if cookie {
            let value = if let Some(refresh) = refresh {
                format!("compallm-access={token}; compallm-refresh={refresh}")
            } else {
                format!("compallm-access={token}")
            };
            request = request.header("cookie", value);
        } else {
            request = request.header("authorization", format!("Bearer {token}"));
            if let Some(refresh) = refresh {
                request = request.header("cookie", format!("compallm-refresh={refresh}"));
            }
        }
        if origin {
            request = request.header("origin", "http://localhost");
        }
        self.execute(request.body(Body::empty()).unwrap()).await
    }

    async fn execute(&self, request: axum::extract::Request) -> axum::response::Response {
        use axum::{Router, middleware::from_fn, routing::get};
        use tower::ServiceExt;
        let service = Arc::new(self.service());
        let settings = Arc::new(auth_controller::delivery::TokenDeliverySettings {
            secure: false,
            public_origin: "http://localhost".into(),
        });
        let state = self.repo.0.clone();
        let router=Router::new().route("/check",get(move |authorization::RequireToken(user):authorization::RequireToken<UserAccessToken>| {
            let state=state.clone();
            async move {
                state.lock().unwrap().handled_requests+=1;
                axum::Json(serde_json::json!({"version":user.session.authorization_version,"roles":user.policy_snapshot.role_ids}))
            }
        })).layer(from_fn(move |request,next| {
            let service=service.clone();let settings=settings.clone();
            async move {authorization::Authorization::authenticate(&service,&settings,request,next).await}
        }));
        self.repo.1.scope(router.oneshot(request)).await.unwrap()
    }
}

#[tokio::test]
async fn middleware_matching_version_uses_one_lookup_and_keeps_snapshot() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let session = stored_session(1, 7, 84, "Team");
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .insert(7, session.clone());
    let claims = fixture.access_for_session(session);
    for cookie in [false, true] {
        let response = fixture.request(&claims, cookie, None, false).await;
        assert_eq!(response.status(), 200);
        assert!(!response.headers().contains_key("set-cookie"));
        assert!(!response.headers().contains_key("x-compallm-access-token"));
    }
    let state = fixture.repo.0.lock().unwrap();
    assert_eq!(state.freshness_reads, 2);
    assert_eq!(state.authority_reads, 0);
    assert_eq!(state.handled_requests, 2);
}

#[tokio::test]
async fn middleware_stale_bearer_requires_explicit_refresh_before_retry() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let session = stored_session(1, 7, 84, "Team");
    let claims = fixture.access_for_session(session.clone());
    let refresh =
        AccessToken::Refresh(RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    let mut current = session;
    current.authorization_version = 2;
    fixture.repo.0.lock().unwrap().sessions.insert(7, current);
    let response = fixture.request(&claims, false, Some(&refresh), false).await;
    assert_eq!(response.status(), 401);
    assert!(
        response.headers()["www-authenticate"]
            .to_str()
            .unwrap()
            .contains("invalid_token")
    );
    assert!(!response.headers().contains_key("set-cookie"));
    assert!(!response.headers().contains_key("x-compallm-access-token"));
    {
        let state = fixture.repo.0.lock().unwrap();
        assert_eq!(state.freshness_reads, 1);
        assert_eq!(state.authority_reads, 0);
        assert_eq!(state.handled_requests, 0);
    }
    let response = fixture.explicit_refresh(Some(&refresh), None).await;
    assert_eq!(response.status(), 200);
    assert!(!response.headers().contains_key("set-cookie"));
    let body = axum::body::to_bytes(response.into_body(), 16384)
        .await
        .unwrap();
    let tokens: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let access = UserAccessToken::try_from(
        decode_token(tokens["accessToken"].as_str().unwrap(), SECRET).unwrap(),
    )
    .unwrap();
    assert_eq!(access.session.authorization_version, 2);
    assert_eq!(access.policy_snapshot.role_ids, vec![1]);
    assert!(tokens["refreshToken"].is_string());
    assert!(tokens["accessLifetime"]["expiresAt"].is_number());
    assert_eq!(
        fixture.request(&access, false, None, false).await.status(),
        200
    );
    assert_eq!(fixture.repo.0.lock().unwrap().handled_requests, 1);
}

#[tokio::test]
async fn middleware_stale_cookie_requires_matching_refresh_and_origin() {
    for case in ["valid", "missing", "foreign", "origin"] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let session = stored_session(1, 7, 84, "Team");
        let claims = fixture.access_for_session(session.clone());
        let mut current = session.clone();
        current.authorization_version = 2;
        fixture.repo.0.lock().unwrap().sessions.insert(7, current);
        let mut refresh_session = session;
        if case == "foreign" {
            refresh_session.user_id = 2;
        }
        let token = AccessToken::Refresh(
            RefreshToken::from_session(&refresh_session, REFRESH_LIFETIME).unwrap(),
        )
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
        let response = fixture
            .request(
                &claims,
                true,
                if case == "missing" {
                    None
                } else {
                    Some(&token)
                },
                case != "origin",
            )
            .await;
        assert_eq!(
            response.status().as_u16(),
            match case {
                "valid" => 200,
                "origin" => 403,
                _ => 401,
            },
            "{case}"
        );
        if case == "valid" {
            assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
            let body = axum::body::to_bytes(response.into_body(), 8192)
                .await
                .unwrap();
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&body).unwrap()["version"],
                2
            );
        }
    }
}

#[tokio::test]
async fn middleware_rejects_invalid_session_state_and_preserves_cookies_on_db_failure() {
    for case in ["missing", "revoked", "foreign", "ahead", "database"] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let session = stored_session(1, 7, 84, "Team");
        let mut claims = fixture.access_for_session(session.clone());
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.sessions.insert(7, session);
            match case {
                "missing" => {
                    state.sessions.clear();
                }
                "revoked" => state.sessions.get_mut(&7).unwrap().revoke(),
                "foreign" => state.sessions.get_mut(&7).unwrap().organization_id = 99,
                "ahead" => claims.session.authorization_version = 2,
                "database" => state.fail_authority_read = true,
                _ => unreachable!(),
            }
        }
        let response = fixture.request(&claims, true, None, false).await;
        assert_eq!(
            response.status().as_u16(),
            if case == "database" { 503 } else { 401 }
        );
        if case == "database" {
            assert!(!response.headers().contains_key("set-cookie"));
        }
    }
}

#[tokio::test]
async fn middleware_invalid_or_wrong_variant_bearer_never_renews_from_cookies() {
    use axum::{body::Body, http::Request};
    for case in ["forged", "expired", "refresh", "otp", "duplicate"] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let session = stored_session(1, 7, 84, "Team");
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(7, session.clone());
        let refresh =
            AccessToken::Refresh(RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap())
                .encode_compressed(SECRET, REFRESH_LIFETIME)
                .unwrap();
        let token = match case {
            "forged" => AccessToken::User(fixture.access_for_session(session.clone()))
                .encode_compressed(
                    "another-secret-at-least-32-bytes-long",
                    Duration::from_secs(60),
                )
                .unwrap(),
            "refresh" => refresh.clone(),
            "otp" => {
                test_challenge("james@midwess.com", SECRET)
                    .unwrap()
                    .access_token
            }
            "expired" => {
                let mut claims = fixture.access_for_session(session.clone());
                claims.validity.issued_at -= 120;
                claims.validity.expires_at -= 120;
                sign_unchecked(&AccessToken::User(claims), Duration::from_secs(60))
            }
            _ => AccessToken::User(fixture.access_for_session(session.clone()))
                .encode_compressed(SECRET, Duration::from_secs(60))
                .unwrap(),
        };
        let mut request = Request::builder()
            .uri("/check")
            .header("authorization", format!("Bearer {token}"))
            .header("cookie", format!("compallm-refresh={refresh}"));
        if case == "duplicate" {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        let response = fixture.execute(request.body(Body::empty()).unwrap()).await;
        assert_eq!(response.status(), 401, "{case}");
        assert!(!response.headers().contains_key("x-compallm-access-token"));
        assert!(!response.headers().contains_key("set-cookie"));
        let state = fixture.repo.0.lock().unwrap();
        assert_eq!(state.freshness_reads, 0);
        assert_eq!(state.authority_reads, 0);
        assert_eq!(state.handled_requests, 0);
    }
}

#[tokio::test]
async fn middleware_expired_cookie_renews_once() {
    use axum::{body::Body, http::Request};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let session = stored_session(1, 7, 84, "Team");
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .insert(7, session.clone());
    let mut expired = fixture.access_for_session(session.clone());
    expired.validity.issued_at -= 120;
    expired.validity.expires_at -= 120;
    let expired = sign_unchecked(&AccessToken::User(expired), Duration::from_secs(60));
    let refresh =
        AccessToken::Refresh(RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    let response = fixture
        .execute(
            Request::builder()
                .uri("/check")
                .header("sec-fetch-site", "same-origin")
                .header(
                    "cookie",
                    format!("compallm-access={expired}; compallm-refresh={refresh}"),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
    assert_eq!(fixture.repo.0.lock().unwrap().handled_requests, 1);
}

#[tokio::test]
async fn middleware_oversized_rebuild_fails_before_handler_without_new_credentials() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let session = stored_session(1, 7, 84, "Team");
    let claims = fixture.access_for_session(session.clone());
    {
        let mut state = fixture.repo.0.lock().unwrap();
        let mut current = session;
        current.authorization_version = 2;
        state.sessions.insert(7, current);
        state.memberships[0].roles = vec![fixture_roles_0.clone().remove(0)];
        make_oversized(&mut state.memberships[0].roles[0]);
        state.full_default_policies = true;
    }
    let refresh = AccessToken::Refresh(
        RefreshToken::from_session(&claims.session, REFRESH_LIFETIME).unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    let response = fixture.request(&claims, true, Some(&refresh), true).await;
    assert_eq!(response.status(), 422);
    assert!(!response.headers().contains_key("x-compallm-access-token"));
    assert!(!response.headers().contains_key("set-cookie"));
    assert_eq!(fixture.repo.0.lock().unwrap().handled_requests, 0);
}

impl Fixture {
    async fn explicit_refresh(
        &self,
        bearer: Option<&str>,
        cookie: Option<&str>,
    ) -> axum::response::Response {
        use axum::{Extension, body::Body, http::Request};
        use tower::ServiceExt;
        let service: &'static AuthService = Box::leak(Box::new(self.service()));
        let settings: &'static auth_controller::delivery::TokenDeliverySettings =
            Box::leak(Box::new(auth_controller::delivery::TokenDeliverySettings {
                secure: false,
                public_origin: "http://localhost".into(),
            }));
        let (router, _) = auth_controller::router().split_for_parts();
        let router = router.with_state(service).layer(Extension(settings));
        let mut request = Request::builder().method("POST").uri("/api/auth/refresh");
        if let Some(bearer) = bearer {
            request = request.header("authorization", format!("Bearer {bearer}"));
        }
        if let Some(cookie) = cookie {
            request = request.header("cookie", format!("compallm-refresh={cookie}"));
        }
        self.repo
            .1
            .scope(router.oneshot(request.body(Body::empty()).unwrap()))
            .await
            .unwrap()
    }
}

#[tokio::test]
async fn explicit_refresh_requires_valid_refresh_bearer_and_ignores_cookies() {
    for case in [
        "cookie-only",
        "access",
        "otp",
        "forged",
        "expired",
        "revoked",
        "missing-member",
    ] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let session = stored_session(1, 7, 84, "Team");
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(7, session.clone());
        let valid =
            AccessToken::Refresh(RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap())
                .encode_compressed(SECRET, REFRESH_LIFETIME)
                .unwrap();
        let token = match case {
            "access" => AccessToken::User(fixture.access_for_session(session.clone()))
                .encode_compressed(SECRET, Duration::from_secs(60))
                .unwrap(),
            "otp" => {
                test_challenge("james@midwess.com", SECRET)
                    .unwrap()
                    .access_token
            }
            "forged" => AccessToken::Refresh(
                RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap(),
            )
            .encode_compressed("a-different-secret-at-least-32-bytes", REFRESH_LIFETIME)
            .unwrap(),
            "expired" => {
                let mut token = RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap();
                token.validity.issued_at -= REFRESH_LIFETIME.as_secs() + 1;
                token.validity.expires_at -= REFRESH_LIFETIME.as_secs() + 1;
                sign_unchecked(&AccessToken::Refresh(token), REFRESH_LIFETIME)
            }
            _ => valid.clone(),
        };
        if case == "revoked" {
            fixture
                .repo
                .0
                .lock()
                .unwrap()
                .sessions
                .get_mut(&7)
                .unwrap()
                .revoke();
        }
        if case == "missing-member" {
            fixture.repo.0.lock().unwrap().memberships.clear();
        }
        let response = fixture
            .explicit_refresh(
                if case == "cookie-only" {
                    None
                } else {
                    Some(&token)
                },
                Some(&valid),
            )
            .await;
        assert_eq!(response.status(), 401, "{case}");
        assert!(!response.headers().contains_key("set-cookie"), "{case}");
        assert_eq!(fixture.repo.0.lock().unwrap().handled_requests, 0);
    }
}

#[tokio::test]
async fn signin_accepts_active_memberships_with_custom_or_no_roles() {
    for custom in [false, true] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.memberships[0].roles = if custom {
                vec![Role {
                    id: 77,
                    ..Fixture::role(84, 0)
                }]
            } else {
                vec![]
            };
        }
        let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
        let tokens = fixture
            .signin(&Fixture::otp(&challenge), &challenge.code)
            .await
            .unwrap();
        let access =
            UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
        assert_eq!(
            access.policy_snapshot.role_ids,
            if custom { vec![77] } else { vec![] }
        );
        assert!(
            serde_json::to_value(access)
                .unwrap()
                .get("organization_role")
                .is_none()
        );
    }
}
