//! Application-owned Cedar vocabulary and policy source normalization.
//! A schema describes valid requests; it does not grant permissions.
use cedar_policy::{EntityId, EntityTypeName, EntityUid, PolicySet, SchemaFragment, SlotId};
use cedar_policy_formatter::lexer::get_token_stream;
use serde_json::{Map, Value, json};
use std::collections::HashMap;

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
    ($( $variant:ident => ($name:literal, $label:literal, $description:literal, $owner:expr, $resource:expr, $context:expr), )*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Action { $( $variant, )* }
        impl Action {
            pub const ALL: &'static [Self] = &[$(Self::$variant,)*];
            pub const fn name(self) -> &'static str { match self { $(Self::$variant => $name,)* } }
            pub const fn display_name(self) -> &'static str { match self { $(Self::$variant => $label,)* } }
            pub const fn description(self) -> &'static str { match self { $(Self::$variant => $description,)* } }
            pub const fn owner(self) -> EntityType { match self { $(Self::$variant => $owner,)* } }
            pub const fn principal(self) -> EntityType { EntityType::User }
            pub const fn resource(self) -> EntityType { match self { $(Self::$variant => $resource,)* } }
            pub const fn context(self) -> Option<Attribute> { match self { $(Self::$variant => $context,)* } }
        }
    };
}

actions! {
    MemberRead => ("member.read", "View members", "View organization member details.", EntityType::Member, EntityType::Organization, None),
    MemberInvite => ("member.invite", "Invite members", "Invite people to the organization.", EntityType::Member, EntityType::Organization, None),
    MemberRemove => ("member.remove", "Remove members", "Remove members from the organization.", EntityType::Member, EntityType::Member, None),
    MemberRoleAssign => ("member.role.assign", "Assign member roles", "Assign roles directly to members.", EntityType::Member, EntityType::Member, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    MemberRoleUnassign => ("member.role.unassign", "Remove member roles", "Remove roles assigned directly to members.", EntityType::Member, EntityType::Member, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    RoleRead => ("role.read", "View roles", "View organization role details.", EntityType::Role, EntityType::Organization, None),
    RoleCreate => ("role.create", "Create roles", "Create organization roles and their policies.", EntityType::Role, EntityType::Organization, None),
    RoleMetadataUpdate => ("role.metadata.update", "Edit role details", "Change a role name or description.", EntityType::Role, EntityType::Role, None),
    RolePolicyRead => ("role.policy.read", "View role policies", "View the Cedar policy assigned to a role.", EntityType::Role, EntityType::Role, None),
    RolePolicyUpdate => ("role.policy.update", "Edit role policies", "Change the Cedar policy assigned to a role.", EntityType::Role, EntityType::Role, None),
    RoleDelete => ("role.delete", "Delete roles", "Delete organization roles.", EntityType::Role, EntityType::Role, None),
    RoleAssign => ("role.assign", "Assign roles", "Assign a role to a member or group.", EntityType::Role, EntityType::Role, None),
    RoleUnassign => ("role.unassign", "Remove role assignments", "Remove a role from a member or group.", EntityType::Role, EntityType::Role, None),
    OrganizationSettingsRead => ("organization.settings.read", "View organization settings", "View organization configuration.", EntityType::Organization, EntityType::Organization, None),
    OrganizationSettingsUpdate => ("organization.settings.update", "Edit organization settings", "Change organization configuration.", EntityType::Organization, EntityType::Organization, None),
    OrganizationDelete => ("organization.delete", "Delete organization", "Delete the organization.", EntityType::Organization, EntityType::Organization, None),
    GroupRead => ("group.read", "View groups", "View organization group details.", EntityType::Group, EntityType::Organization, None),
    GroupCreate => ("group.create", "Create groups", "Create groups in the organization.", EntityType::Group, EntityType::Organization, None),
    GroupMetadataUpdate => ("group.metadata.update", "Edit group details", "Change a group name or description.", EntityType::Group, EntityType::Group, None),
    GroupMemberAdd => ("group.member.add", "Add group members", "Add members to a group.", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Member, EntityType::Member))),
    GroupMemberRemove => ("group.member.remove", "Remove group members", "Remove members from a group.", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Member, EntityType::Member))),
    GroupRoleAssign => ("group.role.assign", "Assign group roles", "Assign roles to a group.", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    GroupRoleUnassign => ("group.role.unassign", "Remove group roles", "Remove roles assigned to a group.", EntityType::Group, EntityType::Group, Some(Attribute::entity(AttributeName::Role, EntityType::Role))),
    GroupDelete => ("group.delete", "Delete groups", "Delete organization groups.", EntityType::Group, EntityType::Group, None),
    ProviderRead => ("provider.read", "View providers", "View providers and their credentials.", EntityType::Provider, EntityType::Organization, None),
    ProviderCreate => ("provider.create", "Create providers", "Create providers in the organization.", EntityType::Provider, EntityType::Organization, None),
    ProviderConfigurationUpdate => ("provider.configuration.update", "Edit provider settings", "Change a provider configuration.", EntityType::Provider, EntityType::Provider, None),
    ProviderDelete => ("provider.delete", "Delete providers", "Delete organization providers.", EntityType::Provider, EntityType::Provider, None),
    ProviderCredentialReplace => ("provider.credential.replace", "Replace provider credentials", "Replace credentials used by a provider.", EntityType::Provider, EntityType::Provider, None),
    ProviderModelRead => ("provider.model.read", "View provider models", "View models configured for providers.", EntityType::ProviderModel, EntityType::Organization, None),
    ProviderModelCreate => ("provider.model.create", "Create provider models", "Add models to a provider.", EntityType::ProviderModel, EntityType::Provider, None),
    ProviderModelConfigurationUpdate => ("provider.model.configuration.update", "Edit provider model settings", "Change a provider model configuration.", EntityType::ProviderModel, EntityType::ProviderModel, None),
    ProviderModelPricingUpdate => ("provider.model.pricing.update", "Edit provider model prices", "Change provider model pricing.", EntityType::ProviderModel, EntityType::ProviderModel, None),
    ProviderModelDelete => ("provider.model.delete", "Delete provider models", "Remove models from a provider.", EntityType::ProviderModel, EntityType::ProviderModel, None),
    ModelRead => ("model.read", "View models", "View organization model details.", EntityType::Model, EntityType::Organization, None),
    ModelCreate => ("model.create", "Create models", "Create models in the organization.", EntityType::Model, EntityType::Organization, None),
    ModelConfigurationUpdate => ("model.configuration.update", "Edit model settings", "Change a model configuration.", EntityType::Model, EntityType::Model, None),
    ModelDelete => ("model.delete", "Delete models", "Delete organization models.", EntityType::Model, EntityType::Model, None),
    ModelUse => ("model.use", "Use models", "Use an enabled model.", EntityType::Model, EntityType::Model, None),
    ModelRouteAdd => ("model.route.add", "Add model routes", "Add provider models to a model route.", EntityType::Model, EntityType::Model, Some(Attribute::entity(AttributeName::ProviderModel, EntityType::ProviderModel))),
    ModelRouteUpdate => ("model.route.update", "Edit model routes", "Change a model route.", EntityType::Model, EntityType::Model, Some(Attribute::entity(AttributeName::ProviderModel, EntityType::ProviderModel))),
    ModelRouteRemove => ("model.route.remove", "Remove model routes", "Remove provider models from a model route.", EntityType::Model, EntityType::Model, Some(Attribute::entity(AttributeName::ProviderModel, EntityType::ProviderModel))),
    ModelPricingUpdate => ("model.pricing.update", "Edit model prices", "Change model pricing.", EntityType::Model, EntityType::Model, None),
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

/// Remove comments and unnecessary whitespace without altering Cedar tokens.
pub fn compact_policy_source(source: &str) -> String {
    let Some((tokens, _)) = get_token_stream(source) else {
        return source.to_owned();
    };
    if tokens.is_empty() {
        return String::new();
    }
    let mut result = String::with_capacity(source.len());
    for (index, token) in tokens.iter().enumerate() {
        if index > 0 {
            let previous = &tokens[index - 1];
            let left = &source[previous.span.clone()];
            let right = &source[token.span.clone()];
            let pair = format!("{left}{right}");
            let unchanged = get_token_stream(&pair).is_some_and(|(pair_tokens, _)| {
                pair_tokens.len() == 2
                    && pair_tokens[0].token == previous.token
                    && pair_tokens[1].token == token.token
                    && pair_tokens[0].span.end == left.len()
            });
            if !unchanged {
                result.push(' ');
            }
        }
        result.push_str(&source[token.span.clone()]);
    }
    result
}

pub fn compact_policy_input(source: &mut String) {
    *source = compact_policy_source(source);
}

/// Link native Cedar slots to the target role and its organization.
/// Returned policies are static so evaluators and row filtering share the same source.
pub fn role_policy_set(
    source: &str,
    organization_id: u64,
    role_id: u64,
) -> anyhow::Result<PolicySet> {
    let mut parsed: PolicySet = source.parse()?;
    let principal: EntityUid =
        format!("Compallm::Role::\"{organization_id}:{role_id}\"").parse()?;
    let resource: EntityUid = format!("Compallm::Organization::\"{organization_id}\"").parse()?;
    let templates = parsed
        .templates()
        .map(|template| {
            let bindings = template
                .slots()
                .map(|slot| {
                    let value = if slot == &SlotId::principal() {
                        principal.clone()
                    } else {
                        resource.clone()
                    };
                    (slot.clone(), value)
                })
                .collect::<HashMap<_, _>>();
            (template.id().clone(), bindings)
        })
        .collect::<Vec<_>>();
    for (id, bindings) in templates {
        let linked_id = format!("linked:{id}").parse()?;
        parsed.link(id, linked_id, bindings)?;
    }
    let mut result = PolicySet::new();
    for policy in parsed.policies() {
        result.add(cedar_policy::Policy::parse(
            Some(policy.id().clone()),
            policy.to_string(),
        )?)?;
    }
    Ok(result)
}

/// Present self-role and organization scope identities as native Cedar slots.
/// Conditions and references to other entities keep their concrete identities.
pub fn role_authoring_source(
    source: &str,
    organization_id: u64,
    role_id: u64,
) -> anyhow::Result<String> {
    let policies: PolicySet = source.parse()?;
    let principal: EntityUid =
        format!("Compallm::Role::\"{organization_id}:{role_id}\"").parse()?;
    let resource: EntityUid = format!("Compallm::Organization::\"{organization_id}\"").parse()?;
    let mut statements = policies
        .policies()
        .map(|policy| Ok((policy.id().clone(), policy.to_json()?, policy.to_string())))
        .chain(policies.templates().map(|template| {
            Ok((
                template.id().clone(),
                template.to_json()?,
                template.to_cedar(),
            ))
        }))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let mut changed = false;
    for (id, json, text) in &mut statements {
        let principal_changed = template_scope(&mut json["principal"], &principal, "?principal");
        let resource_changed = template_scope(&mut json["resource"], &resource, "?resource");
        if principal_changed || resource_changed {
            *text = cedar_policy::Template::from_json(Some(id.clone()), json.clone())?.to_cedar();
            changed = true;
        }
    }
    if !changed {
        return Ok(source.to_owned());
    }
    statements.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(statements
        .into_iter()
        .map(|(_, _, text)| text)
        .collect::<Vec<_>>()
        .join("\n"))
}

fn template_scope(scope: &mut Value, entity: &EntityUid, slot: &str) -> bool {
    let target = match scope["op"].as_str() {
        Some("==" | "in") => scope,
        Some("is") => &mut scope["in"],
        _ => return false,
    };
    let Some(candidate) = target.get("entity") else {
        return false;
    };
    let candidate = candidate.get("__entity").unwrap_or(candidate);
    if candidate["type"].as_str() != Some(entity.type_name().to_string().as_str())
        || candidate["id"].as_str() != Some(entity.id().as_ref())
    {
        return false;
    }
    *target = match target.get("op").cloned() {
        Some(op) => json!({"op": op, "slot": slot}),
        None => json!({"slot": slot}),
    };
    true
}
