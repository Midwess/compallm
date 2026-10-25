use axum::extract::FromRef;
use compallm_core::{
    auth::{AuthError, AuthService, TokenLifetimes},
    mail::EmailService,
    persistent::{DbContext, user::UserRepository},
};
use compallm_postgres::postgres_connection::PostgresConnection;
use loco_rs::app::AppContext;
use serde::Deserialize;
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use tokio::sync::OnceCell;

static DEPENDENCIES: OnceCell<Di> = OnceCell::const_new();
static USER_REPOSITORY: LazyLock<Arc<dyn UserRepository>> =
    LazyLock::new(|| Arc::new(compallm_postgres::repository::user::UserRepository));

#[derive(Deserialize)]
pub struct AuthSettings {
    pub jwt_secret: String,
    pub allowed_email_domain: String,
    pub user_token_ttl_seconds: u64,
    pub refresh_token_ttl_seconds: u64,
}

#[derive(Deserialize)]
struct Settings {
    auth: AuthSettings,
}

/// Clones share the same repository, database context, and auth service.
#[derive(Clone)]
pub struct Di {
    db: DbContext,
    auth: Arc<AuthService>,
}

impl Di {
    /// Loco establishes the database connection before this initialization hook.
    /// Reuse it instead of opening another connection pool.
    pub async fn init(ctx: &AppContext) -> loco_rs::Result<Self> {
        DEPENDENCIES
            .get_or_try_init(|| async {
                let settings: Settings =
                    serde_json::from_value(ctx.config.settings.clone().ok_or_else(|| {
                        loco_rs::Error::Message("auth settings are missing".into())
                    })?)?;
                let db =
                    DbContext::new(Arc::new(PostgresConnection::new(Arc::new(ctx.db.clone()))));
                Self::new(db, settings.auth, None)
                    .map_err(|error| loco_rs::Error::Message(error.to_string()))
            })
            .await
            .cloned()
    }

    /// Explicit construction keeps tests isolated from production singletons.
    pub fn new(
        db: DbContext,
        settings: AuthSettings,
        email: Option<Arc<dyn EmailService>>,
    ) -> Result<Self, AuthError> {
        let users = Self::user_repository();
        let auth = Arc::new(AuthService::new(
            db.clone(),
            Box::new(users.clone()),
            settings.jwt_secret,
            settings.allowed_email_domain,
            TokenLifetimes {
                user: Duration::from_secs(settings.user_token_ttl_seconds),
                refresh: Duration::from_secs(settings.refresh_token_ttl_seconds),
            },
            email,
        )?);
        Ok(Self { db, auth })
    }

    pub fn db_context(&self) -> &DbContext {
        &self.db
    }
    pub fn user_repository() -> Arc<dyn UserRepository> {
        USER_REPOSITORY.clone()
    }
    pub fn auth_service(&self) -> &AuthService {
        &self.auth
    }
}

impl FromRef<AppContext> for Di {
    fn from_ref(ctx: &AppContext) -> Self {
        ctx.shared_store
            .get::<Self>()
            .expect("DI must be initialized before routing")
    }
}
