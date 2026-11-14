use compallm_postgres::Migrator;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::test]
async fn creates_application_schema_with_scoped_usage_and_global_templates() {
    let migrations = Migrator::migrations();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..migrations.len() * 8).map(|_| MockExecResult::default()))
        .append_query_results([Vec::<std::collections::BTreeMap<String, sea_orm::Value>>::new()])
        .into_connection();
    for migration in migrations.iter().filter(|m| {
        !m.name().ends_with("seed_testing_workspace")
            && !m.name().ends_with("seed_user_preferences")
            && !m.name().ends_with("seed_provider_templates")
            && !m.name().ends_with("seed_access_token_dictionary")
            && !m.name().ends_with("replace_membership_role_enum")
            && !m.name().ends_with("remove_reserved_role_ids")
    }) {
        migration.up(&SchemaManager::new(&db)).await.unwrap();
    }
    let log = format!("{:?}", db.into_transaction_log());
    assert!(log.contains(
        r#"CREATE UNIQUE INDEX \"users_email_unique\" ON \"users\" ((LOWER(\"email\")))"#
    ));
    assert!(log.contains(r#"CREATE INDEX \"organization_members_organization_id_idx\" ON \"organization_members\" (\"organization_id\")"#));
    assert!(log.contains(r#"CREATE INDEX \"user_sessions_organization_user_idx\" ON \"user_sessions\" (\"organization_id\", \"user_id\")"#));
    for name in [
        "users_email_unique",
        "organization_members_pkey",
        "organization_members_user_id_idx",
        "organization_members_organization_id_idx",
        "api_keys_value_unique",
        "api_keys_organization_id_id_unique",
        "api_keys_organization_user_id_unique",
        "user_sessions_organization_user_idx",
        "providers_organization_id_id_unique",
        "providers_organization_id_name_unique",
        "providers_organization_id_index",
        "models_organization_id_id_unique",
        "models_organization_id_name_unique",
        "provider_models_organization_id_id_unique",
        "provider_models_organization_provider_id_unique",
        "provider_models_org_provider_name_service_protocol_unique",
        "model_router_organization_provider_model_id_unique",
        "model_router_model_id_provider_model_id_unique",
        "model_router_model_id_priority_unique",
        "model_router_provider_model_id_model_id_idx",
        "model_usage_organization_id_id_unique",
        "model_usage_organization_user_id_unique",
        "model_usage_organization_api_key_idx",
        "model_usage_organization_model_idx",
        "model_usage_limit_organization_user_unique",
        "provider_token_usage_attempt_unique",
        "provider_token_usage_model_usage_idx",
        "provider_token_usage_provider_model_idx",
        "provider_token_usage_route_idx",
        "provider_token_usage_request_idx",
        "provider_image_usage_attempt_idx",
        "provider_image_usage_model_usage_idx",
        "provider_image_usage_provider_model_idx",
        "provider_image_usage_route_idx",
        "provider_image_usage_request_idx",
        "provider_audio_usage_attempt_direction_unique",
        "provider_audio_usage_model_usage_idx",
        "provider_audio_usage_provider_model_idx",
        "provider_audio_usage_route_idx",
        "provider_audio_usage_request_idx",
        "provider_video_usage_attempt_idx",
        "provider_video_usage_model_usage_idx",
        "provider_video_usage_provider_model_idx",
        "provider_video_usage_route_idx",
        "provider_video_usage_request_idx",
    ] {
        assert!(log.contains(name), "missing schema index {name}");
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
    assert_eq!(
        migrations[15].name(),
        "m20261106_000016_create_provider_templates"
    );
    assert_eq!(
        migrations[16].name(),
        "m20261106_000017_create_provider_model_templates"
    );
    assert_eq!(
        migrations[17].name(),
        "m20261106_000018_seed_provider_templates"
    );
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
    let migrations = Migrator::migrations();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results((0..migrations.len() * 8).map(|_| MockExecResult::default()))
        .into_connection();
    for migration in migrations.into_iter().rev().filter(|m| {
        !m.name().ends_with("seed_testing_workspace")
            && !m.name().ends_with("seed_user_preferences")
            && !m.name().ends_with("seed_provider_templates")
            && !m.name().ends_with("seed_access_token_dictionary")
            && !m.name().ends_with("replace_membership_role_enum")
            && !m.name().ends_with("remove_reserved_role_ids")
    }) {
        migration.down(&SchemaManager::new(&db)).await.unwrap();
    }
    let log = format!("{:?}", db.into_transaction_log());
    let drop_position = |table: &str| log.find(&format!(r#"DROP TABLE \"{table}\""#)).unwrap();
    assert!(drop_position("provider_token_usage") < drop_position("model_usage"));
    assert!(drop_position("model_router") < drop_position("providers"));
    assert!(drop_position("provider_model_templates") < drop_position("provider_templates"));
    assert!(drop_position("organization_group_members") < drop_position("organization_groups"));
    assert!(drop_position("organization_group_roles") < drop_position("organization_roles"));
    assert!(drop_position("organization_member_roles") < drop_position("organization_members"));
    assert!(drop_position("users") < log.rfind("DROP TYPE").unwrap());
    assert!(drop_position("provider_token_usage") < log.find("provider_usage_status").unwrap());
    assert!(drop_position("provider_audio_usage") < log.find("usage_direction").unwrap());
}
