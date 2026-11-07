use compallm_core::{
    organization::Role,
    persistent::{DbContext, role::RoleRepository as _},
};
use compallm_postgres::{
    Migrator, OrganizationRoleSchema, postgres_connection::PostgresConnection,
    repository::role::RoleRepository,
};
use sea_orm::{DbBackend, DbErr, IntoMockRow, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::{collections::BTreeMap, sync::Arc};
fn row(revision: i64) -> BTreeMap<String, sea_orm::Value> {
    OrganizationRoleSchema {
        organization_id: 84,
        id: 9,
        name: "Inference".into(),
        description: String::new(),
        cedar_source: String::new(),
        revision,
    }
    .into_mock_row()
    .into_column_value_tuples()
    .collect()
}
fn role() -> Role {
    Role {
        organization_id: 84,
        id: 9,
        name: "Inference".into(),
        description: String::new(),
        cedar_source: String::new(),
        revision: 1,
    }
}
#[tokio::test]
async fn accepted_role_edits_atomically_increment_or_retire_affected_sessions() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![row(2)]])
            .append_exec_results([MockExecResult::default()])
            .into_connection(),
    );
    let ctx = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    assert_eq!(
        ctx.scope(RoleRepository.update(role()))
            .await
            .unwrap()
            .revision,
        2
    );
    drop(ctx);
    let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
    for expected in [
        "BEGIN",
        "COMMIT",
        "revision",
        "organization_id",
        "organization_member_roles",
        "organization_group_roles",
        "organization_group_members",
        "authorization_version+1",
        "4294967295",
        "revoked_at",
    ] {
        assert!(log.contains(expected), "missing {expected}");
    }
}
#[tokio::test]
async fn conflict_or_invalidation_failure_rolls_back_and_never_claims_success() {
    for fail_invalidation in [false, true] {
        let db =
            MockDatabase::new(DbBackend::Postgres).append_query_results([if fail_invalidation {
                vec![row(2)]
            } else {
                vec![]
            }]);
        let db = Arc::new(
            if fail_invalidation {
                db.append_exec_errors([DbErr::Custom("counter write failed".into())])
            } else {
                db
            }
            .into_connection(),
        );
        let ctx = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
        assert!(ctx.scope(RoleRepository.update(role())).await.is_err());
        drop(ctx);
        let log = format!("{:?}", Arc::try_unwrap(db).unwrap().into_transaction_log());
        assert!(log.contains("ROLLBACK"));
        assert!(!log.contains("COMMIT"));
        if !fail_invalidation {
            assert!(!log.contains("UPDATE user_sessions"));
        }
    }
}
#[tokio::test]
async fn enum_transition_backfills_records_and_refuses_lossy_rollback() {
    let m = Migrator::migrations()
        .into_iter()
        .find(|m| m.name().ends_with("replace_membership_role_enum"))
        .unwrap();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results([MockExecResult::default(), MockExecResult::default()])
        .into_connection();
    let manager = SchemaManager::new(&db);
    m.up(&manager).await.unwrap();
    m.down(&manager).await.unwrap();
    let log = db.into_transaction_log();
    let up = &log[0].statements()[0].sql;
    let down = &log[1].statements()[0].sql;
    for expected in [
        "INSERT INTO organization_roles",
        "Owner",
        "Admin",
        "Member",
        "INSERT INTO organization_member_roles",
        "DROP COLUMN role",
        "DROP TYPE organization_role",
        "CREATE TRIGGER",
        "id IN (1,2,3)",
    ] {
        assert!(up.contains(expected), "missing {expected}");
    }
    assert!(!up.contains("builtin_key"));
    assert!(down.contains("cannot downgrade multi-role/custom/group memberships"));
    assert!(down.contains("count(mr.role_id)<>1"));
}
