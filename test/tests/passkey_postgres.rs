use compallm_core::{
    authentication::{AuthError, Session},
    persistent::{DbContext, passkey::PasskeyRepository as _},
};
use compallm_postgres::{
    Migrator, postgres_connection::PostgresConnection, repository::passkey::PasskeyRepository,
};
use sea_orm::{
    ConnectionTrait, Database, ExprTrait,
    sea_query::{Alias, Expr, Query},
};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL; applies migrations in a disposable database"]
async fn session_evidence_upgrade_preserves_otp_and_restores_passkey_reads() -> anyhow::Result<()> {
    let url = std::env::var("COMPALLM_TEST_DATABASE_URL")?;
    let name = format!(
        "passkey_upgrade_{}_{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    let admin = Database::connect(&url).await?;
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await?;
    let result = async {
        let (base, parameters) = url.split_once('?').unwrap_or((&url, ""));
        let (server, _) = base
            .rsplit_once('/')
            .ok_or_else(|| anyhow::anyhow!("database URL"))?;
        let pool = Arc::new(Database::connect(format!("{server}/{name}?{parameters}")).await?);
        let manager = SchemaManager::new(pool.as_ref());
        for migration in Migrator::migrations().into_iter().filter(|m| {
            [
                "create_users",
                "create_organizations",
                "create_user_sessions",
                "passkeys",
            ]
            .iter()
            .any(|suffix| m.name().ends_with(suffix))
        }) {
            migration.up(&manager).await?;
        }
        let now = compallm_utils::time::unix_timestamp();
        let user = Query::insert()
            .into_table(Alias::new("users"))
            .columns(["id", "email", "display_name", "email_verified_at"].map(Alias::new))
            .values_panic([
                1_i64.into(),
                "owner@compallm.com".into(),
                "Owner".into(),
                sea_orm::prelude::ChronoUtc::now().fixed_offset().into(),
            ])
            .to_owned();
        pool.execute(&user).await?;
        let org = Query::insert()
            .into_table(Alias::new("organizations"))
            .columns(["id", "name"].map(Alias::new))
            .values_panic([2_i64.into(), "Workspace".into()])
            .to_owned();
        pool.execute(&org).await?;
        let session = Query::insert()
            .into_table(Alias::new("user_sessions"))
            .columns(["id", "user_id", "organization_id"].map(Alias::new))
            .values_panic([3_i64.into(), 1_i64.into(), 2_i64.into()])
            .to_owned();
        pool.execute(&session).await?;
        let legacy = Query::insert()
            .into_table(Alias::new("session_authentication_methods"))
            .columns(["session_id", "method", "provider", "authenticated_at"].map(Alias::new))
            .values_panic([
                3_i64.into(),
                "email_code".into(),
                "".into(),
                i64::try_from(now)?.into(),
            ])
            .to_owned();
        pool.execute(&legacy).await?;
        let context = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
        let repository = PasskeyRepository;
        let actor = Session {
            id: 3,
            user_id: 1,
            organization_id: 2,
            authorization_version: 1,
            revoked_at: None,
            metadata: Default::default(),
        };
        assert!(matches!(
            context
                .scope(repository.record_method(&actor, "email_code", now, None))
                .await,
            Err(AuthError::PasskeyPersistence(_))
        ));
        assert!(
            context
                .scope(repository.session_passkey(&actor))
                .await
                .is_err()
        );
        let upgrade = Migrator::migrations()
            .into_iter()
            .find(|m| m.name().ends_with("session_passkey_evidence"))
            .unwrap();
        upgrade.up(&manager).await?;
        upgrade.up(&manager).await?;
        context
            .scope(repository.record_method(&actor, "email_code", now, None))
            .await?;
        assert!(
            context
                .scope(repository.session_passkey(&actor))
                .await?
                .is_none()
        );
        assert!(context.scope(repository.list(1)).await?.is_empty());
        let count = Query::select()
            .expr_as(
                Expr::col(Alias::new("session_id")).count(),
                Alias::new("count"),
            )
            .from(Alias::new("session_authentication_methods"))
            .to_owned();
        let stored = pool.query_one(&count).await?.unwrap();
        assert_eq!(stored.try_get::<i64>("", "count")?, 1);
        Ok::<_, anyhow::Error>(())
    }
    .await;
    let cleanup = admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await;
    result?;
    cleanup?;
    Ok(())
}
