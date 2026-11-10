use std::collections::{HashMap, HashSet};
use cedar_policy::RestrictedExpression;
use super::Resource;
use crate::{authorization::{MemberPermissionError, policy::EntityType}, model::Model};

impl TryFrom<&Model> for Resource {
    type Error = MemberPermissionError;
    fn try_from(model: &Model) -> Result<Self, Self::Error> {
        Self::new(EntityType::Model, model.organization_id, model.id,
            HashMap::from([("enabled".into(), RestrictedExpression::new_bool(model.enabled))]), HashSet::new())
    }
}
