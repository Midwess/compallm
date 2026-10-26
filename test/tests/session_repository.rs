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
            revoked_at: None,
        }
    }
    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }
}

#[tokio::test]
async fn owned_lookup_and_conditional_update_preserve_session_guards() {
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
            let mut session = SessionRepository.find(reference).await?.unwrap();
            let created_at = session.created_at;
            let previous = session.clone();
            let token = RefreshToken::from_session(
                &session,
                std::time::Duration::from_secs(30 * 24 * 60 * 60),
            )
            .unwrap();
            session.switch_organization(&token, 85).unwrap();
            assert_eq!(session.created_at, created_at);
            assert!(SessionRepository.update(&session, &previous).await?);
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
    let update = sql.rsplit("UPDATE").next().unwrap();
    assert!(
        !update.contains("created_at"),
        "rotation must not change login time"
    );
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
async fn stale_update_reports_conflict_without_overwriting_the_session() {
    let context = Fixture::context(Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection(),
    ));
    let previous = Session::try_from(Fixture::row()).unwrap();
    let mut updated = previous.clone();
    let token =
        RefreshToken::from_session(&previous, std::time::Duration::from_secs(30 * 24 * 60 * 60))
            .unwrap();
    updated.switch_organization(&token, 85).unwrap();
    assert!(
        !context
            .scope(SessionRepository.update(&updated, &previous))
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn revocation_update_does_not_overwrite_a_concurrent_credential() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let previous = Session::try_from(Fixture::row()).unwrap();
    let mut revoked = previous.clone();
    revoked.revoke();
    assert!(
        context
            .scope(SessionRepository.update(&revoked, &previous))
            .await
            .unwrap()
    );
    drop(context);
    let sql =
        format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log()).replace("\\\"", "\"");
    let set = sql
        .split(" SET ")
        .nth(1)
        .unwrap()
        .split(" WHERE ")
        .next()
        .unwrap();
    assert!(set.contains("revoked_at"));
    for field in [
        "refresh_token_id",
        "last_used_at",
        "expires_at",
        "organization_id",
    ] {
        assert!(
            !set.contains(field),
            "revocation must preserve concurrent {field}: {sql}"
        );
    }
    assert!(
        !sql.contains("refresh_token_id"),
        "unchanged credential must not block revocation"
    );
    assert!(sql.contains("user_id") && sql.contains("organization_id") && sql.contains("IS NULL"));
}

#[tokio::test]
async fn existing_revocation_cannot_be_cleared_or_changed() {
    let mut previous = Session::try_from(Fixture::row()).unwrap();
    previous.revoke();
    let mut active = previous.clone();
    active.revoked_at = None;
    assert!(SessionRepository.update(&active, &previous).await.is_err());
    active.revoked_at = previous.revoked_at.map(|at| at + 1);
    assert!(SessionRepository.update(&active, &previous).await.is_err());
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
            let session = SessionRepository.find(reference).await.unwrap().unwrap();
            assert_eq!(session.id, 7);
            assert_eq!(session.user_id, 42);
            assert!(
                SessionRepository
                    .find(SessionReference {
                        user_id: 99,
                        ..reference
                    })
                    .await
                    .unwrap()
                    .is_none()
            );
            assert!(
                SessionRepository
                    .find(SessionReference {
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
fn mapper_preserves_session_fields_and_rejects_unrepresentable_ids() {
    let row = Fixture::row();
    let session = Session::try_from(row.clone()).unwrap();
    assert_eq!(session.id, row.id as u64);
    assert_eq!(session.refresh_token_id, row.refresh_token_id);
    let mut negative = row;
    negative.user_id = -1;
    assert!(Session::try_from(negative).is_err());
}
