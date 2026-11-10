//! Typed definitions for the application-owned Cedar schema vocabulary.
//! A schema describes valid requests; it does not grant permissions.
use cedar_policy::{EntityId, EntityTypeName, EntityUid, SchemaFragment};
use serde_json::{Map, Value, json};

/// Cedar entity types currently declared by the application schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityType {
    User,
    Organization,
    Member,
    Role,
    Group,
    Provider,
    ProviderModel,
    Model,
}

impl EntityType {
    pub const ALL: &'static [Self] = &[
        Self::User,
        Self::Organization,
        Self::Member,
        Self::Role,
        Self::Group,
        Self::Provider,
        Self::ProviderModel,
        Self::Model,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::User => "User",
            Self::Organization => "Organization",
            Self::Member => "Member",
            Self::Role => "Role",
            Self::Group => "Group",
            Self::Provider => "Provider",
            Self::ProviderModel => "ProviderModel",
            Self::Model => "Model",
        }
    }

    pub const fn parents(self) -> &'static [Self] {
        match self {
            Self::User => &[Self::Role, Self::Group, Self::Organization],
            Self::Group => &[Self::Role, Self::Organization],
            Self::ProviderModel => &[Self::Provider, Self::Organization],
            Self::Member | Self::Role | Self::Provider | Self::Model => &[Self::Organization],
            Self::Organization => &[],
        }
    }

    pub const fn attributes(self) -> &'static [Attribute] {
        const ORG: Attribute =
            Attribute::entity(AttributeName::Organization, EntityType::Organization);
        const USER: Attribute = Attribute::entity(AttributeName::User, EntityType::User);
        const PROVIDER: Attribute =
            Attribute::entity(AttributeName::Provider, EntityType::Provider);
        const ENABLED: Attribute = Attribute {
            name: AttributeName::Enabled,
            kind: AttributeType::Boolean,
        };
        match self {
            Self::Member => &[ORG, USER],
            Self::Role | Self::Group | Self::Provider => &[ORG],
            Self::ProviderModel => &[ORG, PROVIDER, ENABLED],
            Self::Model => &[ORG, ENABLED],
            Self::User | Self::Organization => &[],
        }
    }
}

impl From<EntityType> for EntityTypeName {
    fn from(entity: EntityType) -> Self {
        format!("Compallm::{}", entity.name())
            .parse()
            .expect("static Cedar entity type")
    }
}

/// Names of the required attributes and action-context fields in the schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeName {
    Organization,
    User,
    Provider,
    Enabled,
    Role,
    Member,
    ProviderModel,
}

impl AttributeName {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Organization => "organization",
            Self::User => "user",
            Self::Provider => "provider",
            Self::Enabled => "enabled",
            Self::Role => "role",
            Self::Member => "member",
            Self::ProviderModel => "providerModel",
        }
    }
}

/// A required Cedar record attribute with a statically declared name and type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub name: AttributeName,
    pub kind: AttributeType,
}

impl Attribute {
    pub const fn entity(name: AttributeName, entity: EntityType) -> Self {
        Self {
            name,
            kind: AttributeType::Entity(entity),
        }
    }
    fn json(self) -> Value {
        match self.kind {
            AttributeType::Entity(entity) => {
                json!({"type":"Entity", "name":entity.name(), "required":true})
            }
            AttributeType::Boolean => json!({"type":"Boolean", "required":true}),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeType {
    Entity(EntityType),
    Boolean,
}

// Each row maps a typed action identifier to its owning entity, resource and context.
// SystemPolicy domain fragments use these definitions as their schema source.
macro_rules! actions {
    ($( $variant:ident => ($name:literal, $owner:expr, $resource:expr, $context:expr), )*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Action { $( $variant, )* }
        impl Action {
            pub const ALL: &'static [Self] = &[$(Self::$variant,)*];
            pub const fn name(self) -> &'static str { match self { $(Self::$variant => $name,)* } }
            pub const fn owner(self) -> EntityType { match self { $(Self::$variant => $owner,)* } }
            pub const fn principal(self) -> EntityType { EntityType::User }
            pub const fn resource(self) -> EntityType { match self { $(Self::$variant => $resource,)* } }
            pub const fn context(self) -> Option<Attribute> { match self { $(Self::$variant => $context,)* } }
        }
    };
}

actions! {
    MemberRead => ("member.read", EntityType::Member, EntityType::Organization, None),
    MemberInvite => ("member.invite", EntityType::Member, EntityType::Organization, None),
    MemberRemove => ("member.remove", EntityType::Member, EntityType::Member, None),
    MemberRoleAssign => ("member.role.assign", EntityType::Member, EntityType::Member, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    MemberRoleUnassign => ("member.role.unassign", EntityType::Member, EntityType::Member, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    RoleRead => ("role.read", EntityType::Role, EntityType::Organization, None),
    RoleCreate => ("role.create", EntityType::Role, EntityType::Organization, None),
    RoleMetadataUpdate => ("role.metadata.update", EntityType::Role, EntityType::Role, None),
    RolePolicyRead => ("role.policy.read", EntityType::Role, EntityType::Role, None),
    RolePolicyUpdate => ("role.policy.update", EntityType::Role, EntityType::Role, None),
    RoleDelete => ("role.delete", EntityType::Role, EntityType::Role, None),
    RoleAssign => ("role.assign", EntityType::Role, EntityType::Role, None),
    RoleUnassign => ("role.unassign", EntityType::Role, EntityType::Role, None),
    OrganizationSettingsRead => ("organization.settings.read", EntityType::Organization, EntityType::Organization, None),
    OrganizationSettingsUpdate => ("organization.settings.update", EntityType::Organization, EntityType::Organization, None),
    OrganizationDelete => ("organization.delete", EntityType::Organization, EntityType::Organization, None),
    GroupRead => ("group.read", EntityType::Group, EntityType::Organization, None),
    GroupCreate => ("group.create", EntityType::Group, EntityType::Organization, None),
    GroupMetadataUpdate => ("group.metadata.update", EntityType::Group, EntityType::Group, None),
    GroupMemberAdd => ("group.member.add", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Member, EntityType::Member))),
    GroupMemberRemove => ("group.member.remove", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Member, EntityType::Member))),
    GroupRoleAssign => ("group.role.assign", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    GroupRoleUnassign => ("group.role.unassign", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    GroupDelete => ("group.delete", EntityType::Group, EntityType::Group, None),
    ProviderRead => ("provider.read", EntityType::Provider, EntityType::Organization, None),
    ProviderCreate => ("provider.create", EntityType::Provider, EntityType::Organization, None),
    ProviderConnectionTest => ("provider.connection.test", EntityType::Provider, EntityType::Organization, None),
    ProviderConfigurationUpdate => ("provider.configuration.update", EntityType::Provider, EntityType::Provider, None),
    ProviderDelete => ("provider.delete", EntityType::Provider, EntityType::Provider, None),
    ProviderCredentialReplace => ("provider.credential.replace", EntityType::Provider, EntityType::Provider, None),
    ProviderModelRead => ("provider.model.read", EntityType::ProviderModel, EntityType::Organization, None),
    ProviderModelCreate => ("provider.model.create", EntityType::ProviderModel, EntityType::Provider, None),
    ProviderModelConfigurationUpdate => ("provider.model.configuration.update", EntityType::ProviderModel, EntityType::ProviderModel, None),
    ProviderModelPricingUpdate => ("provider.model.pricing.update", EntityType::ProviderModel, EntityType::ProviderModel, None),
    ProviderModelDelete => ("provider.model.delete", EntityType::ProviderModel, EntityType::ProviderModel, None),
    ModelRead => ("model.read", EntityType::Model, EntityType::Organization, None),
    ModelCreate => ("model.create", EntityType::Model, EntityType::Organization, None),
    ModelConfigurationUpdate => ("model.configuration.update", EntityType::Model, EntityType::Model, None),
    ModelDelete => ("model.delete", EntityType::Model, EntityType::Model, None),
    ModelUse => ("model.use", EntityType::Model, EntityType::Model, None),
    ModelRouteAdd => ("model.route.add", EntityType::Model, EntityType::Model, Some(Attribute::entity(AttributeName::ProviderModel, EntityType::ProviderModel))),
    ModelRouteUpdate => ("model.route.update", EntityType::Model, EntityType::Model, Some(Attribute::entity(AttributeName::ProviderModel, EntityType::ProviderModel))),
    ModelRouteRemove => ("model.route.remove", EntityType::Model, EntityType::Model, Some(Attribute::entity(AttributeName::ProviderModel, EntityType::ProviderModel))),
    ModelPricingUpdate => ("model.pricing.update", EntityType::Model, EntityType::Model, None),
}

impl From<Action> for EntityUid {
    fn from(action: Action) -> Self {
        EntityUid::from_type_name_and_id(
            "Compallm::Action"
                .parse()
                .expect("static Cedar Action type"),
            EntityId::new(action.name()),
        )
    }
}

/// One domain entity and its Cedar schema actions; not a permit/forbid grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    entity: EntityType,
}

impl Policy {
    pub const fn new(entity: EntityType) -> Self {
        Self { entity }
    }
    pub const fn entity(&self) -> EntityType {
        self.entity
    }
    pub fn actions(&self) -> impl Iterator<Item = Action> + '_ {
        Action::ALL
            .iter()
            .copied()
            .filter(|action| action.owner() == self.entity)
    }
}

impl From<Policy> for SchemaFragment {
    fn from(policy: Policy) -> Self {
        let mut entity_types = Map::new();
        let entity = policy.entity;
        let attributes: Map<String, Value> = entity
            .attributes()
            .iter()
            .map(|attribute| (attribute.name.name().to_owned(), attribute.json()))
            .collect();
        let parents: Vec<&str> = entity
            .parents()
            .iter()
            .map(|parent| parent.name())
            .collect();
        entity_types.insert(
            entity.name().into(),
            json!({
                "memberOfTypes": parents,
                "shape": {"type":"Record", "attributes":attributes, "additionalAttributes":false}
            }),
        );
        let mut actions = Map::new();
        for action in policy.actions() {
            let mut attributes = Map::new();
            if let Some(attribute) = action.context() {
                attributes.insert(attribute.name.name().into(), attribute.json());
            }
            actions.insert(action.name().into(), json!({"appliesTo": {
                "principalTypes":[action.principal().name()], "resourceTypes":[action.resource().name()],
                "context":{"type":"Record", "attributes":attributes, "additionalAttributes":false}
            }}));
        }
        SchemaFragment::from_json_value(
            json!({"Compallm":{"entityTypes":entity_types,"actions":actions}}),
        )
        .expect("static typed Cedar schema must be valid")
    }
}
