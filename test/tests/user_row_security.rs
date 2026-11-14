use compallm_core::{
    authentication::{Session, UserAccessToken},
    authorization::PolicySnapshot,
    organization::OrganizationMember,
    persistent::{DbContext, preference::user::UserPreferenceRepository as _},
    preference::UserPreferences,
};
use compallm_postgres::{
    UserPreferenceSchema, UserSchema, postgres_connection::PostgresConnection,
    repository::preference::user::UserPreferenceRepository,
};
use sea_orm::{
    ColumnTrait, ConnectOptions, ConnectionTrait, Database, DbBackend, EntityTrait, ModelTrait,
    QueryFilter, Set, Statement, sea_query::Expr,
};
use std::{sync::Arc, time::Duration};

type Users = <UserSchema as ModelTrait>::Entity;
type UserActiveModel = <Users as EntityTrait>::ActiveModel;
type UserColumn = <Users as EntityTrait>::Column;
type Preferences = <UserPreferenceSchema as ModelTrait>::Entity;
type PreferenceColumn = <Preferences as EntityTrait>::Column;

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with all migrations applied"]
async fn authenticated_profiles_and_preferences_enforce_database_ownership() -> anyhow::Result<()> {
    let mut options = ConnectOptions::new(std::env::var("COMPALLM_TEST_DATABASE_URL")?);
    options.max_connections(1);
    let pool = Arc::new(Database::connect(options).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    let first = compallm_utils::time::unix_timestamp() * 1000;
    let ids = [first, first + 1];
    for id in ids {
        Users::insert(UserActiveModel {
            id: Set(id as i64),
            email: Set(format!("rls-{id}@example.com")),
            username: Set(format!("rls-{id}")),
            display_name: Set("RLS profile".into()),
            email_verified_at: Set(sea_orm::prelude::ChronoUtc::now().fixed_offset()),
            ..Default::default()
        })
        .exec(pool.as_ref())
        .await?;
    }
    let result = async {
        let member = OrganizationMember {
            user_id: first,
            organization_id: 1,
            roles: vec![compallm_core::organization::Role {
                id: 1, organization_id: 1, name: "Model reader".into(), description: String::new(), org_owner: false,
                cedar_source: "permit(principal, action == Compallm::Action::\"model.read\", resource == Compallm::Organization::\"1\");".into(),
            }],
            groups: vec![],
        };
        let actor = UserAccessToken::new(
            Session {
                id: 1,
                user_id: first,
                organization_id: 1,
                authorization_version: 1,
                revoked_at: None,
            },
            PolicySnapshot::new(&member)?,
            Duration::from_secs(60),
        )?;
        let permission = Arc::new(compallm_core::authorization::MemberPermission::try_from(
            actor.clone(),
        )?);
        let personal = root
            .fresh()?
            .restrict(async move { Ok::<_, std::convert::Infallible>(permission) })?;
        let identity: compallm_core::persistent::connection::IdentityLoader = Arc::new(
            async_once_cell::Lazy::new(Box::pin(async move { Ok(actor.session) })),
        );
        personal.postgres().identify(identity)?;
        assert!(personal.fresh().is_err());
        let db = PostgresConnection::from_context(&personal)?;
        let visible = Users::find().all(db).await?;
        for id in ids {
            assert!(visible.iter().any(|user| user.id == id as i64));
        }
        assert_eq!(
            Users::update_many()
                .col_expr(UserColumn::DisplayName, Expr::value("own profile"))
                .exec(db)
                .await?
                .rows_affected,
            1
        );
        assert_eq!(
            Users::find_by_id((first + 1) as i64)
                .one(db)
                .await?
                .unwrap()
                .display_name,
            "RLS profile"
        );
        assert!(
            Users::update_many()
                .col_expr(UserColumn::Email, Expr::value("changed@example.com"))
                .exec(db)
                .await
                .is_err()
        );
        assert!(Users::delete_by_id(first as i64).exec(db).await.is_err());

        let preferences = UserPreferenceRepository;
        root.scope(preferences.create_defaults(first + 1)).await?;
        personal.scope(preferences.create_defaults(first)).await?;
        assert_eq!(Preferences::find().all(db).await?.len(), 1);
        assert!(personal.scope(preferences.find(first + 1)).await?.is_none());
        assert!(
            personal
                .scope(preferences.create_defaults(first + 1))
                .await
                .is_err()
        );
        assert!(
            Preferences::update_many()
                .col_expr(PreferenceColumn::UserId, Expr::value((first + 1) as i64))
                .exec(db)
                .await
                .is_err()
        );
        assert_eq!(
            personal
                .scope(preferences.reset(
                    first,
                    &[compallm_core::preference::patch::UserPreferenceGroup::All]
                ))
                .await?,
            UserPreferences::default()
        );
        assert!(
            personal
                .scope(preferences.reset(
                    first + 1,
                    &[compallm_core::preference::patch::UserPreferenceGroup::All]
                ))
                .await
                .is_err()
        );
        assert!(Preferences::delete_many().exec(db).await.is_err());
        let trusted = PostgresConnection::from_context(&root)?;
        assert_eq!(
            Preferences::find()
                .filter(PreferenceColumn::UserId.is_in(ids.map(|id| id as i64)))
                .all(trusted)
                .await?
                .len(),
            2
        );
        assert!(root.scope(preferences.find(first + 1)).await?.is_some());
        personal.transaction(async {
            let current = DbContext::current()?;
            let connection = PostgresConnection::from_context(&current)?;
            let decision = || Statement::from_string(DbBackend::Postgres,
                "SELECT compallm_rls.organization_allowed('model.read') AS allowed");
            let before = connection.query_one_raw(decision()).await?.unwrap();
            assert!(!before.try_get::<bool>("", "allowed")?);
            current.permission().await?;
            let after = connection.query_one_raw(decision()).await?.unwrap();
            assert!(after.try_get::<bool>("", "allowed")?);
            Ok::<_, anyhow::Error>(())
        }).await?;
        root.transaction(async {
            let current = DbContext::current()?;
            let missing_identity = PostgresConnection::from_context(&current)?;
            missing_identity
                .execute_unprepared("SET LOCAL ROLE compallm_restricted")
                .await?;
            assert!(Users::find().all(missing_identity).await?.is_empty());
            assert!(Preferences::find().all(missing_identity).await?.is_empty());
            Ok::<_, anyhow::Error>(())
        })
        .await?;
        Ok::<_, anyhow::Error>(())
    }
    .await;
    Users::delete_many()
        .filter(UserColumn::Id.is_in(ids.map(|id| id as i64)))
        .exec(pool.as_ref())
        .await?;
    result
}
