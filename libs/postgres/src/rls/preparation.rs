use anyhow::{Context, bail};
use cedar_policy::{ActionConstraint, Effect, EntityUid, PrincipalConstraint, ResourceConstraint};
use compallm_core::authorization::{MemberPermission, policy::{Action, EntityType}};
use serde_json::Value;

use super::{Condition, Entity, Field, Operand, Permission, RlsSettings, Rules};

pub(super) fn prepare(member: &MemberPermission) -> anyhow::Result<RlsSettings> {
    let mut permissions: Vec<_> = Action::ALL.iter().map(|action| Permission {
        action: *action,
        rules: Rules { permits: Vec::new(), forbids: Vec::new() },
    }).collect();
    for policy in member.policies().policies() {
        let principal = member.principal();
        let matches = match policy.principal_constraint() {
            PrincipalConstraint::Any => true,
            PrincipalConstraint::Eq(uid) => principal == &uid,
            PrincipalConstraint::In(uid) => member.entities().is_ancestor_of(&uid, principal),
            PrincipalConstraint::Is(kind) => principal.type_name() == &kind,
            PrincipalConstraint::IsIn(kind, uid) => principal.type_name() == &kind
                && member.entities().is_ancestor_of(&uid, principal),
        };
        if !matches { continue; }
        let json = policy.to_json()?;
        for permission in &mut permissions {
            let action_uid: EntityUid = permission.action.into();
            let matches = match policy.action_constraint() {
                ActionConstraint::Any => true,
                ActionConstraint::Eq(uid) => action_uid == uid,
                ActionConstraint::In(uids) => uids.contains(&action_uid),
            };
            if !matches { continue; }
            let organization = Entity { kind: "Organization".into(), id: member.organization_id().to_string() };
            let organization_field = if permission.action.resource() == EntityType::Organization {
                Field::Resource
            } else {
                Field::ResourceOrganization
            };
            let mut conditions = vec![
                Condition::Equal(Operand::Field(organization_field), Operand::Entity(organization)),
                scope(policy.resource_constraint())?,
            ];
            for condition in json["conditions"].as_array().context("missing policy conditions")? {
                let expression = expression(&condition["body"], permission.action)
                    .with_context(|| format!("cannot prepare {} for PostgreSQL", permission.action.name()))?;
                conditions.push(match condition["kind"].as_str() {
                    Some("when") => expression,
                    Some("unless") => Condition::Not(Box::new(expression)),
                    _ => bail!("unsupported policy condition"),
                });
            }
            let condition = Condition::All(conditions);
            match policy.effect() {
                Effect::Permit => permission.rules.permits.push(condition),
                Effect::Forbid => permission.rules.forbids.push(condition),
            }
        }
    }
    Ok(RlsSettings::new(member.organization_id(), permissions)?)
}

fn entity(uid: &EntityUid) -> anyhow::Result<Entity> {
    let kind = EntityType::ALL.iter().find(|kind| {
        uid.type_name().to_string() == format!("Compallm::{}", kind.name())
    }).context("unsupported entity type")?;
    let id: &str = uid.id().as_ref();
    let id = id.to_owned();
    let valid = |s: &str| s.parse::<u64>().is_ok_and(|id| id > 0 && id <= i64::MAX as u64);
    let valid_id = match kind {
        EntityType::User | EntityType::Organization => valid(&id),
        _ => id.split_once(':').is_some_and(|(org, id)| valid(org) && valid(id)),
    };
    anyhow::ensure!(valid_id, "unsupported entity identity");
    Ok(Entity { kind: kind.name().into(), id })
}

fn scope(scope: ResourceConstraint) -> anyhow::Result<Condition> {
    let resource = Operand::Field(Field::Resource);
    Ok(match scope {
        ResourceConstraint::Any => Condition::Constant(true),
        ResourceConstraint::Eq(uid) => Condition::Equal(resource, Operand::Entity(entity(&uid)?)),
        ResourceConstraint::In(uid) => Condition::In(resource, entity(&uid)?),
        ResourceConstraint::Is(kind) => Condition::Is(resource, entity_type(&kind.to_string())?),
        ResourceConstraint::IsIn(kind, uid) => Condition::All(vec![
            Condition::Is(resource.clone(), entity_type(&kind.to_string())?),
            Condition::In(resource, entity(&uid)?),
        ]),
    })
}

fn entity_type(name: &str) -> anyhow::Result<String> {
    EntityType::ALL.iter().find(|kind| name == format!("Compallm::{}", kind.name()))
        .map(|kind| kind.name().to_owned()).context("unsupported resource type")
}

fn expression(value: &Value, action: Action) -> anyhow::Result<Condition> {
    let object = value.as_object().context("invalid condition expression")?;
    anyhow::ensure!(object.len() == 1, "invalid condition expression");
    let (operator, arguments) = object.iter().next().context("missing condition operator")?;
    Ok(match operator.as_str() {
        "Value" => Condition::Constant(arguments.as_bool().context("unsupported condition literal")?),
        "&&" | "||" => {
            let conditions = vec![expression(&arguments["left"], action)?, expression(&arguments["right"], action)?];
            if operator == "&&" { Condition::All(conditions) } else { Condition::Any(conditions) }
        }
        "!" => Condition::Not(Box::new(expression(arguments.get("arg").unwrap_or(arguments), action)?)),
        "==" | "!=" => {
            let left = &arguments["left"];
            let right = &arguments["right"];
            let equal = if let Some(boolean) = right.get("Value").and_then(Value::as_bool) {
                let field = enabled_field(left, action)?;
                if boolean { Condition::Enabled(field) } else { Condition::Not(Box::new(Condition::Enabled(field))) }
            } else if let Some(boolean) = left.get("Value").and_then(Value::as_bool) {
                let field = enabled_field(right, action)?;
                if boolean { Condition::Enabled(field) } else { Condition::Not(Box::new(Condition::Enabled(field))) }
            } else {
                Condition::Equal(operand(left, action)?, operand(right, action)?)
            };
            if operator == "!=" { Condition::Not(Box::new(equal)) } else { equal }
        }
        "in" => {
            let Operand::Entity(parent) = operand(&arguments["right"], action)? else {
                bail!("unsupported entity ancestry condition");
            };
            Condition::In(operand(&arguments["left"], action)?, parent)
        }
        "." => Condition::Enabled(enabled_field(value, action)?),
        _ => bail!("unsupported data-protection condition operator: {operator}"),
    })
}

fn enabled_field(value: &Value, action: Action) -> anyhow::Result<Field> {
    match operand(value, action)? {
        Operand::Field(field @ (Field::ResourceEnabled | Field::ContextEnabled)) => Ok(field),
        _ => bail!("expected enabled attribute"),
    }
}

fn operand(value: &Value, action: Action) -> anyhow::Result<Operand> {
    if let Some(entity_value) = value.get("Value").and_then(|v| v.get("__entity")) {
        let uid = EntityUid::from_json(entity_value.clone())?;
        return Ok(Operand::Entity(entity(&uid)?));
    }
    if value.get("Var").and_then(Value::as_str) == Some("resource") {
        return Ok(Operand::Field(Field::Resource));
    }
    let access = value.get(".").context("unsupported condition operand")?;
    let attribute = access["attr"].as_str().context("missing attribute")?;
    if access["left"].get("Var").and_then(Value::as_str) == Some("context") {
        let context = action.context().context("action has no context")?;
        anyhow::ensure!(attribute == context.name.name(), "unsupported context attribute");
        return Ok(Operand::Field(Field::Context));
    }
    let base = operand(&access["left"], action)?;
    let field = match (base, attribute) {
        (Operand::Field(Field::Resource), "organization") => Field::ResourceOrganization,
        (Operand::Field(Field::Resource), "provider") => Field::ResourceProvider,
        (Operand::Field(Field::Resource), "user") => Field::ResourceUser,
        (Operand::Field(Field::Resource), "enabled") => Field::ResourceEnabled,
        (Operand::Field(Field::Context), "organization") => Field::ContextOrganization,
        (Operand::Field(Field::Context), "provider") => Field::ContextProvider,
        (Operand::Field(Field::Context), "user") => Field::ContextUser,
        (Operand::Field(Field::Context), "enabled") => Field::ContextEnabled,
        _ => bail!("unsupported resource attribute"),
    };
    Ok(Operand::Field(field))
}
