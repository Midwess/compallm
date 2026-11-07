use compallm_core::{
    auth::{RefreshToken, Session},
    persistent::{DbContext, session::SessionRepository as _},
};
use compallm_postgres::mappers::session::MappedSession;
use compallm_postgres::{
    OrganizationSchema, OrganizationStatusSchema, UserSessionSchema,
    postgres_connection::PostgresConnection, repository::session::SessionRepository,
};
use sea_orm::{DbBackend, IntoMockRow, MockDatabase, MockExecResult, Value};
use std::sync::Arc;

struct Fixture;
impl Fixture {
    fn row() -> UserSessionSchema {
        UserSessionSchema {
            id: 7,
            user_id: 42,
            organization_id: 84,
            authorization_version: 1,
            revoked_at: None,
        }
    }
    fn organization() -> OrganizationSchema {
        OrganizationSchema {
            id: 84,
            name: "Research team".into(),
            status: OrganizationStatusSchema::Active,
        }
    }
    fn joined_row() -> std::collections::BTreeMap<String, Value> {
        let mut row: std::collections::BTreeMap<_, _> = (Self::row(), Self::organization())
            .into_mock_row()
            .into_column_value_tuples()
            .collect();
        row.insert("B_status".into(), "active".into());
        row
    }
    fn context(db: Arc<sea_orm::DatabaseConnection>) -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(db)))
    }
}

#[tokio::test]
async fn owned_lookup_and_conditional_update_preserve_session_guards() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![Fixture::joined_row()]])
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    context
        .transaction(async {
            let token_session = MappedSession::try_from((Fixture::row(), "Research team".into()))
                .map(Session::from)
                .unwrap();
            let mut session = SessionRepository.find(&token_session).await?.unwrap();
            let previous = session.clone();
            let token = RefreshToken::from_session(
                &session,
                std::time::Duration::from_secs(30 * 24 * 60 * 60),
            )
            .unwrap();
            session
                .switch_organization(&token, 85, "Target team")
                .unwrap();
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
    assert!(sql.contains("IS NULL"));
    let update = sql.rsplit("UPDATE").next().unwrap();
    assert!(sql.contains("COMMIT"));
    let lookup = sql.split("UPDATE").next().unwrap();
    assert!(lookup.contains("\"organization_id\" = $3"));
    let conditions = update.split("WHERE").nth(1).unwrap();
    for column in ["\"user_id\"", "\"organization_id\"", "\"revoked_at\""] {
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
    let session = MappedSession::try_from((Fixture::row(), "Research team".into()))
        .map(Session::from)
        .unwrap();
    context
        .transaction(SessionRepository.create(session))
        .await
        .unwrap();
    drop(context);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    assert!(log.contains("INSERT INTO \\\"public\\\".\\\"user_sessions\\\""));
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
    let previous = MappedSession::try_from((Fixture::row(), "Research team".into()))
        .map(Session::from)
        .unwrap();
    let mut updated = previous.clone();
    let token =
        RefreshToken::from_session(&previous, std::time::Duration::from_secs(30 * 24 * 60 * 60))
            .unwrap();
    updated
        .switch_organization(&token, 85, "Target team")
        .unwrap();
    assert!(
        !context
            .scope(SessionRepository.update(&updated, &previous))
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn revocation_update_does_not_overwrite_a_concurrent_organization() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let previous = MappedSession::try_from((Fixture::row(), "Research team".into()))
        .map(Session::from)
        .unwrap();
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
        "last_used_at",
        "expires_at",
        "organization_id",
        "authorization_version",
    ] {
        assert!(
            !set.contains(field),
            "revocation must preserve concurrent {field}: {sql}"
        );
    }
    assert!(sql.contains("user_id") && sql.contains("organization_id") && sql.contains("IS NULL"));
}

#[tokio::test]
async fn existing_revocation_cannot_be_cleared_or_changed() {
    let mut previous = MappedSession::try_from((Fixture::row(), "Research team".into()))
        .map(Session::from)
        .unwrap();
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
            .append_query_results([vec![Fixture::joined_row()], vec![], vec![]])
            .into_connection(),
    ));
    context
        .scope(async {
            let token_session = MappedSession::try_from((Fixture::row(), "Research team".into()))
                .map(Session::from)
                .unwrap();
            let session = SessionRepository
                .find(&token_session)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(session.id, 7);
            assert_eq!(session.user_id, 42);
            assert_eq!(session.organization_name, "Research team");
            assert!(
                SessionRepository
                    .find(&Session {
                        user_id: 99,
                        ..token_session.clone()
                    })
                    .await
                    .unwrap()
                    .is_none()
            );
            assert!(
                SessionRepository
                    .find(&Session {
                        organization_id: 99,
                        ..token_session
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
    let session = MappedSession::try_from((row.clone(), "Research team".into()))
        .map(Session::from)
        .unwrap();
    assert_eq!(session.id, row.id as u64);
    let mut negative = row;
    negative.user_id = -1;
    assert!(
        MappedSession::try_from((negative, "Research team".into()))
            .map(Session::from)
            .is_err()
    );
}

#[test]
fn session_version_mapping_preserves_full_u32_range() {
    for version in [1, 42, u32::MAX] {
        let mut row = Fixture::row();
        row.authorization_version = i64::from(version);
        let session = Session::from(MappedSession::try_from((row, "Team".into())).unwrap());
        assert_eq!(session.authorization_version, version);
    }
    for version in [-1, 0, i64::from(u32::MAX) + 1] {
        let mut row = Fixture::row();
        row.authorization_version = version;
        assert!(MappedSession::try_from((row, "Team".into())).is_err());
    }
}

#[tokio::test]
async fn invalidation_is_one_atomic_scoped_update_and_empty_input_does_no_io() {
    assert_eq!(
        SessionRepository
            .invalidate_permissions(84, &[])
            .await
            .unwrap(),
        0
    );
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 3,
            }])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    assert_eq!(
        context
            .scope(SessionRepository.invalidate_permissions(84, &[42, 43, 42]))
            .await
            .unwrap(),
        3
    );
    drop(context);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(log.len(), 1);
    let sql = format!("{log:?}").replace("\\\"", "\"");
    assert!(sql.contains("UPDATE"));
    assert!(!sql.contains("SELECT") && !sql.contains("BEGIN"));
    assert!(sql.contains("authorization_version") && sql.contains(" + "));
    assert!(
        sql.contains("organization_id")
            && sql.contains("user_id")
            && sql.contains(" IN (")
            && sql.contains("IS NULL")
    );
}

#[tokio::test]
async fn policy_membership_loads_version_and_relationships_in_one_scoped_statement() {
    let mut base: std::collections::BTreeMap<String, Value> = Fixture::row()
        .into_mock_row()
        .into_column_value_tuples()
        .collect();
    base.insert("authorization_version".into(), 9_i64.into());
    base.insert("organization_name".into(), "Current team".into());
    let role = compallm_core::organization::Role::defaults(84).remove(2);
    let mut rows = Vec::new();
    for (group, assigned_role) in [
        (None, Some(&role)),
        (Some(8_i64), Some(&role)),
        (Some(9_i64), None),
    ] {
        let mut row = base.clone();
        row.insert("group_id".into(), group.into());
        row.insert(
            "group_name".into(),
            group.map(|_| "Group".to_string()).into(),
        );
        row.insert(
            "group_description".into(),
            group.map(|_| "Description".to_string()).into(),
        );
        row.insert("role_id".into(), assigned_role.map(|r| r.id as i64).into());
        row.insert(
            "role_name".into(),
            assigned_role.map(|r| r.name.clone()).into(),
        );
        row.insert(
            "role_description".into(),
            assigned_role.map(|r| r.description.clone()).into(),
        );
        row.insert(
            "cedar_source".into(),
            assigned_role.map(|r| r.cedar_source.clone()).into(),
        );
        rows.push(row);
    }
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([rows])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let (session, member) = context
        .scope(SessionRepository.find_policy_membership(&Session::new(42, 7, 84, "Old name")))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(session.authorization_version, 9);
    assert_eq!(session.organization_name, "Current team");
    assert_eq!(member.roles, vec![role.clone()]);
    assert_eq!(member.groups.len(), 2);
    assert_eq!(member.groups[0].roles, vec![role]);
    assert!(member.groups[1].roles.is_empty());
    drop(context);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(log.len(), 1);
    let sql = format!("{log:?}");
    assert!(
        sql.contains("s.organization_id=$1 AND s.user_id=$2 AND s.id=$3 AND s.revoked_at IS NULL")
    );
    for scope in [
        "u.status='active'",
        "o.status='active'",
        "m.status='active'",
        "r.organization_id=mr.organization_id",
        "g.organization_id=gm.organization_id",
        "gr.organization_id=g.organization_id",
        "r.organization_id=gr.organization_id",
        "mr.organization_id=s.organization_id",
        "gm.organization_id=s.organization_id",
        "LIMIT $4",
    ] {
        assert!(sql.contains(scope), "{scope}");
    }
    assert!(!sql.contains("BEGIN") && !sql.contains("FOR UPDATE") && !sql.contains("UPDATE "));
}

#[tokio::test]
async fn unavailable_policy_membership_has_no_authority() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([Vec::<std::collections::BTreeMap<String, Value>>::new()])
            .into_connection(),
    );
    let context = Fixture::context(db);
    assert!(
        context
            .scope(SessionRepository.find_policy_membership(&Session::new(42, 7, 84, "Team")))
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn freshness_lookup_projects_only_scoped_non_revoked_version() {
    let row = std::collections::BTreeMap::from([(
        "authorization_version".to_string(),
        Value::from(42_i64),
    )]);
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![row], vec![]])
            .into_connection(),
    );
    let context = Fixture::context(db.clone());
    let session = Session::new(42, 7, 84, "Not loaded");
    assert_eq!(
        context
            .scope(SessionRepository.authorization_version(&session))
            .await
            .unwrap(),
        Some(42)
    );
    assert_eq!(
        context
            .scope(SessionRepository.authorization_version(&session))
            .await
            .unwrap(),
        None
    );
    drop(context);
    let sql =
        format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log()).replace("\\\"", "\"");
    assert!(
        !sql.contains("JOIN") && !sql.contains("organization_name") && !sql.contains("FOR UPDATE")
    );
    assert!(sql.contains("SELECT \"user_sessions\".\"authorization_version\""));
    assert!(sql.contains("organization_id") && sql.contains("user_id") && sql.contains("IS NULL"));
}
