use super::Resource;
use crate::{
    authorization::{MemberPermissionError, policy::EntityType},
    organization::Role,
};
use std::collections::{HashMap, HashSet};

impl TryFrom<&Role> for Resource {
    type Error = MemberPermissionError;
    fn try_from(role: &Role) -> Result<Self, Self::Error> {
        Self::new(
            EntityType::Role,
            role.organization_id,
            role.id,
            HashMap::new(),
            HashSet::new(),
        )
    }
}
