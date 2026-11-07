use compallm_postgres::Migrator;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

async fn migration_sql(migration: &dyn MigrationTrait, down: bool) -> String {
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_exec_results([MockExecResult::default()])
        .into_connection();
    let manager = SchemaManager::new(&db);
    if down {
        migration.down(&manager).await.unwrap();
    } else {
        migration.up(&manager).await.unwrap();
    }
    format!("{:?}", db.into_transaction_log())
}

#[tokio::test]
async fn global_catalog_tables_are_isolated_and_model_defaults_can_remain_unknown() {
    let migrations = Migrator::migrations();
    let templates = migrations
        .iter()
        .filter(|migration| {
            migration.name().ends_with("create_provider_templates")
                || migration
                    .name()
                    .ends_with("create_provider_model_templates")
        })
        .collect::<Vec<_>>();
    assert_eq!(templates.len(), 2);
    let provider = migration_sql(templates[0].as_ref(), false).await;
    let models = migration_sql(templates[1].as_ref(), false).await;
    assert!(provider.contains(r#"CREATE TABLE \"provider_templates\""#));
    assert!(models.contains(r#"CREATE TABLE \"provider_model_templates\""#));
    for sql in [&provider, &models] {
        for forbidden in [
            "organization_id",
            "user_id",
            "credentials",
            "api_key",
            "created_at",
            "updated_at",
        ] {
            assert!(
                !sql.contains(forbidden),
                "unexpected ownership/secret column {forbidden}"
            );
        }
    }
    assert!(provider.contains(r#"\"name\" varchar(120) NOT NULL"#));
    assert!(provider.contains(r#"\"base_url\" varchar(512) NOT NULL"#));
    assert!(models.contains(r#"PRIMARY KEY (\"provider_template_id\", \"id\")"#));
    assert!(models.contains(
        r#"REFERENCES \"provider_templates\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION"#
    ));
    assert!(models.contains("4294967295"));
    for column in ["input_price", "output_price", "cached_price"] {
        assert!(models.contains(&format!(r#"\"{column}\" decimal(20, 8) NULL"#)));
        assert!(models.contains(&format!(r#"\"{column}\" >= 0"#)));
        assert!(!models.contains(&format!(r#"\"{column}\" decimal(20, 8) NULL DEFAULT"#)));
    }
    for (column, kind) in [
        ("protocols", "array"),
        ("paths", "object"),
        ("capabilities", "array"),
        ("effort_mapping", "object"),
    ] {
        assert!(models.contains(&format!(r#"jsonb_typeof(\"{column}\") = '{kind}'"#)));
    }
    let model_down = migration_sql(templates[1].as_ref(), true).await;
    let provider_down = migration_sql(templates[0].as_ref(), true).await;
    assert!(model_down.contains(r#"DROP TABLE \"provider_model_templates\""#));
    assert!(provider_down.contains(r#"DROP TABLE \"provider_templates\""#));
}
