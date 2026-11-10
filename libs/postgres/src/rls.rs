use compallm_core::authorization::{MemberPermission, policy::Action};
use serde::Serialize;

mod preparation;

pub struct Rls {
    organization_id: u64,
    permissions: Vec<Permission>,
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

impl Rls {
    pub fn organization_id(&self) -> u64 {
        self.organization_id
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn settings(&self) -> Result<Vec<(String, String)>, serde_json::Error> {
        let mut settings = vec![
            ("app.organization_id".to_owned(), self.organization_id.to_string()),
            ("app.rls_version".to_owned(), "1".to_owned()),
        ];
        for permission in &self.permissions {
            settings.push((
                format!("app.{}", permission.action.name()),
                serde_json::to_string(&permission.rules)?,
            ));
        }
        Ok(settings)
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

impl TryFrom<&MemberPermission> for Rls {
    type Error = anyhow::Error;

    fn try_from(permission: &MemberPermission) -> Result<Self, Self::Error> {
        preparation::prepare(permission)
    }
}
