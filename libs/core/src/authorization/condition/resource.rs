use std::collections::{HashMap, HashSet};

use cedar_policy::{Context, Entity, EntityId, EntityUid, RestrictedExpression};

use super::CheckPermission;
use crate::authorization::{
    MemberPermission, MemberPermissionError, all_policies,
    policy::{Action, AttributeName, AttributeType, EntityType},
};

pub struct Resource {
    pub(super) kind: EntityType,
    pub(super) organization_id: u64,
    pub(super) uid: EntityUid,
    pub(super) entities: Vec<Entity>,
    context: Option<(AttributeName, EntityType, EntityUid)>,
}

pub struct ResourceCondition<R> {
    resource: R,
    action: Action,
}

impl<R> ResourceCondition<R> {
    pub(super) fn new(resource: R, action: Action) -> Self {
        Self { resource, action }
    }
}

impl<R: Copy> CheckPermission for ResourceCondition<R>
where
    Resource: TryFrom<R, Error = MemberPermissionError>,
{
    fn check_permission(&self, member: &MemberPermission) -> Result<(), MemberPermissionError> {
        let resource = Resource::try_from(self.resource)?;
        if resource.organization_id != member.organization_id() {
            return Err(MemberPermissionError::Denied);
        }
        if self.action.resource() != resource.kind {
            return Err(MemberPermissionError::Evaluation);
        }
        let context = match (self.action.context(), resource.context) {
            (None, None) => Context::empty(),
            (Some(required), Some((name, kind, uid)))
                if required.name == name && required.kind == AttributeType::Entity(kind) =>
            {
                Context::from_pairs([(
                    name.name().to_owned(),
                    RestrictedExpression::new_entity_uid(uid),
                )])
                .map_err(|_| MemberPermissionError::Evaluation)?
            }
            _ => return Err(MemberPermissionError::Evaluation),
        };
        let entities = member
            .entities()
            .clone()
            .add_entities(resource.entities, Some(all_policies()))
            .map_err(|_| MemberPermissionError::Evaluation)?;
        member.evaluate(self.action, resource.uid, context, &entities)
    }
}

impl Resource {
    pub(super) fn new(
        kind: EntityType,
        organization_id: u64,
        id: u64,
        mut attributes: HashMap<String, RestrictedExpression>,
        mut parents: HashSet<EntityUid>,
    ) -> Result<Self, MemberPermissionError> {
        let valid = |id: u64| id > 0 && id <= i64::MAX as u64;
        if !valid(organization_id)
            || !valid(id)
            || (kind == EntityType::Organization && id != organization_id)
        {
            return Err(MemberPermissionError::Evaluation);
        }
        let organization = EntityUid::from_type_name_and_id(
            EntityType::Organization.into(),
            EntityId::new(organization_id.to_string()),
        );
        let identity = if kind == EntityType::Organization {
            id.to_string()
        } else {
            format!("{organization_id}:{id}")
        };
        let uid = EntityUid::from_type_name_and_id(kind.into(), EntityId::new(identity));
        if kind != EntityType::Organization {
            attributes.insert(
                "organization".into(),
                RestrictedExpression::new_entity_uid(organization.clone()),
            );
            parents.insert(organization);
        }
        let entity = Entity::new(uid.clone(), attributes, parents)
            .map_err(|_| MemberPermissionError::Evaluation)?;
        Ok(Self {
            kind,
            organization_id,
            uid,
            entities: vec![entity],
            context: None,
        })
    }

    pub(super) fn with_context(
        mut self,
        name: AttributeName,
        related: Self,
    ) -> Result<Self, MemberPermissionError> {
        if self.organization_id != related.organization_id {
            return Err(MemberPermissionError::Denied);
        }
        self.context = Some((name, related.kind, related.uid));
        self.entities.extend(related.entities);
        Ok(self)
    }
}
