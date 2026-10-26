use compallm_core::{
    auth::{RefreshToken, Session, SessionReference},
    persistent::{DbContext, session::SessionRepository as _},
};
use compallm_postgres::{
    UserSessionSchema, postgres_connection::PostgresConnection,
    repository::session::SessionRepository,
};
use sea_orm::{DbBackend, MockDatabase, MockExecResult, prelude::DateTimeWithTimeZone};
use std::sync::Arc;

struct Fixture;
impl Fixture {
    fn row() -> UserSessionSchema {
        let at = "2026-10-26T00:00:00Z"
            .parse::<DateTimeWithTimeZone>()
            .unwrap();
        UserSessionSchema {
            id: 7,
            user_id: 42,
            organization_id: 84,
            refresh_token_id: "A".repeat(43),
            created_at: at,
            last_used_at: at,
            expires_at: "2026-11-25T00:00:00Z".parse().unwrap(),
            revoked_at: None,
        }
    }
    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }
}

#[tokio::test]
async fn owned_lookup_and_conditional_rotation_preserve_session_guards() {
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
            let reference = SessionReference {
                id: 7,
                user_id: 42,
                organization_id: 84,
            };
            let mut session = SessionRepository.find_by_id(reference).await?.unwrap();
            let created_at = session.created_at;
            let old = session.refresh_token_id.clone();
            let token = RefreshToken::from_session(&session).unwrap();
            session
                .refresh(
                    &token,
                    85,
                    std::time::Duration::from_secs(30 * 24 * 60 * 60),
                )
                .unwrap();
            assert_eq!(session.created_at, created_at);
            assert!(SessionRepository.rotate(&session, reference, &old).await?);
            Ok::<_, compallm_core::persistent::session::SessionRepositoryError>(())
        })
        .await
        .unwrap();
    drop(context);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(log.len(), 1);
    let sql = format!("{log:?}").replace("\\\"", "\"");
    assert!(!sql.contains("FOR UPDATE"));
    assert!(
        sql.contains("WHERE \"user_sessions\".\"id\" = $1 AND \"user_sessions\".\"user_id\" = $2"),
        "{sql}"
    );
    assert!(sql.contains("refresh_token_id"));
    assert!(sql.contains("IS NULL"));
    assert!(sql.contains("CURRENT_TIMESTAMP"));
    let update = sql.rsplit("UPDATE").next().unwrap();
    assert!(
        !update.contains("created_at"),
        "rotation must not change login time"
    );
    assert!(update.contains("last_used_at"));
    assert!(sql.contains("COMMIT"));
    let lookup = sql.split("UPDATE").next().unwrap();
    assert!(lookup.contains("\"organization_id\" = $3"));
    let conditions = update.split("WHERE").nth(1).unwrap();
    for column in [
        "\"user_id\"",
        "\"organization_id\"",
        "\"refresh_token_id\"",
        "\"revoked_at\"",
    ] {
        assert!(
            conditions.contains(column),
            "rotation omits {column}: {update}"
        );
    }
}

#[tokio::test]
async fn session_creation_records_login_without_updating_the_user() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let session = Session::try_from(Fixture::row()).unwrap();
    context
        .transaction(SessionRepository.create(session))
        .await
        .unwrap();
    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO \\\"public\\\".\\\"user_sessions\\\""));
    assert!(log.contains("refresh_token_id"));
    assert!(log.contains("created_at"));
    assert!(!log.contains("last_login_at"));
    assert!(log.contains("COMMIT"));
}

#[tokio::test]
async fn failed_compare_and_swap_and_repeated_revocation_are_safe() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results((0..3).map(|_| MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }))
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let session = Session::try_from(Fixture::row()).unwrap();
    context
        .scope(async {
            assert!(
                !SessionRepository
                    .rotate(&session, session.reference(), "B".repeat(43).as_str())
                    .await
                    .unwrap()
            );
            SessionRepository
                .revoke(session.reference(), session.created_at)
                .await
                .unwrap();
            SessionRepository
                .revoke(session.reference(), session.created_at)
                .await
                .unwrap();
        })
        .await;
    drop(context);
    let log =
        format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log()).replace("\\\"", "\"");
    for update in log.split("UPDATE").skip(1) {
        let conditions = update.split("WHERE").nth(1).unwrap();
        assert!(conditions.contains("\"user_id\""));
        assert!(conditions.contains("\"organization_id\""));
    }
}

#[tokio::test]
async fn owned_session_lookup_works_with_the_application_context() {
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::row()], vec![], vec![]])
            .into_connection(),
    ));
    context
        .scope(async {
            let reference = SessionReference {
                id: 7,
                user_id: 42,
                organization_id: 84,
            };
            let session = SessionRepository
                .find_by_id(reference)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(session.id, 7);
            assert_eq!(session.user_id, 42);
            assert!(
                SessionRepository
                    .find_by_id(SessionReference {
                        user_id: 99,
                        ..reference
                    })
                    .await
                    .unwrap()
                    .is_none()
            );
            assert!(
                SessionRepository
                    .find_by_id(SessionReference {
                        organization_id: 99,
                        ..reference
                    })
                    .await
                    .unwrap()
                    .is_none()
            );
        })
        .await;
}

#[test]
fn mapper_rejects_invalid_identity_credential_and_timestamps() {
    let mut rows = vec![Fixture::row(); 6];
    rows[0].id = 0;
    rows[1].user_id = -1;
    rows[2].refresh_token_id = "bad".into();
    rows[3].expires_at = rows[3].last_used_at;
    rows[4].revoked_at = Some("2020-01-01T00:00:00Z".parse().unwrap());
    rows[5].organization_id = 0;
    for row in rows {
        assert!(Session::try_from(row).is_err());
    }
}
