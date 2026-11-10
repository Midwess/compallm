use crate::{
    graphql::{ApplicationSchema, build_schema},
    http::{authentication::AuthenticationSettings, middleware::AuthenticationMiddleware},
};
use compallm_core::{
    authentication::{AuthError, AuthService, TokenLifetimes},
    mail::EmailService,
    organization::{ConfigService, CreationService},
    persistent::dictionary::DictionaryRepository,
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

/// HTTP cookie policy and GraphQL request bounds from application configuration.
#[derive(Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GraphqlSettings {
    pub cookie_secure: bool,
    pub public_origin: String,
    pub max_body_bytes: usize,
    pub max_depth: usize,
    pub max_fields: usize,
    pub max_tokens: usize,
}

impl Default for GraphqlSettings {
    fn default() -> Self {
        Self {
            cookie_secure: true,
            public_origin: String::new(),
            max_body_bytes: 65_536,
            max_depth: 16,
            max_fields: 256,
            max_tokens: 15_000,
        }
    }
}

#[derive(Deserialize)]
pub struct AuthSettings {
    pub jwt_secret: String,
    pub user_token_ttl_seconds: u64,
    pub refresh_token_ttl_seconds: u64,
}

#[derive(Deserialize)]
struct Settings {
    auth: AuthSettings,
    #[serde(default)]
    graphql: GraphqlSettings,
    #[serde(default)]
    smtp: Option<compallm_mailing::SmtpSettings>,
}

/// Own all application dependencies in one permanent container.
pub struct Di {
    db: Box<DbContext>,
    users: Box<dyn UserRepository>,
    organizations: Box<dyn OrganizationRepository>,
    sessions: Box<dyn SessionRepository>,
    dictionaries: Box<dyn DictionaryRepository>,
    email: Option<Box<dyn EmailService>>,
    mailing: Option<Box<compallm_mailing::MailingService>>,
    mailing_email: OnceLock<Box<dyn EmailService>>,
    mailing_worker: OnceLock<Box<crate::workers::mailing::MailingWorker>>,
    settings: AuthSettings,
    graphql_settings: GraphqlSettings,
    graphql: OnceLock<ApplicationSchema>,
    authentication: OnceLock<AuthenticationMiddleware>,
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

    // Loco owns this framework error type; keep its required API at this boundary.
    #[allow(clippy::result_large_err)]
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
                let mut di = Self::new(db, settings.auth, settings.graphql, None);
                di.mailing = mailing.map(Box::new);
                DbContext::initialize_default(di.db.postgres().scoped())
                    .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
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
        graphql_settings: GraphqlSettings,
        email: Option<Box<dyn EmailService>>,
    ) -> Self {
        let dictionaries =
            compallm_postgres::repository::dictionary::DictionaryRepository::new(db.clone())
                .expect("application dictionary connection must be committed");
        Self {
            db: Box::new(db),
            users: Box::new(compallm_postgres::repository::user::UserRepository),
            organizations: Box::new(
                compallm_postgres::repository::organization::OrganizationRepository,
            ),
            sessions: Box::new(compallm_postgres::repository::session::SessionRepository),
            dictionaries: Box::new(dictionaries),
            email,
            mailing: None,
            mailing_email: OnceLock::new(),
            mailing_worker: OnceLock::new(),
            settings,
            graphql_settings,
            graphql: OnceLock::new(),
            authentication: OnceLock::new(),
            auth: OnceLock::new(),
            registration: OnceLock::new(),
            organization_creation: OnceLock::new(),
            organization_config: OnceLock::new(),
        }
    }

    /// Supply an existing service before the container reaches its permanent address.
    /// All request projections then borrow this same instance.
    pub fn with_auth_service(mut self, auth: AuthService) -> Self {
        self.auth = OnceLock::from(Box::new(auth));
        self
    }

    pub fn initialize_services(&'static self) -> Result<(), AuthError> {
        self.mailing_worker();
        self.organization_config_service();
        if self.auth.get().is_some() {
            self.initialize_http()?;
            return Ok(());
        }
        let service = Box::new(AuthService::new(
            self.user_repository(),
            self.organization_repository(),
            self.session_repository(),
            &self.dictionaries,
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
        self.initialize_http()?;
        Ok(())
    }

    fn initialize_http(&'static self) -> Result<(), AuthError> {
        let settings = &self.graphql_settings;
        if settings.max_body_bytes == 0
            || settings.max_depth == 0
            || settings.max_fields == 0
            || settings.max_tokens == 0
        {
            return Err(AuthError::InvalidConfiguration);
        }
        if self.authentication.get().is_none() {
            let authentication = AuthenticationMiddleware::optional(
                self.db_context(),
                self.auth_service(),
                AuthenticationSettings {
                    cookie_secure: settings.cookie_secure,
                    public_origin: settings.public_origin.clone(),
                },
            )?;
            let _ = self.authentication.set(authentication);
        }
        self.graphql.get_or_init(|| {
            build_schema(settings.max_depth, settings.max_fields, settings.max_tokens)
        });
        Ok(())
    }

    pub fn graphql_schema(&self) -> &ApplicationSchema {
        self.graphql
            .get()
            .expect("GraphQL schema must be initialized")
    }

    pub fn authentication(&self) -> &AuthenticationMiddleware {
        self.authentication
            .get()
            .expect("HTTP authentication must be initialized")
    }

    pub fn max_request_body_bytes(&self) -> usize {
        self.graphql_settings.max_body_bytes
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

    pub fn auth_service(&self) -> &AuthService {
        self.auth.get().expect("Di services must be initialized")
    }

    pub fn registration_service(&'static self) -> &'static RegistrationService {
        self.registration.get_or_init(|| {
            Box::new(RegistrationService::new(
                self.user_repository(),
                self.organization_creation_service(),
            ))
        })
    }

    pub fn organization_repository(&'static self) -> &'static Box<dyn OrganizationRepository> {
        &self.organizations
    }

    pub fn organization_creation_service(&'static self) -> &'static CreationService {
        self.organization_creation
            .get_or_init(|| Box::new(CreationService::new(self.organization_repository())))
    }

    pub fn organization_config_service(&'static self) -> &'static ConfigService {
        self.organization_config
            .get_or_init(|| Box::new(ConfigService::new(self.organization_repository())))
    }
}

/// Juniper borrows the initialized service from the application's sole DI container.
impl juniper::FromContext<Di> for AuthService {
    fn from(di: &Di) -> &Self {
        di.auth_service()
    }
}
