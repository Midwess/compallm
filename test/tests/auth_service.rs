use compallm_postgres::postgres_connection::PostgresConnection;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use compallm_core::{
    auth::{
        AccessToken, AuthError, AuthService, EmailOtpChallenge, RefreshToken, TokenLifetimes,
        UserAccessToken, UserTokens,
    },
    persistent::{
        DbContext,
        user::{UserRepository, UserRepositoryError},
    },
    user::{User, UserStatus},
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
    fail_record_login: bool,
    logins: Vec<u64>,
}

#[derive(Clone)]
struct Repository(Arc<Mutex<State>>, DbContext);

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

    async fn record_login(&self, user_id: u64) -> Result<(), UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        if state.fail_record_login {
            return Err(UserRepositoryError::database(DbErr::Custom(
                "record login failed".into(),
            )));
        }
        state.logins.push(user_id);
        Ok(())
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
        let repo = Repository(
            Arc::new(Mutex::new(State {
                user,
                ..State::default()
            })),
            DbContext::new(Arc::new(PostgresConnection::new(db.clone()))),
        );
        Self { db, repo }
    }

    fn service(&self) -> AuthService {
        AuthService::new(
            self.repo.1.clone(),
            Box::new(self.repo.clone()),
            SECRET.into(),
            "midwess.com".into(),
            TokenLifetimes {
                user: Duration::from_secs(60),
                refresh: REFRESH_LIFETIME,
            },
        )
        .unwrap()
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
        assert_eq!(
            Arc::try_unwrap(self.db)
                .expect("pool still borrowed")
                .into_transaction_log(),
            vec![Transaction::many([
                Statement::from_string(DbBackend::Postgres, "BEGIN"),
                Statement::from_string(DbBackend::Postgres, finish),
            ])]
        );
    }
}

#[tokio::test]
async fn signin_checks_user_and_commits() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = EmailOtpChallenge::new("james@midwess.com", "midwess.com", SECRET).unwrap();
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
    assert_eq!(contexts.len(), 2);
    assert_eq!(fixture.repo.0.lock().unwrap().logins, [1]);
    assert!(contexts.iter().all(|context| *context == contexts[0]));
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn signup_requires_verified_email_and_creates_on_one_transaction() {
    let fixture = Fixture::new(None);
    let challenge = EmailOtpChallenge::new("James@MIDWESS.COM", "midwess.com", SECRET).unwrap();
    let issued = fixture
        .service()
        .signup(&challenge.access_token, &challenge.code, " James ")
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
            Some(("james@midwess.com".into(), "James".into()))
        );
        assert_eq!(state.contexts.len(), 2);
        assert!(state.logins.is_empty());
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
async fn duplicate_signup_and_disabled_signin_are_rejected() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = EmailOtpChallenge::new("james@midwess.com", "midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code, "James")
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
async fn failed_login_record_rolls_back_signin_without_returning_tokens() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().fail_record_login = true;
    let challenge = EmailOtpChallenge::new("james@midwess.com", "midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signin(&challenge.access_token, &challenge.code)
            .await,
        Err(AuthError::Persistence(UserRepositoryError::Database(_)))
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn failed_repository_operation_rolls_back_and_returns_no_tokens() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_create = true;
    let challenge = EmailOtpChallenge::new("james@midwess.com", "midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code, "James")
            .await,
        Err(AuthError::Persistence(UserRepositoryError::Database(_)))
    ));
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn invalid_otp_or_empty_display_name_does_not_reach_persistence() {
    let fixture = Fixture::new(None);
    let challenge = EmailOtpChallenge::new("james@midwess.com", "midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .service()
            .signin(&challenge.access_token, "wrong")
            .await,
        Err(AuthError::IncorrectCode)
    ));
    assert!(matches!(
        fixture
            .service()
            .signup(&challenge.access_token, &challenge.code, "  ")
            .await,
        Err(AuthError::InvalidDisplayName)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
    drop(fixture.repo);
    assert!(
        Arc::try_unwrap(fixture.db)
            .expect("pool still borrowed")
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn refresh_checks_current_user_and_renews_both_tokens_for_another_month() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = UserTokens::new(1, SECRET, Duration::from_secs(60), REFRESH_LIFETIME).unwrap();
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
    let before_refresh = compallm_utils::time::unix_timestamp();
    let issued = fixture.service().refresh(&old_refresh).await.unwrap();
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
    assert_eq!(
        AccessToken::decode(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .user_id,
        1
    );
    fixture.repo.0.lock().unwrap().user.as_mut().unwrap().status = UserStatus::Disabled;
    assert!(matches!(
        fixture.service().refresh(&original.refresh_token).await,
        Err(AuthError::UserDisabled)
    ));
    assert!(matches!(
        fixture.service().refresh(&original.access_token).await,
        Err(AuthError::WrongTokenVariant)
    ));
}

#[tokio::test]
async fn expired_refresh_requires_signin_before_reading_the_user() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut expired = RefreshToken::new(1, REFRESH_LIFETIME).unwrap();
    let elapsed = REFRESH_LIFETIME.as_secs() + 1;
    expired.validity.issued_at -= elapsed;
    expired.validity.expires_at -= elapsed;
    // A valid JWT envelope must not bypass the expired inner claims.
    let encoded =
        hero::token::jwt::create_jwt_token(AccessToken::Refresh(expired), SECRET, REFRESH_LIFETIME)
            .unwrap();
    assert!(matches!(
        fixture.service().refresh(&encoded).await,
        Err(AuthError::ExpiredToken)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
}
