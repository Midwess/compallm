use compallm_postgres::Migrator;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::test]
async fn creates_diagram_schema_directly_with_scoped_usage_and_no_legacy_tables() {
    let migrations = Migrator::migrations();
    assert_eq!(migrations.len(), 15);
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..73).map(|_| MockExecResult::default()))
        .into_connection();
    for migration in &migrations {
        migration.up(&SchemaManager::new(&db)).await.unwrap();
    }
    let log = format!("{:?}", db.into_transaction_log());
    assert!(log.contains(
        r#"CREATE UNIQUE INDEX \"users_email_unique\" ON \"users\" ((LOWER(\"email\")))"#
    ));
    assert!(log.contains(r#"CREATE INDEX \"organization_members_organization_id_idx\" ON \"organization_members\" (\"organization_id\")"#));
    assert!(log.contains(r#"CREATE INDEX \"user_sessions_organization_user_idx\" ON \"user_sessions\" (\"organization_id\", \"user_id\")"#));
    for line in include_str!("../../dbdiagram.dbml").lines() {
        if let Some((_, named)) = line.split_once("name: '") {
            let name = named.split('\'').next().unwrap();
            assert!(log.contains(name), "missing diagram index {name}");
        }
    }
    for (position, table) in [
        "users",
        "organizations",
        "organization_members",
        "api_keys",
        "user_sessions",
        "providers",
        "models",
        "provider_models",
        "model_router",
        "model_usage",
        "model_usage_limit",
        "provider_token_usage",
        "provider_image_usage",
        "provider_audio_usage",
        "provider_video_usage",
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            migrations[position].name(),
            format!("m20261104_{:06}_create_{table}", position + 1)
        );
        assert!(
            log.contains(&format!("CREATE TABLE \\\"{table}\\\"")),
            "missing {table}"
        );
    }
    for table in ["token", "image", "audio", "video"] {
        let name = format!("provider_{table}_usage");
        for suffix in [
            "model_usage_scope_fkey",
            "route_scope_fkey",
            "provider_model_scope_fkey",
            "provider_scope_fkey",
        ] {
            assert!(log.contains(&format!("{name}_{suffix}")));
        }
    }
    assert!(log.contains("provider_models_org_provider_name_service_protocol_unique"));
    assert!(log.contains("provider_token_usage_attempt_unique"));
    assert!(log.contains("provider_audio_usage_attempt_direction_unique"));
    assert!(log.contains("provider_image_usage_attempt_idx"));
    assert!(log.contains("provider_video_usage_attempt_idx"));
    for removed in [
        "ALTER TABLE",
        "INSERT INTO",
        "UPDATE \\\"providers\\\" SET",
        "UPDATE \\\"provider_models\\\" SET",
        "CREATE TABLE \\\"provider_usage\\\"",
        "CREATE TABLE \\\"balances\\\"",
        "created_at",
        "updated_at",
        "api_protocol",
        "group_index",
        "attempt_group_unique",
        "pricing_snapshot",
    ] {
        assert!(!log.contains(removed), "unexpected {removed}");
    }
    for column in [
        "input_usd_per_million_tokens",
        "output_usd_per_million_tokens",
        "cached_usd_per_million_tokens",
        "usd_per_image",
        "usd_per_second",
        "usd_per_million_characters",
    ] {
        assert!(
            log.contains(&format!(r#"\"{column}\" decimal(20, 8) NULL"#)),
            "missing exact nullable rate {column}"
        );
        assert!(
            log.contains(&format!(r#"\"{column}\" >= 0"#)),
            "missing nonnegative rate check {column}"
        );
    }
}

#[tokio::test]
async fn drops_children_before_parents_and_owned_enums_after_their_tables() {
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..20).map(|_| MockExecResult::default()))
        .into_connection();
    for migration in Migrator::migrations().into_iter().rev() {
        migration.down(&SchemaManager::new(&db)).await.unwrap();
    }
    let log = format!("{:?}", db.into_transaction_log());
    assert!(log.find("provider_token_usage").unwrap() < log.find("model_usage\\\"").unwrap());
    assert!(log.find("model_router").unwrap() < log.find("providers").unwrap());
    assert!(log.find("users").unwrap() < log.rfind("DROP TYPE").unwrap());
    assert!(log.find("provider_token_usage").unwrap() < log.find("provider_usage_status").unwrap());
    assert!(log.find("provider_audio_usage").unwrap() < log.find("usage_direction").unwrap());
}
