use compallm_core::{
    persistent::{
        DbContext,
        user::{UserRepository as _, UserRepositoryError},
    },
    user::{User, UserStatus},
};
use compallm_postgres::{
    UserSchema, postgres_connection::PostgresConnection, repository::user::UserRepository,
};
use sea_orm::{DbBackend, DbErr, MockDatabase, MockExecResult, RuntimeErr};
use std::{borrow::Cow, error::Error, fmt, sync::Arc};

struct Fixture;

impl Fixture {
    fn row() -> UserSchema {
        let at = "2026-10-25T00:00:00+00:00"
            .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
            .unwrap();
        UserSchema {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            status: "active".into(),
            email_verified_at: at,
            last_login_at: None,
            created_at: at,
            updated_at: at,
        }
    }

    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }

    fn user() -> User {
        User {
            id: 42,
            email: " James@MIDWESS.COM ".into(),
            display_name: "James".into(),
            status: UserStatus::Active,
        }
    }
}

#[tokio::test]
async fn lookup_and_login_record_use_one_scoped_transaction() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::row()]])
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    context
        .transaction(async {
            let user = UserRepository
                .find_by_email(" James@MIDWESS.COM ")
                .await?
                .unwrap();
            assert_eq!(user.id, 42);
            assert_eq!(user.status, UserStatus::Active);
            UserRepository.record_login(user.id).await
        })
        .await
        .unwrap();
    drop(context);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(log.len(), 1, "lookup and update must share one transaction");
    let log = format!("{log:?}");
    assert!(log.contains("BEGIN"));
    assert!(log.contains("LOWER"));
    assert!(log.contains("james@midwess.com"));
    assert!(log.contains("UPDATE"));
    assert!(log.contains("last_login_at"));
    assert!(log.contains("updated_at"));
    assert!(log.contains("CURRENT_TIMESTAMP"));
    assert!(log.contains("COMMIT"));
}

#[tokio::test]
async fn create_normalizes_email_and_sets_verification_timestamp() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::row()]])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let created = context
        .scope(UserRepository.create(Fixture::user()))
        .await
        .unwrap();
    assert_eq!(created.id, 42);
    assert_eq!(created.email, "james@midwess.com");
    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO"));
    assert!(log.contains("email_verified_at"));
    assert!(log.contains("james@midwess.com"));
    assert!(!log.contains("MIDWESS.COM"));
}

#[tokio::test]
async fn find_by_id_maps_disabled_users_and_missing_rows() {
    let mut disabled = Fixture::row();
    disabled.status = "disabled".into();
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![disabled], vec![]])
            .into_connection(),
    ));
    context
        .scope(async {
            let user = UserRepository.find_by_id(42).await.unwrap().unwrap();
            assert_eq!(user.status, UserStatus::Disabled);
            assert!(UserRepository.find_by_id(43).await.unwrap().is_none());
        })
        .await;
}

#[tokio::test]
async fn invalid_ids_and_missing_login_targets_are_database_errors() {
    for id in [0, u64::MAX] {
        assert!(matches!(
            UserRepository.find_by_id(id).await,
            Err(UserRepositoryError::Database(_))
        ));
    }
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection(),
    ));
    assert!(matches!(
        context.scope(UserRepository.record_login(42)).await,
        Err(UserRepositoryError::Database(_))
    ));
}

#[tokio::test]
async fn corrupt_rows_are_rejected_instead_of_becoming_active_users() {
    let mut unknown_status = Fixture::row();
    unknown_status.status = "unexpected".into();
    let mut invalid_id = Fixture::row();
    invalid_id.id = -1;
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![unknown_status], vec![invalid_id]])
            .into_connection(),
    ));
    context
        .scope(async {
            for _ in 0..2 {
                assert!(matches!(
                    UserRepository.find_by_id(42).await,
                    Err(UserRepositoryError::Database(_))
                ));
            }
        })
        .await;
}

#[derive(Debug)]
struct UniqueViolation(&'static str);

impl fmt::Display for UniqueViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "duplicate key: {}", self.0)
    }
}

impl Error for UniqueViolation {}

impl sea_orm::sqlx::error::DatabaseError for UniqueViolation {
    fn message(&self) -> &str {
        "duplicate key"
    }
    fn code(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed("23505"))
    }
    fn constraint(&self) -> Option<&str> {
        Some(self.0)
    }
    fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
        self
    }
    fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
        self
    }
    fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
        self
    }
    fn kind(&self) -> sea_orm::sqlx::error::ErrorKind {
        sea_orm::sqlx::error::ErrorKind::UniqueViolation
    }
}

#[tokio::test]
async fn only_email_unique_conflicts_become_email_already_exists() {
    for constraint in ["users_email_unique", "users_pkey"] {
        let error = DbErr::Query(RuntimeErr::SqlxError(Arc::new(
            sea_orm::sqlx::Error::Database(Box::new(UniqueViolation(constraint))),
        )));
        let context = Fixture::context(Arc::new(
            MockDatabase::new(DbBackend::Postgres)
                .append_query_errors([error])
                .into_connection(),
        ));
        let error = context
            .scope(UserRepository.create(Fixture::user()))
            .await
            .unwrap_err();
        if constraint == "users_email_unique" {
            assert!(matches!(error, UserRepositoryError::EmailAlreadyExists));
        } else {
            assert!(matches!(error, UserRepositoryError::Database(_)));
        }
    }
}
