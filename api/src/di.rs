use axum::{Router, extract::FromRef};
use compallm_core::{
    auth::{AuthError, AuthService, TokenLifetimes},
    mail::EmailService,
    persistent::{DbContext, user::UserRepository},
};
use compallm_postgres::postgres_connection::PostgresConnection;
use loco_rs::app::AppContext;
use serde::Deserialize;
use std::{sync::OnceLock, time::Duration};
use tokio::sync::OnceCell;

static DI: OnceCell<Di> = OnceCell::const_new();

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

/// Own all application dependencies in one permanent container.
pub struct Di {
    db: Box<DbContext>,
    users: Box<dyn UserRepository>,
    email: Option<Box<dyn EmailService>>,
    settings: AuthSettings,
    // Filled after Di reaches its permanent address so services can borrow it.
    auth: OnceLock<Box<AuthService>>,
}

/// Axum state only carries references; ownership remains with Di.
#[derive(Clone, Copy)]
#[allow(clippy::borrowed_box)]
pub struct ServiceState {
    auth: &'static AuthService,
    users: &'static Box<dyn UserRepository>,
    db: &'static DbContext,
}

impl Di {
    pub fn instance() -> &'static Self {
        DI.get()
            .expect("Di::init must finish before using dependencies")
    }

    pub async fn init(ctx: &AppContext) -> loco_rs::Result<&'static Self> {
        let di = DI
            .get_or_try_init(|| async {
                let settings: Settings =
                    serde_json::from_value(ctx.config.settings.clone().ok_or_else(|| {
                        loco_rs::Error::Message("auth settings are missing".into())
                    })?)?;
                let db = PostgresConnection::context(ctx.db.clone());
                Ok::<_, loco_rs::Error>(Self::new(db, settings.auth, None))
            })
            .await?;
        di.initialize_services()
            .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
        Ok(di)
    }

    pub fn new(
        db: DbContext,
        settings: AuthSettings,
        email: Option<Box<dyn EmailService>>,
    ) -> Self {
        Self {
            db: Box::new(db),
            users: Box::new(compallm_postgres::repository::user::UserRepository),
            email,
            settings,
            auth: OnceLock::new(),
        }
    }

    pub fn initialize_services(&'static self) -> Result<(), AuthError> {
        if self.auth.get().is_some() {
            return Ok(());
        }
        let service = Box::new(AuthService::new(
            self.db_context(),
            self.user_repository(),
            self.settings.jwt_secret.clone(),
            self.settings.allowed_email_domain.clone(),
            TokenLifetimes {
                user: Duration::from_secs(self.settings.user_token_ttl_seconds),
                refresh: Duration::from_secs(self.settings.refresh_token_ttl_seconds),
            },
            self.email_service(),
        )?);
        // Concurrent initialization may build a second candidate, but only one
        // box is retained; all callers subsequently borrow that same service.
        let _ = self.auth.set(service);
        Ok(())
    }

    pub fn db_context(&'static self) -> &'static DbContext {
        &self.db
    }

    #[allow(clippy::borrowed_box)]
    pub fn user_repository(&'static self) -> &'static Box<dyn UserRepository> {
        &self.users
    }

    #[allow(clippy::borrowed_box)]
    pub fn email_service(&'static self) -> Option<&'static Box<dyn EmailService>> {
        self.email.as_ref()
    }

    pub fn auth_service(&'static self) -> &'static AuthService {
        self.auth.get().expect("Di services must be initialized")
    }

    /// Attach typed service state and the scoped database context to API routes.
    pub fn inject<S>(&'static self, router: &mut Router<S>)
    where
        S: Clone + Send + Sync + 'static,
    {
        let state = ServiceState {
            auth: self.auth_service(),
            users: self.user_repository(),
            db: self.db_context(),
        };
        let (routes, _) = crate::controllers::api_router().split_for_parts();
        let db = self.db_context();
        let routes = routes.with_state(state).layer(axum::middleware::from_fn(
            move |request: axum::extract::Request, next: axum::middleware::Next| async move {
                db.scope(next.run(request)).await
            },
        ));
        *router = std::mem::take(router).merge(routes);
    }
}

impl FromRef<ServiceState> for &'static AuthService {
    fn from_ref(state: &ServiceState) -> Self {
        state.auth
    }
}

impl FromRef<ServiceState> for &'static DbContext {
    fn from_ref(state: &ServiceState) -> Self {
        state.db
    }
}

#[allow(clippy::borrowed_box)]
impl FromRef<ServiceState> for &'static Box<dyn UserRepository> {
    fn from_ref(state: &ServiceState) -> Self {
        state.users
    }
}
