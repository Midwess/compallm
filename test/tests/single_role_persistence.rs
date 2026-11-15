use compallm_postgres::migration::Migrator;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, TransactionTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL pointing to a disposable PostgreSQL database"]
async fn single_role_migration_preserves_direct_grants_and_enforces_membership_invariants()
-> anyhow::Result<()> {
    let url = std::env::var("COMPALLM_TEST_DATABASE_URL")?;
    let name = format!(
        "single_role_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    let admin = Database::connect(&url).await?;
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await?;
    let result = async {
        let (base, parameters) = url.split_once('?').unwrap_or((&url, ""));
        let (server, _) = base.rsplit_once('/').ok_or_else(|| anyhow::anyhow!("expected database URL"))?;
        let db = Database::connect(format!("{server}/{name}?{parameters}")).await?;
        db.execute_unprepared(
            "CREATE TABLE organization_roles (
                organization_id bigint, id bigint, org_owner boolean NOT NULL,
                PRIMARY KEY (organization_id,id)
            );
            CREATE TABLE organization_members (
                organization_id bigint, user_id bigint,
                PRIMARY KEY (organization_id,user_id)
            );
            CREATE TABLE organization_member_roles (
                organization_id bigint, user_id bigint, role_id bigint,
                PRIMARY KEY (organization_id,user_id,role_id),
                FOREIGN KEY (organization_id,user_id) REFERENCES organization_members ON DELETE CASCADE,
                FOREIGN KEY (organization_id,role_id) REFERENCES organization_roles ON DELETE CASCADE
            );
            CREATE TABLE organization_group_roles (
                organization_id bigint, group_id bigint, role_id bigint,
                PRIMARY KEY (organization_id,group_id,role_id),
                FOREIGN KEY (organization_id,role_id) REFERENCES organization_roles ON DELETE CASCADE
            );
            CREATE TABLE organization_group_members (organization_id bigint, group_id bigint, user_id bigint);
            CREATE TABLE user_sessions (
                id bigint PRIMARY KEY, organization_id bigint, user_id bigint,
                authorization_version bigint NOT NULL, revoked_at timestamptz
            );
            INSERT INTO organization_roles VALUES (1,10,true),(1,11,false),(1,12,false),(2,11,false);
            INSERT INTO organization_members VALUES (1,42),(1,43),(2,42);
            INSERT INTO organization_member_roles VALUES (1,42,11),(1,43,12),(2,42,11);
            INSERT INTO organization_group_roles VALUES (1,20,10),(1,21,12);
            INSERT INTO organization_group_members VALUES (1,20,42),(1,21,42);
            INSERT INTO user_sessions VALUES (100,1,42,1,NULL),(101,1,43,1,NULL),(102,2,42,1,NULL);"
        ).await?;
        let migrations = Migrator::migrations();
        let migration = migrations.iter().find(|m| m.name() == "m20261115_000040_single_member_role").unwrap();

        db.execute_unprepared("INSERT INTO organization_member_roles VALUES (1,42,12)").await?;
        let tx = db.begin().await?;
        assert!(migration.up(&SchemaManager::new(&tx)).await.is_err());
        tx.rollback().await?;
        db.execute_unprepared("DELETE FROM organization_member_roles WHERE organization_id=1 AND user_id=42 AND role_id=12; DELETE FROM organization_member_roles WHERE organization_id=1 AND user_id=43").await?;
        let tx = db.begin().await?;
        assert!(migration.up(&SchemaManager::new(&tx)).await.is_err());
        tx.rollback().await?;
        db.execute_unprepared("INSERT INTO organization_member_roles VALUES (1,43,12)").await?;

        let tx = db.begin().await?;
        migration.up(&SchemaManager::new(&tx)).await?;
        tx.commit().await?;
        assert_eq!(scalar(&db, "SELECT role_id AS value FROM organization_member_roles WHERE organization_id=1 AND user_id=42").await?, 11);
        assert_eq!(scalar(&db, "SELECT count(*) AS value FROM organization_group_roles WHERE role_id=10").await?, 0);
        assert_eq!(scalar(&db, "SELECT count(*) AS value FROM organization_group_members WHERE organization_id=1 AND user_id=42").await?, 2);
        for (id, version) in [(100,2),(101,1),(102,1)] {
            assert_eq!(scalar(&db, &format!("SELECT authorization_version AS value FROM user_sessions WHERE id={id}")).await?, version);
        }
        for sql in [
            "INSERT INTO organization_member_roles VALUES (1,42,12)",
            "INSERT INTO organization_members VALUES (1,44)",
            "DELETE FROM organization_member_roles WHERE organization_id=1 AND user_id=42",
            "INSERT INTO organization_group_roles (organization_id,group_id,role_id) VALUES (1,20,10)",
            "INSERT INTO organization_group_roles VALUES (1,20,10,true)",
            "UPDATE organization_roles SET org_owner=true WHERE organization_id=1 AND id=12",
        ] {
            assert!(db.execute_unprepared(sql).await.is_err());
        }
        let tx = db.begin().await?;
        tx.execute_unprepared("INSERT INTO organization_members VALUES (1,44); INSERT INTO organization_member_roles VALUES (1,44,11)").await?;
        tx.commit().await?;
        let tx = db.begin().await?;
        tx.execute_unprepared("DELETE FROM organization_member_roles WHERE organization_id=1 AND user_id=42; INSERT INTO organization_member_roles VALUES (1,42,12)").await?;
        tx.commit().await?;
        assert_eq!(scalar(&db, "SELECT role_id AS value FROM organization_member_roles WHERE organization_id=1 AND user_id=42").await?, 12);
        assert_eq!(scalar(&db, "SELECT role_id AS value FROM organization_member_roles WHERE organization_id=2 AND user_id=42").await?, 11);
        db.execute_unprepared("DELETE FROM organization_members WHERE organization_id=1 AND user_id=44").await?;
        db.close().await?;
        Ok::<_, anyhow::Error>(())
    }.await;
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await?;
    result
}

async fn scalar(db: &DatabaseConnection, sql: &str) -> anyhow::Result<i64> {
    Ok(db
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            sql,
        ))
        .await?
        .unwrap()
        .try_get("", "value")?)
}
