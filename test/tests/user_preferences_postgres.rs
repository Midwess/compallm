use compallm_core::{
    persistent::{DbContext, preference::user::UserPreferenceRepository as _},
    preference::{UserPreferences, patch::*, sidebar::SidebarEntry, values::*},
};
use compallm_postgres::{
    Migrator, postgres_connection::PostgresConnection,
    repository::preference::user::UserPreferenceRepository,
};
use sea_orm::{ConnectionTrait, Database, EntityTrait, ModelTrait, Set};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL; creates and removes a disposable database"]
async fn preferences_backfill_concurrency_reset_and_owner_isolation() -> anyhow::Result<()> {
    let url = std::env::var("COMPALLM_TEST_DATABASE_URL")?;
    let name = format!(
        "preferences_{}_{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    let admin = Database::connect(&url).await?;
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await?;
    let result = async {
        let (base, params) = url.split_once('?').unwrap_or((&url, ""));
        let (server, _) = base
            .rsplit_once('/')
            .ok_or_else(|| anyhow::anyhow!("database URL"))?;
        let pool = Arc::new(Database::connect(format!("{server}/{name}?{params}")).await?);
        let context = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
        let manager = SchemaManager::new(pool.as_ref());
        for migration in Migrator::migrations().iter().filter(|migration| {
            migration.name().ends_with("create_users")
                || migration.name().ends_with("create_user_preferences")
        }) {
            migration.up(&manager).await?;
        }
        type Users = <compallm_postgres::UserSchema as ModelTrait>::Entity;
        type UserActive = <Users as EntityTrait>::ActiveModel;
        for id in [1, 2, 3] {
            Users::insert(UserActive {
                id: Set(id),
                email: Set(format!("preferences-{id}@example.com")),
                display_name: Set("Preferences".into()),
                status: Set(compallm_postgres::schema::UserStatusSchema::Active),
                email_verified_at: Set(sea_orm::prelude::ChronoUtc::now().fixed_offset()),
                ..Default::default()
            })
            .exec(pool.as_ref())
            .await?;
        }
        let repository = UserPreferenceRepository;
        assert!(context.scope(repository.find(3)).await?.is_none());
        let patch = UserPreferencesPatch {
            accessibility: Some(AccessibilityPatch {
                underline_links: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        context.scope(repository.update(1, &patch)).await?;
        let backfill = Migrator::migrations()
            .into_iter()
            .find(|migration| migration.name().ends_with("seed_user_preferences"))
            .unwrap();
        backfill.up(&manager).await?;
        backfill.up(&manager).await?;
        assert!(
            context
                .scope(repository.find(1))
                .await?
                .unwrap()
                .accessibility
                .underline_links
        );
        assert_eq!(
            context.scope(repository.find(2)).await?.unwrap(),
            UserPreferences::default()
        );
        let left = UserPreferencesPatch {
            accessibility: Some(AccessibilityPatch {
                pointer_cursors: Some(true),
                ..Default::default()
            }),
            code_appearance: Some(CodeAppearancePatch {
                font: Some(CodeFontPatch {
                    family: Some(FontFamily::Monospace),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let right = UserPreferencesPatch {
            accessibility: Some(AccessibilityPatch {
                reduce_motion: Some(false),
                ..Default::default()
            }),
            code_appearance: Some(CodeAppearancePatch {
                font: Some(CodeFontPatch {
                    size: Some(PixelFontSize::Px16),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let (a, b) = tokio::join!(
            context.scope(repository.update(1, &left)),
            context.scope(repository.update(1, &right))
        );
        a?;
        b?;
        let value = context.scope(repository.find(1)).await?.unwrap();
        assert!(value.accessibility.underline_links && value.accessibility.pointer_cursors);
        assert!(!value.accessibility.reduce_motion);
        assert_eq!(value.code_appearance.font.family, FontFamily::Monospace);
        assert_eq!(value.code_appearance.font.size, PixelFontSize::Px16);
        let order = UserPreferencesPatch {
            sidebar: Some(SidebarPatch {
                order: Some(vec![Destination::Plugins, Destination::Models]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let visibility = UserPreferencesPatch {
            sidebar: Some(SidebarPatch {
                visibility: Some(vec![SidebarEntry {
                    item: Destination::Plugins,
                    visibility: SidebarVisibility::Hidden,
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let (a, b) = tokio::join!(
            context.scope(repository.update(1, &order)),
            context.scope(repository.update(1, &visibility))
        );
        a?;
        b?;
        let saved = context.scope(repository.find(1)).await?.unwrap();
        assert_eq!(saved.sidebar.entries[0].item, Destination::Plugins);
        assert_eq!(
            saved.sidebar.entries[0].visibility,
            SidebarVisibility::Hidden
        );
        let reset = context
            .scope(repository.reset(1, &[UserPreferenceGroup::Sidebar]))
            .await?;
        assert_eq!(reset.sidebar, UserPreferences::default().sidebar);
        assert_eq!(reset.accessibility, saved.accessibility);
        assert_eq!(
            context
                .scope(repository.reset(1, &[UserPreferenceGroup::All]))
                .await?,
            UserPreferences::default()
        );
        assert_eq!(
            context.scope(repository.find(2)).await?.unwrap(),
            UserPreferences::default()
        );
        backfill.down(&manager).await?;
        assert!(context.scope(repository.find(3)).await?.is_some());
        drop(context);
        drop(manager);
        Arc::try_unwrap(pool).unwrap().close().await?;
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
