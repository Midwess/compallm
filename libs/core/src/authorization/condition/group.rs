use std::collections::{HashMap, HashSet};
use cedar_policy::Entity;
use super::Resource;
use crate::{authorization::{MemberPermissionError, policy::EntityType}, organization::Group};

impl TryFrom<&Group> for Resource {
    type Error = MemberPermissionError;
    fn try_from(group: &Group) -> Result<Self, Self::Error> {
        let mut resource = Self::new(EntityType::Group, group.organization_id, group.id, HashMap::new(), HashSet::new())?;
        resource.entities = vec![Entity::try_from(group).map_err(|_| MemberPermissionError::Evaluation)?];
        for role in &group.roles { resource.entities.extend(Self::try_from(role)?.entities); }
        Ok(resource)
    }
}
