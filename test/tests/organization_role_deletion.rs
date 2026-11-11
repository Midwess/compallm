#[path = "support/session.rs"]
mod session_fixture;

use compallm_core::{
    organization::{Group, Role},
    persistent::{
        DbContext, organization::OrganizationRepository as _, session::SessionRepository as _,
    },
};
use compallm_postgres::{
    postgres_connection::PostgresConnection,
    repository::{organization::OrganizationRepository, session::SessionRepository},
};
use sea_orm::{ConnectionTrait, Database};
use std::{sync::Arc, time::SystemTime};

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL pointing to a disposable PostgreSQL database"]
async fn role_deletion_blocks_new_group_links_and_invalidates_existing_assignees()
-> anyhow::Result<()> {
    let database_url = std::env::var("COMPALLM_TEST_DATABASE_URL")?;
    let database_name = format!(
        "role_deletion_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos()
    );
    let admin = Database::connect(&database_url).await?;
    admin
        .execute_unprepared(&format!("CREATE DATABASE {database_name}"))
        .await?;
    let result = async {
        let (base, parameters) = database_url.split_once('?').unwrap_or((&database_url, ""));
        let (server, _) = base.rsplit_once('/').ok_or_else(|| anyhow::anyhow!("expected PostgreSQL database URL"))?;
        let mut options = sea_orm::ConnectOptions::new(format!("{server}/{database_name}?{parameters}"));
        options.max_connections(1);
        let database = Arc::new(Database::connect(options.clone()).await?);
        let competing = Arc::new(Database::connect(options).await?);
        // Scoped fixtures only: never run application migrations or reuse its tables.
        database
            .execute_unprepared(
                "CREATE TABLE organization_roles (
                    organization_id bigint, id bigint, name text NOT NULL,
                    description text NOT NULL, cedar_source text NOT NULL, org_owner boolean NOT NULL,
                    PRIMARY KEY (organization_id,id)
                );
                CREATE TABLE organization_groups (
                    organization_id bigint, id bigint, name text NOT NULL, description text NOT NULL,
                    PRIMARY KEY (organization_id,id)
                );
                CREATE TABLE organization_group_roles (
                    organization_id bigint, group_id bigint, role_id bigint,
                    PRIMARY KEY (organization_id,group_id,role_id),
                    FOREIGN KEY (organization_id,group_id) REFERENCES organization_groups ON DELETE CASCADE,
                    FOREIGN KEY (organization_id,role_id) REFERENCES organization_roles ON DELETE CASCADE
                );
                CREATE TABLE organization_group_members (
                    organization_id bigint, group_id bigint, user_id bigint,
                    PRIMARY KEY (organization_id,group_id,user_id),
                    FOREIGN KEY (organization_id,group_id) REFERENCES organization_groups ON DELETE CASCADE
                );
                CREATE TABLE organization_member_roles (
                    organization_id bigint, user_id bigint, role_id bigint,
                    PRIMARY KEY (organization_id,user_id,role_id),
                    FOREIGN KEY (organization_id,role_id) REFERENCES organization_roles ON DELETE CASCADE
                );
                CREATE TABLE user_sessions (
                    id bigint PRIMARY KEY, organization_id bigint, user_id bigint,
                    authorization_version bigint NOT NULL, revoked_at timestamptz
                );
                CREATE TYPE user_status AS ENUM ('active','disabled');
                CREATE TYPE organization_status AS ENUM ('active','disabled');
                CREATE TABLE users (id bigint PRIMARY KEY, status user_status NOT NULL);
                CREATE TABLE organizations (id bigint PRIMARY KEY, name text NOT NULL, status organization_status NOT NULL);
                CREATE TABLE organization_members (
                    organization_id bigint, user_id bigint, status user_status NOT NULL,
                    PRIMARY KEY (organization_id,user_id)
                );
                INSERT INTO users VALUES (42,'active');
                INSERT INTO organizations VALUES (1,'Team','active');
                INSERT INTO organization_members VALUES (1,42,'active');
                INSERT INTO organization_roles VALUES (1,10,'Reports','','',false);
                INSERT INTO user_sessions VALUES (100,1,42,1,NULL);",
            )
            .await?;
        competing
            .execute_unprepared("SET lock_timeout = '250ms'")
            .await?;
        let context = DbContext::new(Arc::new(PostgresConnection::new(database.clone())));
        let competing_context = DbContext::new(Arc::new(PostgresConnection::new(competing.clone())));
        let organizations = OrganizationRepository;
        let sessions = SessionRepository;
        let group = Group {
            id: 21,
            organization_id: 1,
            name: "Concurrent".into(),
            description: String::new(),
            roles: vec![Role {
                id: 10,
                organization_id: 1,
                name: "Reports".into(),
                description: String::new(),
                cedar_source: String::new(),
                org_owner: false,
            }],
        };
        context
            .scope(organizations.create_group(Group {
                id: 20,
                name: "Existing".into(),
                ..group.clone()
            }))
            .await?;
        database
            .execute_unprepared("INSERT INTO organization_group_members VALUES (1,20,42)")
            .await?;
        let identity = session_fixture::stored_session(42,100,1,"Team");
        let (session, membership) = context.scope(sessions.find_policy_membership(&identity)).await?.unwrap();
        assert_eq!(session.authorization_version,1);
        assert_eq!(membership.groups[0].roles[0].id,10);
        let loaded = context.scope(organizations.find_membership(1,42)).await?.unwrap().0;
        assert_eq!(loaded.roles,membership.roles);
        assert_eq!(loaded.groups[0].roles,membership.groups[0].roles);
        let mut assigned = membership.clone();
        assigned.assign_role(membership.groups[0].roles[0].clone())?;
        context.transaction(async {
            organizations.save_member(&assigned,&membership).await?;
            sessions.invalidate_permissions(1,&[42]).await?;
            Ok::<_,anyhow::Error>(())
        }).await?;
        context
            .transaction(async {
                organizations.lock_role(1, 10).await?;
                organizations.lock_role_groups(1, 10).await?;
                let users = organizations.role_assignees(1, 10).await?;
                assert_eq!(users, vec![42]);
                // A separate task has its own connection scope and competes with deletion.
                let competing_write = tokio::spawn(async move {
                    competing_context
                        .scope(OrganizationRepository.create_group(group))
                        .await
                });
                assert!(competing_write.await?.is_err());
                organizations.delete_role(1, 10).await?;
                assert_eq!(sessions.invalidate_permissions(1, &users).await?, 1);
                Ok::<_, anyhow::Error>(())
            })
            .await?;
        context
            .scope(async {
                assert!(organizations.find_role(1, 10).await?.is_none());
                assert!(organizations.find_group(1, 21).await?.is_none());
                assert!(organizations.find_group(1, 20).await?.unwrap().roles.is_empty());
                assert_eq!(
                    sessions
                        .authorization_version(&session_fixture::stored_session(42, 100, 1, "Team"))
                        .await?,
                    Some(3)
                );
                Ok::<_, anyhow::Error>(())
            })
            .await?;
        drop(context);
        Arc::try_unwrap(database).unwrap().close().await?;
        Arc::try_unwrap(competing).unwrap().close().await?;
        Ok::<_, anyhow::Error>(())
    }
    .await;
    let cleanup = admin
        .execute_unprepared(&format!("DROP DATABASE {database_name} WITH (FORCE)"))
        .await;
    result?;
    cleanup?;
    Ok(())
}
