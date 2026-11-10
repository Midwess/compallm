use std::collections::{HashMap, HashSet};
use cedar_policy::RestrictedExpression;
use super::Resource;
use crate::{authorization::{MemberPermissionError, policy::EntityType}, provider::{Provider, ProviderModel}};

impl TryFrom<&Provider> for Resource {
    type Error = MemberPermissionError;
    fn try_from(provider: &Provider) -> Result<Self, Self::Error> {
        Self::new(EntityType::Provider, provider.organization_id, provider.id, HashMap::new(), HashSet::new())
    }
}

impl TryFrom<&ProviderModel> for Resource {
    type Error = MemberPermissionError;
    fn try_from(model: &ProviderModel) -> Result<Self, Self::Error> {
        let provider = Self::new(EntityType::Provider, model.organization_id, model.provider_id, HashMap::new(), HashSet::new())?;
        let mut resource = Self::new(EntityType::ProviderModel, model.organization_id, model.id,
            HashMap::from([
                ("enabled".into(), RestrictedExpression::new_bool(model.enabled)),
                ("provider".into(), RestrictedExpression::new_entity_uid(provider.uid.clone())),
            ]), HashSet::from([provider.uid]))?;
        resource.entities.extend(provider.entities);
        Ok(resource)
    }
}
