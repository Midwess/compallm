use compallm_postgres::postgres_connection::PostgresConnection;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use compallm_core::{
    auth::{
        AccessToken, AuthError, AuthService, EmailOtpChallenge, RefreshToken, Session,
        SessionReference, TokenLifetimes, UserAccessToken, UserTokens,
    },
    organization::{CreationService, Organization, OrganizationMember, OrganizationRole},
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

#[derive(Default)]
struct State {
    user: Option<User>,
    contexts: Vec<usize>,
    created: Option<(String, String)>,
    fail_create: bool,
    fail_session: bool,
    fail_rotation: bool,
    competing_rotation: bool,
    competing_update_during_revocation: bool,
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
    async fn find_membership(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
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
            .cloned())
    }

    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
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
            .cloned())
    }

    async fn find_first_admin_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
        DbContext::current()?;
        let state = self.0.lock().unwrap();
        Ok(state
            .memberships
            .iter()
            .find(|member| {
                member.user_id == user_id
                    && matches!(
                        member.role,
                        OrganizationRole::Owner | OrganizationRole::Admin
                    )
                    && !state
                        .disabled_organizations
                        .contains(&member.organization_id)
            })
            .cloned())
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

    async fn find(
        &self,
        reference: SessionReference,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        DbContext::current()?;
        Ok(self
            .0
            .lock()
            .unwrap()
            .sessions
            .get(&reference.id)
            .filter(|session| session.reference() == reference)
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
            state
                .sessions
                .get_mut(&session.id)
                .unwrap()
                .refresh_token_id = "B".repeat(43);
            state.competing_rotation = false;
            return Ok(false);
        }
        if state.competing_update_during_revocation && session.revoked_at.is_some() {
            let current = state.sessions.get_mut(&session.id).unwrap();
            current.refresh_token_id = "C".repeat(43);
            current.last_used_at += 1;
            current.expires_at += 1;
            state.competing_update_during_revocation = false;
        }
        let Some(current) = state.sessions.get_mut(&session.id) else {
            return Ok(false);
        };
        if current.reference() != previous.reference()
            || current.revoked_at != previous.revoked_at
            || (session.refresh_token_id != previous.refresh_token_id
                && current.refresh_token_id != previous.refresh_token_id)
            || (session.last_used_at != previous.last_used_at
                && current.last_used_at != previous.last_used_at)
            || (session.expires_at != previous.expires_at
                && current.expires_at != previous.expires_at)
        {
            return Ok(false);
        }
        if session.organization_id != previous.organization_id {
            current.organization_id = session.organization_id;
        }
        if session.refresh_token_id != previous.refresh_token_id {
            current.refresh_token_id = session.refresh_token_id.clone();
        }
        if session.last_used_at != previous.last_used_at {
            current.last_used_at = session.last_used_at;
        }
        if session.expires_at != previous.expires_at {
            current.expires_at = session.expires_at;
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
                    role: OrganizationRole::Owner,
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
        AuthService::new(
            db,
            users,
            organizations,
            Box::leak(Box::new(
                Box::new(self.repo.clone()) as Box<dyn SessionRepository>
            )),
            registration,
            SECRET.into(),
            TokenLifetimes {
                user: Duration::from_secs(60),
                refresh: REFRESH_LIFETIME,
            },
            None,
        )
        .unwrap()
    }

    async fn refresh(
        &self,
        token: &str,
        organization_id: Option<u64>,
    ) -> Result<UserTokens, AuthError> {
        self.repo
            .1
            .scope(self.service().refresh(token, organization_id))
            .await
    }

    fn remember(&self, token: &str) {
        let refresh = RefreshToken::try_from(AccessToken::decode(token, SECRET).unwrap()).unwrap();
        self.repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(refresh.session_id, Session::new(&refresh));
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
}

#[tokio::test]
async fn direct_registration_commits_user_organization_and_owner_together() {
    let fixture = Fixture::new(None);
    let user = fixture
        .registration()
        .register("james@midwess.com")
        .await
        .unwrap();
    {
        let state = fixture.repo.0.lock().unwrap();
        assert!(state.created.is_some());
        let organization = state.organization.as_ref().unwrap();
        let owner = state.owner.as_ref().unwrap();
        assert_eq!(organization.created_by, user.id);
        assert_eq!(owner.user_id, user.id);
        assert_eq!(owner.organization_id, organization.id);
        assert_eq!(owner.role, OrganizationRole::Owner);
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
            fixture.registration().register("james@midwess.com").await,
            Err(compallm_core::user::RegistrationError::Organization(_))
        ));
        assert!(fixture.repo.0.lock().unwrap().created.is_some());
        assert!(fixture.repo.0.lock().unwrap().owner.is_none());
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn signin_checks_user_and_commits() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    let issued = fixture
        .service()
        .signin(&challenge.access_token, &challenge.code)
        .await
        .unwrap();
    assert_eq!(
        AccessToken::decode(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .user_id,
        1
    );
    let contexts = fixture.repo.0.lock().unwrap().contexts.clone();
    let access =
        UserAccessToken::try_from(AccessToken::decode(&issued.access_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(access.organization_id, 84);
    assert_eq!(access.organization_role, OrganizationRole::Owner);
    assert_eq!(contexts.len(), 2);
    assert_eq!(fixture.repo.0.lock().unwrap().logins, [1]);
    assert!(contexts.iter().all(|context| *context == contexts[0]));
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn signup_requires_verified_email_and_creates_on_one_transaction() {
    let fixture = Fixture::new(None);
    let challenge = EmailOtpChallenge::new("James@MIDWESS.COM", SECRET).unwrap();
    let issued = fixture
        .service()
        .signup(&challenge.access_token, &challenge.code)
        .await
        .unwrap();
    assert!(
        AccessToken::decode(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
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
        assert_eq!(organization.created_by, owner.user_id);
        assert_eq!(organization.id, owner.organization_id);
        assert_eq!(owner.role, OrganizationRole::Owner);
        let access =
            UserAccessToken::try_from(AccessToken::decode(&issued.access_token, SECRET).unwrap())
                .unwrap();
        assert_eq!(access.organization_id, organization.id);
        assert_eq!(access.organization_role, OrganizationRole::Owner);
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
        let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
        assert!(matches!(
            fixture
                .service()
                .signup(&challenge.access_token, &challenge.code)
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
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code)
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
            .service()
            .signin(&challenge.access_token, &challenge.code)
            .await,
        Err(AuthError::UserDisabled)
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn failed_session_insert_rolls_back_signin_without_returning_tokens() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().fail_session = true;
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signin(&challenge.access_token, &challenge.code)
            .await,
        Err(AuthError::SessionPersistence(
            SessionRepositoryError::Database(_)
        ))
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn failed_repository_operation_rolls_back_and_returns_no_tokens() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_create = true;
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code)
            .await,
        Err(AuthError::Persistence(UserRepositoryError::Database(_)))
    ));
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn invalid_otp_does_not_reach_persistence() {
    let fixture = Fixture::new(None);
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signin(&challenge.access_token, "wrong")
            .await,
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
async fn invalid_registration_emails_are_rejected_before_persistence() {
    for email in [
        "not-an-email",
        "@midwess.com",
        "james dang@midwess.com",
        "james@midwess.com@elsewhere.com",
    ] {
        let fixture = Fixture::new(None);
        assert!(fixture.registration().register(email).await.is_err());
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
        let challenge = EmailOtpChallenge::new(&email, SECRET).unwrap();
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code)
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
    let tokens = UserTokens::new(
        &Session::new(&RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap()),
        OrganizationRole::Owner,
        SECRET,
        Duration::from_secs(60),
    )
    .unwrap();

    assert!(matches!(
        service.refresh(&tokens.refresh_token, None).await,
        Err(AuthError::Connection(_))
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());

    fixture.remember(&tokens.refresh_token);
    fixture
        .repo
        .1
        .scope(service.refresh(&tokens.refresh_token, None))
        .await
        .unwrap();
    assert_eq!(fixture.repo.0.lock().unwrap().contexts.len(), 1);
    assert!(DbContext::current().is_err());
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn refresh_checks_current_user_and_renews_both_tokens_for_another_month() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = UserTokens::new(
        &Session::new(&RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap()),
        OrganizationRole::Owner,
        SECRET,
        Duration::from_secs(60),
    )
    .unwrap();
    let mut previous =
        RefreshToken::try_from(AccessToken::decode(&original.refresh_token, SECRET).unwrap())
            .unwrap();
    // Simulate a credential issued 29 days ago without waiting or replacing the clock.
    let elapsed = 29 * 24 * 60 * 60;
    previous.validity.issued_at -= elapsed;
    previous.validity.expires_at -= elapsed;
    let original_id = previous.token_id.clone();
    let previous_expiry = previous.validity.expires_at;
    let old_refresh = AccessToken::Refresh(previous)
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&old_refresh);
    let before_refresh = compallm_utils::time::unix_timestamp();
    let issued = fixture.refresh(&old_refresh, None).await.unwrap();
    let after_refresh = compallm_utils::time::unix_timestamp();
    let renewed =
        RefreshToken::try_from(AccessToken::decode(&issued.refresh_token, SECRET).unwrap())
            .unwrap();
    assert_ne!(issued.refresh_token, old_refresh);
    assert_ne!(renewed.token_id, original_id);
    assert_eq!(renewed.user_id, 1);
    assert!(renewed.validity.issued_at >= before_refresh);
    assert!(renewed.validity.issued_at <= after_refresh);
    assert_eq!(
        renewed.validity.expires_at - renewed.validity.issued_at,
        REFRESH_LIFETIME.as_secs()
    );
    assert!(renewed.validity.expires_at > previous_expiry);
    assert_eq!(renewed.session_id, 7);
    {
        let state = fixture.repo.0.lock().unwrap();
        let session = state.sessions.get(&7).unwrap();
        assert_eq!(
            session.created_at,
            previous_expiry - REFRESH_LIFETIME.as_secs()
        );
        assert_eq!(session.last_used_at, renewed.validity.issued_at);
        assert_eq!(session.expires_at, renewed.validity.expires_at);
        assert_eq!(session.refresh_token_id, renewed.token_id);
        assert_eq!(state.sessions.len(), 1);
        assert!(state.logins.is_empty(), "refresh is not a new login");
    }
    assert_eq!(
        AccessToken::decode(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .user_id,
        1
    );
    fixture.repo.0.lock().unwrap().user.as_mut().unwrap().status = UserStatus::Disabled;
    assert!(matches!(
        fixture.refresh(&issued.refresh_token, None).await,
        Err(AuthError::UserDisabled)
    ));
    assert!(matches!(
        fixture.refresh(&original.access_token, None).await,
        Err(AuthError::WrongTokenVariant)
    ));
}

#[tokio::test]
async fn expired_refresh_requires_signin_before_reading_the_user() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut expired = RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap();
    let elapsed = REFRESH_LIFETIME.as_secs() + 1;
    expired.validity.issued_at -= elapsed;
    expired.validity.expires_at -= elapsed;
    // A valid JWT envelope must not bypass the expired inner claims.
    let encoded =
        hero::token::jwt::create_jwt_token(AccessToken::Refresh(expired), SECRET, REFRESH_LIFETIME)
            .unwrap();
    assert!(matches!(
        fixture.refresh(&encoded, None).await,
        Err(AuthError::ExpiredToken)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
}

#[tokio::test]
async fn signin_selects_an_admin_organization_instead_of_a_member_organization() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().memberships = vec![
        OrganizationMember {
            organization_id: 80,
            user_id: 1,
            role: OrganizationRole::Member,
        },
        OrganizationMember {
            organization_id: 85,
            user_id: 1,
            role: OrganizationRole::Admin,
        },
    ];
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .service()
        .signin(&challenge.access_token, &challenge.code)
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(claims.organization_id, 85);
    assert_eq!(claims.organization_role, OrganizationRole::Admin);
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn refresh_can_switch_organizations_using_current_membership_roles() {
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
            role: OrganizationRole::Member,
        });
    let refresh = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&refresh);
    let tokens = fixture.refresh(&refresh, Some(85)).await.unwrap();
    let claims =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(claims.organization_id, 85);
    assert_eq!(claims.organization_role, OrganizationRole::Member);
    let refreshed =
        RefreshToken::try_from(AccessToken::decode(&tokens.refresh_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(claims.session(), refreshed.session());
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].organization_id,
        85
    );
    assert!(matches!(
        fixture.refresh(&refresh, Some(84)).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    // Membership changes are checked again instead of copying role from an old JWT.
    fixture.repo.0.lock().unwrap().memberships[1].role = OrganizationRole::Admin;
    let tokens = fixture
        .refresh(&tokens.refresh_token, Some(85))
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(claims.organization_role, OrganizationRole::Admin);
    assert!(matches!(
        fixture.refresh(&tokens.refresh_token, Some(999)).await,
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
        fixture.refresh(&tokens.refresh_token, Some(85)).await,
        Err(AuthError::OrganizationAccessDenied)
    ));
}

#[tokio::test]
async fn signin_rejects_missing_or_disabled_administrator_memberships() {
    for disabled in [false, true] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        {
            let mut state = fixture.repo.0.lock().unwrap();
            if disabled {
                state.disabled_organizations.push(84);
            } else {
                state.memberships[0].role = OrganizationRole::Member;
            }
        }
        let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
        assert!(matches!(
            fixture
                .service()
                .signin(&challenge.access_token, &challenge.code)
                .await,
            Err(AuthError::OrganizationAccessDenied)
        ));
        assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn refresh_without_selection_keeps_the_session_organization() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().memberships.insert(
        0,
        OrganizationMember {
            organization_id: 80,
            user_id: 1,
            role: OrganizationRole::Member,
        },
    );
    let refresh = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&refresh);
    let tokens = fixture.refresh(&refresh, None).await.unwrap();
    let claims =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(claims.organization_id, 84);
    assert_eq!(claims.organization_role, OrganizationRole::Owner);
}

#[tokio::test]
async fn replay_revokes_the_session_and_commits_before_returning_denial() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&original);
    let replacement = fixture.refresh(&original, None).await.unwrap();
    assert!(matches!(
        fixture.refresh(&original, None).await,
        Err(AuthError::RefreshTokenReused)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_some()
    );
    let log = fixture.db.as_ref().clone().into_transaction_log();
    assert_eq!(log.len(), 2);
    assert!(
        format!("{log:?}").matches("COMMIT").count() == 2,
        "denying reuse must commit revocation, not roll it back"
    );
    assert!(matches!(
        fixture.refresh(&replacement.refresh_token, None).await,
        Err(AuthError::InvalidSession)
    ));
    assert_eq!(fixture.repo.0.lock().unwrap().sessions.len(), 1);
}

#[tokio::test]
async fn session_insert_failure_rolls_back_signup_and_returns_no_credentials() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_session = true;
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code)
            .await,
        Err(AuthError::SessionPersistence(_))
    ));
    assert!(fixture.repo.0.lock().unwrap().sessions.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn each_successful_signin_creates_an_independent_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    let service = fixture.service();
    let one = service
        .signin(&challenge.access_token, &challenge.code)
        .await
        .unwrap();
    let two = service
        .signin(&challenge.access_token, &challenge.code)
        .await
        .unwrap();
    let one =
        RefreshToken::try_from(AccessToken::decode(&one.refresh_token, SECRET).unwrap()).unwrap();
    let two =
        RefreshToken::try_from(AccessToken::decode(&two.refresh_token, SECRET).unwrap()).unwrap();
    assert_ne!(one.session_id, two.session_id);
    assert_eq!(fixture.repo.0.lock().unwrap().sessions.len(), 2);
    fixture
        .repo
        .1
        .scope(async {
            let previous = SessionRepository::find(&fixture.repo, one.session())
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
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    let two = AccessToken::Refresh(two)
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&one, None).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(fixture.refresh(&two, None).await.is_ok());
}

#[tokio::test]
async fn missing_expired_revoked_or_foreign_sessions_cannot_refresh() {
    for state in ["missing", "expired", "revoked", "foreign"] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let refresh = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
            .encode(SECRET, REFRESH_LIFETIME)
            .unwrap();
        fixture.remember(&refresh);
        {
            let mut data = fixture.repo.0.lock().unwrap();
            match state {
                "missing" => {
                    data.sessions.clear();
                }
                "expired" => {
                    data.sessions.get_mut(&7).unwrap().expires_at =
                        compallm_utils::time::unix_timestamp();
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
                fixture.refresh(&refresh, None).await,
                Err(AuthError::InvalidSession)
            ),
            "accepted {state} session"
        );
        assert!(
            fixture.repo.0.lock().unwrap().contexts.is_empty(),
            "invalid session must fail before user lookup"
        );
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn invalid_signature_cannot_revoke_a_valid_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims = RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap();
    let valid = AccessToken::Refresh(claims.clone())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&valid);
    let forged = AccessToken::Refresh(claims)
        .encode("wrong-key-at-least-32-bytes-long", REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&forged, None).await,
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
async fn refresh_refuses_an_enclosing_transaction_that_could_undo_revocation() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let refresh = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&refresh);
    let service = fixture.service();
    fixture
        .repo
        .1
        .transaction(async {
            assert!(matches!(
                service.refresh(&refresh, None).await,
                Err(AuthError::RefreshTransactionAlreadyActive)
            ));
            Ok::<_, AuthError>(())
        })
        .await
        .unwrap();
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn failed_rotation_returns_no_credentials_and_rolls_back() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims = RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap();
    let token = AccessToken::Refresh(claims.clone())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&token);
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    assert!(matches!(
        fixture.refresh(&token, None).await,
        Err(AuthError::InvalidSession)
    ));
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].refresh_token_id,
        claims.token_id
    );
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn competing_refresh_after_lookup_revokes_the_winning_session_and_commits() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims = RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap();
    let token = AccessToken::Refresh(claims)
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&token);
    fixture.repo.0.lock().unwrap().competing_rotation = true;
    assert!(matches!(
        fixture.refresh(&token, None).await,
        Err(AuthError::RefreshTokenReused)
    ));
    let session = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert!(session.revoked_at.is_some());
    assert_eq!(session.refresh_token_id, "B".repeat(43));
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn token_for_another_organization_cannot_read_or_revoke_an_owned_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let valid = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&valid);
    let foreign = AccessToken::Refresh(RefreshToken::new(1, 7, 85, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&foreign, None).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(fixture.refresh(&valid, None).await.is_ok());
}

#[tokio::test]
async fn revocation_preserves_a_refresh_that_commits_after_the_session_read() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = AccessToken::Refresh(RefreshToken::new(1, 7, 84, REFRESH_LIFETIME).unwrap())
        .encode(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&original);
    fixture.refresh(&original, None).await.unwrap();
    let previous = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .competing_update_during_revocation = true;
    assert!(matches!(
        fixture.refresh(&original, None).await,
        Err(AuthError::RefreshTokenReused)
    ));
    let persisted = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert!(persisted.revoked_at.is_some());
    assert_eq!(persisted.refresh_token_id, "C".repeat(43));
    assert_eq!(persisted.last_used_at, previous.last_used_at + 1);
    assert_eq!(persisted.expires_at, previous.expires_at + 1);
    let log = format!("{:?}", fixture.db.as_ref().clone().into_transaction_log());
    assert_eq!(log.matches("COMMIT").count(), 2);
}
