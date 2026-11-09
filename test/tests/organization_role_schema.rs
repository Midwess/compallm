use compallm_postgres::Migrator;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::test]
async fn role_storage_keeps_canonical_source_and_enforces_organization_identity() {
    let migration = Migrator::migrations()
        .into_iter()
        .find(|migration| migration.name() == "m20261107_000019_create_organization_roles")
        .unwrap();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results([MockExecResult::default(), MockExecResult::default()])
        .into_connection();
    let manager = SchemaManager::new(&db);
    migration.up(&manager).await.unwrap();
    migration.down(&manager).await.unwrap();
    let statements = db.into_transaction_log();
    let up = &statements[0].statements()[0].sql;
    assert!(up.contains("PRIMARY KEY (\"organization_id\", \"id\")"));
    assert!(up.contains("UNIQUE (\"organization_id\", \"name\")"));
    assert!(up.contains("\"cedar_source\" text NOT NULL"));
    assert!(!up.contains("\"cedar_source\" text NOT NULL DEFAULT"));
    assert!(up.contains("\"description\" text NOT NULL DEFAULT ''"));
    assert!(!up.contains("revision"));
    for column in ["organization_id", "id"] {
        assert!(up.contains(&format!("\"{column}\" > 0")));
    }
    assert!(
        up.to_lowercase()
            .contains("char_length(btrim(\"name\")) > 0"),
        "{up}"
    );
    assert!(
        up.contains("REFERENCES \"organizations\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION")
    );
    assert!(!up.contains("created_at"));
    assert!(!up.contains("updated_at"));
    assert!(
        statements[1].statements()[0]
            .sql
            .contains("DROP TABLE \"organization_roles\"")
    );
}

#[tokio::test]
async fn owner_marker_is_backfilled_and_unique_per_organization() {
    let migration = Migrator::migrations()
        .into_iter()
        .find(|migration| migration.name() == "m20261108_000027_add_organization_owner_role")
        .unwrap();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results([MockExecResult::default(), MockExecResult::default()])
        .into_connection();
    let manager = SchemaManager::new(&db);
    migration.up(&manager).await.unwrap();
    migration.down(&manager).await.unwrap();
    let log = db.into_transaction_log();
    let up = &log[0].statements()[0].sql;
    assert!(up.contains("org_owner boolean NOT NULL DEFAULT false"));
    assert!(up.contains("WHERE name = 'Owner'"));
    assert!(up.contains("(organization_id) WHERE org_owner"));
    assert!(log[1].statements()[0].sql.contains("DROP COLUMN org_owner"));
}
