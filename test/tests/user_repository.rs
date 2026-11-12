use compallm_core::repository::ListOptions;
use compallm_core::{
    persistent::DbContext,
    repository::user::{UserQuery, UserRepository as CoreUserRepository, UserRepositoryError},
    user::{User, UserStatus},
};
use compallm_postgres::{
    UserSchema, postgres_connection::PostgresConnection, repository::user::UserRepository,
    schema::UserStatusSchema,
};
use sea_orm::{ActiveEnum, DbBackend, DbErr, IntoMockRow, MockDatabase, RuntimeErr};
use std::{borrow::Cow, collections::BTreeMap, error::Error, fmt, sync::Arc};

struct Fixture;

impl Fixture {
    fn mock_row(model: UserSchema) -> BTreeMap<String, sea_orm::Value> {
        // PostgreSQL returns enum labels as strings; SeaORM's mock row builder
        // otherwise stores its internal Enum value, which cannot decode as text.
        let status = model.status.to_value().value.to_string();
        let mut row: BTreeMap<_, _> = model.into_mock_row().into_column_value_tuples().collect();
        row.insert("status".into(), status.into());
        row
    }

    fn row() -> UserSchema {
        let at = "2026-10-25T00:00:00+00:00"
            .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
            .unwrap();
        UserSchema {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            status: UserStatusSchema::Active,
            email_verified_at: at,
            last_login_at: None,
        }
    }

    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
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
async fn lookup_uses_the_callers_scoped_transaction() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::mock_row(Fixture::row())]])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    context
        .transaction(async {
            let repository: &dyn CoreUserRepository = &UserRepository;
            let user = repository
                .one(
                    UserQuery {
                        email: Some("  JAMES@MIDWESS.COM  ".into()),
                        ..Default::default()
                    },
                    ListOptions::default(),
                )
                .await?
                .unwrap();
            assert_eq!(user.id, 42);
            assert_eq!(user.status, UserStatus::Active);
            Ok::<_, UserRepositoryError>(())
        })
        .await
        .unwrap();
    drop(context);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(log.len(), 1, "lookup must use the supplied transaction");
    let log = format!("{log:?}");
    assert!(log.contains("BEGIN"));
    assert!(log.contains("LOWER"));
    assert!(log.contains("james@midwess.com"));
    assert!(log.contains("COMMIT"));
}

#[tokio::test]
async fn create_persists_prepared_email_and_sets_verification_timestamp() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::mock_row(Fixture::row())]])
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
    disabled.status = UserStatusSchema::Disabled;
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::mock_row(disabled)], vec![]])
            .into_connection(),
    ));
    context
        .scope(async {
            let repository: &dyn CoreUserRepository = &UserRepository;
            let user = repository
                .one(
                    UserQuery {
                        id: Some(42_i64),
                        ..Default::default()
                    },
                    ListOptions::default(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(user.status, UserStatus::Disabled);
            assert!(
                repository
                    .one(
                        UserQuery {
                            id: Some(43_i64),
                            ..Default::default()
                        },
                        ListOptions::default()
                    )
                    .await
                    .unwrap()
                    .is_none()
            );
        })
        .await;
}

#[tokio::test]
async fn invalid_database_ids_are_rejected() {
    let mut invalid_id = Fixture::row();
    invalid_id.id = -1;
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::mock_row(invalid_id)]])
            .into_connection(),
    ));
    context
        .scope(async {
            assert!(matches!(
                UserRepository
                    .one(
                        UserQuery {
                            id: Some(42_i64),
                            ..Default::default()
                        },
                        ListOptions::default()
                    )
                    .await,
                Err(UserRepositoryError::Database(_))
            ));
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

#[tokio::test]
async fn list_returns_multiple_users_with_their_statuses() {
    let first = Fixture::row();
    let mut second = Fixture::row();
    second.id = 43;
    second.email = "other@midwess.com".into();
    second.status = UserStatusSchema::Disabled;
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::mock_row(first), Fixture::mock_row(second)]])
            .into_connection(),
    ));
    context
        .scope(async {
            let repository: &dyn CoreUserRepository = &UserRepository;
            let users = repository
                .list(
                    UserQuery {
                        ids: Some(vec![42, 43]),
                        ..Default::default()
                    },
                    ListOptions::from_limit(20),
                )
                .await
                .unwrap();
            assert_eq!(
                users.iter().map(|user| user.id).collect::<Vec<_>>(),
                vec![42, 43]
            );
            assert_eq!(users[0].status, UserStatus::Active);
            assert_eq!(users[1].status, UserStatus::Disabled);
        })
        .await;
}

#[derive(Default)]
struct IdQuery {
    id: Option<i64>,
}

impl compallm_postgres::repository::query::IntoQuery for IdQuery {
    type Table = <UserSchema as sea_orm::ModelTrait>::Entity;

    fn into_query(self) -> sea_orm::Select<Self::Table> {
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
        type Column = <<UserSchema as sea_orm::ModelTrait>::Entity as EntityTrait>::Column;
        Self::Table::find()
            .filter(sea_orm::Condition::all().add_option(self.id.map(|id| Column::Id.eq(id))))
    }
}

#[tokio::test]
async fn repository_accepts_another_native_query_without_repository_impls() {
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::mock_row(Fixture::row())], vec![]])
            .into_connection(),
    ));
    context
        .scope(async {
            let users = UserRepository
                .list(IdQuery { id: Some(42) }, ListOptions::from_limit(20))
                .await
                .unwrap();
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].id, 42);
            assert!(
                UserRepository
                    .one(IdQuery { id: Some(43) }, ListOptions::default())
                    .await
                    .unwrap()
                    .is_none()
            );
        })
        .await;
}
