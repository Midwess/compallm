use compallm_postgres::Migrator;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};

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
