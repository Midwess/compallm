use compallm_postgres::Migrator;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};

async fn sql(suffix: &str, down: bool) -> Vec<String> {
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name().ends_with(suffix))
        .unwrap();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..3).map(|_| MockExecResult::default()))
        .into_connection();
    let manager = SchemaManager::new(&db);
    if down {
        migration.down(&manager).await.unwrap();
    } else {
        migration.up(&manager).await.unwrap();
    }
    db.into_transaction_log()
        .into_iter()
        .flat_map(|t| {
            t.statements()
                .iter()
                .map(|s| s.sql.clone())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[tokio::test]
async fn assignments_enforce_same_organization_and_index_both_query_directions() {
    for (table, left, right, left_parent, right_parent, reverse_left, reverse_right) in [
        (
            "organization_member_roles",
            "user_id",
            "role_id",
            "organization_members",
            "organization_roles",
            "role_id",
            "user_id",
        ),
        (
            "organization_group_members",
            "group_id",
            "user_id",
            "organization_groups",
            "organization_members",
            "user_id",
            "group_id",
        ),
        (
            "organization_group_roles",
            "group_id",
            "role_id",
            "organization_groups",
            "organization_roles",
            "role_id",
            "group_id",
        ),
    ] {
        let statements = sql(&format!("create_{table}"), false).await;
        let table_sql = &statements[0];
        assert!(table_sql.contains(&format!(
            "PRIMARY KEY (\"organization_id\", \"{left}\", \"{right}\")"
        )));
        for (field, parent) in [(left, left_parent), (right, right_parent)] {
            let target = if parent == "organization_members" {
                "user_id"
            } else {
                "id"
            };
            assert!(table_sql.contains(&format!("FOREIGN KEY (\"organization_id\", \"{field}\") REFERENCES \"{parent}\" (\"organization_id\", \"{target}\") ON DELETE CASCADE")));
        }
        assert!(statements[1].contains(&format!(
            "ON \"{table}\" (\"organization_id\", \"{reverse_left}\", \"{reverse_right}\")"
        )));
        assert!(!table_sql.contains("created_at"));
        assert!(!table_sql.contains("updated_at"));
        assert!(
            sql(&format!("create_{table}"), true).await[0]
                .contains(&format!("DROP TABLE \"{table}\""))
        );
    }
}

#[tokio::test]
async fn groups_have_scoped_identity_and_sessions_cover_the_full_positive_u32_range() {
    let groups = sql("create_organization_groups", false).await;
    assert!(groups[0].contains("PRIMARY KEY (\"organization_id\", \"id\")"));
    assert!(groups[0].contains("UNIQUE (\"organization_id\", \"name\")"));
    assert!(groups[0].contains("REFERENCES \"organizations\" (\"id\") ON DELETE CASCADE"));
    assert!(groups[0].contains("\"revision\" bigint NOT NULL DEFAULT 1"));
    let sessions = sql("add_session_authorization_version", false).await;
    assert!(sessions[0].contains("\"authorization_version\" bigint NOT NULL DEFAULT 1"));
    assert!(sessions[0].contains("\"authorization_version\" BETWEEN 1 AND 4294967295"));
    assert!(sessions[1].contains("(\"organization_id\", \"id\", \"user_id\")"));
    let down = sql("add_session_authorization_version", true).await;
    assert!(down[0].contains("DROP INDEX"));
    assert!(down[1].contains("DROP COLUMN \"authorization_version\""));
    let names = Migrator::migrations()
        .iter()
        .map(|m| m.name().to_owned())
        .collect::<Vec<_>>();
    assert!(
        names
            .iter()
            .position(|n| n.ends_with("create_organization_groups"))
            .unwrap()
            < names
                .iter()
                .position(|n| n.ends_with("create_organization_group_members"))
                .unwrap()
    );
}
