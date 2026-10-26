use axum::Router;
use compallm_core::{
    auth::{AuthError, AuthService, TokenLifetimes},
    mail::EmailService,
    organization::{ConfigService, CreationService},
    persistent::session::SessionRepository,
    persistent::{DbContext, organization::OrganizationRepository, user::UserRepository},
    user::RegistrationService,
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
    pub user_token_ttl_seconds: u64,
    pub refresh_token_ttl_seconds: u64,
    pub cookie_secure: bool,
    pub public_origin: String,
}

#[derive(Deserialize)]
struct Settings {
    auth: AuthSettings,
    #[serde(default)]
    smtp: Option<compallm_mailing::SmtpSettings>,
}

/// Own all application dependencies in one permanent container.
pub struct Di {
    db: Box<DbContext>,
    users: Box<dyn UserRepository>,
    organizations: Box<dyn OrganizationRepository>,
    sessions: Box<dyn SessionRepository>,
    email: Option<Box<dyn EmailService>>,
    mailing: Option<Box<compallm_mailing::MailingService>>,
    mailing_email: OnceLock<Box<dyn EmailService>>,
    mailing_worker: OnceLock<Box<crate::workers::mailing::MailingWorker>>,
    settings: AuthSettings,
    delivery: Box<crate::controllers::auth::delivery::TokenDeliverySettings>,
    // Filled after Di reaches its permanent address so services can borrow it.
    auth: OnceLock<Box<AuthService>>,
    registration: OnceLock<Box<RegistrationService>>,
    organization_creation: OnceLock<Box<CreationService>>,
    organization_config: OnceLock<Box<ConfigService>>,
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
                let mailing = settings
                    .smtp
                    .filter(|settings| settings.enabled)
                    .map(compallm_mailing::MailingService::new)
                    .transpose()
                    .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
                let mut di = Self::new(db, settings.auth, None);
                di.mailing = mailing.map(Box::new);
                Ok::<_, loco_rs::Error>(di)
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
        let delivery = Box::new(crate::controllers::auth::delivery::TokenDeliverySettings {
            secure: settings.cookie_secure,
            public_origin: settings.public_origin.clone(),
            access_max_age_seconds: settings.user_token_ttl_seconds,
            refresh_max_age_seconds: settings.refresh_token_ttl_seconds,
        });
        Self {
            delivery,
            db: Box::new(db),
            users: Box::new(compallm_postgres::repository::user::UserRepository),
            organizations: Box::new(
                compallm_postgres::repository::organization::OrganizationRepository,
            ),
            sessions: Box::new(compallm_postgres::repository::session::SessionRepository),
            email,
            mailing: None,
            mailing_email: OnceLock::new(),
            mailing_worker: OnceLock::new(),
            settings,
            auth: OnceLock::new(),
            registration: OnceLock::new(),
            organization_creation: OnceLock::new(),
            organization_config: OnceLock::new(),
        }
    }

    pub fn initialize_services(&'static self) -> Result<(), AuthError> {
        self.mailing_worker();
        self.organization_config_service();
        if self.auth.get().is_some() {
            return Ok(());
        }
        let service = Box::new(AuthService::new(
            self.db_context(),
            self.user_repository(),
            self.organization_repository(),
            self.session_repository(),
            self.registration_service(),
            self.settings.jwt_secret.clone(),
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

    pub fn user_repository(&'static self) -> &'static Box<dyn UserRepository> {
        &self.users
    }

    pub fn session_repository(&'static self) -> &'static Box<dyn SessionRepository> {
        &self.sessions
    }

    pub fn email_service(&'static self) -> Option<&'static Box<dyn EmailService>> {
        self.email.as_ref().or_else(|| {
            self.mailing_service()
                .map(|service| self.mailing_email.get_or_init(|| Box::new(service)))
        })
    }

    pub fn mailing_service(&'static self) -> Option<&'static compallm_mailing::MailingService> {
        self.mailing.as_deref()
    }

    pub fn mailing_worker(
        &'static self,
    ) -> Option<&'static crate::workers::mailing::MailingWorker> {
        self.mailing_service().map(|service| {
            self.mailing_worker
                .get_or_init(|| Box::new(crate::workers::mailing::MailingWorker::new(service)))
                .as_ref()
        })
    }

    pub fn auth_service(&'static self) -> &'static AuthService {
        self.auth.get().expect("Di services must be initialized")
    }

    pub fn registration_service(&'static self) -> &'static RegistrationService {
        self.registration.get_or_init(|| {
            Box::new(RegistrationService::new(
                self.db_context(),
                self.user_repository(),
                self.organization_creation_service(),
            ))
        })
    }

    pub fn organization_repository(&'static self) -> &'static Box<dyn OrganizationRepository> {
        &self.organizations
    }

    pub fn organization_creation_service(&'static self) -> &'static CreationService {
        self.organization_creation.get_or_init(|| {
            Box::new(CreationService::new(
                self.db_context(),
                self.organization_repository(),
            ))
        })
    }

    pub fn organization_config_service(&'static self) -> &'static ConfigService {
        self.organization_config.get_or_init(|| {
            Box::new(ConfigService::new(
                self.db_context(),
                self.organization_repository(),
            ))
        })
    }

    /// Attach typed service state and the scoped database context to API routes.
    pub fn inject<S>(&'static self, router: &mut Router<S>)
    where
        S: Clone + Send + Sync + 'static,
    {
        let (routes, _) = crate::controllers::api_router().split_for_parts();
        let db = self.db_context();
        let routes = routes
            .with_state(self.auth_service())
            .layer(axum::Extension(self.delivery.as_ref()))
            .layer(axum::middleware::from_fn(
                move |request: axum::extract::Request, next: axum::middleware::Next| async move {
                    db.scope(next.run(request)).await
                },
            ));
        *router = std::mem::take(router).merge(routes);
    }
}
