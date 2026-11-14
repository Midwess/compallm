use async_once_cell::Lazy;
use compallm_core::authorization::{MemberPermission, policy::Action};
use compallm_core::persistent::{PermissionLoader, connection::IdentityLoader};
use serde::Serialize;
use std::{
    error::Error,
    future::Future,
    pin::Pin,
    sync::{Arc, OnceLock},
};

mod preparation;

type PreparationError = Arc<dyn Error + Send + Sync>;
type SettingsResult = Result<RlsSettings, PreparationError>;
type SettingsFuture = Pin<Box<dyn Future<Output = SettingsResult> + Send>>;

#[derive(Clone)]
pub struct Rls {
    settings: Arc<Lazy<SettingsResult, SettingsFuture>>,
    permission: PermissionLoader,
    identity: Arc<OnceLock<IdentityLoader>>,
    identity_settings: Arc<tokio::sync::OnceCell<SettingsResult>>,
}

pub struct RlsSettings {
    user_id: u64,
    organization_id: u64,
    permissions: Vec<Permission>,
    settings: Vec<(String, String)>,
}

impl Rls {
    pub fn new(permission: PermissionLoader) -> Self {
        let source = permission.clone();
        let loader: SettingsFuture = Box::pin(async move {
            let permission = source.get_unpin().await.as_ref().map_err(Arc::clone)?;
            RlsSettings::try_from(permission.as_ref())
                .map_err(|error| Arc::from(error.into_boxed_dyn_error()))
        });
        Self {
            settings: Arc::new(Lazy::new(loader)),
            permission,
            identity: Arc::new(OnceLock::new()),
            identity_settings: Arc::new(tokio::sync::OnceCell::new()),
        }
    }

    pub fn identify(&self, identity: IdentityLoader) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.settings.try_get().is_none() && self.identity_settings.get().is_none(),
            "database work has already started"
        );
        self.identity
            .set(identity)
            .map_err(|_| anyhow::anyhow!("identity loader is already set"))
    }

    pub async fn get(&self) -> Result<&RlsSettings, PreparationError> {
        if let Some(identity) = self.identity.get() {
            let settings = self
                .identity_settings
                .get_or_init(|| async {
                    let access = identity.get_unpin().await.as_ref().map_err(Arc::clone)?;
                    RlsSettings::new(access.user_id, access.organization_id, Vec::new())
                        .map_err(|error| Arc::new(error) as PreparationError)
                })
                .await
                .as_ref()
                .map_err(Arc::clone)?;
            if !matches!(self.permission.try_get(), Some(Ok(_))) {
                return Ok(settings);
            }
        }
        self.settings.get_unpin().await.as_ref().map_err(Arc::clone)
    }
}

pub struct Permission {
    action: Action,
    rules: Rules,
}

#[derive(Clone, Debug, Serialize)]
pub struct Rules {
    permits: Vec<Condition>,
    forbids: Vec<Condition>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Constant(bool),
    Equal(Operand, Operand),
    In(Operand, Entity),
    Is(Operand, String),
    Enabled(Field),
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operand {
    Field(Field),
    Entity(Entity),
}

#[derive(Clone, Debug, Serialize)]
pub struct Entity {
    kind: String,
    id: String,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Resource,
    ResourceOrganization,
    ResourceProvider,
    ResourceUser,
    ResourceEnabled,
    Context,
    ContextOrganization,
    ContextProvider,
    ContextUser,
    ContextEnabled,
}

impl RlsSettings {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn organization_id(&self) -> u64 {
        self.organization_id
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn settings(&self) -> &[(String, String)] {
        &self.settings
    }

    fn new(
        user_id: u64,
        organization_id: u64,
        permissions: Vec<Permission>,
    ) -> Result<Self, serde_json::Error> {
        let mut settings = vec![
            ("app.user_id".to_owned(), user_id.to_string()),
            (
                "app.organization_id".to_owned(),
                organization_id.to_string(),
            ),
            ("app.rls_version".to_owned(), "1".to_owned()),
        ];
        for permission in &permissions {
            settings.push((
                format!("app.{}", permission.action.name()),
                serde_json::to_string(&permission.rules)?,
            ));
        }
        Ok(Self {
            user_id,
            organization_id,
            permissions,
            settings,
        })
    }
}

impl Permission {
    pub fn action(&self) -> Action {
        self.action
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }
}

impl Rules {
    pub fn permits(&self) -> &[Condition] {
        &self.permits
    }

    pub fn forbids(&self) -> &[Condition] {
        &self.forbids
    }
}

impl TryFrom<&MemberPermission> for RlsSettings {
    type Error = anyhow::Error;

    fn try_from(permission: &MemberPermission) -> Result<Self, Self::Error> {
        preparation::prepare(permission)
    }
}

pub fn validate_policy_source(source: &str) -> anyhow::Result<()> {
    preparation::validate_source(source)
}
